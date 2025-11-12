//! # Additional methods to manage streams
//!
//! _These functions are only available when the `manage` feature is enabled._
//!
//! Module that contains methods and functions to manage streams.
//!
//! # Generated methods
//!
//! [`DshApiClient`] methods that are generated from the `openapi` specification.
//!
//! * [`delete_stream_internal_access_read(streamid, tenant)`](DshApiClient::delete_stream_internal_access_read)
//! * [`delete_stream_internal_access_write(streamid, tenant)`](DshApiClient::delete_stream_internal_access_write)
//! * [`delete_stream_internal_configuration(streamid)`](DshApiClient::delete_stream_internal_configuration)
//! * [`delete_stream_public_access_read(streamid, tenant)`](DshApiClient::delete_stream_public_access_read)
//! * [`delete_stream_public_access_write(streamid, tenant)`](DshApiClient::delete_stream_public_access_write)
//! * [`delete_stream_public_configuration(streamid)`](DshApiClient::delete_stream_public_configuration)
//! * [`get_stream_internal_access_reads(streamid) -> [id]`](DshApiClient::get_stream_internal_access_reads)
//! * [`get_stream_internal_access_writes(streamid) -> [id]`](DshApiClient::get_stream_internal_access_writes)
//! * [`get_stream_internal_configuration(streamid) -> ManagedStream`](DshApiClient::get_stream_internal_configuration)
//! * [`get_stream_internals() -> [ManagedStreamId]`](DshApiClient::get_stream_internals)
//! * [`get_stream_public_access_reads(streamid) -> [id]`](DshApiClient::get_stream_public_access_reads)
//! * [`get_stream_public_access_writes(streamid) -> [id]`](DshApiClient::get_stream_public_access_writes)
//! * [`get_stream_public_configuration(streamid) -> PublicManagedStream`](DshApiClient::get_stream_public_configuration)
//! * [`get_stream_publics() -> [ManagedStreamId]`](DshApiClient::get_stream_publics)
//! * [`head_stream_internal_access_read(streamid, tenant)`](DshApiClient::head_stream_internal_access_read)
//! * [`head_stream_internal_access_write(streamid, tenant)`](DshApiClient::head_stream_internal_access_write)
//! * [`head_stream_public_access_read(streamid, tenant)`](DshApiClient::head_stream_public_access_read)
//! * [`head_stream_public_access_write(streamid, tenant)`](DshApiClient::head_stream_public_access_write)
//! * [`post_stream_internal_configuration(streamid, body)`](DshApiClient::post_stream_internal_configuration)
//! * [`post_stream_public_configuration(streamid, body)`](DshApiClient::post_stream_public_configuration)
//! * [`put_stream_internal_access_read(streamid, tenant)`](DshApiClient::put_stream_internal_access_read)
//! * [`put_stream_internal_access_write(streamid, tenant)`](DshApiClient::put_stream_internal_access_write)
//! * [`put_stream_public_access_read(streamid, tenant)`](DshApiClient::put_stream_public_access_read)
//! * [`put_stream_public_access_write(streamid, tenant)`](DshApiClient::put_stream_public_access_write)
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`managed_stream_access_rights(stream id, tenant name) -> rights`](DshApiClient::managed_stream_access_rights)
//! * [`managed_stream_configuration(stream id) -> stream`](DshApiClient::managed_stream_configuration)
//! * [`managed_stream_configurations() -> [(stream id, stream)]`](DshApiClient::managed_stream_configurations)
//! * [`managed_stream_configurations_internal() -> [(stream id, stream)]`](DshApiClient::managed_stream_configurations_internal)
//! * [`managed_stream_configurations_public() -> [(stream id, stream)]`](DshApiClient::managed_stream_configurations_public)
//! * [`managed_stream_grant_access_rights(stream id, tenant name, rights) -> stream`](DshApiClient::managed_stream_grant_access_rights)
//! * [`managed_stream_revoke_access_rights(stream id, tenant name, rights) -> stream`](DshApiClient::managed_stream_revoke_access_rights)
//! * [`managed_stream_tenants_with_access_rights(stream id) -> [(tenant name, rights)]`](DshApiClient::managed_stream_tenants_with_access_rights)

