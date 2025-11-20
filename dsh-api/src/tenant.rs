//! # Additional methods to manage tenants
//!
//! _These functions are only available when the `manage` feature is enabled._
//!
//! Module that contains methods and functions to manage tenants.
//!
//! # Generated methods
//!
//! [`DshApiClient`] methods that are generated from the `openapi` specification.
//!
//! * [`delete_tenant_configuration(tenant)`](DshApiClient::delete_tenant_configuration)
//! * [`get_tenant_actual(tenant) -> ManagedTenant`](DshApiClient::get_tenant_actual)
//! * [`get_tenant_configuration(tenant) -> ManagedTenant`](DshApiClient::get_tenant_configuration)
//! * [`get_tenant_ids() -> [id]`](DshApiClient::get_tenant_ids)
//! * [`get_tenant_limit(tenant, kind) -> LimitValue`](DshApiClient::get_tenant_limit)
//! * [`get_tenant_limits(tenant) -> [LimitValue]`](DshApiClient::get_tenant_limits)
//! * [`get_tenant_status(tenant) -> AllocationStatus`](DshApiClient::get_tenant_status)
//! * [`head_stream_internal_access_read(streamid, tenant)`](DshApiClient::head_stream_internal_access_read)
//! * [`head_stream_internal_access_write(streamid, tenant)`](DshApiClient::head_stream_internal_access_write)
//! * [`head_stream_public_access_read(streamid, tenant)`](DshApiClient::head_stream_public_access_read)
//! * [`head_stream_public_access_write(streamid, tenant)`](DshApiClient::head_stream_public_access_write)
//! * [`patch_tenant_limit(tenant, body<LimitValue>)`](DshApiClient::patch_tenant_limit)
//! * [`put_stream_internal_access_read(streamid, tenant)`](DshApiClient::put_stream_internal_access_read)
//! * [`put_stream_internal_access_write(streamid, tenant)`](DshApiClient::put_stream_internal_access_write)
//! * [`put_stream_public_access_read(streamid, tenant)`](DshApiClient::put_stream_public_access_read)
//! * [`put_stream_public_access_write(streamid, tenant)`](DshApiClient::put_stream_public_access_write)
//! * [`put_tenant_configuration(tenant, body)`](DshApiClient::put_tenant_configuration)
//! * [`put_tenant_limit(tenant, kind, body)`](DshApiClient::put_tenant_limit)
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`managed_tenant_granted_internal_streams(tenant name) -> [(stream id, stream, rights)]`](DshApiClient::managed_tenant_granted_internal_streams)
//! * [`managed_tenant_granted_managed_streams(tenant name) -> [(stream id, stream, rights)]`](DshApiClient::managed_tenant_granted_managed_streams)
//! * [`managed_tenant_granted_public_streams(tenant name) -> [(stream id, stream, rights)]`](DshApiClient::managed_tenant_granted_public_streams)
//! * [`managed_tenant_has_internal_read_access(tenant name, stream id) -> bool`](DshApiClient::managed_tenant_has_internal_read_access)
//! * [`managed_tenant_has_internal_write_access(tenant name, stream id) -> bool`](DshApiClient::managed_tenant_has_internal_write_access)
//! * [`managed_tenant_has_public_read_access(tenant name, stream id) -> bool`](DshApiClient::managed_tenant_has_public_read_access)
//! * [`managed_tenant_has_public_write_access(tenant, name, stream id) -> bool`](DshApiClient::managed_tenant_has_public_write_access)
//! * [`managed_tenant_internal_streams_access_rights(tenant name) -> [(stream id, rights)]`](DshApiClient::managed_tenant_internal_streams_access_rights)
//! * [`managed_tenant_limit(tenant name, kind) -> limit`](DshApiClient::managed_tenant_limit)
//! * [`managed_tenant_limits(tenant name) -> limits`](DshApiClient::managed_tenant_limits)
//! * [`managed_tenant_public_streams_access_rights(tenant name) -> [(stream id, rights)]`](DshApiClient::managed_tenant_public_streams_access_rights)

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
/// * [`managed_tenant_granted_internal_streams(tenant name) -> [(stream id, stream, rights)]`](DshApiClient::managed_tenant_granted_internal_streams)
/// * [`managed_tenant_granted_managed_streams(tenant name) -> [(stream id, stream, rights)]`](DshApiClient::managed_tenant_granted_managed_streams)
/// * [`managed_tenant_granted_public_streams(tenant name) -> [(stream id, stream, rights)]`](DshApiClient::managed_tenant_granted_public_streams)
/// * [`managed_tenant_has_internal_read_access(tenant name, stream id) -> bool`](DshApiClient::managed_tenant_has_internal_read_access)
/// * [`managed_tenant_has_internal_write_access(tenant name, stream id) -> bool`](DshApiClient::managed_tenant_has_internal_write_access)
/// * [`managed_tenant_has_public_read_access(tenant name, stream id) -> bool`](DshApiClient::managed_tenant_has_public_read_access)
/// * [`managed_tenant_has_public_write_access(tenant, name, stream id) -> bool`](DshApiClient::managed_tenant_has_public_write_access)
/// * [`managed_tenant_internal_streams_access_rights(tenant name) -> [(stream id, rights)]`](DshApiClient::managed_tenant_internal_streams_access_rights)
/// * [`managed_tenant_limit(tenant name, kind) -> limit`](DshApiClient::managed_tenant_limit)
/// * [`managed_tenant_limits(tenant name) -> limits`](DshApiClient::managed_tenant_limits)
/// * [`managed_tenant_public_streams_access_rights(tenant name) -> [(stream id, rights)]`](DshApiClient::managed_tenant_public_streams_access_rights)
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
  pub async fn managed_tenant_granted_internal_streams(&self, managed_tenant: &str) -> Result<Vec<(ManagedStreamId, ManagedStream, AccessRights)>, DshApiError> {
    let access_rights = self.managed_tenant_internal_streams_access_rights(managed_tenant).await?;
    let streams = try_join_all(
      access_rights
        .iter()
        .map(|(managed_stream, _)| self.get_stream_internal_configuration(managed_stream)),
    )
    .await?;
    Ok(
      access_rights
        .into_iter()
        .zip(streams)
        .map(|((managed_stream, access_rights), internal_stream)| (managed_stream, internal_stream, access_rights))
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
  pub async fn managed_tenant_granted_managed_streams(&self, managed_tenant: &str) -> DshApiResult<Vec<(ManagedStreamId, Stream, AccessRights)>> {
    let (internal_streams, public_streams) = try_join(
      self.managed_tenant_granted_internal_streams(managed_tenant),
      self.managed_tenant_granted_public_streams(managed_tenant),
    )
    .await?;
    let mut internal_streams = internal_streams.into_iter().map(|(a, b, c)| (a, Stream::Internal(b), c)).collect_vec();
    let mut public_streams = public_streams.into_iter().map(|(a, b, c)| (a, Stream::Public(b), c)).collect_vec();
    internal_streams.append(&mut public_streams);
    internal_streams.sort_by(|(managed_stream_a, _, _), (managed_stream_b, _, _)| managed_stream_a.cmp(managed_stream_b));
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
  pub async fn managed_tenant_granted_public_streams(&self, managed_tenant: &str) -> Result<Vec<(ManagedStreamId, PublicManagedStream, AccessRights)>, DshApiError> {
    let access_rights = self.managed_tenant_public_streams_access_rights(managed_tenant).await?;
    let streams = try_join_all(access_rights.iter().map(|(managed_stream, _)| self.get_stream_public_configuration(managed_stream))).await?;
    Ok(
      access_rights
        .into_iter()
        .zip(streams)
        .map(|((managed_stream, access_rights), public_stream)| (managed_stream, public_stream, access_rights))
        .collect_vec(),
    )
  }

  /// # Check whether a managed tenant has read access to an internal managed stream
  ///
  /// Note that this method will return `Ok(false)` when either the managed tenant
  /// or the internal managed stream does not exist.
  ///
  /// # Parameters
  /// * `managed_tenant` - Managed tenant id.
  /// * `managed_stream` - Internal managed stream id.
  ///
  /// # Returns
  /// * `Ok(true)` - when the managed tenant has read access to the internal managed stream
  /// * `Ok(false)` - when the managed tenant does not have read access to the internal managed
  ///   stream, or when the internal managed stream or the managed tenant does not exist
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn managed_tenant_has_internal_read_access(&self, managed_tenant: &str, managed_stream: &ManagedStreamId) -> DshApiResult<bool> {
    match self.head_stream_internal_access_read(managed_stream, managed_tenant).await {
      Ok(()) => Ok(true),
      Err(DshApiError::NotFound(_)) => Ok(false),
      Err(other_error) => Err(other_error),
    }
  }

  /// # Check whether a managed tenant has write access to an internal managed stream
  ///
  /// Note that this method will return `Ok(false)` when either the managed tenant
  /// or the internal managed stream does not exist.
  ///
  /// # Parameters
  /// * `managed_tenant` - Managed tenant id.
  /// * `managed_stream` - Internal managed stream id.
  ///
  /// # Returns
  /// * `Ok(true)` - when the managed tenant has write access to the internal managed stream
  /// * `Ok(false)` - when the managed tenant does not have write access to the internal managed
  ///   stream, or when the internal managed stream or the managed tenant does not exist
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn managed_tenant_has_internal_write_access(&self, managed_tenant: &str, managed_stream: &ManagedStreamId) -> DshApiResult<bool> {
    match self.head_stream_internal_access_write(managed_stream, managed_tenant).await {
      Ok(()) => Ok(true),
      Err(DshApiError::NotFound(_)) => Ok(false),
      Err(other_error) => Err(other_error),
    }
  }

  /// # Check whether a managed tenant has read access to a public managed stream
  ///
  /// Note that this method will return `Ok(false)` when either the managed tenant
  /// or the public managed stream does not exist.
  ///
  /// # Parameters
  /// * `managed_tenant` - Managed tenant id.
  /// * `managed_stream` - Public managed stream id.
  ///
  /// # Returns
  /// * `Ok(true)` - when the managed tenant has read access to the public managed stream
  /// * `Ok(false)` - when the managed tenant does not have read access to the public managed
  ///   stream, or when the public managed stream or the managed tenant does not exist
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn managed_tenant_has_public_read_access(&self, managed_tenant: &str, managed_stream: &ManagedStreamId) -> DshApiResult<bool> {
    match self.head_stream_public_access_read(managed_stream, managed_tenant).await {
      Ok(()) => Ok(true),
      Err(DshApiError::NotFound(_)) => Ok(false),
      Err(other_error) => Err(other_error),
    }
  }

  /// # Check whether a managed tenant has write access to a public managed stream
  ///
  /// Note that this method will return `Ok(false)` when either the managed tenant
  /// or the public managed stream does not exist.
  ///
  /// # Parameters
  /// * `managed_tenant` - Managed tenant id.
  /// * `managed_stream` - Public managed stream id.
  ///
  /// # Returns
  /// * `Ok(true)` - when the managed tenant has write access to the public managed stream
  /// * `Ok(false)` - when the managed tenant does not have write access to the public managed
  ///   stream, or when the public managed stream or the managed tenant does not exist
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn managed_tenant_has_public_write_access(&self, managed_tenant: &str, managed_stream: &ManagedStreamId) -> DshApiResult<bool> {
    match self.head_stream_public_access_write(managed_stream, managed_tenant).await {
      Ok(()) => Ok(true),
      Err(DshApiError::NotFound(_)) => Ok(false),
      Err(other_error) => Err(other_error),
    }
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
  pub async fn managed_tenant_internal_streams_access_rights(&self, managed_tenant: &str) -> Result<Vec<(ManagedStreamId, AccessRights)>, DshApiError> {
    let internal_managed_streams = self.get_stream_internals().await?;
    let internal_access = try_join_all(internal_managed_streams.iter().map(|managed_stream| {
      try_join(
        self.managed_tenant_has_internal_read_access(managed_tenant, managed_stream),
        self.managed_tenant_has_internal_write_access(managed_tenant, managed_stream),
      )
    }))
    .await?;
    let mut internal_access_rights: Vec<(ManagedStreamId, AccessRights)> = internal_managed_streams
      .into_iter()
      .zip(internal_access)
      .filter_map(|(managed_stream, read_write)| match read_write {
        (false, false) => None,
        (false, true) => Some((managed_stream, AccessRights::Write)),
        (true, false) => Some((managed_stream, AccessRights::Read)),
        (true, true) => Some((managed_stream, AccessRights::ReadWrite)),
      })
      .collect_vec();
    internal_access_rights.sort_by(|(managed_stream_a, _), (managed_stream_b, _)| managed_stream_a.cmp(managed_stream_b));
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
  pub async fn managed_tenant_limit<T: TryInto<GetTenantLimitByManagerByTenantByKindKind>>(&self, managed_tenant: &str, kind: T) -> DshApiResult<LimitValue> {
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

  /// # Get managed tenant limits struct
  ///
  /// # Parameters
  /// * `managed_tenant` - managed tenants id
  ///
  /// # Returns
  /// * `Ok<`[`TenantLimits`]`>` - struct containing the limits of the managed tenant
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn managed_tenant_limits(&self, managed_tenant: &str) -> DshApiResult<TenantLimits> {
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
  pub async fn managed_tenant_public_streams_access_rights(&self, managed_tenant: &str) -> Result<Vec<(ManagedStreamId, AccessRights)>, DshApiError> {
    let public_managed_streams = self.get_stream_publics().await?;
    let public_access = try_join_all(public_managed_streams.iter().map(|managed_stream| {
      try_join(
        self.managed_tenant_has_public_read_access(managed_tenant, managed_stream),
        self.managed_tenant_has_public_write_access(managed_tenant, managed_stream),
      )
    }))
    .await?;
    let mut public_access_rights = public_managed_streams
      .into_iter()
      .zip(public_access)
      .filter_map(|(managed_stream, read_write)| match read_write {
        (false, false) => None,
        (false, true) => Some((managed_stream, AccessRights::Write)),
        (true, false) => Some((managed_stream, AccessRights::Read)),
        (true, true) => Some((managed_stream, AccessRights::ReadWrite)),
      })
      .collect_vec();
    public_access_rights.sort_by(|(managed_stream_a, _), (managed_stream_b, _)| managed_stream_a.cmp(managed_stream_b));
    Ok(public_access_rights)
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
