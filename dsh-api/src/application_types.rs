use crate::types::{Application, ApplicationSecret, ApplicationVolumes, HealthCheck, Metrics, PortMapping};
#[allow(unused_imports)]
use crate::DshApiError;
use itertools::Itertools;
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Database {}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Vhost {}

/// Represents a resource id and a list of environment variable keys
///
/// Implements `Debug`, `Eq`, `From<(&str, Vec<&str>)>`, `PartialEq`, `PartialOrd` and `Ord` traits.
/// Provides a `new` constructor associated function.
#[derive(Clone, Debug, Serialize)]
pub struct EnvVarInjection<'a> {
  /// Id of the resource referenced by the environment variables in `env_keys`
  pub id: &'a str,
  /// List of environment variable keys referencing the resource with `id`
  pub env_var_keys: Vec<&'a str>,
}

impl<'a> EnvVarInjection<'a> {
  pub fn new(id: &'a str, env_var_keys: Vec<&'a str>) -> Self {
    EnvVarInjection { id, env_var_keys }
  }
}

impl Eq for EnvVarInjection<'_> {}

impl PartialEq for EnvVarInjection<'_> {
  /// Equality check uses `id` only
  fn eq(&self, other: &Self) -> bool {
    self.id.eq(other.id)
  }
}

impl PartialOrd<Self> for EnvVarInjection<'_> {
  /// Ordering uses `id` only
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for EnvVarInjection<'_> {
  /// Ordering uses `id` only
  fn cmp(&self, other: &Self) -> Ordering {
    self.id.cmp(other.id)
  }
}

impl<'a> From<(&'a str, Vec<&'a str>)> for EnvVarInjection<'a> {
  fn from((id, env_var_keys): (&'a str, Vec<&'a str>)) -> EnvVarInjection<'a> {
    EnvVarInjection { id, env_var_keys }
  }
}

/// Represents an application with its id and a list of values used by the application
///
/// Implements `Debug`, `Eq`, `PartialEq`, `PartialOrd` and `Ord` traits.
/// Provides a `new` constructor associated function.
#[derive(Clone, Debug, Serialize)]
pub struct ApplicationValues<'a, T> {
  /// Application id
  pub id: &'a str,
  /// Reference to the application
  pub application: &'a Application,
  /// List of values associated with the application
  pub values: Vec<T>,
}

impl<'a, T> ApplicationValues<'a, T> {
  pub fn new(id: &'a str, application: &'a Application, values: Vec<T>) -> Self {
    Self { id, application, values }
  }
}

impl<T> Eq for ApplicationValues<'_, T> {}

impl<T> PartialEq for ApplicationValues<'_, T> {
  /// Equality check uses `id` only
  fn eq(&self, other: &Self) -> bool {
    self.id.eq(other.id)
  }
}

impl<T> PartialOrd<Self> for ApplicationValues<'_, T> {
  /// Ordering uses `id` only
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<T> Ord for ApplicationValues<'_, T> {
  /// Ordering uses `id` only
  fn cmp(&self, other: &Self) -> Ordering {
    self.id.cmp(other.id)
  }
}

/// Represents a value with a list of application ids and applications that use the value
///
/// Implements `Eq`, Vec<&str>)>`, `PartialEq`, `PartialOrd` and `Ord` traits.
/// Provides a `new` constructor associated function.
#[derive(Clone, Debug, Serialize)]
pub struct ValueApplications<'a, T>
where
  T: Eq + Ord,
{
  /// Value that is used in the applications
  pub value: T,
  /// Application ids and application that use the value
  pub applications: Vec<(&'a str, &'a Application)>,
}

impl<'a, T> ValueApplications<'a, T>
where
  T: Eq + Ord,
{
  pub fn new(value: T, applications: Vec<(&'a str, &'a Application)>) -> Self {
    Self { value, applications }
  }
}

impl<T> Eq for ValueApplications<'_, T> where T: Eq + Ord {}

impl<T> PartialEq for ValueApplications<'_, T>
where
  T: Eq + Ord,
{
  /// Testing for equality uses `value` only
  fn eq(&self, other: &Self) -> bool {
    self.value.eq(&other.value)
  }
}

