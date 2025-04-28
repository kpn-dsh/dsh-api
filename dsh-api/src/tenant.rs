//! # Additional methods and functions to manage tenant
//!
//! _These functions are only available when the `manage` feature is enabled._
//!
//! Module that contains methods and functions to manage tenant and tenant limits.
//! * Derived methods - DshApiClient methods that add extra capabilities
//!   but depend on the API methods.
//! * Functions - Functions that add extra capabilities but do not depend directly on the API.
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`get_tenantlimits(tenant) -> tenant limits`](DshApiClient::get_tenantlimits)

// This module contains some workarounds and tests for deserializing the LimitValue enum,
// since this required a patch to the open api specification.

use crate::dsh_api_client::DshApiClient;
use crate::types::{
  LimitValue, LimitValueCertificateCount, LimitValueCertificateCountName, LimitValueConsumerRate, LimitValueConsumerRateName, LimitValueCpu, LimitValueCpuName,
  LimitValueKafkaAclGroupCount, LimitValueKafkaAclGroupCountName, LimitValueMem, LimitValueMemName, LimitValuePartitionCount, LimitValuePartitionCountName, LimitValueProducerRate,
  LimitValueProducerRateName, LimitValueRequestRate, LimitValueRequestRateName, LimitValueSecretCount, LimitValueSecretCountName, LimitValueTopicCount, LimitValueTopicCountName,
};
use crate::DshApiResult;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// # Additional methods and functions to manage tenants
///
/// Module that contains methods and functions to manage tenant and tenant limits.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
/// * Functions - Functions that add extra capabilities but do not depend directly on the API.
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`get_tenantlimits(tenant) -> tenant limits`](DshApiClient::get_tenantlimits)
impl DshApiClient {
  /// # Get managed tenant limits struct
  ///
  /// # Parameters
  /// * `managed_tenant` - managed tenants id
  ///
  /// # Returns
  /// * `Ok<`[`TenantLimits`]`>` - struct containing the limits of the managed tenant
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn get_tenantlimits(&self, managed_tenant: &str) -> DshApiResult<TenantLimits> {
    Ok(TenantLimits::from(&self.get_tenant_limits(managed_tenant).await?))
  }
}

/// Structure that describes the resource limits for a managed tenant
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TenantLimits {
  /// Limit for the number of certificates available for the managed tenant
  pub certificate_count: Option<i64>,
  /// Limit for the maximum allowed consumer rate (bytes/sec)
  pub consumer_rate: Option<i64>,
  /// Limit for the number of cpus to provision for the managed tenant (factions of a vCPU core, 1.0 equals 1 vCPU)
  pub cpu: Option<f64>,
  /// Limit for the number of Kafka ACL groups available for the managed tenant
  pub kafka_acl_group_count: Option<i64>,
  /// Limit for the amount of memory available for the managed tenant (MiB)
  pub mem: Option<i64>,
  /// Limit for the number of partitions available for the managed tenant
  pub partition_count: Option<i64>,
  /// Limit for the maximum allowed producer rate (bytes/sec)
  pub producer_rate: Option<i64>,
  /// Limit for the maximum allowed request rate (%)
  pub request_rate: Option<i64>,
  /// Limit for the number of secrets available for the managed tenant
  pub secret_count: Option<i64>,
  /// Limit for the number of topics available for the managed tenant
  pub topic_count: Option<i64>,
}

// Serializer for `TenantLimits` first converts to a `Vec<LimitValue>` and then serializes.
impl Serialize for TenantLimits {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    <&TenantLimits as Into<Vec<LimitValue>>>::into(self).serialize(serializer)
  }
}

// Deserializer for `TenantLimits` first deserializes a `Vec<LimitValue>` and then converts.
impl<'de> Deserialize<'de> for TenantLimits {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    Ok(TenantLimits::from(&Vec::deserialize(d)?))
  }
}

impl TenantLimits {
  pub fn is_empty(&self) -> bool {
    self.certificate_count.is_none()
      && self.consumer_rate.is_none()
      && self.cpu.is_none()
      && self.kafka_acl_group_count.is_none()
      && self.mem.is_none()
      && self.partition_count.is_none()
      && self.producer_rate.is_none()
      && self.request_rate.is_none()
      && self.secret_count.is_none()
      && self.topic_count.is_none()
  }

