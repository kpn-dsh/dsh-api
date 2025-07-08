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
//! * [`get_granted_internal_streams(tenant) ->
//!     [(stream id, stream, rights)]`](DshApiClient::get_granted_internal_streams)
//! * [`get_granted_managed_streams(tenant) ->
//!     [(stream id, stream, rights)]`](DshApiClient::get_granted_managed_streams)
//! * [`get_granted_public_streams(tenant) ->
//!     [(stream id, stream, rights)]`](DshApiClient::get_granted_public_streams)
//! * [`get_internal_streams_access_rights(tenant) ->
//!     (stream id, access right)`](DshApiClient::get_internal_streams_access_rights)
//! * [`get_managed_tenant_limit(tenant, kind) ->
//!     limit value`](DshApiClient::get_managed_tenant_limit)
//! * [`get_managed_tenant_limits(tenant) ->
//!     tenant limits`](DshApiClient::get_managed_tenant_limits)
//! * [`get_public_streams_access_rights(tenant) ->
//!     (stream id, access right)`](DshApiClient::get_public_streams_access_rights)

use crate::dsh_api_client::DshApiClient;
use crate::stream::Stream;
use crate::types::error::ConversionError;
use crate::types::{
  GetTenantLimitByManagerByTenantByKindKind, LimitValue, LimitValueCertificateCount, LimitValueCertificateCountName, LimitValueConsumerRate, LimitValueConsumerRateName,
  LimitValueCpu, LimitValueCpuName, LimitValueKafkaAclGroupCount, LimitValueKafkaAclGroupCountName, LimitValueMem, LimitValueMemName, LimitValuePartitionCount,
  LimitValuePartitionCountName, LimitValueProducerRate, LimitValueProducerRateName, LimitValueRequestRate, LimitValueRequestRateName, LimitValueSecretCount,
  LimitValueSecretCountName, LimitValueTopicCount, LimitValueTopicCountName, ManagedStream, ManagedStreamId, PublicManagedStream,
};
use crate::{AccessRights, DshApiError, DshApiResult};
use futures::future::{try_join, try_join_all};
use itertools::Itertools;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};

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
/// * [`get_granted_internal_streams(tenant) ->
///     [(stream id, stream, rights)]`](DshApiClient::get_granted_internal_streams)
/// * [`get_granted_managed_streams(tenant) ->
///     [(stream id, stream, rights)]`](DshApiClient::get_granted_managed_streams)
/// * [`get_granted_public_streams(tenant) ->
///     [(stream id, stream, rights)]`](DshApiClient::get_granted_public_streams)
/// * [`get_internal_streams_access_rights(tenant) ->
///     (stream id, access right)`](DshApiClient::get_internal_streams_access_rights)
/// * [`get_managed_tenant_limit(tenant, kind) ->
///     limit value`](DshApiClient::get_managed_tenant_limit)
/// * [`get_managed_tenant_limits(tenant) ->
///     tenant limits`](DshApiClient::get_managed_tenant_limits)
/// * [`get_public_streams_access_rights(tenant) ->
///     (stream id, access right)`](DshApiClient::get_public_streams_access_rights)
impl DshApiClient {
  /// # Get internal managed streams that the tenant has access to
  ///
  /// # Parameters
  /// * `managed_tenant` - managed tenants id
  ///
  /// # Returns
  /// * `Ok<Vec<(ManagedStreamId, `[`ManagedStream`]`, `[`AccessRights`]`)>>` -
  ///   list of tuples consisting of stream ids, public streams and access rights
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn get_granted_internal_streams(&self, managed_tenant: &str) -> Result<Vec<(ManagedStreamId, ManagedStream, AccessRights)>, DshApiError> {
    let access_rights = self.get_internal_streams_access_rights(managed_tenant).await?;
    let streams = try_join_all(access_rights.iter().map(|(stream_id, _)| self.get_stream_internal_configuration(stream_id))).await?;
    Ok(
      access_rights
        .into_iter()
        .zip(streams)
        .map(|((stream_id, access_rights), internal_stream)| (stream_id, internal_stream, access_rights))
        .collect_vec(),
    )
  }