use crate::dsh_api_client::DshApiClient;
use crate::types::{ManagedStream, ManagedStreamId, PublicManagedStream};
use crate::{AccessRights, DshApiError, DshApiResult};
use futures::future::try_join_all;
use futures::{join, try_join};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

/// # Managed stream, either internal or public
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Stream {
  Internal(ManagedStream),
  Public(PublicManagedStream),
}

impl Display for Stream {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Stream::Internal(internal_managed_stream) => Display::fmt(&internal_managed_stream, f),
      Stream::Public(public_managed_stream) => Display::fmt(public_managed_stream, f),
    }
  }
}

/// # Additional methods and functions to manage streams
///
/// _These functions are only available when the `manage` feature is enabled._
///
/// Module that contains methods and functions to manage internal and public streams.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
/// * Functions - Functions that add extra capabilities but do not depend directly on the API.
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`managed_stream_access_rights(stream id, tenant name) -> rights`](DshApiClient::managed_stream_access_rights)
/// * [`managed_stream_configuration(stream id) -> stream`](DshApiClient::managed_stream_configuration)
/// * [`managed_stream_configurations() -> [(stream id, stream)]`](DshApiClient::managed_stream_configurations)
/// * [`managed_stream_configurations_internal() -> [(stream id, stream)]`](DshApiClient::managed_stream_configurations_internal)
/// * [`managed_stream_configurations_public() -> [(stream id, stream)]`](DshApiClient::managed_stream_configurations_public)
/// * [`managed_stream_grant_access_rights(stream id, tenant name, rights) -> stream`](DshApiClient::managed_stream_grant_access_rights)
/// * [`managed_stream_revoke_access_rights(stream id, tenant name, rights) -> stream`](DshApiClient::managed_stream_revoke_access_rights)
/// * [`managed_stream_tenants_with_access_rights(stream id) -> [(tenant name, rights)]`](DshApiClient::managed_stream_tenants_with_access_rights)
impl DshApiClient {
  /// # Check whether a managed tenant has access to a managed stream
  ///
  /// Note that this method will return `Ok(None)` when either the managed tenant
  /// or the managed stream does not exist.
  ///
  /// # Parameters
  /// * `stream_id` - managed stream id
  /// * `tenant_id` - managed tenant id
  ///
  /// # Returns
  /// * `Ok(Some(AccessRights::Read))` - when the managed tenant has read access to the managed stream
  /// * `Ok(Some(AccessRights::ReadWrite))` - when the managed tenant has both read and write access to the managed stream
  /// * `Ok(Some(AccessRights::Write))` - when the managed tenant has write access to the managed stream
  /// * `Ok(None)` - when the managed tenant does not have access to the managed stream,
  ///   or when the managed stream or the managed tenant does not exist
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn managed_stream_access_rights(&self, stream_id: &ManagedStreamId, tenant_id: &str) -> DshApiResult<Option<AccessRights>> {
    let (internal_read_access, internal_write_access, public_read_access, public_write_access) = try_join!(
      self.managed_tenant_has_internal_read_access(tenant_id, stream_id),
      self.managed_tenant_has_internal_write_access(tenant_id, stream_id),
      self.managed_tenant_has_public_read_access(tenant_id, stream_id),
      self.managed_tenant_has_public_write_access(tenant_id, stream_id)
    )?;
    match (internal_read_access, internal_write_access) {
      (false, false) => match (public_read_access, public_write_access) {
        (false, false) => Ok(None),
        (false, true) => Ok(Some(AccessRights::Write)),
        (true, false) => Ok(Some(AccessRights::Read)),
        (true, true) => Ok(Some(AccessRights::ReadWrite)),
      },
      (false, true) => Ok(Some(AccessRights::Write)),
      (true, false) => Ok(Some(AccessRights::Read)),
      (true, true) => Ok(Some(AccessRights::ReadWrite)),
    }
  }