  pub fn update(&mut self, other: &TenantLimits) {
    if let Some(count) = other.certificate_count {
      self.certificate_count = Some(count)
    }
    if let Some(rate) = other.consumer_rate {
      self.consumer_rate = Some(rate)
    }
    if let Some(cpu) = other.cpu {
      self.cpu = Some(cpu)
    }
    if let Some(count) = other.kafka_acl_group_count {
      self.kafka_acl_group_count = Some(count)
    }
    if let Some(mem) = other.mem {
      self.mem = Some(mem)
    }
    if let Some(count) = other.partition_count {
      self.partition_count = Some(count)
    }
    if let Some(rate) = other.producer_rate {
      self.producer_rate = Some(rate)
    }
    if let Some(rate) = other.request_rate {
      self.request_rate = Some(rate)
    }
    if let Some(count) = other.secret_count {
      self.secret_count = Some(count)
    }
    if let Some(count) = other.topic_count {
      self.topic_count = Some(count)
    }
  }
}

// TODO Replace by the correct version
// Due to a bug in the open api specification, the deserializer in the generated code returns
// `LimitValue::Cpu` for all types. This method will handle this situation.
// Once the open api is corrected, this implementation should be replaced
// by the correct version below.
impl From<&Vec<LimitValue>> for TenantLimits {
  fn from(limits: &Vec<LimitValue>) -> Self {
    let mut tenant_limits = TenantLimits::default();
    for limit in limits {
      match limit {
        LimitValue::Cpu(cpu) => match cpu.name.to_string().as_str() {
          "certificateCount" => tenant_limits.certificate_count = Some(cpu.value as i64),
          "consumerRate" => tenant_limits.consumer_rate = Some(cpu.value as i64),
          "cpu" => tenant_limits.cpu = Some(cpu.value),
          "kafkaAclGroupCount" => tenant_limits.kafka_acl_group_count = Some(cpu.value as i64),
          "mem" => tenant_limits.mem = Some(cpu.value as i64),
          "partitionCount" => tenant_limits.partition_count = Some(cpu.value as i64),
          "producerRate" => tenant_limits.producer_rate = Some(cpu.value as i64),
          "requestRate" => tenant_limits.request_rate = Some(cpu.value as i64),
          "secretCount" => tenant_limits.secret_count = Some(cpu.value as i64),
          "topicCount" => tenant_limits.topic_count = Some(cpu.value as i64),
          _ => {}
        },
        other => panic!("unexpected limit value {:?}", other),
      }
    }
    tenant_limits
  }
}

// Correct version
// impl From<&Vec<LimitValue>> for TenantLimits {
//   fn from(limits: &Vec<LimitValue>) -> Self {
//     let mut tenant_limits = TenantLimits::default();
//     for limit in limits {
//       match limit {
//         LimitValue::Cpu(cpu) => tenant_limits.cpu = Some(cpu.value),
//         LimitValue::CertificateCount(certificate_count) => tenant_limits.certificate_count = Some(certificate_count.value as u64),
//         LimitValue::ConsumerRate(consumer_rate) => tenant_limits.consumer_rate = Some(consumer_rate.value as u64),
//         LimitValue::KafkaAclGroupCount(kafka_acl_group_count) => tenant_limits.kafka_acl_group_count = Some(kafka_acl_group_count.value as u64),
//         LimitValue::Mem(mem) => tenant_limits.mem = Some(mem.value as u64),
//         LimitValue::PartitionCount(partition_count) => tenant_limits.partition_count = Some(partition_count.value as u64),
//         LimitValue::ProducerRate(producer_rate) => tenant_limits.producer_rate = Some(producer_rate.value as u64),
//         LimitValue::RequestRate(request_rate) => tenant_limits.request_rate = Some(request_rate.value as u64),
//         LimitValue::SecretCount(secret_count) => tenant_limits.secret_count = Some(secret_count.value as u64),
//         LimitValue::TopicCount(topic_count) => tenant_limits.topic_count = Some(topic_count.value as u64),
//       }
//     }
//     tenant_limits
//   }
// }

