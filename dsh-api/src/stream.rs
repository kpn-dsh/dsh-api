//! # Additional methods and functions to manage streams
//!
//! _These functions are only available when the `manage` feature is enabled._
//!
//! Module that contains methods and functions to manage internal and public streams.
//! * Derived methods - DshApiClient methods that add extra capabilities
//!   but depend on the API methods.
//! * Functions - Functions that add extra capabilities but do not depend directly on the API.
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`get_tenants_with_access_rights(stream) ->
//!   [(tenant, rights)]`](DshApiClient::get_tenants_with_access_rights)
//! * [`get_internal_stream_configurations() ->
//!   [(id, stream)]`](DshApiClient::get_internal_stream_configurations)
//! * [`get_public_stream_configurations() ->
//!   [(id, stream)]`](DshApiClient::get_public_stream_configurations)
//! * [`get_stream_configuration(stream) -> stream`](DshApiClient::get_stream_configuration)
//! * [`get_stream_configurations() -> [(id, stream)]`](DshApiClient::get_stream_configurations)
//! * [`grant_managed_stream_access_rights(stream, tenant, rights) ->
//!   stream`](DshApiClient::grant_managed_stream_access_rights)
//! * [`revoke_managed_stream_access_rights(stream, tenant, rights) ->
//!   stream`](DshApiClient::revoke_managed_stream_access_rights)

use crate::dsh_api_client::DshApiClient;
use crate::types::{ManagedStream, ManagedStreamId, PublicManagedStream};
use crate::{AccessRights, DshApiError, DshApiResult};
use futures::future::try_join_all;
use futures::{join, try_join};
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
/// * [`get_tenants_with_access_rights(stream) ->
///   [(tenant, rights)]`](DshApiClient::get_tenants_with_access_rights)
/// * [`get_internal_stream_configurations() ->
///   [(id, stream)]`](DshApiClient::get_internal_stream_configurations)
/// * [`get_public_stream_configurations() ->
///   [(id, stream)]`](DshApiClient::get_public_stream_configurations)
/// * [`get_stream_configuration(stream) -> stream`](DshApiClient::get_stream_configuration)
/// * [`get_stream_configurations() -> [(id, stream)]`](DshApiClient::get_stream_configurations)
/// * [`grant_managed_stream_access_rights(stream, tenant, rights) ->
///   stream`](DshApiClient::grant_managed_stream_access_rights)
/// * [`revoke_managed_stream_access_rights(stream, tenant, rights) ->
///   stream`](DshApiClient::revoke_managed_stream_access_rights)
impl DshApiClient {
  /// # Get tenants that have been granted access rights
  ///
  /// # Parameters
  /// * `managed_stream_id` - internal or public managed stream to get granted rights for
  ///
  /// # Returns
  /// * `Ok<Vec<(String, AccessRights)>` - tuples consisting of tenant ids and granted access rights
  /// * `Err<DshApiError>` - when the managed stream does not exist or the request
  ///   could not be processed by the DSH
  pub async fn get_tenants_with_access_rights(&self, managed_stream_id: &ManagedStreamId) -> DshApiResult<Vec<(String, AccessRights)>> {
    let (tenant_ids_reads, tenant_ids_writes) = match self.get_stream_configuration(managed_stream_id).await? {
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
    let mut tenant_ids = tenant_ids_reads.iter().collect::<Vec<_>>();
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
        .collect::<Vec<_>>(),
    )
  }

  /// # Get named internal managed stream configurations
  ///
  /// Returns a list of (stream id, stream)-tuples containing the ids and configurations
  /// of the available internal managed streams.
  /// When there are no internal managed streams, an empty list will be returned.
  /// The list will be sorted by stream id.
  ///
  /// # Returns
  /// * `Ok<Vec<(ManagedStreamId, ManagedStream)>>` - when request was successful
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn get_internal_stream_configurations(&self) -> DshApiResult<Vec<(ManagedStreamId, ManagedStream)>> {
    let internal_stream_ids = self.get_stream_internals().await?;
    let internal_streams = try_join_all(internal_stream_ids.iter().map(|stream_id| self.get_stream_internal_configuration(stream_id))).await?;
    let mut tuples = internal_stream_ids.into_iter().zip(internal_streams).collect::<Vec<_>>();
    tuples.sort_by(|(stream_id_a, _), (stream_id_b, _)| stream_id_a.cmp(stream_id_b));
    Ok(tuples)
  }

  /// # Get named public managed stream configurations
  ///
  /// Returns a list of (stream id, stream)-tuples containing the ids and configurations
  /// of the available public managed streams.
  /// When there are no public managed streams, an empty list will be returned.
  /// The list will be sorted by stream id.
  ///
  /// # Returns
  /// * `Ok<Vec<(ManagedStreamId, PublicManagedStream)>>` - when request was successful
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn get_public_stream_configurations(&self) -> DshApiResult<Vec<(ManagedStreamId, PublicManagedStream)>> {
    let public_stream_ids = self.get_stream_publics().await?;
    let public_streams = try_join_all(public_stream_ids.iter().map(|stream_id| self.get_stream_public_configuration(stream_id))).await?;
    let mut tuples = public_stream_ids.into_iter().zip(public_streams).collect::<Vec<_>>();
    tuples.sort_by(|(stream_id_a, _), (stream_id_b, _)| stream_id_a.cmp(stream_id_b));
    Ok(tuples)
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
  pub async fn get_stream_configuration(&self, managed_stream_id: &ManagedStreamId) -> DshApiResult<Option<Stream>> {
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

  /// # Get named managed stream configurations
  ///
  /// Returns a list of (stream id, stream)-tuples containing the ids and configurations
  /// of the available internal or public managed streams.
  /// When there are no managed streams, an empty list will be returned.
  /// The list will be sorted by stream id.
  ///
  /// # Returns
  /// * `Ok<Vec<(ManagedStreamId, Stream)>>` - when request was successful
  /// * `Err<DshApiError>` - when the request could not be processed by the DSH
  pub async fn get_stream_configurations(&self) -> DshApiResult<Vec<(ManagedStreamId, Stream)>> {
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
      .zip(internal_streams.into_iter().map(Stream::Internal).collect::<Vec<_>>())
      .collect::<Vec<_>>();
    tuples.append(
      &mut public_ids
        .into_iter()
        .zip(public_streams.into_iter().map(Stream::Public).collect::<Vec<_>>())
        .collect::<Vec<_>>(),
    );
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
  pub async fn grant_managed_stream_access_rights(&self, managed_stream_id: &ManagedStreamId, managed_tenant_id: &str, access_rights: &AccessRights) -> DshApiResult<Stream> {
    match self.get_stream_configuration(managed_stream_id).await? {
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
  pub async fn revoke_managed_stream_access_rights(&self, managed_stream_id: &ManagedStreamId, managed_tenant_id: &str, access_rights: &AccessRights) -> DshApiResult<Stream> {
    match self.get_stream_configuration(managed_stream_id).await? {
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
}
