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

/// Implements a client for the Service Health API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_servicehealth_v1::client::ServiceHealth;
/// let client = ServiceHealth::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Request service health events relevant to your Google Cloud project.
///
/// # Configuration
///
/// To configure `ServiceHealth` use the `with_*` methods in the type returned
/// by [builder()][ServiceHealth::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://servicehealth.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::service_health::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::service_health::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `ServiceHealth` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `ServiceHealth` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct ServiceHealth {
    inner: Arc<dyn super::stub::dynamic::ServiceHealth>,
}

impl ServiceHealth {
    /// Returns a builder for [ServiceHealth].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_servicehealth_v1::client::ServiceHealth;
    /// let client = ServiceHealth::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::service_health::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::service_health::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::ServiceHealth + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::ServiceHealth>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::ServiceHealth> {
        super::transport::ServiceHealth::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::ServiceHealth> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::ServiceHealth::new)
    }

    /// Lists events under a given project and location.
    pub fn list_events(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::service_health::ListEvents {
        super::builder::service_health::ListEvents::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Retrieves a resource containing information about an event.
    pub fn get_event(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::service_health::GetEvent {
        super::builder::service_health::GetEvent::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists organization events under a given organization and location.
    pub fn list_organization_events(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::service_health::ListOrganizationEvents {
        super::builder::service_health::ListOrganizationEvents::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Retrieves a resource containing information about an event affecting an
    /// organization .
    pub fn get_organization_event(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::service_health::GetOrganizationEvent {
        super::builder::service_health::GetOrganizationEvent::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists assets impacted by organization events under a given organization and
    /// location.
    pub fn list_organization_impacts(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::service_health::ListOrganizationImpacts {
        super::builder::service_health::ListOrganizationImpacts::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Retrieves a resource containing information about impact to an asset under
    /// an organization affected by a service health event.
    pub fn get_organization_impact(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::service_health::GetOrganizationImpact {
        super::builder::service_health::GetOrganizationImpact::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::service_health::ListLocations {
        super::builder::service_health::ListLocations::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::service_health::GetLocation {
        super::builder::service_health::GetLocation::new(self.inner.clone()).set_name(name.into())
    }
}
