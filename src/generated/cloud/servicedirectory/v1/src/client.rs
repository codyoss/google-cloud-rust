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

/// Implements a client for the Service Directory API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_servicedirectory_v1::client::LookupService;
/// let client = LookupService::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Service Directory API for looking up service data at runtime.
///
/// # Configuration
///
/// To configure `LookupService` use the `with_*` methods in the type returned
/// by [builder()][LookupService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://servicedirectory.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::lookup_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::lookup_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `LookupService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `LookupService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct LookupService {
    inner: Arc<dyn super::stub::dynamic::LookupService>,
}

impl LookupService {
    /// Returns a builder for [LookupService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_servicedirectory_v1::client::LookupService;
    /// let client = LookupService::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::lookup_service::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::lookup_service::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::LookupService + 'static,
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
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::LookupService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::LookupService> {
        super::transport::LookupService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::LookupService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::LookupService::new)
    }

    /// Returns a [service][google.cloud.servicedirectory.v1.Service] and its
    /// associated endpoints.
    /// Resolving a service is not considered an active developer method.
    ///
    /// [google.cloud.servicedirectory.v1.Service]: crate::model::Service
    pub fn resolve_service(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::lookup_service::ResolveService {
        super::builder::lookup_service::ResolveService::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::lookup_service::ListLocations {
        super::builder::lookup_service::ListLocations::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::lookup_service::GetLocation {
        super::builder::lookup_service::GetLocation::new(self.inner.clone()).set_name(name.into())
    }
}

/// Implements a client for the Service Directory API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_servicedirectory_v1::client::RegistrationService;
/// let client = RegistrationService::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Service Directory API for registering services. It defines the following
/// resource model:
///
/// - The API has a collection of
///   [Namespace][google.cloud.servicedirectory.v1.Namespace]
///   resources, named `projects/*/locations/*/namespaces/*`.
///
/// - Each Namespace has a collection of
///   [Service][google.cloud.servicedirectory.v1.Service] resources, named
///   `projects/*/locations/*/namespaces/*/services/*`.
///
/// - Each Service has a collection of
///   [Endpoint][google.cloud.servicedirectory.v1.Endpoint]
///   resources, named
///   `projects/*/locations/*/namespaces/*/services/*/endpoints/*`.
///
///
/// [google.cloud.servicedirectory.v1.Endpoint]: crate::model::Endpoint
/// [google.cloud.servicedirectory.v1.Namespace]: crate::model::Namespace
/// [google.cloud.servicedirectory.v1.Service]: crate::model::Service
///
/// # Configuration
///
/// To configure `RegistrationService` use the `with_*` methods in the type returned
/// by [builder()][RegistrationService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://servicedirectory.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::registration_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::registration_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `RegistrationService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `RegistrationService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct RegistrationService {
    inner: Arc<dyn super::stub::dynamic::RegistrationService>,
}

impl RegistrationService {
    /// Returns a builder for [RegistrationService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_servicedirectory_v1::client::RegistrationService;
    /// let client = RegistrationService::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::registration_service::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::registration_service::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::RegistrationService + 'static,
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
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::RegistrationService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::RegistrationService> {
        super::transport::RegistrationService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::RegistrationService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::RegistrationService::new)
    }

    /// Creates a namespace, and returns the new namespace.
    pub fn create_namespace(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::registration_service::CreateNamespace {
        super::builder::registration_service::CreateNamespace::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists all namespaces.
    pub fn list_namespaces(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::registration_service::ListNamespaces {
        super::builder::registration_service::ListNamespaces::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a namespace.
    pub fn get_namespace(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::registration_service::GetNamespace {
        super::builder::registration_service::GetNamespace::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates a namespace.
    pub fn update_namespace(
        &self,
        namespace: impl Into<crate::model::Namespace>,
    ) -> super::builder::registration_service::UpdateNamespace {
        super::builder::registration_service::UpdateNamespace::new(self.inner.clone())
            .set_namespace(namespace.into())
    }

    /// Deletes a namespace. This also deletes all services and endpoints in
    /// the namespace.
    pub fn delete_namespace(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::registration_service::DeleteNamespace {
        super::builder::registration_service::DeleteNamespace::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a service, and returns the new service.
    pub fn create_service(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::registration_service::CreateService {
        super::builder::registration_service::CreateService::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists all services belonging to a namespace.
    pub fn list_services(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::registration_service::ListServices {
        super::builder::registration_service::ListServices::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a service.
    pub fn get_service(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::registration_service::GetService {
        super::builder::registration_service::GetService::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates a service.
    pub fn update_service(
        &self,
        service: impl Into<crate::model::Service>,
    ) -> super::builder::registration_service::UpdateService {
        super::builder::registration_service::UpdateService::new(self.inner.clone())
            .set_service(service.into())
    }

    /// Deletes a service. This also deletes all endpoints associated with
    /// the service.
    pub fn delete_service(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::registration_service::DeleteService {
        super::builder::registration_service::DeleteService::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates an endpoint, and returns the new endpoint.
    pub fn create_endpoint(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::registration_service::CreateEndpoint {
        super::builder::registration_service::CreateEndpoint::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists all endpoints.
    pub fn list_endpoints(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::registration_service::ListEndpoints {
        super::builder::registration_service::ListEndpoints::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets an endpoint.
    pub fn get_endpoint(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::registration_service::GetEndpoint {
        super::builder::registration_service::GetEndpoint::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates an endpoint.
    pub fn update_endpoint(
        &self,
        endpoint: impl Into<crate::model::Endpoint>,
    ) -> super::builder::registration_service::UpdateEndpoint {
        super::builder::registration_service::UpdateEndpoint::new(self.inner.clone())
            .set_endpoint(endpoint.into())
    }

    /// Deletes an endpoint.
    pub fn delete_endpoint(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::registration_service::DeleteEndpoint {
        super::builder::registration_service::DeleteEndpoint::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets the IAM Policy for a resource (namespace or service only).
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::registration_service::GetIamPolicy {
        super::builder::registration_service::GetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Sets the IAM Policy for a resource (namespace or service only).
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::registration_service::SetIamPolicy {
        super::builder::registration_service::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Tests IAM permissions for a resource (namespace or service only).
    pub fn test_iam_permissions(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::registration_service::TestIamPermissions {
        super::builder::registration_service::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::registration_service::ListLocations {
        super::builder::registration_service::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::registration_service::GetLocation {
        super::builder::registration_service::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }
}
