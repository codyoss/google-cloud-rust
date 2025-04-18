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

/// Implements a client for the Cloud Video Intelligence API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_videointelligence_v1::client::VideoIntelligenceService;
/// let client = VideoIntelligenceService::builder().build().await?;
/// // use `client` to make requests to the Cloud Video Intelligence API.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Service that implements the Video Intelligence API.
///
/// # Configuration
///
/// To configure `VideoIntelligenceService` use the `with_*` methods in the type returned
/// by [builder()][VideoIntelligenceService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://videointelligence.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::video_intelligence_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::video_intelligence_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `VideoIntelligenceService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `VideoIntelligenceService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct VideoIntelligenceService {
    inner: Arc<dyn super::stub::dynamic::VideoIntelligenceService>,
}

impl VideoIntelligenceService {
    /// Returns a builder for [VideoIntelligenceService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_videointelligence_v1::client::VideoIntelligenceService;
    /// let client = VideoIntelligenceService::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::video_intelligence_service::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::video_intelligence_service::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::VideoIntelligenceService + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::VideoIntelligenceService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::VideoIntelligenceService> {
        super::transport::VideoIntelligenceService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::VideoIntelligenceService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::VideoIntelligenceService::new)
    }

    /// Performs asynchronous video annotation. Progress and results can be
    /// retrieved through the `google.longrunning.Operations` interface.
    /// `Operation.metadata` contains `AnnotateVideoProgress` (progress).
    /// `Operation.response` contains `AnnotateVideoResponse` (results).
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
    pub fn annotate_video(&self) -> super::builder::video_intelligence_service::AnnotateVideo {
        super::builder::video_intelligence_service::AnnotateVideo::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::video_intelligence_service::ListOperations {
        super::builder::video_intelligence_service::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::video_intelligence_service::GetOperation {
        super::builder::video_intelligence_service::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::video_intelligence_service::DeleteOperation {
        super::builder::video_intelligence_service::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::video_intelligence_service::CancelOperation {
        super::builder::video_intelligence_service::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
