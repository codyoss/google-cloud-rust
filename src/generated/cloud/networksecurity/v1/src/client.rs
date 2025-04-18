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

/// Implements a client for the Network Security API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_networksecurity_v1::client::NetworkSecurity;
/// let client = NetworkSecurity::builder().build().await?;
/// // use `client` to make requests to the Network Security API.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Network Security API provides resources to configure authentication and
/// authorization policies. Refer to per API resource documentation for more
/// information.
///
/// # Configuration
///
/// To configure `NetworkSecurity` use the `with_*` methods in the type returned
/// by [builder()][NetworkSecurity::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://networksecurity.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::network_security::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::network_security::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `NetworkSecurity` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `NetworkSecurity` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct NetworkSecurity {
    inner: Arc<dyn super::stub::dynamic::NetworkSecurity>,
}

impl NetworkSecurity {
    /// Returns a builder for [NetworkSecurity].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_networksecurity_v1::client::NetworkSecurity;
    /// let client = NetworkSecurity::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::network_security::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::network_security::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::NetworkSecurity + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::NetworkSecurity>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::NetworkSecurity> {
        super::transport::NetworkSecurity::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::NetworkSecurity> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::NetworkSecurity::new)
    }

    /// Lists AuthorizationPolicies in a given project and location.
    pub fn list_authorization_policies(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::network_security::ListAuthorizationPolicies {
        super::builder::network_security::ListAuthorizationPolicies::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single AuthorizationPolicy.
    pub fn get_authorization_policy(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::network_security::GetAuthorizationPolicy {
        super::builder::network_security::GetAuthorizationPolicy::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new AuthorizationPolicy in a given project and location.
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
    pub fn create_authorization_policy(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::network_security::CreateAuthorizationPolicy {
        super::builder::network_security::CreateAuthorizationPolicy::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a single AuthorizationPolicy.
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
    pub fn update_authorization_policy(
        &self,
        authorization_policy: impl Into<crate::model::AuthorizationPolicy>,
    ) -> super::builder::network_security::UpdateAuthorizationPolicy {
        super::builder::network_security::UpdateAuthorizationPolicy::new(self.inner.clone())
            .set_authorization_policy(authorization_policy.into())
    }

    /// Deletes a single AuthorizationPolicy.
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
    pub fn delete_authorization_policy(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::network_security::DeleteAuthorizationPolicy {
        super::builder::network_security::DeleteAuthorizationPolicy::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists ServerTlsPolicies in a given project and location.
    pub fn list_server_tls_policies(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::network_security::ListServerTlsPolicies {
        super::builder::network_security::ListServerTlsPolicies::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single ServerTlsPolicy.
    pub fn get_server_tls_policy(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::network_security::GetServerTlsPolicy {
        super::builder::network_security::GetServerTlsPolicy::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new ServerTlsPolicy in a given project and location.
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
    pub fn create_server_tls_policy(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::network_security::CreateServerTlsPolicy {
        super::builder::network_security::CreateServerTlsPolicy::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a single ServerTlsPolicy.
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
    pub fn update_server_tls_policy(
        &self,
        server_tls_policy: impl Into<crate::model::ServerTlsPolicy>,
    ) -> super::builder::network_security::UpdateServerTlsPolicy {
        super::builder::network_security::UpdateServerTlsPolicy::new(self.inner.clone())
            .set_server_tls_policy(server_tls_policy.into())
    }

    /// Deletes a single ServerTlsPolicy.
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
    pub fn delete_server_tls_policy(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::network_security::DeleteServerTlsPolicy {
        super::builder::network_security::DeleteServerTlsPolicy::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists ClientTlsPolicies in a given project and location.
    pub fn list_client_tls_policies(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::network_security::ListClientTlsPolicies {
        super::builder::network_security::ListClientTlsPolicies::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single ClientTlsPolicy.
    pub fn get_client_tls_policy(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::network_security::GetClientTlsPolicy {
        super::builder::network_security::GetClientTlsPolicy::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new ClientTlsPolicy in a given project and location.
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
    pub fn create_client_tls_policy(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::network_security::CreateClientTlsPolicy {
        super::builder::network_security::CreateClientTlsPolicy::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a single ClientTlsPolicy.
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
    pub fn update_client_tls_policy(
        &self,
        client_tls_policy: impl Into<crate::model::ClientTlsPolicy>,
    ) -> super::builder::network_security::UpdateClientTlsPolicy {
        super::builder::network_security::UpdateClientTlsPolicy::new(self.inner.clone())
            .set_client_tls_policy(client_tls_policy.into())
    }

    /// Deletes a single ClientTlsPolicy.
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
    pub fn delete_client_tls_policy(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::network_security::DeleteClientTlsPolicy {
        super::builder::network_security::DeleteClientTlsPolicy::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::network_security::ListLocations {
        super::builder::network_security::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::network_security::GetLocation {
        super::builder::network_security::GetLocation::new(self.inner.clone()).set_name(name.into())
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::network_security::SetIamPolicy {
        super::builder::network_security::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::network_security::GetIamPolicy {
        super::builder::network_security::GetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    pub fn test_iam_permissions(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::network_security::TestIamPermissions {
        super::builder::network_security::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::network_security::ListOperations {
        super::builder::network_security::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::network_security::GetOperation {
        super::builder::network_security::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::network_security::DeleteOperation {
        super::builder::network_security::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::network_security::CancelOperation {
        super::builder::network_security::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
