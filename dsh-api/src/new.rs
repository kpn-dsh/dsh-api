//! # New functions for selected types
//!
//! This module provides some `new` constructor/factory functions for selected types.
//!
//! * [`AllocationStatus::new(derived from, provisioned)`](AllocationStatus::new)
//! * [`AppCatalogApp::new(name, manifest urn)`](AppCatalogApp::new)
//! * [`AppCatalogAppConfiguration::new(name, manifest urn, stopped)`](AppCatalogAppConfiguration::new)
//! * [`Application::new(image, cpus, mem, instances)`](Application::new)
//! * [`ApplicationSecret::new(name, injections)`](ApplicationSecret::new)
//! * [`ApplicationVolumes::new(name)`](ApplicationVolumes::new)
//! * [`Bucket::new(encrypted, versioned)`](Bucket::new)
//! * [`Empty::new()`](Empty::new)
//! * [`HealthCheck::new(path, port)`](HealthCheck::new)
//! * [`LimitValueCertificateCount::new(certificate count)`](LimitValueCertificateCount::new)
//! * [`LimitValueConsumerRate::new(consumer rate)`](LimitValueConsumerRate::new)
//! * [`LimitValueCpu::new(cpus)`](LimitValueCpu::new)
//! * [`LimitValueKafkaAclGroupCount::new(kafka acl group count)`](LimitValueKafkaAclGroupCount::new)
//! * [`LimitValueMem::new(mem)`](LimitValueMem::new)
//! * [`LimitValuePartitionCount::new(partition count)`](LimitValuePartitionCount::new)
//! * [`LimitValueProducerRate::new(producer rate)`](LimitValueProducerRate::new)
//! * [`LimitValueRequestRate::new(request rate)`](LimitValueRequestRate::new)
//! * [`LimitValueSecretCount::new(secret count)`](LimitValueSecretCount::new)
//! * [`LimitValueTopicCount::new(topic count)`](LimitValueTopicCount::new)
//! * [`ManagedStreamId::new(manager, stream id)`](ManagedStreamId::new)
//! * [`ManagedTenant::new(manager, tenant name)`](ManagedTenant::new)
//! * [`Metrics::new(path, port)`](Metrics::new)
//! * [`Notification::new(message, remove)`](Notification::new)
//! * [`PathSpec::new(prefix)`](PathSpec::new)
//! * [`Secret::new(name, value)`](Secret::new)
//! * [`Vhost::new(value)`](Vhost::new)
//! * [`Volume::new(size)`](Volume::new)

use crate::types::{
  AllocationStatus, AppCatalogApp, AppCatalogAppConfiguration, Application, ApplicationSecret, ApplicationVolumes, Bucket, Empty, HealthCheck, HealthCheckProtocol,
  LimitValueCertificateCount, LimitValueCertificateCountName, LimitValueConsumerRate, LimitValueConsumerRateName, LimitValueCpu, LimitValueCpuName, LimitValueKafkaAclGroupCount,
  LimitValueKafkaAclGroupCountName, LimitValueMem, LimitValueMemName, LimitValuePartitionCount, LimitValuePartitionCountName, LimitValueProducerRate, LimitValueProducerRateName,
  LimitValueRequestRate, LimitValueRequestRateName, LimitValueSecretCount, LimitValueSecretCountName, LimitValueTopicCount, LimitValueTopicCountName, ManagedStreamId,
  ManagedTenant, ManagedTenantServices, ManagedTenantServicesName, Metrics, Notification, PathSpec, Secret, Vhost, Volume,
};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

