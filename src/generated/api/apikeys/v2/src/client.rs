// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the API Keys API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_apikeys_v2::client::ApiKeys;
/// let client = ApiKeys::builder().build().await?;
/// // use `client` to make requests to the API Keys API.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Manages the API keys associated with projects.
///
/// # Configuration
///
/// To configure `ApiKeys` use the `with_*` methods in the type returned
/// by [builder()][ApiKeys::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://apikeys.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::api_keys::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::api_keys::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `ApiKeys` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `ApiKeys` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct ApiKeys {
    inner: Arc<dyn super::stub::dynamic::ApiKeys>,
}

impl ApiKeys {
    /// Returns a builder for [ApiKeys].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_apikeys_v2::client::ApiKeys;
    /// let client = ApiKeys::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::api_keys::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::api_keys::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::ApiKeys + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(
        conf: gaxi::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::ApiKeys>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::ApiKeys> {
        super::transport::ApiKeys::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::ApiKeys> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::ApiKeys::new)
    }

    /// Creates a new API key.
    ///
    /// NOTE: Key is a global resource; hence the only supported value for
    /// location is `global`.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_key(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::api_keys::CreateKey {
        super::builder::api_keys::CreateKey::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Lists the API keys owned by a project. The key string of the API key
    /// isn't included in the response.
    ///
    /// NOTE: Key is a global resource; hence the only supported value for
    /// location is `global`.
    pub fn list_keys(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::api_keys::ListKeys {
        super::builder::api_keys::ListKeys::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets the metadata for an API key. The key string of the API key
    /// isn't included in the response.
    ///
    /// NOTE: Key is a global resource; hence the only supported value for
    /// location is `global`.
    pub fn get_key(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::api_keys::GetKey {
        super::builder::api_keys::GetKey::new(self.inner.clone()).set_name(name.into())
    }

    /// Get the key string for an API key.
    ///
    /// NOTE: Key is a global resource; hence the only supported value for
    /// location is `global`.
    pub fn get_key_string(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::api_keys::GetKeyString {
        super::builder::api_keys::GetKeyString::new(self.inner.clone()).set_name(name.into())
    }

    /// Patches the modifiable fields of an API key.
    /// The key string of the API key isn't included in the response.
    ///
    /// NOTE: Key is a global resource; hence the only supported value for
    /// location is `global`.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_key(
        &self,
        key: impl Into<crate::model::Key>,
    ) -> super::builder::api_keys::UpdateKey {
        super::builder::api_keys::UpdateKey::new(self.inner.clone()).set_key(key.into())
    }

    /// Deletes an API key. Deleted key can be retrieved within 30 days of
    /// deletion. Afterward, key will be purged from the project.
    ///
    /// NOTE: Key is a global resource; hence the only supported value for
    /// location is `global`.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_key(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::api_keys::DeleteKey {
        super::builder::api_keys::DeleteKey::new(self.inner.clone()).set_name(name.into())
    }

    /// Undeletes an API key which was deleted within 30 days.
    ///
    /// NOTE: Key is a global resource; hence the only supported value for
    /// location is `global`.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn undelete_key(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::api_keys::UndeleteKey {
        super::builder::api_keys::UndeleteKey::new(self.inner.clone()).set_name(name.into())
    }

    /// Find the parent project and resource name of the API
    /// key that matches the key string in the request. If the API key has been
    /// purged, resource name will not be set.
    /// The service account must have the `apikeys.keys.lookup` permission
    /// on the parent project.
    pub fn lookup_key(&self) -> super::builder::api_keys::LookupKey {
        super::builder::api_keys::LookupKey::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::api_keys::GetOperation {
        super::builder::api_keys::GetOperation::new(self.inner.clone()).set_name(name.into())
    }
}