  /// # Get managed streams that the tenant has access to
  ///
  /// # Parameters
  /// * `managed_tenant` - managed tenants id
  ///
  /// # Returns
  /// * `Ok<Vec<(ManagedStreamId, `[`Stream`]`, `[`AccessRights`]`)>>` -
  ///   list of tuples consisting of stream ids, streams and access rights
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn get_granted_managed_streams(&self, managed_tenant: &str) -> DshApiResult<Vec<(ManagedStreamId, Stream, AccessRights)>> {
    let (internal_streams, public_streams) = try_join(self.get_granted_internal_streams(managed_tenant), self.get_granted_public_streams(managed_tenant)).await?;
    let mut internal_streams = internal_streams.into_iter().map(|(a, b, c)| (a, Stream::Internal(b), c)).collect_vec();
    let mut public_streams = public_streams.into_iter().map(|(a, b, c)| (a, Stream::Public(b), c)).collect_vec();
    internal_streams.append(&mut public_streams);
    internal_streams.sort_by(|(stream_id_a, _, _), (stream_id_b, _, _)| stream_id_a.cmp(stream_id_b));
    Ok(internal_streams)
  }

  /// # Get public managed streams that the tenant has access to
  ///
  /// # Parameters
  /// * `managed_tenant` - managed tenants id
  ///
  /// # Returns
  /// * `Ok<Vec<(ManagedStreamId, `[`PublicManagedStream`]`, `[`AccessRights`]`)>>` -
  ///   list of tuples consisting of stream ids, public streams and access rights
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn get_granted_public_streams(&self, managed_tenant: &str) -> Result<Vec<(ManagedStreamId, PublicManagedStream, AccessRights)>, DshApiError> {
    let access_rights = self.get_public_streams_access_rights(managed_tenant).await?;
    let streams = try_join_all(access_rights.iter().map(|(stream_id, _)| self.get_stream_public_configuration(stream_id))).await?;
    Ok(
      access_rights
        .into_iter()
        .zip(streams)
        .map(|((stream_id, access_rights), public_stream)| (stream_id, public_stream, access_rights))
        .collect_vec(),
    )
  }

  /// # Get ids of internal managed streams that the tenant has access to
  ///
  /// # Parameters
  /// * `managed_tenant` - managed tenants id
  ///
  /// # Returns
  /// * `Ok<Vec<(ManagedStreamId, `[`AccessRights`]`)>>` -
  ///   list of tuples consisting of stream ids and access rights
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn get_internal_streams_access_rights(&self, managed_tenant: &str) -> Result<Vec<(ManagedStreamId, AccessRights)>, DshApiError> {
    let internal_stream_ids = self.get_stream_internals().await?;
    let internal_access = try_join_all(internal_stream_ids.iter().map(|stream_id| {
      try_join(
        self.has_internal_read_access(stream_id, managed_tenant),
        self.has_internal_write_access(stream_id, managed_tenant),
      )
    }))
    .await?;
    let mut internal_access_rights: Vec<(ManagedStreamId, AccessRights)> = internal_stream_ids
      .into_iter()
      .zip(internal_access)
      .filter_map(|(stream_id, read_write)| match read_write {
        (false, false) => None,
        (false, true) => Some((stream_id, AccessRights::Write)),
        (true, false) => Some((stream_id, AccessRights::Read)),
        (true, true) => Some((stream_id, AccessRights::ReadWrite)),
      })
      .collect_vec();
    internal_access_rights.sort_by(|(stream_id_a, _), (stream_id_b, _)| stream_id_a.cmp(stream_id_b));
    Ok(internal_access_rights)
  }

  /// # Get managed tenant limit
  ///
  /// # Parameters
  /// * `managed_tenant` - managed tenants id
  /// * `kind` - represents requested limit [kind](GetTenantLimitByManagerByTenantByKindKind)
  ///
  /// # Returns
  /// * `Ok<`[`LimitValue`]`>` - limit of the managed tenant
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn get_managed_tenant_limit<T: TryInto<GetTenantLimitByManagerByTenantByKindKind>>(&self, managed_tenant: &str, kind: T) -> DshApiResult<LimitValue> {
    let kind = kind.try_into().map_err(|_| ConversionError::from("invalid limit kind"))?;
    let limit = self.get_tenant_limit(managed_tenant, kind).await?;
    match limit {
      // The code (generated by Progenitor) for the LimitValue enum is incorrect and will
      // deserialize all kind of limit value into a LimitValueCpu.
      // Explicit conversion is therefor required.
      LimitValue::Cpu(cpu) => Ok(Self::convert(&kind, cpu.value)?),
      // The patterns below will never occur with the current deserializer.
      // They are included for completeness.
      LimitValue::CertificateCount(certificate_count) => Ok(LimitValue::from(certificate_count)),
      LimitValue::ConsumerRate(consumer_rate) => Ok(LimitValue::from(consumer_rate)),
      LimitValue::KafkaAclGroupCount(kafka_acl_group_count) => Ok(LimitValue::from(kafka_acl_group_count)),
      LimitValue::Mem(mem) => Ok(LimitValue::from(mem)),
      LimitValue::PartitionCount(partition_count) => Ok(LimitValue::from(partition_count)),
      LimitValue::ProducerRate(producer_rate) => Ok(LimitValue::from(producer_rate)),
      LimitValue::RequestRate(request_rate) => Ok(LimitValue::from(request_rate)),
      LimitValue::SecretCount(secret_count) => Ok(LimitValue::from(secret_count)),
      LimitValue::TopicCount(topic_count) => Ok(LimitValue::from(topic_count)),
    }
  }

