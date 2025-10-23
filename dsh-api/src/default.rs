//! # `Default` implementations for selected types
//!
//! This module provides implementations of the [`Default`] trait for selected types.
//!
//! * [`ActualCertificate`](ActualCertificate::default())
//! * [`AllocationStatus`](AllocationStatus::default())
//! * [`AppCatalogApp`](AppCatalogApp::default())
//! * [`AppCatalogAppConfiguration`](AppCatalogAppConfiguration::default())
//! * [`AppCatalogManifest`](AppCatalogManifest::default())
//! * [`Application`](Application::default())
//! * [`ApplicationSecret`](ApplicationSecret::default())
//! * [`ApplicationVolumes`](ApplicationVolumes::default())
//! * [`Bucket`](Bucket::default())
//! * [`BucketStatus`](BucketStatus::default())
//! * [`Certificate`](Certificate::default())
//! * [`CertificateStatus`](CertificateStatus::default())
//! * [`Empty`](Empty::default())
//! * [`HealthCheck`](HealthCheck::default())
//! * [`LimitValueCertificateCount`](LimitValueCertificateCount::default())
//! * [`LimitValueConsumerRate`](LimitValueConsumerRate::default())
//! * [`LimitValueCpu`](LimitValueCpu::default())
//! * [`LimitValueKafkaAclGroupCount`](LimitValueKafkaAclGroupCount::default())
//! * [`LimitValueMem`](LimitValueMem::default())
//! * [`LimitValuePartitionCount`](LimitValuePartitionCount::default())
//! * [`LimitValueProducerRate`](LimitValueProducerRate::default())
//! * [`LimitValueRequestRate`](LimitValueRequestRate::default())
//! * [`LimitValueSecretCount`](LimitValueSecretCount::default())
//! * [`LimitValueTopicCount`](LimitValueTopicCount::default())
//! * [`ManagedStream`](ManagedStream::default())
//! * [`ManagedTenant`](ManagedTenant::default())
//! * [`Metrics`](Metrics::default())
//! * [`Notification`](Notification::default())
//! * [`PathSpec`](PathSpec::default())
//! * [`PortMapping`](PortMapping::default())
//! * [`PublicManagedStream`](PublicManagedStream::default())
//! * [`PublicManagedStreamContract`](PublicManagedStreamContract::default())
//! * [`Secret`](Secret::default())
//! * [`Task`](Task::default())
//! * [`TaskStatus`](TaskStatus::default())
//! * [`Topic`](Topic::default())
//! * [`TopicStatus`](TopicStatus::default())
//! * [`Vhost`](Vhost::default())
//! * [`Volume`](Volume::default())
//! * [`VolumeStatus`](VolumeStatus::default())

use crate::types::{
  ActualCertificate, AllocationStatus, AppCatalogApp, AppCatalogAppConfiguration, AppCatalogManifest, Application, ApplicationSecret, ApplicationVolumes, Bucket, BucketStatus,
  Certificate, CertificateStatus, Empty, HealthCheck, LimitValueCertificateCount, LimitValueCertificateCountName, LimitValueConsumerRate, LimitValueConsumerRateName,
  LimitValueCpu, LimitValueCpuName, LimitValueKafkaAclGroupCount, LimitValueKafkaAclGroupCountName, LimitValueMem, LimitValueMemName, LimitValuePartitionCount,
  LimitValuePartitionCountName, LimitValueProducerRate, LimitValueProducerRateName, LimitValueRequestRate, LimitValueRequestRateName, LimitValueSecretCount,
  LimitValueSecretCountName, LimitValueTopicCount, LimitValueTopicCountName, ManagedStream, ManagedTenant, Metrics, Notification, PathSpec, PortMapping, PublicManagedStream,
  PublicManagedStreamContract, PublicManagedStreamContractPartitioner, PublicManagedStreamKafkaDefaultPartitioner, PublicManagedStreamKafkaDefaultPartitionerKind, Secret, Task,
  TaskState, TaskStatus, Topic, TopicStatus, Vhost, Volume, VolumeStatus,
};