impl AllocationStatus {
  /// # Create a new `AllocationStatus`
  ///
  /// Create a new `AllocationStatus` from the provided parameters.
  /// The other fields of the `AllocationStatus` instance will be set to their default values.
  ///
  /// # Parameters
  /// * `derived_from` - name of the service that the resource for this allocation status
  ///   was derived from
  /// * `provisioned` - whether the service was provisioned
  ///
  /// # Returns
  /// The created `AllocationStatus`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::AllocationStatus;
  /// let allocation_status = AllocationStatus::new("my-service", true);
  /// assert_eq!(allocation_status.derived_from.unwrap(), "my-service".to_string());
  /// assert_eq!(allocation_status.provisioned, true);
  /// assert!(allocation_status.notifications.is_empty());
  /// ```
  pub fn new<T>(derived_from: T, provisioned: bool) -> Self
  where
    T: Into<String>,
  {
    Self { derived_from: Some(derived_from.into()), provisioned, ..Self::default() }
  }
}

impl AppCatalogApp {
  /// # Create a new `AppCatalogApp`
  ///
  /// Create a new `AppCatalogApp` from the provided parameters.
  /// The other fields of the `AppCatalogApp` instance will be set to their default values.
  ///
  /// # Parameters
  /// * `name` - name of the app catalog app
  /// * `manifest_urn` -  the manifest urn of the app
  ///
  /// # Returns
  /// The created `AppCatalogApp`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::AppCatalogApp;
  /// let app_catalog_app =
  ///   AppCatalogApp::new("my-service", "appcatalog/manifest/kpn/my-container/0.0.0");
  /// assert_eq!(app_catalog_app.name, "my-service");
  /// assert_eq!(app_catalog_app.manifest_urn, "appcatalog/manifest/kpn/my-container/0.0.0");
  /// assert!(app_catalog_app.configuration.is_none());
  /// assert!(app_catalog_app.resources.is_empty());
  /// ```
  pub fn new<S, T>(name: S, manifest_urn: T) -> Self
  where
    S: Into<String>,
    T: Into<String>,
  {
    Self { name: name.into(), manifest_urn: manifest_urn.into(), ..Self::default() }
  }
}

impl AppCatalogAppConfiguration {
  /// # Create a new `AppCatalogAppConfiguration`
  ///
  /// Create a new `AppCatalogAppConfiguration` from the provided parameters.
  /// The other fields of the `AppCatalogAppConfiguration` instance will be set to their default values.
  ///
  /// # Parameters
  /// * `name` - name of the app catalog app
  /// * `manifest_urn` -  the manifest urn of the app
  ///
  /// # Returns
  /// The created `AppCatalogAppConfiguration`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::AppCatalogAppConfiguration;
  /// let app_catalog_app_configuration =
  ///   AppCatalogAppConfiguration::new("service", "appcatalog/manifest/kpn/my-container/0.0.0", false);
  /// assert_eq!(app_catalog_app_configuration.name, "service");
  /// assert_eq!(
  ///   app_catalog_app_configuration.manifest_urn,
  ///   "appcatalog/manifest/kpn/my-container/0.0.0"
  /// );
  /// assert_eq!(app_catalog_app_configuration.stopped, false);
  /// assert!(app_catalog_app_configuration.configuration.is_empty());
  /// ```
  pub fn new<S, T>(name: S, manifest_urn: T, stopped: bool) -> Self
  where
    S: Into<String>,
    T: Into<String>,
  {
    Self { name: name.into(), manifest_urn: manifest_urn.into(), stopped, ..Self::default() }
  }
}

impl Application {
  /// # Create a new `Application`
  ///
  /// Create a new `Application` from the provided parameters.
  /// The other fields of the `Application` instance will be set to their default values.
  ///
  /// # Parameters
  /// * `image` - docker image for the service
  /// * `cpus` - numbers of cpus required
  /// * `mem` - amount of memory required (in megabytes)
  /// * `instances` - number of instances that will be started
  ///
  /// # Returns
  /// The created `Application`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::Application;
  /// let application = Application::new("registry:my-container:0.0.0", 1.0, 1024, 1);
  /// assert_eq!(application.image, "registry:my-container:0.0.0");
  /// assert_eq!(application.cpus, 1.0);
  /// assert_eq!(application.mem, 1024);
  /// assert_eq!(application.instances, 1);
  /// assert!(application.env.is_empty());
  /// assert!(application.health_check.is_none());
  /// ```
  pub fn new<T>(image: T, cpus: f64, mem: u64, instances: u64) -> Self
  where
    T: Into<String>,
  {
    Self { image: image.into(), cpus, mem, instances, ..Self::default() }
  }
}

