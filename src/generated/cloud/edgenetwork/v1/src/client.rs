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

/// Implements a client for the Distributed Cloud Edge Network API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_edgenetwork_v1::client::EdgeNetwork;
/// let client = EdgeNetwork::builder().build().await?;
/// // use `client` to make requests to the Distributed Cloud Edge Network API.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// EdgeNetwork API provides managed, highly available cloud dynamic network
/// configuration service to the GEC customer to enable edge application and
/// network function solutions. This allows the customers to easily define and
/// configure the network setup and property to meet the workload requirement.
///
/// # Configuration
///
/// To configure `EdgeNetwork` use the `with_*` methods in the type returned
/// by [builder()][EdgeNetwork::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://edgenetwork.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::edge_network::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::edge_network::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `EdgeNetwork` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `EdgeNetwork` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct EdgeNetwork {
    inner: Arc<dyn super::stub::dynamic::EdgeNetwork>,
}

impl EdgeNetwork {
    /// Returns a builder for [EdgeNetwork].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_edgenetwork_v1::client::EdgeNetwork;
    /// let client = EdgeNetwork::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::edge_network::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::edge_network::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::EdgeNetwork + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::EdgeNetwork>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::EdgeNetwork> {
        super::transport::EdgeNetwork::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::EdgeNetwork> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::EdgeNetwork::new)
    }

    /// InitializeZone will initialize resources for a zone in a project.
    pub fn initialize_zone(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::InitializeZone {
        super::builder::edge_network::InitializeZone::new(self.inner.clone()).set_name(name.into())
    }

    /// Deprecated: not implemented.
    /// Lists Zones in a given project and location.
    pub fn list_zones(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::edge_network::ListZones {
        super::builder::edge_network::ListZones::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Deprecated: not implemented.
    /// Gets details of a single Zone.
    pub fn get_zone(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::GetZone {
        super::builder::edge_network::GetZone::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists Networks in a given project and location.
    pub fn list_networks(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::edge_network::ListNetworks {
        super::builder::edge_network::ListNetworks::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single Network.
    pub fn get_network(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::GetNetwork {
        super::builder::edge_network::GetNetwork::new(self.inner.clone()).set_name(name.into())
    }

    /// Get the diagnostics of a single network resource.
    pub fn diagnose_network(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::DiagnoseNetwork {
        super::builder::edge_network::DiagnoseNetwork::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a new Network in a given project and location.
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
    pub fn create_network(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::edge_network::CreateNetwork {
        super::builder::edge_network::CreateNetwork::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a single Network.
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
    pub fn delete_network(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::DeleteNetwork {
        super::builder::edge_network::DeleteNetwork::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists Subnets in a given project and location.
    pub fn list_subnets(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::edge_network::ListSubnets {
        super::builder::edge_network::ListSubnets::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets details of a single Subnet.
    pub fn get_subnet(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::GetSubnet {
        super::builder::edge_network::GetSubnet::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a new Subnet in a given project and location.
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
    pub fn create_subnet(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::edge_network::CreateSubnet {
        super::builder::edge_network::CreateSubnet::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a single Subnet.
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
    pub fn update_subnet(
        &self,
        subnet: impl Into<crate::model::Subnet>,
    ) -> super::builder::edge_network::UpdateSubnet {
        super::builder::edge_network::UpdateSubnet::new(self.inner.clone())
            .set_subnet(subnet.into())
    }

    /// Deletes a single Subnet.
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
    pub fn delete_subnet(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::DeleteSubnet {
        super::builder::edge_network::DeleteSubnet::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists Interconnects in a given project and location.
    pub fn list_interconnects(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::edge_network::ListInterconnects {
        super::builder::edge_network::ListInterconnects::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single Interconnect.
    pub fn get_interconnect(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::GetInterconnect {
        super::builder::edge_network::GetInterconnect::new(self.inner.clone()).set_name(name.into())
    }

    /// Get the diagnostics of a single interconnect resource.
    pub fn diagnose_interconnect(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::DiagnoseInterconnect {
        super::builder::edge_network::DiagnoseInterconnect::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists InterconnectAttachments in a given project and location.
    pub fn list_interconnect_attachments(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::edge_network::ListInterconnectAttachments {
        super::builder::edge_network::ListInterconnectAttachments::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single InterconnectAttachment.
    pub fn get_interconnect_attachment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::GetInterconnectAttachment {
        super::builder::edge_network::GetInterconnectAttachment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new InterconnectAttachment in a given project and location.
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
    pub fn create_interconnect_attachment(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::edge_network::CreateInterconnectAttachment {
        super::builder::edge_network::CreateInterconnectAttachment::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a single InterconnectAttachment.
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
    pub fn delete_interconnect_attachment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::DeleteInterconnectAttachment {
        super::builder::edge_network::DeleteInterconnectAttachment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists Routers in a given project and location.
    pub fn list_routers(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::edge_network::ListRouters {
        super::builder::edge_network::ListRouters::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets details of a single Router.
    pub fn get_router(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::GetRouter {
        super::builder::edge_network::GetRouter::new(self.inner.clone()).set_name(name.into())
    }

    /// Get the diagnostics of a single router resource.
    pub fn diagnose_router(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::DiagnoseRouter {
        super::builder::edge_network::DiagnoseRouter::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a new Router in a given project and location.
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
    pub fn create_router(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::edge_network::CreateRouter {
        super::builder::edge_network::CreateRouter::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a single Router.
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
    pub fn update_router(
        &self,
        router: impl Into<crate::model::Router>,
    ) -> super::builder::edge_network::UpdateRouter {
        super::builder::edge_network::UpdateRouter::new(self.inner.clone())
            .set_router(router.into())
    }

    /// Deletes a single Router.
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
    pub fn delete_router(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::DeleteRouter {
        super::builder::edge_network::DeleteRouter::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::ListLocations {
        super::builder::edge_network::ListLocations::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::GetLocation {
        super::builder::edge_network::GetLocation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::ListOperations {
        super::builder::edge_network::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::GetOperation {
        super::builder::edge_network::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::DeleteOperation {
        super::builder::edge_network::DeleteOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::edge_network::CancelOperation {
        super::builder::edge_network::CancelOperation::new(self.inner.clone()).set_name(name.into())
    }
}
