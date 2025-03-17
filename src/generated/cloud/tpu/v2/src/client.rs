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

/// Implements a client for the Cloud TPU API.
///
/// # Service Description
///
/// Manages TPU nodes and other resources
///
/// TPU API v2
///
/// # Configuration
///
/// `Tpu` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `Tpu` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Tpu` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Tpu {
    inner: Arc<dyn super::stubs::dynamic::Tpu>,
}

impl Tpu {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stubs::Tpu + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stubs::dynamic::Tpu>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gax::options::ClientConfig) -> Result<impl super::stubs::Tpu> {
        super::transport::Tpu::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::Tpu> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::Tpu::new)
    }

    /// Lists nodes.
    pub fn list_nodes(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::tpu::ListNodes {
        super::builders::tpu::ListNodes::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets the details of a node.
    pub fn get_node(&self, name: impl Into<std::string::String>) -> super::builders::tpu::GetNode {
        super::builders::tpu::GetNode::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a node.
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
    pub fn create_node(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::tpu::CreateNode {
        super::builders::tpu::CreateNode::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Deletes a node.
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
    pub fn delete_node(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::tpu::DeleteNode {
        super::builders::tpu::DeleteNode::new(self.inner.clone()).set_name(name.into())
    }

    /// Stops a node. This operation is only available with single TPU nodes.
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
    pub fn stop_node(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::tpu::StopNode {
        super::builders::tpu::StopNode::new(self.inner.clone()).set_name(name.into())
    }

    /// Starts a node.
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
    pub fn start_node(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::tpu::StartNode {
        super::builders::tpu::StartNode::new(self.inner.clone()).set_name(name.into())
    }

    /// Updates the configurations of a node.
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
    pub fn update_node(
        &self,
        node: impl Into<crate::model::Node>,
    ) -> super::builders::tpu::UpdateNode {
        super::builders::tpu::UpdateNode::new(self.inner.clone()).set_node(node.into())
    }

    /// Lists queued resources.
    pub fn list_queued_resources(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::tpu::ListQueuedResources {
        super::builders::tpu::ListQueuedResources::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets details of a queued resource.
    pub fn get_queued_resource(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::tpu::GetQueuedResource {
        super::builders::tpu::GetQueuedResource::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a QueuedResource TPU instance.
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
    pub fn create_queued_resource(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::tpu::CreateQueuedResource {
        super::builders::tpu::CreateQueuedResource::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a QueuedResource TPU instance.
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
    pub fn delete_queued_resource(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::tpu::DeleteQueuedResource {
        super::builders::tpu::DeleteQueuedResource::new(self.inner.clone()).set_name(name.into())
    }

    /// Resets a QueuedResource TPU instance
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
    pub fn reset_queued_resource(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::tpu::ResetQueuedResource {
        super::builders::tpu::ResetQueuedResource::new(self.inner.clone()).set_name(name.into())
    }

    /// Generates the Cloud TPU service identity for the project.
    pub fn generate_service_identity(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::tpu::GenerateServiceIdentity {
        super::builders::tpu::GenerateServiceIdentity::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists accelerator types supported by this API.
    pub fn list_accelerator_types(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::tpu::ListAcceleratorTypes {
        super::builders::tpu::ListAcceleratorTypes::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets AcceleratorType.
    pub fn get_accelerator_type(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::tpu::GetAcceleratorType {
        super::builders::tpu::GetAcceleratorType::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists runtime versions supported by this API.
    pub fn list_runtime_versions(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::tpu::ListRuntimeVersions {
        super::builders::tpu::ListRuntimeVersions::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets a runtime version.
    pub fn get_runtime_version(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::tpu::GetRuntimeVersion {
        super::builders::tpu::GetRuntimeVersion::new(self.inner.clone()).set_name(name.into())
    }

    /// Retrieves the guest attributes for the node.
    pub fn get_guest_attributes(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::tpu::GetGuestAttributes {
        super::builders::tpu::GetGuestAttributes::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::tpu::ListLocations {
        super::builders::tpu::ListLocations::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::tpu::GetLocation {
        super::builders::tpu::GetLocation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::tpu::ListOperations {
        super::builders::tpu::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::tpu::GetOperation {
        super::builders::tpu::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::tpu::DeleteOperation {
        super::builders::tpu::DeleteOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::tpu::CancelOperation {
        super::builders::tpu::CancelOperation::new(self.inner.clone()).set_name(name.into())
    }
}