  /// # Get internal or public managed stream configuration
  ///
  /// # Parameters
  /// * `managed_stream_id` - managed stream identifier
  ///
  /// # Returns
  /// * `Ok<Stream::Internal>` - when request was successful for internal managed stream
  /// * `Ok<Stream::Public>` - when request was successful for public managed stream
  /// * `Ok<None>` - when internal and public managed stream with the provided id do not exist
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn managed_stream_configuration(&self, managed_stream_id: &ManagedStreamId) -> DshApiResult<Option<Stream>> {
    let r = join!(
      self.get_stream_internal_configuration(managed_stream_id),
      self.get_stream_public_configuration(managed_stream_id)
    );
    match r {
      (Err(internal_stream_error), Err(public_stream_error)) => match (internal_stream_error, public_stream_error) {
        (DshApiError::NotFound(_), DshApiError::NotFound(_)) => Ok(None),
        (internal_stream_error, DshApiError::NotFound(_)) => Err(internal_stream_error),
        (DshApiError::NotFound(_), public_stream_error) => Err(public_stream_error),
        (internal_stream_error, _) => Err(internal_stream_error),
      },
      (Ok(internal_stream), Err(public_stream_error)) => match public_stream_error {
        DshApiError::NotFound(_) => Ok(Some(Stream::Internal(internal_stream))),
        error => Err(error),
      },
      (Err(internal_stream_error), Ok(public_stream)) => match internal_stream_error {
        DshApiError::NotFound(_) => Ok(Some(Stream::Public(public_stream))),
        error => Err(error),
      },
      (Ok(_), Ok(_)) => Err(DshApiError::Unexpected(
        format!("both internal and public managed stream '{}' exist", managed_stream_id),
        None,
      )),
    }
  }

  /// # Get managed stream configurations
  ///
  /// Returns a list of (stream id, stream)-tuples containing the ids and configurations
  /// of the available internal or public managed streams.
  /// When there are no managed streams, an empty list will be returned.
  /// The list will be sorted by stream id.
  ///
  /// # Returns
  /// * `Ok<Vec<(ManagedStreamId, Stream)>>` - when request was successful
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn managed_stream_configurations(&self) -> DshApiResult<Vec<(ManagedStreamId, Stream)>> {
    let (internal_ids, public_ids) = try_join!(self.get_stream_internals(), self.get_stream_publics())?;
    let (internal_streams, public_streams) = try_join!(
      try_join_all(
        internal_ids
          .iter()
          .map(|managed_stream_id| self.get_stream_internal_configuration(managed_stream_id))
      ),
      try_join_all(public_ids.iter().map(|managed_stream_id| self.get_stream_public_configuration(managed_stream_id))),
    )?;
    let mut tuples: Vec<(ManagedStreamId, Stream)> = internal_ids
      .into_iter()
      .zip(internal_streams.into_iter().map(Stream::Internal).collect_vec())
      .collect_vec();
    tuples.append(
      &mut public_ids
        .into_iter()
        .zip(public_streams.into_iter().map(Stream::Public).collect_vec())
        .collect_vec(),
    );
    tuples.sort_by(|(stream_id_a, _), (stream_id_b, _)| stream_id_a.cmp(stream_id_b));
    Ok(tuples)
  }

