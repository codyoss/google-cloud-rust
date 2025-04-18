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

/// Implements a client for the Storage Batch Operations API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_storagebatchoperations_v1::client::StorageBatchOperations;
/// let client = StorageBatchOperations::builder().build().await?;
/// // use `client` to make requests to the Storage Batch Operations API.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Storage Batch Operations offers a managed experience to perform batch
/// operations on millions of Cloud Storage objects in a serverless fashion. With
/// this service, you can automate and simplify large scale API operations
/// performed on Cloud Storage objects.
///
/// # Configuration
///
/// To configure `StorageBatchOperations` use the `with_*` methods in the type returned
/// by [builder()][StorageBatchOperations::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://storagebatchoperations.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::storage_batch_operations::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::storage_batch_operations::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `StorageBatchOperations` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `StorageBatchOperations` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct StorageBatchOperations {
    inner: Arc<dyn super::stub::dynamic::StorageBatchOperations>,
}

impl StorageBatchOperations {
    /// Returns a builder for [StorageBatchOperations].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_storagebatchoperations_v1::client::StorageBatchOperations;
    /// let client = StorageBatchOperations::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::storage_batch_operations::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::storage_batch_operations::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::StorageBatchOperations + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::StorageBatchOperations>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::StorageBatchOperations> {
        super::transport::StorageBatchOperations::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::StorageBatchOperations> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::StorageBatchOperations::new)
    }

    /// Lists Jobs in a given project and location.
    pub fn list_jobs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::storage_batch_operations::ListJobs {
        super::builder::storage_batch_operations::ListJobs::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a batch job.
    pub fn get_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::storage_batch_operations::GetJob {
        super::builder::storage_batch_operations::GetJob::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a batch job.
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
    pub fn create_job(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::storage_batch_operations::CreateJob {
        super::builder::storage_batch_operations::CreateJob::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a batch job.
    pub fn delete_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::storage_batch_operations::DeleteJob {
        super::builder::storage_batch_operations::DeleteJob::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Cancels a batch job.
    pub fn cancel_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::storage_batch_operations::CancelJob {
        super::builder::storage_batch_operations::CancelJob::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::storage_batch_operations::ListLocations {
        super::builder::storage_batch_operations::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::storage_batch_operations::GetLocation {
        super::builder::storage_batch_operations::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::storage_batch_operations::ListOperations {
        super::builder::storage_batch_operations::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::storage_batch_operations::GetOperation {
        super::builder::storage_batch_operations::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::storage_batch_operations::DeleteOperation {
        super::builder::storage_batch_operations::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::storage_batch_operations::CancelOperation {
        super::builder::storage_batch_operations::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