impl<T> PartialOrd<Self> for ValueApplications<'_, T>
where
  T: Eq + Ord,
{
  /// Ordering uses `value` only
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<T> Ord for ValueApplications<'_, T>
where
  T: Eq + Ord,
{
  /// Ordering uses `value` only
  fn cmp(&self, other: &Self) -> Ordering {
    self.value.cmp(&other.value)
  }
}

/// Structure that contains the differences between two `Application`s
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationDiff {
  pub cpus: Option<(f64, f64)>,
  pub env: Option<(HashMap<String, String>, HashMap<String, String>)>,
  pub exposed_ports: Option<(HashMap<String, PortMapping>, HashMap<String, PortMapping>)>,
  pub health_check: Option<(Option<HealthCheck>, Option<HealthCheck>)>,
  pub image: Option<(String, String)>,
  pub instances: Option<(u64, u64)>,
  pub mem: Option<(u64, u64)>,
  pub metrics: Option<(Option<Metrics>, Option<Metrics>)>,
  pub needs_token: Option<(bool, bool)>,
  pub readable_streams: Option<(Vec<String>, Vec<String>)>,
  pub secrets: Option<(Vec<ApplicationSecret>, Vec<ApplicationSecret>)>,
  pub single_instance: Option<(bool, bool)>,
  pub spread_group: Option<(Option<String>, Option<String>)>,
  pub topics: Option<(Vec<String>, Vec<String>)>,
  pub user: Option<(String, String)>,
  pub volumes: Option<(HashMap<String, ApplicationVolumes>, HashMap<String, ApplicationVolumes>)>,
  pub writable_streams: Option<(Vec<String>, Vec<String>)>,
}

impl ApplicationDiff {
  /// # Compare Applications
  ///
  /// # Parameters
  /// * `baseline` - baseline application to compare against
  /// * `sample` - sample application that will be compared against the baseline
  ///
  /// # Returns
  /// * `[`ApplicationDiff`]` - struct that describes the differences between the two `[`Application`]`s
  pub fn differences_between_applications(baseline: &Application, sample: &Application) -> ApplicationDiff {
    ApplicationDiff {
      cpus: if baseline.cpus == sample.cpus { None } else { Some((baseline.cpus, sample.cpus)) },
      env: if baseline.env == sample.env { None } else { Some((baseline.env.clone(), sample.env.clone())) },
      exposed_ports: if baseline.exposed_ports == sample.exposed_ports.clone() { None } else { Some((baseline.exposed_ports.clone(), sample.exposed_ports.clone())) },
      health_check: if baseline.health_check == sample.health_check { None } else { Some((baseline.health_check.clone(), sample.health_check.clone())) },
      image: if baseline.image == sample.image.clone() { None } else { Some((baseline.image.clone(), sample.image.clone())) },
      instances: if baseline.instances == sample.instances { None } else { Some((baseline.instances, sample.instances)) },
      mem: if baseline.mem == sample.mem { None } else { Some((baseline.mem, sample.mem)) },
      metrics: if baseline.metrics == sample.metrics { None } else { Some((baseline.metrics.clone(), sample.metrics.clone())) },
      needs_token: if baseline.needs_token == sample.needs_token { None } else { Some((baseline.needs_token, sample.needs_token)) },
      readable_streams: if baseline.readable_streams == sample.readable_streams { None } else { Some((baseline.readable_streams.clone(), sample.readable_streams.clone())) },
      secrets: if baseline.secrets == sample.secrets { None } else { Some((baseline.secrets.clone(), sample.secrets.clone())) },
      single_instance: if baseline.single_instance == sample.single_instance { None } else { Some((baseline.single_instance, sample.single_instance)) },
      spread_group: if baseline.spread_group == sample.spread_group { None } else { Some((baseline.spread_group.clone(), sample.spread_group.clone())) },
      topics: if baseline.topics == sample.topics { None } else { Some((baseline.topics.clone(), sample.topics.clone())) },
      user: if baseline.user == sample.user { None } else { Some((baseline.user.clone(), sample.user.clone())) },
      volumes: if baseline.volumes == sample.volumes { None } else { Some((baseline.volumes.clone(), sample.volumes.clone())) },
      writable_streams: if baseline.writable_streams == sample.writable_streams { None } else { Some((baseline.writable_streams.clone(), sample.writable_streams.clone())) },
    }
  }

  /// # Check if there are any differences
  ///
  /// # Returns
  /// * `true` - struct does not contain any differences
  /// * `false` - struct does contain differences
  pub fn is_empty(&self) -> bool {
    self.cpus.is_none()
      && self.env.is_none()
      && self.exposed_ports.is_none()
      && self.health_check.is_none()
      && self.image.is_none()
      && self.instances.is_none()
      && self.mem.is_none()
      && self.metrics.is_none()
      && self.needs_token.is_none()
      && self.readable_streams.is_none()
      && self.secrets.is_none()
      && self.single_instance.is_none()
      && self.spread_group.is_none()
      && self.topics.is_none()
      && self.user.is_none()
      && self.volumes.is_none()
      && self.writable_streams.is_none()
  }

  /// # List the differences
  ///
  /// If there are no differences, an empty list will be returned.
  ///
  /// # Returns
  /// * `Vec<(String, String)>` - list of key/value pairs describing all differences
  pub fn differences(&self) -> Vec<(String, String)> {
    vec![
      self.env.as_ref().map(|value| ("env".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self
        .exposed_ports
        .as_ref()
        .map(|value| ("exposed ports".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self
        .health_check
        .as_ref()
        .map(|value| ("healt check".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self.image.as_ref().map(|value| ("image".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self
        .instances
        .map(|value| ("number of instances".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self.mem.map(|value| ("memory".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self.metrics.as_ref().map(|value| ("metrics".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self.needs_token.map(|value| ("needs token".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self
        .readable_streams
        .as_ref()
        .map(|value| ("readable streams".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self.secrets.as_ref().map(|value| ("secrets".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self
        .single_instance
        .map(|value| ("single instance".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self
        .spread_group
        .as_ref()
        .map(|value| ("spread group".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self.topics.as_ref().map(|value| ("topics".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self.user.as_ref().map(|value| ("user".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self.volumes.as_ref().map(|value| ("volumes".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self
        .writable_streams
        .as_ref()
        .map(|value| ("writable streams".to_string(), format!("{:?} / {:?}", value.0, value.1))),
    ]
    .iter()
    .flatten()
    .collect_vec()
    .iter()
    .map(|p| p.to_owned().to_owned())
    .collect_vec()
  }
}