  /// # Get internal managed stream configurations
  ///
  /// Returns a list of (stream id, stream)-tuples containing the ids and configurations
  /// of the available internal managed streams.
  /// When there are no internal managed streams, an empty list will be returned.
  /// The list will be sorted by stream id.
  ///
  /// # Returns
  /// * `Ok<Vec<(ManagedStreamId, ManagedStream)>>` - when request was successful
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn managed_stream_configurations_internal(&self) -> DshApiResult<Vec<(ManagedStreamId, ManagedStream)>> {
    let internal_stream_ids = self.get_stream_internals().await?;
    let internal_streams = try_join_all(internal_stream_ids.iter().map(|stream_id| self.get_stream_internal_configuration(stream_id))).await?;
    let mut tuples = internal_stream_ids.into_iter().zip(internal_streams).collect_vec();
    tuples.sort_by(|(stream_id_a, _), (stream_id_b, _)| stream_id_a.cmp(stream_id_b));
    Ok(tuples)
  }

  /// # Get public managed stream configurations
  ///
  /// Returns a list of (stream id, stream)-tuples containing the ids and configurations
  /// of the available public managed streams.
  /// When there are no public managed streams, an empty list will be returned.
  /// The list will be sorted by stream id.
  ///
  /// # Returns
  /// * `Ok<Vec<(ManagedStreamId, PublicManagedStream)>>` - when request was successful
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn managed_stream_configurations_public(&self) -> DshApiResult<Vec<(ManagedStreamId, PublicManagedStream)>> {
    let public_stream_ids = self.get_stream_publics().await?;
    let public_streams = try_join_all(public_stream_ids.iter().map(|stream_id| self.get_stream_public_configuration(stream_id))).await?;
    let mut tuples = public_stream_ids.into_iter().zip(public_streams).collect_vec();
    tuples.sort_by(|(stream_id_a, _), (stream_id_b, _)| stream_id_a.cmp(stream_id_b));
    Ok(tuples)
  }

  /// # Grant managed stream access rights to managed tenant
  ///
  /// # Parameters
  /// * `managed_stream_id` - internal or public managed stream to grant access rights to
  /// * `managed_tenant_id` - managed tenant which is granted access rights
  /// * `access_rights` - read, read/write or write access rights
  ///
  /// # Returns
  /// * `Ok<Stream>` - when request was successfully made the internal or public managed stream
  ///   will be returned
  /// * `Err<DshApiError>` - when the managed stream does not exist or the request
  ///   could not be processed by the DSH
  pub async fn managed_stream_grant_access_rights(&self, managed_stream_id: &ManagedStreamId, managed_tenant_id: &str, access_rights: &AccessRights) -> DshApiResult<Stream> {
    match self.managed_stream_configuration(managed_stream_id).await? {
      Some(Stream::Internal(internal)) => {
        match access_rights {
          AccessRights::Read => self.put_stream_internal_access_read(managed_stream_id, managed_tenant_id).await?,
          AccessRights::ReadWrite => {
            try_join!(
              self.put_stream_internal_access_read(managed_stream_id, managed_tenant_id),
              self.put_stream_internal_access_write(managed_stream_id, managed_tenant_id),
            )?;
          }
          AccessRights::Write => self.put_stream_internal_access_write(managed_stream_id, managed_tenant_id).await?,
        }
        Ok(Stream::Internal(internal))
      }
      Some(Stream::Public(public)) => {
        match access_rights {
          AccessRights::Read => self.put_stream_public_access_read(managed_stream_id, managed_tenant_id).await?,
          AccessRights::ReadWrite => {
            try_join!(
              self.put_stream_public_access_read(managed_stream_id, managed_tenant_id),
              self.put_stream_public_access_write(managed_stream_id, managed_tenant_id),
            )?;
          }
          AccessRights::Write => self.put_stream_public_access_write(managed_stream_id, managed_tenant_id).await?,
        }
        Ok(Stream::Public(public))
      }
      None => Err(DshApiError::NotFound(Some(format!("managed stream '{}' does not exist", managed_stream_id)))),
    }
  }

