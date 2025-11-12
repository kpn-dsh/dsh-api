//! # `Display` implementations for selected types
//!
//! This module provides implementations of the [`Display`] trait for selected types.
//! Note that if these implementations might change in a future version, this will not be
//! considered a breaking change. Use these methods for documentation purposes only,
//! not for business logic.
//!
//! * [`ActualCertificate::fmt()`](ActualCertificate::fmt)
//! * [`AllocationStatus::fmt()`](AllocationStatus::fmt)
//! * [`AppCatalogApp::fmt()`](AppCatalogApp::fmt)
//! * [`AppCatalogAppConfiguration::fmt()`](AppCatalogAppConfiguration::fmt)
//! * [`AppCatalogAppResourcesValue::fmt()`](AppCatalogAppResourcesValue::fmt)
//! * [`AppCatalogManifest::fmt()`](AppCatalogManifest::fmt)
//! * [`Application::fmt()`](Application::fmt)
//! * [`ApplicationSecret::fmt()`](ApplicationSecret::fmt)
//! * [`ApplicationVolumes::fmt()`](ApplicationVolumes::fmt)
//! * [`Bucket::fmt()`](Bucket::fmt)
//! * [`BucketStatus::fmt()`](BucketStatus::fmt)
//! * [`Certificate::fmt()`](Certificate::fmt)
//! * [`CertificateStatus::fmt()`](CertificateStatus::fmt)
//! * [`Empty::fmt()`](Empty::fmt)
//! * [`HealthCheck::fmt()`](HealthCheck::fmt)
//! * [`LimitValue::fmt()`](LimitValue::fmt)
//! * [`LimitValueCertificateCount::fmt()`](LimitValueCertificateCount::fmt)
//! * [`LimitValueConsumerRate::fmt()`](LimitValueConsumerRate::fmt)
//! * [`LimitValueCpu::fmt()`](LimitValueCpu::fmt)
//! * [`LimitValueKafkaAclGroupCount::fmt()`](LimitValueKafkaAclGroupCount::fmt)
//! * [`LimitValueMem::fmt()`](LimitValueMem::fmt)
//! * [`LimitValuePartitionCount::fmt()`](LimitValuePartitionCount::fmt)
//! * [`LimitValueProducerRate::fmt()`](LimitValueProducerRate::fmt)
//! * [`LimitValueRequestRate::fmt()`](LimitValueRequestRate::fmt)
//! * [`LimitValueSecretCount::fmt()`](LimitValueSecretCount::fmt)
//! * [`LimitValueTopicCount::fmt()`](LimitValueTopicCount::fmt)
//! * [`ManagedStream::fmt()`](ManagedStream::fmt)
//! * [`ManagedStreamId::fmt()`](ManagedStreamId::fmt)
//! * [`ManagedTenant::fmt()`](ManagedTenant::fmt)
//! * [`Metrics::fmt()`](Metrics::fmt)
//! * [`Notification::fmt()`](Notification::fmt)
//! * [`PathSpec::fmt()`](PathSpec::fmt)
//! * [`PortMapping::fmt()`](PortMapping::fmt)
//! * [`PublicManagedStream::fmt()`](PublicManagedStream::fmt)
//! * [`Secret::fmt()`](Secret::fmt)
//! * [`Task::fmt()`](Task::fmt)
//! * [`TaskStatus::fmt()`](TaskStatus::fmt)
//! * [`Topic::fmt()`](Topic::fmt)
//! * [`TopicStatus::fmt()`](TopicStatus::fmt)
//! * [`Vhost::fmt()`](Vhost::fmt)
//! * [`Volume::fmt()`](Volume::fmt)
//! * [`VolumeStatus::fmt()`](VolumeStatus::fmt)

use crate::types::{
  ActualCertificate, AllocationStatus, AppCatalogApp, AppCatalogAppConfiguration, AppCatalogAppResourcesValue, AppCatalogManifest, Application, ApplicationSecret,
  ApplicationVolumes, Bucket, BucketStatus, Certificate, CertificateStatus, Empty, HealthCheck, LimitValue, LimitValueCertificateCount, LimitValueConsumerRate, LimitValueCpu,
  LimitValueKafkaAclGroupCount, LimitValueMem, LimitValuePartitionCount, LimitValueProducerRate, LimitValueRequestRate, LimitValueSecretCount, LimitValueTopicCount, ManagedStream,
  ManagedStreamId, ManagedTenant, Metrics, Notification, PathSpec, PortMapping, PublicManagedStream, Secret, Task, TaskStatus, Topic, TopicStatus, Vhost, Volume, VolumeStatus,
};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

impl Display for ActualCertificate {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "key: {}", self.key_secret)?;
    if let Some(ref passphrase_secret) = self.passphrase_secret {
      write!(f, ", passphrase: {}", passphrase_secret)?;
    }
    write!(f, ", cert chain: {}", self.cert_chain_secret)
  }
}

