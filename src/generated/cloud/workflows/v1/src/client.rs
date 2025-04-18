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
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::bare_urls)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Workflows API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_workflows_v1::client::Workflows;
/// let client = Workflows::builder().build().await?;
/// // use `client` to make requests to the Workflows API.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Workflows is used to deploy and execute workflow programs.
/// Workflows makes sure the program executes reliably, despite hardware and
/// networking interruptions.
///
/// # Configuration
///
/// To configure `Workflows` use the `with_*` methods in the type returned
/// by [builder()][Workflows::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://workflows.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::workflows::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::workflows::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `Workflows` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Workflows` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Workflows {
    inner: Arc<dyn super::stub::dynamic::Workflows>,
}

impl Workflows {
    /// Returns a builder for [Workflows].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_workflows_v1::client::Workflows;
    /// let client = Workflows::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::workflows::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::workflows::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::Workflows + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::Workflows>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::Workflows> {
        super::transport::Workflows::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::Workflows> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::Workflows::new)
    }

    /// Lists workflows in a given project and location.
    /// The default order is not specified.
    pub fn list_workflows(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::workflows::ListWorkflows {
        super::builder::workflows::ListWorkflows::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets details of a single workflow.
    pub fn get_workflow(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::workflows::GetWorkflow {
        super::builder::workflows::GetWorkflow::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a new workflow. If a workflow with the specified name already
    /// exists in the specified project and location, the long running operation
    /// returns a [ALREADY_EXISTS][google.rpc.Code.ALREADY_EXISTS] error.
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
    pub fn create_workflow(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::workflows::CreateWorkflow {
        super::builder::workflows::CreateWorkflow::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Deletes a workflow with the specified name.
    /// This method also cancels and deletes all running executions of the
    /// workflow.
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
    pub fn delete_workflow(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::workflows::DeleteWorkflow {
        super::builder::workflows::DeleteWorkflow::new(self.inner.clone()).set_name(name.into())
    }

    /// Updates an existing workflow.
    /// Running this method has no impact on already running executions of the
    /// workflow. A new revision of the workflow might be created as a result of a
    /// successful update operation. In that case, the new revision is used
    /// in new workflow executions.
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
    pub fn update_workflow(
        &self,
        workflow: impl Into<crate::model::Workflow>,
    ) -> super::builder::workflows::UpdateWorkflow {
        super::builder::workflows::UpdateWorkflow::new(self.inner.clone())
            .set_workflow(workflow.into())
    }

    /// Lists revisions for a given workflow.
    pub fn list_workflow_revisions(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::workflows::ListWorkflowRevisions {
        super::builder::workflows::ListWorkflowRevisions::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::workflows::ListLocations {
        super::builder::workflows::ListLocations::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::workflows::GetLocation {
        super::builder::workflows::GetLocation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::workflows::ListOperations {
        super::builder::workflows::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::workflows::GetOperation {
        super::builder::workflows::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::workflows::DeleteOperation {
        super::builder::workflows::DeleteOperation::new(self.inner.clone()).set_name(name.into())
    }
}