impl From<&TenantLimits> for Vec<LimitValue> {
  fn from(value: &TenantLimits) -> Self {
    let mut limit_values = vec![];
    if let Some(certificate_count) = value.certificate_count {
      limit_values.push(LimitValue::CertificateCount(LimitValueCertificateCount {
        name: LimitValueCertificateCountName::CertificateCount,
        value: certificate_count,
      }))
    }
    if let Some(consumer_rate) = value.consumer_rate {
      limit_values.push(LimitValue::ConsumerRate(LimitValueConsumerRate {
        name: LimitValueConsumerRateName::ConsumerRate,
        value: consumer_rate,
      }))
    }
    if let Some(cpu) = value.cpu {
      limit_values.push(LimitValue::Cpu(LimitValueCpu { name: LimitValueCpuName::Cpu, value: cpu }))
    }
    if let Some(kafka_acl_group_count) = value.kafka_acl_group_count {
      limit_values.push(LimitValue::KafkaAclGroupCount(LimitValueKafkaAclGroupCount {
        name: LimitValueKafkaAclGroupCountName::KafkaAclGroupCount,
        value: kafka_acl_group_count,
      }))
    }
    if let Some(mem) = value.mem {
      limit_values.push(LimitValue::Mem(LimitValueMem { name: LimitValueMemName::Mem, value: mem }))
    }
    if let Some(partition_count) = value.partition_count {
      limit_values.push(LimitValue::PartitionCount(LimitValuePartitionCount {
        name: LimitValuePartitionCountName::PartitionCount,
        value: partition_count,
      }))
    }
    if let Some(producer_rate) = value.producer_rate {
      limit_values.push(LimitValue::ProducerRate(LimitValueProducerRate {
        name: LimitValueProducerRateName::ProducerRate,
        value: producer_rate,
      }))
    }
    if let Some(request_rate) = value.request_rate {
      limit_values.push(LimitValue::RequestRate(LimitValueRequestRate {
        name: LimitValueRequestRateName::RequestRate,
        value: request_rate,
      }))
    }
    if let Some(secret_count) = value.secret_count {
      limit_values.push(LimitValue::SecretCount(LimitValueSecretCount {
        name: LimitValueSecretCountName::SecretCount,
        value: secret_count,
      }))
    }
    if let Some(topic_count) = value.topic_count {
      limit_values.push(LimitValue::TopicCount(LimitValueTopicCount {
        name: LimitValueTopicCountName::TopicCount,
        value: topic_count,
      }))
    }
    limit_values
  }
}

#[cfg(test)]
mod test {
  use crate::tenant::TenantLimits;
  use crate::types::{
    LimitValue, LimitValueCertificateCount, LimitValueConsumerRate, LimitValueCpu, LimitValueKafkaAclGroupCount, LimitValueMem, LimitValuePartitionCount, LimitValueProducerRate,
    LimitValueRequestRate, LimitValueSecretCount, LimitValueTopicCount,
  };