impl Display for AllocationStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.provisioned {
      write!(f, "provisioned")?;
    } else {
      write!(f, "not-provisioned")?;
    }
    if let Some(ref derived_from) = self.derived_from {
      write!(f, ", derived from: {}", derived_from)?;
    }
    if !self.notifications.is_empty() {
      write!(
        f,
        ", notifications: [{}]",
        self.notifications.iter().map(|notification| notification.to_string()).collect_vec().join(", ")
      )?;
    };
    Ok(())
  }
}

impl Display for AppCatalogApp {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "name: {}", self.name)?;
    if let Some(ref configuration) = self.configuration {
      write!(f, ", configuration: {}", configuration)?;
    }
    write!(f, ", manifest urn: {}", self.manifest_urn)
  }
}

impl Display for AppCatalogAppConfiguration {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "name: {}", self.name)?;
    write!(f, ", manifest urn: {}", self.manifest_urn)?;
    if self.stopped {
      write!(f, ", stopped")?;
    }
    for (key, value) in &self.configuration {
      write!(f, ", {}: {}", key, value)?;
    }
    Ok(())
  }
}

impl Display for AppCatalogAppResourcesValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Application(application) => write!(f, "application({})", application),
      Self::Bucket(bucket) => write!(f, "bucket({})", bucket),
      Self::Certificate(certificate) => write!(f, "certificate({})", certificate),
      Self::Secret(secret) => write!(f, "secret({})", secret),
      Self::Topic(topic) => write!(f, "topic({})", topic),
      Self::Vhost(vhost) => write!(f, "vhost({})", vhost),
      Self::Volume(volume) => write!(f, "volume({})", volume),
    }
  }
}

impl Display for AppCatalogManifest {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.draft {
      write!(f, "draft, {}", self.last_modified)?
    } else {
      write!(f, "{}", self.last_modified)?
    }
    write!(f, "payload: {} bytes", self.payload.len())
  }
}

impl Display for Application {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "instances: {}, cpus: {}, mem: {}, token: {}, single: {}, image: {}",
      self.instances, self.cpus, self.mem, self.needs_token, self.single_instance, self.image
    )
  }
}

impl Display for ApplicationSecret {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "name: {}, injections: {}",
      self.name,
      self
        .injections
        .iter()
        .map(|injection| {
          format!(
            "[{}]",
            injection.iter().map(|(key, value)| { format!("{}->{}", key, value) }).collect_vec().join(", ")
          )
        })
        .collect_vec()
        .join("")
    )
  }
}

impl Display for ApplicationVolumes {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}

impl Display for Bucket {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match (self.encrypted, self.versioned) {
      (false, false) => Ok(()),
      (false, true) => write!(f, "versioned"),
      (true, false) => write!(f, "encrypted"),
      (true, true) => write!(f, "encrypted, versioned"),
    }
  }
}

impl Display for BucketStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "status: {}", self.status)?;
    if let Some(ref actual) = self.actual {
      write!(f, ", actual: {}", actual)?;
    }
    if let Some(ref configuration) = self.configuration {
      write!(f, ", configuration: {}", configuration)?;
    }
    Ok(())
  }
}

impl Display for Certificate {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "key: {}", self.key_secret)?;
    if let Some(ref passphrase_secret) = self.passphrase_secret {
      write!(f, ", passphrase: {}", passphrase_secret)?;
    }
    write!(f, ", cert chain: {}", self.cert_chain_secret)
  }
}

impl Display for CertificateStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "status: {}", self.status)?;
    if let Some(ref actual) = self.actual {
      write!(f, ", actual: {}", actual)?;
    }
    if let Some(ref configuration) = self.configuration {
      write!(f, ", configuration: {}", configuration)?;
    }
    Ok(())
  }
}

impl Display for Empty {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "empty")
  }
}

impl Display for HealthCheck {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self.protocol {
      Some(protocol) => write!(f, "{}:{}{}", protocol.to_string(), self.port, self.path),
      None => write!(f, "{}{}", self.port, self.path),
    }
  }
}

impl Display for LimitValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      LimitValue::CertificateCount(count) => write!(f, "{}", count),
      LimitValue::ConsumerRate(rate) => write!(f, "{}", rate),
      LimitValue::Cpu(cpu) => write!(f, "{}", cpu),
      LimitValue::KafkaAclGroupCount(count) => write!(f, "{}", count),
      LimitValue::Mem(mem) => write!(f, "{}", mem),
      LimitValue::PartitionCount(count) => write!(f, "{}", count),
      LimitValue::ProducerRate(rate) => write!(f, "{}", rate),
      LimitValue::RequestRate(rate) => write!(f, "{}", rate),
      LimitValue::SecretCount(count) => write!(f, "{}", count),
      LimitValue::TopicCount(count) => write!(f, "{}", count),
    }
  }
}

impl Display for LimitValueCertificateCount {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "certificates: {}", self.value)
  }
}

