//! # secret
//!
//! Manage secrets.
//!
//! * `create_secrets() -> ()`
//! * `delete_secret(secret_id) -> ()`
//! * `get_secret(secret_id) -> ByteStream`
//! * `get_secret_actual(secret_id) -> Empty`
//! * `get_secret_configuration(secret_id) -> Empty`
//! * `get_secret_status(secret_id) -> AllocationStatus`
//! * `get_secrets() -> Vec<String>`
//! * `update_secret(secret_id) -> ()`

use progenitor_client::ByteStream;

use crate::types::{AllocationStatus, Empty, Secret};
use crate::DshApiClient;
use crate::DshApiResult;

impl DshApiClient<'_> {
  /// `GET /allocation/{tenant}/secret`
  pub async fn get_secrets(&self) -> DshApiResult<Vec<String>> {
    self
      .process(self.generated_client().secret_get_by_tenant_secret(self.tenant(), self.token()).await)
      .map(|result| result.1)
      .map(|secret_ids| secret_ids.iter().map(|secret_id| secret_id.to_string()).collect())
  }

  /// `POST /allocation/{tenant}/secret`
  pub async fn create_secrets(&self, secret: Secret) -> DshApiResult<()> {
    self
      .process(self.generated_client().secret_post_by_tenant_secret(self.tenant(), self.token(), &secret).await)
      .map(|result| result.1)
  }

  /// `DELETE /allocation/{tenant}/secret/{id}/configuration`
  pub async fn delete_secret(&self, secret_id: &str) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client()
          .secret_delete_by_tenant_secret_by_id_configuration(self.tenant(), secret_id, self.token())
          .await,
      )
      .map(|result| result.1)
  }

  /// `GET /allocation/{tenant}/secret/{id}/configuration`
  // TODO Returns Empty?
  pub async fn get_secret_configuration(&self, secret_id: &str) -> DshApiResult<Empty> {
    self
      .process(
        self
          .generated_client()
          .secret_get_by_tenant_secret_by_id_configuration(self.tenant(), secret_id, self.token())
          .await,
      )
      .map(|result| result.1)
  }

  /// `GET /allocation/{tenant}/secret/{id}/actual`
  pub async fn get_secret_actual(&self, secret_id: &str) -> DshApiResult<Empty> {
    // TODO Returns Empty?
    self
      .process(
        self
          .generated_client()
          .secret_get_by_tenant_secret_by_id_actual(self.tenant(), secret_id, self.token())
          .await,
      )
      .map(|result| result.1)
  }

  /// `GET /allocation/{tenant}/secret/{id}/status`
  pub async fn get_secret_status(&self, secret_id: &str) -> DshApiResult<AllocationStatus> {
    self
      .process(
        self
          .generated_client()
          .secret_get_by_tenant_secret_by_id_status(self.tenant(), secret_id, self.token())
          .await,
      )
      .map(|result| result.1)
  }

  /// `GET /allocation/{tenant}/secret/{id}`
  pub async fn get_secret(&self, secret_id: &str) -> DshApiResult<ByteStream> {
    self
      .process_raw(
        self
          .generated_client()
          .secret_get_by_tenant_secret_by_id(self.tenant(), secret_id, self.token())
          .await,
      )
      .map(|result| result.1)
  }

  /// `PUT /allocation/{tenant}/secret/{id}`
  pub async fn update_secret(&self, secret_id: &str, secret: String) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client()
          .secret_put_by_tenant_secret_by_id(self.tenant(), secret_id, self.token(), secret)
          .await,
      )
      .map(|result| result.1)
  }
}