  const LIMIT_VALUES_JSON: &str = r#"[
    { "value": 5, "name": "certificateCount" },
    { "value": 1048576, "name": "consumerRate" },
    { "value": 0.5, "name": "cpu" },
    { "value": 5, "name": "kafkaAclGroupCount" },
    { "value": 2048, "name": "mem" },
    { "value": 5, "name": "partitionCount" },
    { "value": 1048576, "name": "producerRate" },
    { "value": 5, "name": "requestRate" },
    { "value": 5, "name": "secretCount" },
    { "value": 5, "name": "topicCount" }
  ]"#;

  fn mock_tenant_limits() -> TenantLimits {
    TenantLimits {
      certificate_count: Some(5),
      consumer_rate: Some(1048576),
      cpu: Some(0.5),
      kafka_acl_group_count: Some(5),
      mem: Some(2048),
      partition_count: Some(5),
      producer_rate: Some(1048576),
      request_rate: Some(5),
      secret_count: Some(5),
      topic_count: Some(5),
    }
  }

  #[test]
  fn test_default_tenant_limits_is_empty() {
    assert!(TenantLimits::default().is_empty())
  }

  #[test]
  fn test_mock_tenant_limits_is_not_empty() {
    assert!(!mock_tenant_limits().is_empty())
  }

  #[test]
  fn test_update_tenant_limits() {
    let mut tenant_limits = TenantLimits::default();
    tenant_limits.update(&mock_tenant_limits());
    assert!(!tenant_limits.is_empty());
    assert_eq!(tenant_limits, mock_tenant_limits());
  }

  #[test]
  fn test_parse_limit_value_certificate_count() {
    const CERTIFICATE_COUNT_JSON: &str = r#"{ "name": "certificateCount", "value": 5 }"#;
    let certificate_count: LimitValueCertificateCount = serde_json::from_str::<LimitValueCertificateCount>(CERTIFICATE_COUNT_JSON).unwrap();
    assert_eq!(certificate_count.value, 5_i64);
    assert_eq!(certificate_count.name.to_string(), "certificateCount".to_string());
  }

  #[test]
  fn test_parse_limit_value_consumer_rate() {
    const CONSUMER_RATE_JSON: &str = r#"{ "name": "consumerRate", "value": 1048576 }"#;
    let consumer_rate: LimitValueConsumerRate = serde_json::from_str::<LimitValueConsumerRate>(CONSUMER_RATE_JSON).unwrap();
    assert_eq!(consumer_rate.value, 1048576_i64);
    assert_eq!(consumer_rate.name.to_string(), "consumerRate".to_string());
  }

  #[test]
  fn test_parse_limit_value_cpu() {
    const CPU_JSON: &str = r#"{ "name": "cpu", "value": 0.5 }"#;
    let cpu: LimitValueCpu = serde_json::from_str::<LimitValueCpu>(CPU_JSON).unwrap();
    assert_eq!(cpu.value, 0.5_f64);
    assert_eq!(cpu.name.to_string(), "cpu".to_string());
  }

  #[test]
  fn test_parse_limit_value_kafka_acl_group_count() {
    const KAFKA_ACL_GROUP_COUNT_JSON: &str = r#"{ "name": "kafkaAclGroupCount", "value": 5 }"#;
    let kafka_acl_group_count: LimitValueKafkaAclGroupCount = serde_json::from_str::<LimitValueKafkaAclGroupCount>(KAFKA_ACL_GROUP_COUNT_JSON).unwrap();
    assert_eq!(kafka_acl_group_count.value, 5_i64);
    assert_eq!(kafka_acl_group_count.name.to_string(), "kafkaAclGroupCount".to_string());
  }

  #[test]
  fn test_parse_limit_value_mem() {
    const MEM_JSON: &str = r#"{ "name": "mem", "value": 2048 }"#;
    let mem: LimitValueMem = serde_json::from_str::<LimitValueMem>(MEM_JSON).unwrap();
    assert_eq!(mem.value, 2048_i64);
    assert_eq!(mem.name.to_string(), "mem".to_string());
  }

  #[test]
  fn test_parse_limit_value_partition_count() {
    const PARTITION_COUNT_JSON: &str = r#"{ "name": "partitionCount", "value": 5 }"#;
    let partition_count: LimitValuePartitionCount = serde_json::from_str::<LimitValuePartitionCount>(PARTITION_COUNT_JSON).unwrap();
    assert_eq!(partition_count.value, 5_i64);
    assert_eq!(partition_count.name.to_string(), "partitionCount".to_string());
  }

  #[test]
  fn test_parse_limit_value_producer_rate() {
    const PRODUCER_RATE_JSON: &str = r#"{ "name": "producerRate", "value": 1048576 }"#;
    let producer_rate: LimitValueProducerRate = serde_json::from_str::<LimitValueProducerRate>(PRODUCER_RATE_JSON).unwrap();
    assert_eq!(producer_rate.value, 1048576_i64);
    assert_eq!(producer_rate.name.to_string(), "producerRate".to_string());
  }

  #[test]
  fn test_parse_limit_value_request_rate() {
    const REQUEST_RATE_JSON: &str = r#"{ "name": "requestRate", "value": 5  }"#;
    let request_rate: LimitValueRequestRate = serde_json::from_str::<LimitValueRequestRate>(REQUEST_RATE_JSON).unwrap();
    assert_eq!(request_rate.value, 5_i64);
    assert_eq!(request_rate.name.to_string(), "requestRate".to_string());
  }

  #[test]
  fn test_parse_limit_value_secret_count() {
    const SECRET_COUNT_JSON: &str = r#"{ "name": "secretCount", "value": 5 }"#;
    let secret_count: LimitValueSecretCount = serde_json::from_str::<LimitValueSecretCount>(SECRET_COUNT_JSON).unwrap();
    assert_eq!(secret_count.value, 5_i64);
    assert_eq!(secret_count.name.to_string(), "secretCount".to_string());
  }

  #[test]
  fn test_parse_limit_value_topic_count() {
    const TOPIC_COUNT_JSON: &str = r#"{ "name": "topicCount", "value": 5 }"#;
    let topic_count: LimitValueTopicCount = serde_json::from_str::<LimitValueTopicCount>(TOPIC_COUNT_JSON).unwrap();
    assert_eq!(topic_count.value, 5_i64);
    assert_eq!(topic_count.name.to_string(), "topicCount".to_string());
  }

  #[test]
  fn test_parse_vec_limit_values() {
    let deserialized_limit_values = serde_json::from_str::<Vec<LimitValue>>(LIMIT_VALUES_JSON).unwrap();
    let tenant_limits = TenantLimits::from(&deserialized_limit_values);
    assert_eq!(tenant_limits, mock_tenant_limits());
  }

  #[test]
  fn test_parse_tenant_limits() {
    let deserialized_tenant_limits = serde_json::from_str::<TenantLimits>(LIMIT_VALUES_JSON).unwrap();
    assert_eq!(deserialized_tenant_limits, mock_tenant_limits());
  }

  #[test]
  fn test_tenant_limits_from_limit_values() {
    let limit_values = serde_json::from_str::<Vec<LimitValue>>(LIMIT_VALUES_JSON).unwrap();
    let tenant_limits = TenantLimits::from(&limit_values);
    assert_eq!(tenant_limits, mock_tenant_limits());
  }

  #[test]
  fn test_tenant_limits_serde() {
    let mock = mock_tenant_limits();
    let mock_json = serde_json::to_string_pretty(&mock).unwrap();
    let deserialized_mock = serde_json::from_str::<TenantLimits>(&mock_json).unwrap();
    assert_eq!(deserialized_mock, mock_tenant_limits());
  }
}
