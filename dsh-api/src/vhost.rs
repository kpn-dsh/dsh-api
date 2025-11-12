//! # Additional methods to manage vhosts
//!
//! Module that contains methods and functions to manage vhosts.
//!
//! _Since the DSH resource management API does not support vhosts, there are no generated methods
//! to manage them. All derived methods act only on vhosts that are configured in either
//! applications or app resources._
//!
//! # Generated methods
//!
//! Not supported by the DSH resource management API.
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`vhosts_with_dependant_applications() -> [vhost id, [(application id, instances, [injection])]]`](DshApiClient::vhosts_with_dependant_applications)
//! * [`vhosts_with_dependant_apps() -> [vhost id, [(app id, [resource])]]`](DshApiClient::vhosts_with_dependant_apps)
//! * [`vhosts_with_dependants() -> [vhost id, [injection]]`](DshApiClient::vhosts_with_dependants)

use crate::app::app_resources;
use crate::application_types::ApplicationValues;
/// # Additional method to manage vhosts
///
/// Module that contains methods and functions to manage vhosts.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
///
/// # Derived methods
/// * [`list_vhosts_with_usage() -> [id, [usage]]`](DshApiClient::list_vhosts_with_usage)
use crate::dsh_api_client::DshApiClient;
use crate::parse::parse_function;
use crate::types::{AppCatalogApp, AppCatalogAppResourcesValue, Application, PortMapping, Vhost};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{Dependant, DependantApp, DependantApplication, DshApiResult};
use futures::try_join;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// # Describes an injection of a resource in an application
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum VhostInjection {
  /// Environment variable injection, where the value is the name of the environment variable.
  #[serde(rename = "env")]
  EnvVar(String),
  /// Variable function, where the values are the name of the function and the parameter.
  #[serde(rename = "variable")]
  Variable(String),
  /// Vhost injection, where the values are the exposed port and the zone
  #[serde(rename = "vhost")]
  Vhost(String, Option<String>),
}

impl Display for VhostInjection {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      VhostInjection::EnvVar(env_var) => write!(f, "{}", env_var),
      VhostInjection::Variable(variable) => write!(f, "{{ vhost('{}') }}", variable),
      VhostInjection::Vhost(port, zone) => match zone {
        Some(a_zone) => write!(f, "vhost({}:{})", port, a_zone),
        None => write!(f, "{}", port),
      },
    }
  }
}

impl DshApiClient {
  /// # Returns all vhosts with dependant applications
  ///
  /// Returns a sorted list of all vhosts together with the applications use them.
  /// Note that only vhosts that are actually referenced in the applications will be included.
  pub async fn vhosts_with_dependant_applications(&self) -> DshApiResult<Vec<(String, Vec<DependantApplication<VhostInjection>>)>> {
    let applications = self.get_application_configuration_map().await?;
    let mut vhosts_map = HashMap::<String, Vec<DependantApplication<VhostInjection>>>::new();
    for ApplicationValues { id, application, values } in vhosts_from_applications(&applications) {
      for (vhost, port, _) in values {
        let dependant_applications = vhosts_map.entry(vhost.clone()).or_default();
        dependant_applications.push(DependantApplication::new(
          id.to_string(),
          application.instances,
          vec![VhostInjection::Vhost(port.to_string(), None)],
        ));
      }
    }
    let mut vhosts: Vec<(String, Vec<DependantApplication<VhostInjection>>)> = Vec::from_iter(vhosts_map.into_iter());
    vhosts.sort_by(|(vhost_id_a, _), (vhost_id_b, _)| vhost_id_a.cmp(vhost_id_b));
    Ok(vhosts)
  }