use std::net::Ipv4Addr;

impl Default for ActualCertificate {
  fn default() -> Self {
    Self {
      cert_chain_secret: "".to_string(),
      distinguished_name: "".to_string(),
      dns_names: vec![],
      key_secret: "".to_string(),
      not_after: Default::default(),
      not_before: Default::default(),
      passphrase_secret: None,
      serial_number: "".to_string(),
    }
  }
}

#[allow(clippy::derivable_impls)]
impl Default for AllocationStatus {
  fn default() -> Self {
    Self { derived_from: None, notifications: vec![], provisioned: false }
  }
}

impl Default for AppCatalogApp {
  fn default() -> Self {
    Self { configuration: None, manifest_urn: "".to_string(), name: "".to_string(), resources: Default::default() }
  }
}

impl Default for AppCatalogAppConfiguration {
  fn default() -> Self {
    Self { configuration: Default::default(), manifest_urn: "".to_string(), name: "".to_string(), stopped: false }
  }
}

impl Default for AppCatalogManifest {
  fn default() -> Self {
    Self { draft: false, last_modified: 0.0, payload: "".to_string() }
  }
}

impl Default for Application {
  fn default() -> Self {
    Self {
      cpus: 0.0,
      env: Default::default(),
      exposed_ports: Default::default(),
      health_check: None,
      image: "".to_string(),
      instances: 0,
      mem: 0,
      metrics: None,
      needs_token: false,
      readable_streams: vec![],
      secrets: vec![],
      single_instance: false,
      spread_group: None,
      topics: vec![],
      user: "".to_string(),
      volumes: Default::default(),
      writable_streams: vec![],
    }
  }
}

impl Default for ApplicationSecret {
  fn default() -> Self {
    Self { injections: vec![], name: "".to_string() }
  }
}

impl Default for ApplicationVolumes {
  fn default() -> Self {
    Self { name: "".to_string() }
  }
}

#[allow(clippy::derivable_impls)]
impl Default for Bucket {
  fn default() -> Self {
    Self { encrypted: false, versioned: false }
  }
}

#[allow(clippy::derivable_impls)]
impl Default for BucketStatus {
  fn default() -> Self {
    Self { actual: None, configuration: None, status: Default::default() }
  }
}

impl Default for Certificate {
  fn default() -> Self {
    Self { cert_chain_secret: "".to_string(), key_secret: "".to_string(), passphrase_secret: None }
  }
}

#[allow(clippy::derivable_impls)]
impl Default for CertificateStatus {
  fn default() -> Self {
    Self { actual: None, configuration: None, status: Default::default() }
  }
}

#[allow(clippy::derivable_impls)]
impl Default for Empty {
  fn default() -> Self {
    Self {}
  }
}

impl Default for HealthCheck {
  fn default() -> Self {
    Self { path: "".to_string(), port: 0, protocol: None }
  }
}

impl Default for LimitValueCertificateCount {
  fn default() -> Self {
    Self { name: LimitValueCertificateCountName::CertificateCount, value: 0 }
  }
}

impl Default for LimitValueConsumerRate {
  fn default() -> Self {
    Self { name: LimitValueConsumerRateName::ConsumerRate, value: 0 }
  }
}

impl Default for LimitValueCpu {
  fn default() -> Self {
    Self { name: LimitValueCpuName::Cpu, value: 0.0 }
  }
}

impl Default for LimitValueKafkaAclGroupCount {
  fn default() -> Self {
    Self { name: LimitValueKafkaAclGroupCountName::KafkaAclGroupCount, value: 0 }
  }
}

impl Default for LimitValueMem {
  fn default() -> Self {
    Self { name: LimitValueMemName::Mem, value: 0 }
  }
}

impl Default for LimitValuePartitionCount {
  fn default() -> Self {
    Self { name: LimitValuePartitionCountName::PartitionCount, value: 0 }
  }
}