impl ApplicationSecret {
  /// # Create a new `ApplicationSecret`
  ///
  /// Create a new `ApplicationSecret` from the provided parameters.
  ///
  /// # Parameters
  /// * `name` - name of the application secret
  /// * `injections` - list of environment variable keys that will be used to inject the secret in
  ///   the configuration
  ///
  /// # Returns
  /// The created `ApplicationSecret`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::ApplicationSecret;
  /// let secret = ApplicationSecret::new("secret_name", &["KEY1", "KEY2"]);
  /// assert_eq!(secret.name, "secret_name");
  /// assert_eq!(secret.injections.len(), 2);
  /// let first_injection_map = secret.injections.first().unwrap();
  /// assert_eq!(first_injection_map.get("env").unwrap(), "KEY1");
  /// ```
  pub fn new<S, T>(name: S, injections: &[T]) -> Self
  where
    S: Into<String>,
    T: ToString,
  {
    Self {
      name: name.into(),
      injections: injections
        .iter()
        .map(|injection| HashMap::from([("env".to_string(), injection.to_string())]))
        .collect_vec(),
    }
  }
}

impl ApplicationVolumes {
  /// # Create a new `ApplicationVolumes`
  ///
  /// Create a new `ApplicationVolumes` from the provided parameter.
  ///
  /// # Parameters
  /// * `name` - name of the application volume
  ///
  /// # Returns
  /// The created `ApplicationVolumes`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::ApplicationVolumes;
  /// let application_volume = ApplicationVolumes::new("my-volume");
  /// assert_eq!(application_volume.name, "my-volume");
  /// ```
  pub fn new<T>(name: T) -> Self
  where
    T: Into<String>,
  {
    Self { name: name.into() }
  }
}

impl Bucket {
  /// # Create a new `Bucket`
  ///
  /// Create a new `Bucket` from the provided parameters.
  ///
  /// # Parameters
  /// * `encrypted` - indicates whether the volume will be encrypted
  /// * `versioned` - indicates whether the volume will be versioned
  ///
  /// # Returns
  /// The created `Bucket`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::Bucket;
  /// let bucket = Bucket::new(true, true);
  /// assert!(bucket.encrypted);
  /// assert!(bucket.versioned);
  /// ```
  pub fn new(encrypted: bool, versioned: bool) -> Self {
    Self { encrypted, versioned }
  }
}

impl Empty {
  /// # Create a new `Empty`
  ///
  /// Create a new `Empty`.
  ///
  /// # Returns
  /// The created `Empty`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::Empty;
  /// let empty = Empty::new();
  /// ```
  pub fn new() -> Self {
    Self::default()
  }
}

impl HealthCheck {
  /// # Create a new `HealthCheck`
  ///
  /// Create a new `HealthCheck` from the provided parameters.
  /// The `protocol` field of the `HealthCheck` instance will be set to `Https`.
  ///
  /// # Parameters
  /// * `path` - path for the health check
  /// * `port` - port for the health check
  ///
  /// # Returns
  /// The created `HealthCheck`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::{HealthCheck, HealthCheckProtocol};
  /// let health_check = HealthCheck::new("/my/health/check", 1234);
  /// assert_eq!(health_check.path, "/my/health/check");
  /// assert_eq!(health_check.port, 1234);
  /// assert_eq!(health_check.protocol, Some(HealthCheckProtocol::Https));
  /// ```
  pub fn new<T>(path: T, port: u64) -> Self
  where
    T: Into<String>,
  {
    Self { path: path.into(), port, protocol: Some(HealthCheckProtocol::Https) }
  }
}