  /// # Returns all vhosts with dependant apps
  ///
  /// Returns a sorted list of all vhosts together with the apps that use them.
  /// Note that only vhosts that are actually referenced in the apps will be included.
  pub async fn vhosts_with_dependant_apps(&self) -> DshApiResult<Vec<(String, Vec<DependantApp>)>> {
    let apps = self.get_appcatalogapp_configuration_map().await?;
    let mut vhosts_map = HashMap::<String, Vec<DependantApp>>::new();
    let mut app_ids = apps.keys().collect_vec();
    app_ids.sort();
    for app_id in app_ids {
      let app = apps.get(app_id).unwrap();
      for (vhost, injection) in vhost_resources_from_app(app) {
        let dependant_apps = vhosts_map.entry(vhost.to_string()).or_default();
        dependant_apps.push(DependantApp::new(app_id.clone(), vec![injection.to_string()]));
      }
    }
    let mut vhosts: Vec<(String, Vec<DependantApp>)> = Vec::from_iter(vhosts_map);
    vhosts.sort_by(|(vhost_id_a, _), (vhost_id_b, _)| vhost_id_a.cmp(vhost_id_b));
    Ok(vhosts)
  }

  /// # Returns all vhosts with dependant applications and apps
  ///
  /// Returns a sorted list of all vhosts together with the applications and apps that use them.
  /// Note that only vhosts that are actually referenced in the applications and apps
  /// will be included.
  pub async fn vhosts_with_dependants(&self) -> DshApiResult<Vec<(String, Vec<Dependant<VhostInjection>>)>> {
    let (application_configuration_map, appcatalogapp_configuration_map) = try_join!(self.get_application_configuration_map(), self.get_appcatalogapp_configuration_map())?;
    let mut vhosts_with_dependants_map = HashMap::<String, Vec<Dependant<VhostInjection>>>::new();
    for ApplicationValues { id, application, values } in vhosts_from_applications(&application_configuration_map) {
      for (vhost, port, _) in values {
        let dependants = vhosts_with_dependants_map.entry(vhost.clone()).or_default();
        dependants.push(Dependant::application(
          id.to_string(),
          application.instances,
          vec![VhostInjection::Vhost(port.to_string(), None)],
        ));
      }
    }
    let mut app_ids = appcatalogapp_configuration_map.keys().collect_vec();
    app_ids.sort();
    for app_id in app_ids {
      let app = appcatalogapp_configuration_map.get(app_id).unwrap();
      for (vhost, injection) in vhost_resources_from_app(app) {
        let dependants = vhosts_with_dependants_map.entry(vhost.to_string()).or_default();
        dependants.push(Dependant::app(app_id.clone(), vec![injection.to_string()]));
      }
    }
    let mut vhosts: Vec<(String, Vec<Dependant<VhostInjection>>)> = Vec::from_iter(vhosts_with_dependants_map.into_iter());
    vhosts.sort_by(|(vhost_id_a, _), (vhost_id_b, _)| vhost_id_a.cmp(vhost_id_b));
    Ok(vhosts)
  }
}

/// # Get application port mappings for vhost id
///
/// Get all port mappings from an `Application` that use a vhost with `vhost_id`.
/// When `vhost_id` is not used in `application`, an empty list will be returned.
///
/// # Parameters
/// * `vhost_id` - id of the vhost to look for
/// * `application` - reference to the `Application`
///
/// # Returns
/// `Vec<(&str, &PortMapping)>` - list of tuples containing:
/// * port number
/// * reference to port mapping
///
/// The list is sorted by port number.
pub fn vhost_port_mappings_from_application<'a>(vhost_id: &str, application: &'a Application) -> Vec<(&'a str, &'a PortMapping)> {
  let mut port_mappings: Vec<(&'a str, &'a PortMapping)> = application
    .exposed_ports
    .iter()
    .filter_map(|(port, port_mapping)| {
      port_mapping.vhost.clone().and_then(|vhost_string| {
        VhostString::from_str(vhost_string.as_str())
          .ok()
          .and_then(|vhost| if vhost.vhost_name == vhost_id { Some((port.as_str(), port_mapping)) } else { None })
      })
    })
    .collect_vec();
  port_mappings.sort_by(|(port_a, _), (port_b, _)| port_a.cmp(port_b));
  port_mappings
}

