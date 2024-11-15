//! # Display implementations for selected types
//!
//! This module provides implementations of the [`Display`] trait for selected types.
//!
//! * [`AllocationStatus`]
//! * [`Application`]
//! * [`ApplicationSecret`]
//! * [`AppCatalogApp`]
//! * [`Bucket`]
//! * [`BucketStatus`]
//! * [`Certificate`]
//! * [`CertificateStatus`]
//! * [`Notification`]
//! * [`Task`]
//! * [`TaskStatus`]
use crate::types::{
  ActualCertificate, AllocationStatus, AppCatalogApp, Application, ApplicationSecret, Bucket, BucketStatus, Certificate, CertificateStatus, Notification, Task, TaskStatus,
};
use std::fmt::{Display, Formatter};

// AppCatalogApp
// AppCatalogAppConfiguration
// AppCatalogAppResourcesValue
// AppCatalogManifest
// ApplicationSecret
// ApplicationVolumes
// Empty
// HealthCheck
// InternalManagedStream
// KafkaProxy
// ManagedInternalStreamId
// ManagedPublicStreamId
// Metrics
// PortMapping
// PublicManagedStream
// Secret
// Topic
// TopicStatus
// Volume
// VolumeStatus

impl Display for AppCatalogApp {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "app_catalog_app(name: {}", self.name)?;
    if let Some(ref configuraton) = self.configuration {
      write!(f, ", configuration: {}", configuraton)?;
    }
    write!(f, ", manifest urn: {})", self.manifest_urn)
  }
}

impl Display for ActualCertificate {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "certificate(key: {}", self.key_secret)?;
    if let Some(ref passphrase_secret) = self.passphrase_secret {
      write!(f, ", passphrase: {}", passphrase_secret)?;
    }
    write!(f, ", cert chain: {})", self.cert_chain_secret)
  }
}

impl Display for AllocationStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.provisioned {
      write!(f, "allocation_status(provisioned")?;
    } else {
      write!(f, "allocation_status(")?;
    }
    if let Some(ref derived_from) = self.derived_from {
      write!(f, ", derived from: {}", derived_from)?;
    }
    if !self.notifications.is_empty() {
      write!(
        f,
        ", notifications: [{}]",
        self
          .notifications
          .iter()
          .map(|notification| notification.to_string())
          .collect::<Vec<_>>()
          .join(", ")
      )?;
    }
    write!(f, ")")
  }
}

impl Display for Application {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "application(instances: {}, cpus: {}, mem: {}, token: {}, single: {}, image: {})",
      self.instances, self.cpus, self.mem, self.needs_token, self.single_instance, self.image
    )
  }
}

impl Display for ApplicationSecret {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "application_secret(name: {}, injections: {})",
      self.name,
      self
        .injections
        .iter()
        .map(|injection| { format!("[{}]", injection.iter().map(|kv| { format!("{}->{}", kv.0, kv.1) }).collect::<Vec<_>>().join(", ")) })
        .collect::<Vec<_>>()
        .join("")
    )
  }
}

impl Display for Bucket {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "bucket(")?;
    match (self.encrypted, self.versioned) {
      (false, false) => (),
      (false, true) => write!(f, "versioned")?,
      (true, false) => write!(f, "encrypted")?,
      (true, true) => write!(f, "encrypted, versioned")?,
    }
    write!(f, ")")
  }
}

impl Display for BucketStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "bucket_status(status: {}", self.status)?;
    if let Some(ref actual) = self.actual {
      write!(f, ", actual: {}", actual)?;
    }
    if let Some(ref configuration) = self.configuration {
      write!(f, ", configuration: {}", configuration)?;
    }
    write!(f, ")")
  }
}

impl Display for Certificate {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "certificate(key: {}", self.key_secret)?;
    if let Some(ref passphrase_secret) = self.passphrase_secret {
      write!(f, ", passphrase: {}", passphrase_secret)?;
    }
    write!(f, ", cert chain: {})", self.cert_chain_secret)
  }
}

impl Display for CertificateStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "certificate_status(status: {}", self.status)?;
    if let Some(ref actual) = self.actual {
      write!(f, ", actual: {}", actual)?;
    }
    if let Some(ref configuration) = self.configuration {
      write!(f, ", configuration: {}", configuration)?;
    }
    write!(f, ")")
  }
}

impl Display for Notification {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "notification(remove: {}", self.remove)?;
    if !self.args.is_empty() {
      write!(
        f,
        ", args: {}",
        self.args.iter().map(|(key, value)| format!("{}->{}", key, value)).collect::<Vec<_>>().join(", ")
      )?;
    }
    write!(f, ", message: {})", self.message)
  }
}

impl Display for Task {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "task(host: {}, state: {})", self.host, self.state.to_string())
  }
}

impl Display for TaskStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "task_status(status: {}", self.status)?;
    if let Some(ref task) = self.actual {
      write!(f, ", actual: {}", task)?;
    }
    if let Some(ref configuration) = self.configuration {
      write!(f, ", configuration: {}", configuration)?;
    }
    write!(f, ")")
  }
}