impl LimitValueCertificateCount {
  /// # Create a new `LimitValueCertificateCount`
  ///
  /// Create a new `LimitValueCertificateCount` from the provided parameter.
  /// The `name` field will be set to [LimitValueCertificateCountName::CertificateCount].
  ///
  /// # Parameters
  /// * `certificate_count` - maximum number of certificates
  ///
  /// # Returns
  /// The created `LimitValueCertificateCount`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::{LimitValueCertificateCount, LimitValueCertificateCountName};
  /// let limit_value = LimitValueCertificateCount::new(10);
  /// assert_eq!(limit_value.name, LimitValueCertificateCountName::CertificateCount);
  /// assert_eq!(limit_value.value, 10);
  /// ```
  pub fn new(certificate_count: i64) -> Self {
    Self { name: LimitValueCertificateCountName::CertificateCount, value: certificate_count }
  }
}

impl LimitValueConsumerRate {
  /// # Create a new `LimitValueConsumerRate`
  ///
  /// Create a new `LimitValueConsumerRate` from the provided parameter.
  /// The `name` field will be set to [LimitValueCertificateCountName::CertificateCount].
  ///
  /// # Parameters
  /// * `consumer_rate` - maximum consumer rate
  ///
  /// # Returns
  /// The created `LimitValueConsumerRate`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::{LimitValueConsumerRate, LimitValueConsumerRateName};
  /// let limit_value = LimitValueConsumerRate::new(1048576);
  /// assert_eq!(limit_value.name, LimitValueConsumerRateName::ConsumerRate);
  /// assert_eq!(limit_value.value, 1048576);
  /// ```
  pub fn new(consumer_rate: i64) -> Self {
    Self { name: LimitValueConsumerRateName::ConsumerRate, value: consumer_rate }
  }
}

impl LimitValueCpu {
  /// # Create a new `LimitValueCpu`
  ///
  /// Create a new `LimitValueCpu` from the provided parameter.
  /// The `name` field will be set to [LimitValueCpuName::Cpu].
  ///
  /// # Parameters
  /// * `cpus` - maximum number of cpus
  ///
  /// # Returns
  /// The created `LimitValueCpu`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::{LimitValueCpu, LimitValueCpuName};
  /// let limit_value = LimitValueCpu::new(2.0);
  /// assert_eq!(limit_value.name, LimitValueCpuName::Cpu);
  /// assert_eq!(limit_value.value, 2.0);
  /// ```
  pub fn new(cpus: f64) -> Self {
    Self { name: LimitValueCpuName::Cpu, value: cpus }
  }
}

impl LimitValueKafkaAclGroupCount {
  /// # Create a new `LimitValueKafkaAclGroupCount`
  ///
  /// Create a new `LimitValueKafkaAclGroupCount` from the provided parameter.
  /// The `name` field will be set to [LimitValueKafkaAclGroupCountName::KafkaAclGroupCount].
  ///
  /// # Parameters
  /// * `kafka_acl_group_count` - maximum number of kafka acl groups
  ///
  /// # Returns
  /// The created `LimitValueKafkaAclGroupCount`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::{LimitValueKafkaAclGroupCount, LimitValueKafkaAclGroupCountName};
  /// let limit_value = LimitValueKafkaAclGroupCount::new(40);
  /// assert_eq!(limit_value.name, LimitValueKafkaAclGroupCountName::KafkaAclGroupCount);
  /// assert_eq!(limit_value.value, 40);
  /// ```
  pub fn new(kafka_acl_group_count: i64) -> Self {
    Self { name: LimitValueKafkaAclGroupCountName::KafkaAclGroupCount, value: kafka_acl_group_count }
  }
}

impl LimitValueMem {
  /// # Create a new `LimitValueMem`
  ///
  /// Create a new `LimitValueMem` from the provided parameter.
  /// The `name` field will be set to [LimitValueMemName::Mem].
  ///
  /// # Parameters
  /// * `mem` - maximum amount of memory in megabytes
  ///
  /// # Returns
  /// The created `LimitValueMem`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::{LimitValueMem, LimitValueMemName};
  /// let limit_value = LimitValueMem::new(1024);
  /// assert_eq!(limit_value.name, LimitValueMemName::Mem);
  /// assert_eq!(limit_value.value, 1024);
  /// ```
  pub fn new(mem: i64) -> Self {
    Self { name: LimitValueMemName::Mem, value: mem }
  }
}