/// # Get applications port mappings for vhost id
///
/// Get all port mappings from multiple `Application`s that use a vhost with `vhost_id`.
/// Applications are only included if they reference `vhost_id` at least once.
///
/// # Parameters
/// * `vhost_id` - id of the vhost to look for
/// * `applications` - hashmap containing id/application pairs
///
/// # Returns
/// `Vec<ApplicationTuple<(&str, &PortMapping)>>` - list of tuples containing:
/// * application id
/// * reference to application
/// * list of pairs of port number and port mapping, sorted by port number
///
/// The list is sorted by application id.
pub fn vhost_port_mappings_from_applications<'a>(vhost_id: &str, applications: &'a HashMap<String, Application>) -> Vec<ApplicationValues<'a, (&'a str, &'a PortMapping)>> {
  let mut application_tuples: Vec<ApplicationValues<(&str, &PortMapping)>> = applications
    .iter()
    .filter_map(|(application_id, application)| {
      let port_mappings: Vec<(&str, &PortMapping)> = vhost_port_mappings_from_application(vhost_id, application);
      if port_mappings.is_empty() {
        None
      } else {
        Some(ApplicationValues::new(application_id, application, port_mappings))
      }
    })
    .collect_vec();
  application_tuples.sort();
  application_tuples
}

/// Get vhost resources from `AppCatalogApp`
///
/// # Parameters
/// * `app` - app to get the vhost resources from
///
/// # Returns
/// Either `None` when the `app` does not have any vhost resources,
/// or a `Some` that contains tuples describing the vhost resources:
/// * resource id
/// * reference to the `Vhost`
pub fn vhost_resources_from_app(app: &AppCatalogApp) -> Vec<(&str, &Vhost)> {
  app_resources(app, &|resource_value| match resource_value {
    AppCatalogAppResourcesValue::Vhost(vhost) => Some(vhost),
    _ => None,
  })
}

/// # Get vhosts from application
///
/// Get all vhosts used in `Application`.
///
/// # Parameters
/// * `application` - reference to the `Application`
///
/// # Returns
/// `Vec<(String, &PortMapping)>` - list of tuples containing:
/// * vhost id
/// * port
/// * port mapping
///
/// The list is sorted by vhost id.
pub fn vhosts_from_application(application: &Application) -> Vec<(String, &str, &PortMapping)> {
  let mut vhosts: Vec<(String, &str, &PortMapping)> = application
    .exposed_ports
    .iter()
    .filter_map(|(port, port_mapping)| {
      port_mapping.vhost.clone().and_then(|vhost_string| {
        VhostString::from_str(vhost_string.as_str())
          .ok()
          .map(|vhost| (vhost.vhost_name, port.as_str(), port_mapping))
      })
    })
    .collect_vec();
  vhosts.sort_by(|(vhost_name_a, _, _), (vhost_name_b, _, _)| vhost_name_a.cmp(vhost_name_b));
  vhosts
}

/// # Get all vhosts from applications
///
/// Get all vhosts from all `Application`s.
/// Applications without configured vhosts will be contained in the list
/// with an empty list of topics.
///
/// # Parameters
/// * `applications` - hashmap containing id/application pairs
///
/// # Returns
/// `Vec<ApplicationValues<(String, &str, &PortMapping)>>` - sorted list of tuples containing:
/// * application id
/// * application reference
/// * lists of vhost ids, ports and port mappings used in the application
pub fn vhosts_from_applications(applications: &HashMap<String, Application>) -> Vec<ApplicationValues<(String, &str, &PortMapping)>> {
  let mut vhosts: Vec<ApplicationValues<(String, &str, &PortMapping)>> = vec![];
  for (application_id, application) in applications {
    for (port, port_mapping) in &application.exposed_ports {
      if let Some(vhost_string) = port_mapping.vhost.clone() {
        if let Ok(vhost) = VhostString::from_str(vhost_string.as_str()) {
          vhosts.push(ApplicationValues::new(application_id, application, vec![(vhost.vhost_name, port, port_mapping)]));
        }
      }
    }
  }
  vhosts.sort();
  // vhosts.sort_by(|application_tuple_a, application_tuple_b| application_tuple_a.cmp(application_tuple_b));
  vhosts
}

