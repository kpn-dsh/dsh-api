pub use crate::generated::types;
use crate::types::{AllocationStatus, Application, Bucket, BucketStatus, Notification, Task, TaskStatus};
use std::fmt::{Display, Formatter};

pub mod generated {
  include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

// AppCatalogApp
// AppCatalogAppConfiguration
// AppCatalogAppResourcesValue
// AppCatalogManifest
// ApplicationSecret
// ApplicationVolumes
// Certificate
// CertificateStatus
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

impl Display for AllocationStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.provisioned {
      write!(f, "AllocationStatus(provisioned")?;
    } else {
      write!(f, "AllocationStatus(")?;
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
      "Application(instances: {}, cpus: {}, mem: {}, token: {}, single: {}, image: {})",
      self.instances, self.cpus, self.mem, self.needs_token, self.single_instance, self.image
    )
  }
}

impl Display for Bucket {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Bucket(")?;
    match (self.encrypted, self.versioned) {
      (false, false) => write!(f, "")?,
      (false, true) => write!(f, "versioned")?,
      (true, false) => write!(f, "encrypted")?,
      (true, true) => write!(f, "encrypted, versioned")?,
    }
    write!(f, ")")
  }
}

impl Display for BucketStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "BucketStatus(status: {}", self.status)?;
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
    write!(f, "Notification(remove: {}", self.remove)?;
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
    write!(f, "Task(host: {}, state: {})", self.host, self.state.to_string())
  }
}

impl Display for TaskStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "TaskStatus(status: {}", self.status)?;
    if let Some(ref task) = self.actual {
      write!(f, ", actual: {}", task)?;
    }
    if let Some(ref configuration) = self.configuration {
      write!(f, ", configuration: {}", configuration)?;
    }
    write!(f, ")")
  }
}