impl LimitValuePartitionCount {
  /// # Create a new `LimitValuePartitionCount`
  ///
  /// Create a new `LimitValuePartitionCount` from the provided parameter.
  /// The `name` field will be set to [LimitValuePartitionCountName::PartitionCount].
  ///
  /// # Parameters
  /// * `partition_count` - maximum number of partitions
  ///
  /// # Returns
  /// The created `LimitValuePartitionCount`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::{LimitValuePartitionCount, LimitValuePartitionCountName};
  /// let limit_value = LimitValuePartitionCount::new(20);
  /// assert_eq!(limit_value.name, LimitValuePartitionCountName::PartitionCount);
  /// assert_eq!(limit_value.value, 20);
  /// ```
  pub fn new(partition_count: i64) -> Self {
    Self { name: LimitValuePartitionCountName::PartitionCount, value: partition_count }
  }
}

impl LimitValueProducerRate {
  /// # Create a new `LimitValueProducerRate`
  ///
  /// Create a new `LimitValueProducerRate` from the provided parameter.
  /// The `name` field will be set to [LimitValueProducerRateName::ProducerRate].
  ///
  /// # Parameters
  /// * `producer_rate` - maximum producer rate
  ///
  /// # Returns
  /// The created `LimitValueProducerRate`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::{LimitValueProducerRate, LimitValueProducerRateName};
  /// let limit_value = LimitValueProducerRate::new(1250000000);
  /// assert_eq!(limit_value.name, LimitValueProducerRateName::ProducerRate);
  /// assert_eq!(limit_value.value, 1250000000);
  /// ```
  pub fn new(producer_rate: i64) -> Self {
    Self { name: LimitValueProducerRateName::ProducerRate, value: producer_rate }
  }
}

impl LimitValueRequestRate {
  /// # Create a new `LimitValueRequestRate`
  ///
  /// Create a new `LimitValueRequestRate` from the provided parameter.
  /// The `name` field will be set to [LimitValueRequestRateName::RequestRate].
  ///
  /// # Parameters
  /// * `request_rate` - maximum request rate
  ///
  /// # Returns
  /// The created `LimitValueRequestRate`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::{LimitValueRequestRate, LimitValueRequestRateName};
  /// let limit_value = LimitValueRequestRate::new(50);
  /// assert_eq!(limit_value.name, LimitValueRequestRateName::RequestRate);
  /// assert_eq!(limit_value.value, 50);
  /// ```
  pub fn new(request_rate: i64) -> Self {
    Self { name: LimitValueRequestRateName::RequestRate, value: request_rate }
  }
}

impl LimitValueSecretCount {
  /// # Create a new `LimitValueSecretCount`
  ///
  /// Create a new `LimitValueSecretCount` from the provided parameter.
  /// The `name` field will be set to [LimitValueSecretCountName::SecretCount].
  ///
  /// # Parameters
  /// * `secret_count` - maximum number of secrets
  ///
  /// # Returns
  /// The created `LimitValueSecretCount`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::{LimitValueSecretCount, LimitValueSecretCountName};
  /// let limit_value = LimitValueSecretCount::new(20);
  /// assert_eq!(limit_value.name, LimitValueSecretCountName::SecretCount);
  /// assert_eq!(limit_value.value, 20);
  /// ```
  pub fn new(secret_count: i64) -> Self {
    Self { name: LimitValueSecretCountName::SecretCount, value: secret_count }
  }
}

impl LimitValueTopicCount {
  /// # Create a new `LimitValueTopicCount`
  ///
  /// Create a new `LimitValueTopicCount` from the provided parameter.
  /// The `name` field will be set to [LimitValueTopicCountName::TopicCount].
  ///
  /// # Parameters
  /// * `topic_count` - maximum number of topics
  ///
  /// # Returns
  /// The created `LimitValueTopicCount`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::{LimitValueTopicCount, LimitValueTopicCountName};
  /// let limit_value = LimitValueTopicCount::new(20);
  /// assert_eq!(limit_value.name, LimitValueTopicCountName::TopicCount);
  /// assert_eq!(limit_value.value, 20);
  /// ```
  pub fn new(topic_count: i64) -> Self {
    Self { name: LimitValueTopicCountName::TopicCount, value: topic_count }
  }
}