/// Structure that describes a vhost string. Vhost strings are used in the `exposedPorts` section
/// of a service definition file and are deserialized into the `vhost` field of the
/// [`PortMapping`] data structure.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct VhostString {
  /// Domain name of the vhost
  pub vhost_name: String,
  /// Indicates whether the vhost name contains the substring `.kafka`
  pub kafka: bool,
  /// Optional tenant name
  pub tenant_name: Option<String>,
  /// Optional zone
  pub zone: Option<String>,
}

impl VhostString {
  /// # Create a `VhostString`
  ///
  /// # Parameters
  /// * `vhost_name` - mandatory identifier of the vhost
  /// * `kafka` - whether the vhost name contains the substring `.kafka`
  /// * `tenant_name` - optional tenant name
  /// * `zone` - optional zone, typically `private` or `public`
  pub fn new<T, U, V>(vhost_name: T, kafka: bool, tenant_name: Option<U>, zone: Option<V>) -> Self
  where
    T: Into<String>,
    U: Into<String>,
    V: Into<String>,
  {
    Self { vhost_name: vhost_name.into(), kafka, tenant_name: tenant_name.map(Into::<String>::into), zone: zone.map(Into::<String>::into) }
  }
}

impl FromStr for VhostString {
  type Err = String;

  /// # Parse vhost string
  ///
  /// Multiple vhosts using the `join` function are not supported.
  ///
  /// # Example
  ///
  /// ```
  /// # use std::str::FromStr;
  /// # use dsh_api::vhost::VhostString;
  /// assert_eq!(
  ///   VhostString::from_str("{ vhost('my-vhost-name') }"),
  ///   Ok(VhostString::new("my-vhost-name".to_string(), false, None::<String>, None::<String>))
  /// );
  /// assert_eq!(
  ///   VhostString::from_str("{ vhost('my-vhost-name.kafka.my-tenant','public') }"),
  ///   Ok(VhostString::new(
  ///     "my-vhost-name".to_string(),
  ///     true,
  ///     Some("my-tenant".to_string()),
  ///     Some("public".to_string())
  ///   ))
  /// );
  /// ```
  ///
  /// # Parameters
  /// * `vhost_string` - the vhost string to be parsed
  ///
  /// # Returns
  /// When the provided string is valid, the method returns an instance of the `VhostString`
  /// struct, describing the auth string.
  fn from_str(vhost_string: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref VALUE_REGEX: Regex = Regex::new(r"([a-zA-Z0-9_-]+)(\.kafka)?(?:\.([a-zA-Z0-9_-]+))?").unwrap();
    }
    let (value_string, zone) = parse_function(vhost_string, "vhost")?;
    VALUE_REGEX
      .captures(value_string)
      .map(|captures| {
        VhostString::new(
          captures.get(1).map(|vhost_match| vhost_match.as_str()).unwrap_or_default(),
          captures.get(2).is_some(),
          captures.get(3).map(|tenant_match| tenant_match.as_str()),
          zone,
        )
      })
      .ok_or(format!("invalid value in vhost string (\"{}\")", vhost_string))
  }
}

impl TryFrom<&PortMapping> for VhostString {
  type Error = String;

  fn try_from(port_mapping: &PortMapping) -> Result<Self, Self::Error> {
    match &port_mapping.vhost {
      Some(vhost) => VhostString::from_str(vhost),
      None => Err("port mapping has no vhost".to_string()),
    }
  }
}

impl Display for VhostString {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.vhost_name)?;
    if self.kafka {
      write!(f, ".kafka")?;
    }
    if let Some(tenant_name) = &self.tenant_name {
      write!(f, ".{}", tenant_name)?;
    }
    if let Some(zone) = &self.zone {
      write!(f, ".{}", zone)?;
    }
    Ok(())
  }
}
