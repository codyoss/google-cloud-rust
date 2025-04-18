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

/// Implements a client for the Assured Workloads API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_assuredworkloads_v1::client::AssuredWorkloadsService;
/// let client = AssuredWorkloadsService::builder().build().await?;
/// // use `client` to make requests to the Assured Workloads API.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Service to manage AssuredWorkloads.
///
/// # Configuration
///
/// To configure `AssuredWorkloadsService` use the `with_*` methods in the type returned
/// by [builder()][AssuredWorkloadsService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://assuredworkloads.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::assured_workloads_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::assured_workloads_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `AssuredWorkloadsService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `AssuredWorkloadsService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct AssuredWorkloadsService {
    inner: Arc<dyn super::stub::dynamic::AssuredWorkloadsService>,
}

impl AssuredWorkloadsService {
    /// Returns a builder for [AssuredWorkloadsService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_assuredworkloads_v1::client::AssuredWorkloadsService;
    /// let client = AssuredWorkloadsService::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::assured_workloads_service::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::assured_workloads_service::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::AssuredWorkloadsService + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::AssuredWorkloadsService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::AssuredWorkloadsService> {
        super::transport::AssuredWorkloadsService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::AssuredWorkloadsService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::AssuredWorkloadsService::new)
    }

    /// Creates Assured Workload.
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
    pub fn create_workload(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::assured_workloads_service::CreateWorkload {
        super::builder::assured_workloads_service::CreateWorkload::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates an existing workload.
    /// Currently allows updating of workload display_name and labels.
    /// For force updates don't set etag field in the Workload.
    /// Only one update operation per workload can be in progress.
    pub fn update_workload(
        &self,
        workload: impl Into<crate::model::Workload>,
    ) -> super::builder::assured_workloads_service::UpdateWorkload {
        super::builder::assured_workloads_service::UpdateWorkload::new(self.inner.clone())
            .set_workload(workload.into())
    }

    /// Restrict the list of resources allowed in the Workload environment.
    /// The current list of allowed products can be found at
    /// <https://cloud.google.com/assured-workloads/docs/supported-products>
    /// In addition to assuredworkloads.workload.update permission, the user should
    /// also have orgpolicy.policy.set permission on the folder resource
    /// to use this functionality.
    pub fn restrict_allowed_resources(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::assured_workloads_service::RestrictAllowedResources {
        super::builder::assured_workloads_service::RestrictAllowedResources::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes the workload. Make sure that workload's direct children are already
    /// in a deleted state, otherwise the request will fail with a
    /// FAILED_PRECONDITION error.
    pub fn delete_workload(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::assured_workloads_service::DeleteWorkload {
        super::builder::assured_workloads_service::DeleteWorkload::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets Assured Workload associated with a CRM Node
    pub fn get_workload(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::assured_workloads_service::GetWorkload {
        super::builder::assured_workloads_service::GetWorkload::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists Assured Workloads under a CRM Node.
    pub fn list_workloads(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::assured_workloads_service::ListWorkloads {
        super::builder::assured_workloads_service::ListWorkloads::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::assured_workloads_service::ListOperations {
        super::builder::assured_workloads_service::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::assured_workloads_service::GetOperation {
        super::builder::assured_workloads_service::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