impl Default for LimitValueProducerRate {
  fn default() -> Self {
    Self { name: LimitValueProducerRateName::ProducerRate, value: 0 }
  }
}

impl Default for LimitValueRequestRate {
  fn default() -> Self {
    Self { name: LimitValueRequestRateName::RequestRate, value: 0 }
  }
}

impl Default for LimitValueSecretCount {
  fn default() -> Self {
    Self { name: LimitValueSecretCountName::SecretCount, value: 0 }
  }
}

impl Default for LimitValueTopicCount {
  fn default() -> Self {
    Self { name: LimitValueTopicCountName::TopicCount, value: 0 }
  }
}

#[allow(clippy::derivable_impls)]
impl Default for ManagedStream {
  fn default() -> Self {
    Self(Default::default())
  }
}

impl Default for ManagedTenant {
  fn default() -> Self {
    Self { manager: "".to_string(), name: "".to_string(), services: vec![] }
  }
}

impl Default for Metrics {
  fn default() -> Self {
    Self { path: "".to_string(), port: 0 }
  }
}

impl Default for Notification {
  fn default() -> Self {
    Self { args: Default::default(), message: "".to_string(), remove: false }
  }
}

impl Default for PathSpec {
  fn default() -> Self {
    Self { prefix: "".to_string() }
  }
}

#[allow(clippy::derivable_impls)]
impl Default for PortMapping {
  fn default() -> Self {
    Self { auth: None, mode: None, paths: vec![], service_group: None, tls: None, vhost: None, whitelist: None }
  }
}

#[allow(clippy::derivable_impls)]
impl Default for PublicManagedStream {
  fn default() -> Self {
    Self { contract: Default::default(), kafka_properties: Default::default(), partitions: 0, replication_factor: 0 }
  }
}

impl Default for PublicManagedStreamContract {
  /// # Returns the "default value" for a `PublicManagedStreamContract`.
  ///
  /// Note that the `partitioner` field will be set to the default Kafka partitioner.
  fn default() -> Self {
    Self {
      can_be_retained: false,
      partitioner: PublicManagedStreamContractPartitioner::KafkaDefaultPartitioner(PublicManagedStreamKafkaDefaultPartitioner {
        kind: PublicManagedStreamKafkaDefaultPartitionerKind::KafkaDefault,
      }),
    }
  }
}

impl Default for Secret {
  fn default() -> Self {
    Self { name: "".to_string(), value: "".to_string() }
  }
}

impl Default for Task {
  /// # Returns the "default value" for a `Task`.
  ///
  /// Note that
  /// * the `host` field will be set to `Ipv4Addr::UNSPECIFIED`
  /// * the `state` will be set to `TaskState::Unknown`
  fn default() -> Self {
    Self {
      healthy: None,
      host: Ipv4Addr::UNSPECIFIED,
      last_update: None,
      logs: None,
      staged_at: Default::default(),
      started_at: Default::default(),
      state: TaskState::Unknown,
      stopped_at: None,
    }
  }
}

#[allow(clippy::derivable_impls)]
impl Default for TaskStatus {
  fn default() -> Self {
    Self { actual: None, configuration: None, status: Default::default() }
  }
}

#[allow(clippy::derivable_impls)]
impl Default for Topic {
  fn default() -> Self {
    Self { kafka_properties: Default::default(), partitions: 0, replication_factor: 0 }
  }
}

#[allow(clippy::derivable_impls)]
impl Default for TopicStatus {
  fn default() -> Self {
    Self { actual: None, configuration: None, status: Default::default() }
  }
}

impl Default for Vhost {
  fn default() -> Self {
    Self { value: "".to_string() }
  }
}

#[allow(clippy::derivable_impls)]
impl Default for Volume {
  fn default() -> Self {
    Self { size_gi_b: 0 }
  }
}

#[allow(clippy::derivable_impls)]
impl Default for VolumeStatus {
  fn default() -> Self {
    Self { actual: None, configuration: None, status: Default::default() }
  }
}