impl ManagedStreamId {
  /// # Create a new `ManagedStreamId`
  ///
  /// Create a new `ManagedStreamId` from the provided parameters.
  ///
  /// # Parameters
  /// * `manager` - name of the managing tenant,
  ///   which must validate against the regular expression<br/>
  ///   `^[a-z][a-z0-9-]{0,38}[a-z]$`
  /// * `stream_id` - name of the managed stream,
  ///   which must validate against the regular expression<br/>
  ///   `^[a-z][a-z0-9-]{1,98}[a-z0-9]$`
  ///
  /// # Returns
  /// The created `ManagedStreamId`.
  ///
  /// # Panics
  /// This function will `panic` when the parameters do not validate against the regular
  /// expressions. Use [ManagedStreamId::try_from] when you want to avoid this.
  ///
  /// # Examples
  ///
  /// ```
  /// # use dsh_api::types::ManagedStreamId;
  /// let managed_stream_id = ManagedStreamId::new("manager", "stream-id");
  /// assert_eq!(*managed_stream_id, "manager---stream-id");
  /// ```
  ///
  /// The following example will panic since underscores are not allowed in managed stream names.
  ///
  /// ```should_panic(expected = "manager---stream_id is not a valid managed stream id")
  /// # use dsh_api::types::ManagedStreamId;
  /// ManagedStreamId::new("manager", "stream_id");
  /// ```
  pub fn new<S, T>(manager: S, stream_id: T) -> Self
  where
    S: Display,
    T: Display,
  {
    let managed_stream_id = format!("{}---{}", manager, stream_id);
    Self::from_str(&managed_stream_id).unwrap_or_else(|_| panic!("{} is not a valid managed stream id", managed_stream_id))
  }
}

impl ManagedTenant {
  /// # Create a new `ManagedTenant`
  ///
  /// Create a new `ManagedTenant` from the provided parameters.
  /// The `services` field will be configured such that `Monitoring` is enabled
  /// and `Tracing` and `Vpn` are disabled.
  ///
  /// # Parameters
  /// * `manager` - name of the managing tenant
  /// * `tenant_name` - name of the managed tenant
  ///
  /// # Returns
  /// The created `ManagedTenant`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::{ManagedTenant, ManagedTenantServices, ManagedTenantServicesName};
  /// let managed_tenant = ManagedTenant::new("managing-tenant", "managed-tenant");
  /// assert_eq!(managed_tenant.manager, "managing-tenant");
  /// assert_eq!(managed_tenant.name, "managed-tenant");
  /// assert_eq!(managed_tenant.services, vec![
  ///   ManagedTenantServices { enabled: true, name: ManagedTenantServicesName::Monitoring },
  ///   ManagedTenantServices { enabled: false, name: ManagedTenantServicesName::Tracing },
  ///   ManagedTenantServices { enabled: false, name: ManagedTenantServicesName::Vpn },
  /// ]);
  /// ```
  pub fn new<S, T>(manager: S, tenant_name: T) -> Self
  where
    S: Into<String>,
    T: Into<String>,
  {
    Self {
      manager: manager.into(),
      name: tenant_name.into(),
      services: vec![
        ManagedTenantServices { enabled: true, name: ManagedTenantServicesName::Monitoring },
        ManagedTenantServices { enabled: false, name: ManagedTenantServicesName::Tracing },
        ManagedTenantServices { enabled: false, name: ManagedTenantServicesName::Vpn },
      ],
    }
  }
}