  fn convert(kind: &GetTenantLimitByManagerByTenantByKindKind, float_value: f64) -> Result<LimitValue, ConversionError> {
    match kind {
      GetTenantLimitByManagerByTenantByKindKind::Certificatecount => Ok(LimitValue::CertificateCount(LimitValueCertificateCount {
        name: LimitValueCertificateCountName::CertificateCount,
        value: float_value as i64,
      })),
      GetTenantLimitByManagerByTenantByKindKind::Consumerrate => Ok(LimitValue::ConsumerRate(LimitValueConsumerRate {
        name: LimitValueConsumerRateName::ConsumerRate,
        value: float_value as i64,
      })),
      GetTenantLimitByManagerByTenantByKindKind::Cpu => Ok(LimitValue::Cpu(LimitValueCpu { name: LimitValueCpuName::Cpu, value: float_value })),
      GetTenantLimitByManagerByTenantByKindKind::Kafkaaclgroupcount => Ok(LimitValue::KafkaAclGroupCount(LimitValueKafkaAclGroupCount {
        name: LimitValueKafkaAclGroupCountName::KafkaAclGroupCount,
        value: float_value as i64,
      })),
      GetTenantLimitByManagerByTenantByKindKind::Mem => Ok(LimitValue::Mem(LimitValueMem { name: LimitValueMemName::Mem, value: float_value as i64 })),
      GetTenantLimitByManagerByTenantByKindKind::Partitioncount => Ok(LimitValue::PartitionCount(LimitValuePartitionCount {
        name: LimitValuePartitionCountName::PartitionCount,
        value: float_value as i64,
      })),
      GetTenantLimitByManagerByTenantByKindKind::Producerrate => Ok(LimitValue::ProducerRate(LimitValueProducerRate {
        name: LimitValueProducerRateName::ProducerRate,
        value: float_value as i64,
      })),
      GetTenantLimitByManagerByTenantByKindKind::Requestrate => Ok(LimitValue::RequestRate(LimitValueRequestRate {
        name: LimitValueRequestRateName::RequestRate,
        value: float_value as i64,
      })),
      GetTenantLimitByManagerByTenantByKindKind::Secretcount => Ok(LimitValue::SecretCount(LimitValueSecretCount {
        name: LimitValueSecretCountName::SecretCount,
        value: float_value as i64,
      })),
      GetTenantLimitByManagerByTenantByKindKind::Topiccount => Ok(LimitValue::TopicCount(LimitValueTopicCount {
        name: LimitValueTopicCountName::TopicCount,
        value: float_value as i64,
      })),
    }
  }

  /// # Get managed tenant limits struct
  ///
  /// # Parameters
  /// * `managed_tenant` - managed tenants id
  ///
  /// # Returns
  /// * `Ok<`[`TenantLimits`]`>` - struct containing the limits of the managed tenant
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn get_managed_tenant_limits(&self, managed_tenant: &str) -> DshApiResult<TenantLimits> {
    Ok(TenantLimits::from(&self.get_tenant_limits(managed_tenant).await?))
  }

