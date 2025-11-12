use dsh_api::tenant::TenantLimits;
use dsh_api::types::{
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

const LIMIT_VALUES_JSON_PARTIAL: &str = r#"[
    { "value": 0.5, "name": "cpu" },
    { "value": 5, "name": "kafkaAclGroupCount" },
    { "value": 2048, "name": "mem" }
  ]"#;

fn mock_tenant_limits_partial() -> TenantLimits {
  TenantLimits {
    certificate_count: None,
    consumer_rate: None,
    cpu: Some(0.5),
    kafka_acl_group_count: Some(5),
    mem: Some(2048),
    partition_count: None,
    producer_rate: None,
    request_rate: None,
    secret_count: None,
    topic_count: None,
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
fn test_mock_tenant_limits_partial_is_not_empty() {
  assert!(!mock_tenant_limits_partial().is_empty())
}

#[test]
fn test_update_tenant_limits() {
  let mut tenant_limits = TenantLimits::default();
  tenant_limits.update(&mock_tenant_limits());
  assert!(!tenant_limits.is_empty());
  assert_eq!(tenant_limits, mock_tenant_limits());
}

#[test]
fn test_update_tenant_limits_partial() {
  let mut tenant_limits = TenantLimits::default();
  tenant_limits.update(&mock_tenant_limits_partial());
  assert!(!tenant_limits.is_empty());
  assert_eq!(tenant_limits, mock_tenant_limits_partial());
}

#[test]
fn test_deserialize_limit_value_certificate_count() {
  const CERTIFICATE_COUNT_JSON: &str = r#"{ "name": "certificateCount", "value": 5 }"#;
  let certificate_count: LimitValueCertificateCount = serde_json::from_str::<LimitValueCertificateCount>(CERTIFICATE_COUNT_JSON).unwrap();
  assert_eq!(certificate_count.value, 5_i64);
  assert_eq!(certificate_count.name.to_string(), "certificateCount".to_string());
}

#[test]
fn test_deserialize_limit_value_consumer_rate() {
  const CONSUMER_RATE_JSON: &str = r#"{ "name": "consumerRate", "value": 1048576 }"#;
  let consumer_rate: LimitValueConsumerRate = serde_json::from_str::<LimitValueConsumerRate>(CONSUMER_RATE_JSON).unwrap();
  assert_eq!(consumer_rate.value, 1048576_i64);
  assert_eq!(consumer_rate.name.to_string(), "consumerRate".to_string());
}

#[test]
fn test_deserialize_limit_value_cpu() {
  const CPU_JSON: &str = r#"{ "name": "cpu", "value": 0.5 }"#;
  let cpu: LimitValueCpu = serde_json::from_str::<LimitValueCpu>(CPU_JSON).unwrap();
  assert_eq!(cpu.value, 0.5_f64);
  assert_eq!(cpu.name.to_string(), "cpu".to_string());
}

#[test]
fn test_deserialize_limit_value_kafka_acl_group_count() {
  const KAFKA_ACL_GROUP_COUNT_JSON: &str = r#"{ "name": "kafkaAclGroupCount", "value": 5 }"#;
  let kafka_acl_group_count: LimitValueKafkaAclGroupCount = serde_json::from_str::<LimitValueKafkaAclGroupCount>(KAFKA_ACL_GROUP_COUNT_JSON).unwrap();
  assert_eq!(kafka_acl_group_count.value, 5_i64);
  assert_eq!(kafka_acl_group_count.name.to_string(), "kafkaAclGroupCount".to_string());
}

#[test]
fn test_deserialize_limit_value_mem() {
  const MEM_JSON: &str = r#"{ "name": "mem", "value": 2048 }"#;
  let mem: LimitValueMem = serde_json::from_str::<LimitValueMem>(MEM_JSON).unwrap();
  assert_eq!(mem.value, 2048_i64);
  assert_eq!(mem.name.to_string(), "mem".to_string());
}

#[test]
fn test_deserialize_limit_value_partition_count() {
  const PARTITION_COUNT_JSON: &str = r#"{ "name": "partitionCount", "value": 5 }"#;
  let partition_count: LimitValuePartitionCount = serde_json::from_str::<LimitValuePartitionCount>(PARTITION_COUNT_JSON).unwrap();
  assert_eq!(partition_count.value, 5_i64);
  assert_eq!(partition_count.name.to_string(), "partitionCount".to_string());
}

#[test]
fn test_deserialize_limit_value_producer_rate() {
  const PRODUCER_RATE_JSON: &str = r#"{ "name": "producerRate", "value": 1048576 }"#;
  let producer_rate: LimitValueProducerRate = serde_json::from_str::<LimitValueProducerRate>(PRODUCER_RATE_JSON).unwrap();
  assert_eq!(producer_rate.value, 1048576_i64);
  assert_eq!(producer_rate.name.to_string(), "producerRate".to_string());
}

#[test]
fn test_deserialize_limit_value_request_rate() {
  const REQUEST_RATE_JSON: &str = r#"{ "name": "requestRate", "value": 5  }"#;
  let request_rate: LimitValueRequestRate = serde_json::from_str::<LimitValueRequestRate>(REQUEST_RATE_JSON).unwrap();
  assert_eq!(request_rate.value, 5_i64);
  assert_eq!(request_rate.name.to_string(), "requestRate".to_string());
}

#[test]
fn test_deserialize_limit_value_secret_count() {
  const SECRET_COUNT_JSON: &str = r#"{ "name": "secretCount", "value": 5 }"#;
  let secret_count: LimitValueSecretCount = serde_json::from_str::<LimitValueSecretCount>(SECRET_COUNT_JSON).unwrap();
  assert_eq!(secret_count.value, 5_i64);
  assert_eq!(secret_count.name.to_string(), "secretCount".to_string());
}

#[test]
fn test_deserialize_limit_value_topic_count() {
  const TOPIC_COUNT_JSON: &str = r#"{ "name": "topicCount", "value": 5 }"#;
  let topic_count: LimitValueTopicCount = serde_json::from_str::<LimitValueTopicCount>(TOPIC_COUNT_JSON).unwrap();
  assert_eq!(topic_count.value, 5_i64);
  assert_eq!(topic_count.name.to_string(), "topicCount".to_string());
}

#[test]
fn test_deserialize_vec_limit_values() {
  let deserialized_limit_values = serde_json::from_str::<Vec<LimitValue>>(LIMIT_VALUES_JSON).unwrap();
  let tenant_limits = TenantLimits::from(&deserialized_limit_values);
  assert_eq!(tenant_limits, mock_tenant_limits());
}

#[test]
fn test_deserialize_vec_limit_values_partial() {
  let deserialized_limit_values_partial = serde_json::from_str::<Vec<LimitValue>>(LIMIT_VALUES_JSON_PARTIAL).unwrap();
  let tenant_limits_partial = TenantLimits::from(&deserialized_limit_values_partial);
  assert_eq!(tenant_limits_partial, mock_tenant_limits_partial());
}

#[test]
fn test_deserialize_tenant_limits() {
  let deserialized_tenant_limits = serde_json::from_str::<TenantLimits>(LIMIT_VALUES_JSON).unwrap();
  assert_eq!(deserialized_tenant_limits, mock_tenant_limits());
}

#[test]
fn test_deserialize_tenant_limits_partial() {
  let deserialized_tenant_limits_partial = serde_json::from_str::<TenantLimits>(LIMIT_VALUES_JSON_PARTIAL).unwrap();
  assert_eq!(deserialized_tenant_limits_partial, mock_tenant_limits_partial());
}

#[test]
fn test_tenant_limits_from_limit_values() {
  let limit_values = serde_json::from_str::<Vec<LimitValue>>(LIMIT_VALUES_JSON).unwrap();
  let tenant_limits = TenantLimits::from(&limit_values);
  assert_eq!(tenant_limits, mock_tenant_limits());
}

#[test]
fn test_tenant_limits_from_limit_values_partial() {
  let limit_values_partial = serde_json::from_str::<Vec<LimitValue>>(LIMIT_VALUES_JSON_PARTIAL).unwrap();
  let tenant_limits_partial = TenantLimits::from(&limit_values_partial);
  assert_eq!(tenant_limits_partial, mock_tenant_limits_partial());
}

#[test]
fn test_tenant_limits_serde() {
  let mock = mock_tenant_limits();
  let mock_json = serde_json::to_string_pretty(&mock).unwrap();
  let deserialized_mock = serde_json::from_str::<TenantLimits>(&mock_json).unwrap();
  assert_eq!(deserialized_mock, mock_tenant_limits());
}

#[test]
fn test_tenant_limits_serde_partial() {
  let mock_partial = mock_tenant_limits_partial();
  let mock_json_partial = serde_json::to_string_pretty(&mock_partial).unwrap();
  let deserialized_mock_partial = serde_json::from_str::<TenantLimits>(&mock_json_partial).unwrap();
  assert_eq!(deserialized_mock_partial, mock_tenant_limits_partial());
}