  /// # Revoke managed stream access rights from managed tenant
  ///
  /// # Parameters
  /// * `managed_stream_id` - internal or public managed stream to revoke access rights from
  /// * `managed_tenant_id` - managed tenant from which access rights are revoked
  /// * `access_rights` - read, read/write or write access rights
  ///
  /// # Returns
  /// * `Ok<Stream>` - when request was successfully made the internal or public managed stream
  ///   will be returned
  /// * `Err<DshApiError>` - when the managed stream does not exist or the request
  ///   could not be processed by the DSH
  pub async fn managed_stream_revoke_access_rights(&self, managed_stream_id: &ManagedStreamId, managed_tenant_id: &str, access_rights: &AccessRights) -> DshApiResult<Stream> {
    match self.managed_stream_configuration(managed_stream_id).await? {
      Some(Stream::Internal(internal)) => {
        match access_rights {
          AccessRights::Read => self.delete_stream_internal_access_read(managed_stream_id, managed_tenant_id).await?,
          AccessRights::ReadWrite => {
            try_join!(
              self.delete_stream_internal_access_read(managed_stream_id, managed_tenant_id),
              self.delete_stream_internal_access_write(managed_stream_id, managed_tenant_id),
            )?;
          }
          AccessRights::Write => self.delete_stream_internal_access_write(managed_stream_id, managed_tenant_id).await?,
        }
        Ok(Stream::Internal(internal))
      }
      Some(Stream::Public(public)) => {
        match access_rights {
          AccessRights::Read => self.delete_stream_public_access_read(managed_stream_id, managed_tenant_id).await?,
          AccessRights::ReadWrite => {
            try_join!(
              self.delete_stream_public_access_read(managed_stream_id, managed_tenant_id),
              self.delete_stream_public_access_write(managed_stream_id, managed_tenant_id),
            )?;
          }
          AccessRights::Write => self.delete_stream_public_access_write(managed_stream_id, managed_tenant_id).await?,
        }
        Ok(Stream::Public(public))
      }
      None => Err(DshApiError::NotFound(Some(format!("managed stream '{}' does not exist", managed_stream_id)))),
    }
  }

  /// # Get tenants that have been granted access rights
  ///
  /// # Parameters
  /// * `managed_stream_id` - internal or public managed stream to get granted rights for
  ///
  /// # Returns
  /// * `Ok<Vec<(String, AccessRights)>` - tuples consisting of tenant ids and granted access rights
  /// * `Err<DshApiError>` - when the managed stream does not exist or the request
  ///   could not be processed by the DSH
  pub async fn managed_stream_tenants_with_access_rights(&self, managed_stream_id: &ManagedStreamId) -> DshApiResult<Vec<(String, AccessRights)>> {
    let (tenant_ids_reads, tenant_ids_writes) = match self.managed_stream_configuration(managed_stream_id).await? {
      Some(stream_configuration) => match stream_configuration {
        Stream::Internal(_) => try_join!(
          self.get_stream_internal_access_reads(managed_stream_id),
          self.get_stream_internal_access_writes(managed_stream_id)
        )?,
        Stream::Public(_) => try_join!(
          self.get_stream_public_access_reads(managed_stream_id),
          self.get_stream_public_access_writes(managed_stream_id)
        )?,
      },
      None => return Err(DshApiError::NotFound(Some(format!("managed stream '{}' does not exist", managed_stream_id)))),
    };
    let mut tenant_ids = tenant_ids_reads.iter().collect_vec();
    for id in &tenant_ids_writes {
      tenant_ids.push(id);
    }
    tenant_ids.sort();
    tenant_ids.dedup();
    Ok(
      tenant_ids
        .into_iter()
        .filter_map(|tenant_id| {
          AccessRights::from(tenant_ids_reads.contains(tenant_id), tenant_ids_writes.contains(tenant_id)).map(|acess_rights| (tenant_id.clone(), acess_rights))
        })
        .collect_vec(),
    )
  }
}