  /// # Get ids of public managed streams that the tenant has access to
  ///
  /// # Parameters
  /// * `managed_tenant` - managed tenants id
  ///
  /// # Returns
  /// * `Ok<Vec<(ManagedStreamId, `[`AccessRights`]`)>>` -
  ///   list of tuples consisting of stream ids and access rights
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn get_public_streams_access_rights(&self, managed_tenant: &str) -> Result<Vec<(ManagedStreamId, AccessRights)>, DshApiError> {
    let public_stream_ids = self.get_stream_publics().await?;
    let public_access = try_join_all(public_stream_ids.iter().map(|stream_id| {
      try_join(
        self.has_public_read_access(stream_id, managed_tenant),
        self.has_public_write_access(stream_id, managed_tenant),
      )
    }))
    .await?;
    let mut public_access_rights = public_stream_ids
      .into_iter()
      .zip(public_access)
      .filter_map(|(stream_id, read_write)| match read_write {
        (false, false) => None,
        (false, true) => Some((stream_id, AccessRights::Write)),
        (true, false) => Some((stream_id, AccessRights::Read)),
        (true, true) => Some((stream_id, AccessRights::ReadWrite)),
      })
      .collect_vec();
    public_access_rights.sort_by(|(stream_id_a, _), (stream_id_b, _)| stream_id_a.cmp(stream_id_b));
    Ok(public_access_rights)
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
  /// # Checks if empty
  ///
  /// # Returns
  /// * `true` - if all fields are `None`
  /// * `false` - otherwise
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

  /// # Update this struct from another struct
  ///
  /// This method copies all non-empty fields from `other` into `self`.
  /// Existing non-empty fields will be overwritten.
  ///
  /// # Parameters
  /// * `other` - `TenantLimits` struct to get the update fields from
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

// Due to the design of enums in the open api specification, the deserializer in the
// generated code returns `LimitValue::Cpu` for all types instead of the proper fields types.
// This method will handle this correctly.
// Furthermore, since the api operation "GET /manage/{manager}/tenant/{tenant}/limit"
// returns values that are not defined as key/value pairs with value 0,
// these pairs are filtered out.
impl From<&Vec<LimitValue>> for TenantLimits {
  fn from(limits: &Vec<LimitValue>) -> Self {
    let mut tenant_limits = TenantLimits::default();
    for limit in limits {
      match limit {
        LimitValue::Cpu(cpu) => match cpu.name.to_string().as_str() {
          "certificateCount" => tenant_limits.certificate_count = if cpu.value != 0.0 { Some(cpu.value as i64) } else { None },
          "consumerRate" => tenant_limits.consumer_rate = if cpu.value != 0.0 { Some(cpu.value as i64) } else { None },
          "cpu" => tenant_limits.cpu = if cpu.value != 0.0 { Some(cpu.value) } else { None },
          "kafkaAclGroupCount" => tenant_limits.kafka_acl_group_count = if cpu.value != 0.0 { Some(cpu.value as i64) } else { None },
          "mem" => tenant_limits.mem = if cpu.value != 0.0 { Some(cpu.value as i64) } else { None },
          "partitionCount" => tenant_limits.partition_count = if cpu.value != 0.0 { Some(cpu.value as i64) } else { None },
          "producerRate" => tenant_limits.producer_rate = if cpu.value != 0.0 { Some(cpu.value as i64) } else { None },
          "requestRate" => tenant_limits.request_rate = if cpu.value != 0.0 { Some(cpu.value as i64) } else { None },
          "secretCount" => tenant_limits.secret_count = if cpu.value != 0.0 { Some(cpu.value as i64) } else { None },
          "topicCount" => tenant_limits.topic_count = if cpu.value != 0.0 { Some(cpu.value as i64) } else { None },
          _ => {}
        },
        other => panic!("unexpected limit value {:?}", other),
      }
    }
    tenant_limits
  }
}

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

impl Display for TenantLimits {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut limits = vec![];
    if let Some(certificate_count) = self.certificate_count {
      limits.push(format!("certificates: {}", certificate_count));
    }
    if let Some(consumer_rate) = self.consumer_rate {
      limits.push(format!("consumer rate: {}", consumer_rate));
    }
    if let Some(cpu) = self.cpu {
      limits.push(format!("cpu: {}", cpu));
    }
    if let Some(kafka_acl_group_count) = self.kafka_acl_group_count {
      limits.push(format!("kafka acl groups: {}", kafka_acl_group_count));
    }
    if let Some(mem) = self.mem {
      limits.push(format!("mem: {}", mem));
    }
    if let Some(partition_count) = self.partition_count {
      limits.push(format!("partitions: {}", partition_count));
    }
    if let Some(producer_rate) = self.producer_rate {
      limits.push(format!("producer rate: {}", producer_rate));
    }
    if let Some(request_rate) = self.request_rate {
      limits.push(format!("request rate: {}", request_rate));
    }
    if let Some(secret_count) = self.secret_count {
      limits.push(format!("secrets: {}", secret_count));
    }
    if let Some(topic_count) = self.topic_count {
      limits.push(format!("topics: {}", topic_count));
    }
    if f.alternate() {
      write!(f, "{}", limits.join("\n"))
    } else {
      write!(f, "{}", limits.join(", "))
    }
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
}