impl Metrics {
  /// # Create a new `Metrics`
  ///
  /// Create a new `Metrics` from the provided parameters.
  /// The other fields of the `Metrics` instance will be set to their default values.
  ///
  /// # Parameters
  /// * `path` - path for the metric
  /// * `port` - port for the metric
  ///
  /// # Returns
  /// The created `Metrics`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::Metrics;
  /// let metrics = Metrics::new("/my/metrics", 1234);
  /// assert_eq!(metrics.path, "/my/metrics");
  /// assert_eq!(metrics.port, 1234);
  /// ```
  pub fn new<T>(path: T, port: u64) -> Self
  where
    T: Into<String>,
  {
    Self { path: path.into(), port }
  }
}

impl Notification {
  /// # Create a new `Notification`
  ///
  /// Create a new `Notification` from the provided parameters.
  /// The other field of the `Notification` instance will be set to its default values.
  ///
  /// # Parameters
  /// * `args` - Attributes to be applied to the template.
  /// * `message` - Template for the text of the notification.
  /// * `remove` - `true` if the notification has to do with removal of the allocation,
  ///   `false` if it relates to creation/update of the resource
  ///
  /// # Returns
  /// The created `Notification`.
  ///
  /// # Example
  ///
  /// ```
  /// # use std::collections::HashMap;
  /// use dsh_api::types::Notification;
  /// let args = HashMap::<String, String>::new();
  /// let notification = Notification::new(args, "my notification", true);
  /// assert_eq!(notification.message, "my notification");
  /// assert!(notification.remove);
  /// assert!(notification.args.is_empty());
  /// ```
  pub fn new<T>(args: HashMap<String, String>, message: T, remove: bool) -> Self
  where
    T: Into<String>,
  {
    Self { args, message: message.into(), remove }
  }
}

impl PathSpec {
  /// # Create a new `PathSpec`
  ///
  /// Create a new `PathSpec` from the provided parameter.
  ///
  /// # Parameters
  /// * `prefix` - path prefix (starting with `/`, ending without `/`) that will be matched
  ///   for routing to this service
  ///
  /// # Returns
  /// The created `PathSpec`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::PathSpec;
  /// let path_spec = PathSpec::new("/my/path/spec");
  /// assert_eq!(path_spec.prefix, "/my/path/spec");
  /// ```
  pub fn new<T>(prefix: T) -> Self
  where
    T: Into<String>,
  {
    Self { prefix: prefix.into() }
  }
}

impl Secret {
  /// # Create a new `Secret`
  ///
  /// Create a new `Secret` from the provided parameters.
  ///
  /// # Parameters
  /// * `name` - name of the secret
  /// * `value` - value of the secret
  ///
  /// # Returns
  /// The created `Secret`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::Secret;
  /// let secret = Secret::new("my-secret", "le6Y])Q;!B>W");
  /// assert_eq!(secret.name, "my-secret");
  /// assert_eq!(secret.value, "le6Y])Q;!B>W");
  /// ```
  pub fn new<S, T>(name: S, value: T) -> Self
  where
    S: Into<String>,
    T: Into<String>,
  {
    Self { name: name.into(), value: value.into() }
  }
}

impl Vhost {
  /// # Create a new `Vhost`
  ///
  /// Create a new `Vhost` from the provided parameter.
  ///
  /// # Parameters
  /// * `value` - vhost identifier
  ///
  /// # Returns
  /// The created `Vhost`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::Vhost;
  /// let vhost = Vhost::new("my-vhost");
  /// assert_eq!(vhost.value, "my-vhost");
  /// ```
  pub fn new<T>(value: T) -> Self
  where
    T: Into<String>,
  {
    Self { value: value.into() }
  }
}

impl Volume {
  /// # Create a new `Volume`
  ///
  /// Create a new `Volume` from the provided parameter.
  ///
  /// # Parameters
  /// * `size_gi_b` - size of the volume in gigabytes
  ///
  /// # Returns
  /// The created `Volume`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::types::Volume;
  /// let volume = Volume::new(4);
  /// assert_eq!(volume.size_gi_b, 4);
  /// ```
  pub fn new(size_gi_b: i64) -> Self {
    Self { size_gi_b }
  }
}