impl Display for LimitValueConsumerRate {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "consumer rate: {}", self.value)
  }
}

impl Display for LimitValueCpu {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "cpu: {}", self.value)
  }
}

impl Display for LimitValueKafkaAclGroupCount {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "kafka acl groups: {}", self.value)
  }
}

impl Display for LimitValueMem {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "mem: {}", self.value)
  }
}

impl Display for LimitValuePartitionCount {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "partitions: {}", self.value)
  }
}

impl Display for LimitValueProducerRate {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "producer rate: {}", self.value)
  }
}

impl Display for LimitValueRequestRate {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "request rate: {}", self.value)
  }
}

impl Display for LimitValueSecretCount {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "secrets: {}", self.value)
  }
}

impl Display for LimitValueTopicCount {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "topics: {}", self.value)
  }
}

impl Display for ManagedStream {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write_topic(f, Some("internal"), self.partitions, self.replication_factor, &self.kafka_properties)?;
    Ok(())
  }
}

impl Display for ManagedStreamId {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.deref())
  }
}

impl Display for ManagedTenant {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "name: {}", self.name)?;
    let enabled_services = self
      .services
      .iter()
      .filter_map(|service| if service.enabled { Some(service.name.to_string()) } else { None })
      .collect_vec();
    if enabled_services.is_empty() {
      Ok(())
    } else {
      write!(f, " ({})", enabled_services.join(", "))
    }
  }
}

impl Display for Metrics {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", self.path, self.port)
  }
}

impl Display for Notification {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "remove: {}", self.remove)?;
    if !self.args.is_empty() {
      write!(
        f,
        ", args: {}",
        self.args.iter().map(|(key, value)| format!("{}->{}", key, value)).collect_vec().join(", ")
      )?;
    }
    write!(f, ", message: {}", self.message)
  }
}

impl Display for PathSpec {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.prefix)
  }
}

impl Display for PortMapping {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut fields = vec![];
    if let Some(ref auth) = self.auth {
      fields.push(format!("auth: {}", auth))
    }
    if let Some(ref mode) = self.mode {
      fields.push(format!("mode: {}", mode))
    }
    if let Some(ref service_group) = self.service_group {
      fields.push(format!("service group: {}", service_group))
    }
    if let Some(ref tls) = self.tls {
      fields.push(format!("port mapping: {}", tls.to_string()))
    }
    if let Some(ref vhost) = self.vhost {
      fields.push(format!("vhost: {}", vhost))
    }
    if let Some(ref whitelist) = self.whitelist {
      fields.push(format!("whitelist: {}", whitelist))
    }
    if !self.paths.is_empty() {
      fields.push(format!("paths: {}", self.paths.iter().map(|p| p.prefix.to_string()).collect_vec().join(", ")))
    }
    write!(f, "{}", fields.join(", "))
  }
}

impl Display for PublicManagedStream {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write_topic(f, Some("public"), self.partitions, self.replication_factor, &self.kafka_properties)?;
    if self.contract.can_be_retained {
      write!(f, ", retained")?;
    }
    Ok(())
  }
}

impl Display for Secret {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}

impl Display for Task {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "host: {}, state: {}", self.host, self.state.to_string())
  }
}

impl Display for TaskStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "status: ({})", self.status)?;
    if let Some(ref task) = self.actual {
      write!(f, ", actual: ({})", task)?;
    }
    if let Some(ref configuration) = self.configuration {
      write!(f, ", configuration: ({})", configuration)?;
    }
    Ok(())
  }
}

impl Display for Topic {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write_topic(f, None, self.partitions, self.replication_factor, &self.kafka_properties)
  }
}

impl Display for TopicStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "status: ({})", self.status)?;
    if let Some(ref actual) = self.actual {
      write!(f, ", actual: ({})", actual)?;
    }
    if let Some(ref configuration) = self.configuration {
      write!(f, ", configuration: ({})", configuration)?;
    }
    Ok(())
  }
}

impl Display for Vhost {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.value)
  }
}

impl Display for Volume {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "size: {} GB", self.size_gi_b)
  }
}

impl Display for VolumeStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if let Some(ref actual) = self.actual {
      write!(f, "actual({}), ", actual)?
    }
    if let Some(ref configuration) = self.configuration {
      write!(f, "configuration({}), ", configuration)?
    }
    write!(f, "{}", self.status)
  }
}

fn write_topic(f: &mut Formatter<'_>, kind: Option<&str>, partitions: i64, replication_factor: i64, kafka_properties: &HashMap<String, String>) -> std::fmt::Result {
  if let Some(kind) = kind {
    write!(f, "{} managed stream, ", kind)?;
  }
  write!(f, "partitions: {}", partitions)?;
  write!(f, ", replication factor: {}", replication_factor)?;
  for (key, value) in kafka_properties {
    write!(f, ", {}: {}", key, value)?
  }
  Ok(())
}
