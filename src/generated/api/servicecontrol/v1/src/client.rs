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

/// Implements a client for the Service Control API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_api_servicecontrol_v1::client::QuotaController;
/// let client = QuotaController::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// [Google Quota Control API](/service-control/overview)
///
/// Allows clients to allocate and release quota against a [managed
/// service](https://cloud.google.com/service-management/reference/rpc/google.api/servicemanagement.v1#google.api.servicemanagement.v1.ManagedService).
///
/// # Configuration
///
/// To configure `QuotaController` use the `with_*` methods in the type returned
/// by [builder()][QuotaController::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://servicecontrol.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::quota_controller::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::quota_controller::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `QuotaController` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `QuotaController` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct QuotaController {
    inner: Arc<dyn super::stub::dynamic::QuotaController>,
}

impl QuotaController {
    /// Returns a builder for [QuotaController].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_api_servicecontrol_v1::client::QuotaController;
    /// let client = QuotaController::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::quota_controller::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::quota_controller::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::QuotaController + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::QuotaController>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::QuotaController> {
        super::transport::QuotaController::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::QuotaController> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::QuotaController::new)
    }

    /// Attempts to allocate quota for the specified consumer. It should be called
    /// before the operation is executed.
    ///
    /// This method requires the `servicemanagement.services.quota`
    /// permission on the specified service. For more information, see
    /// [Cloud IAM](https://cloud.google.com/iam).
    ///
    /// **NOTE:** The client **must** fail-open on server errors `INTERNAL`,
    /// `UNKNOWN`, `DEADLINE_EXCEEDED`, and `UNAVAILABLE`. To ensure system
    /// reliability, the server may inject these errors to prohibit any hard
    /// dependency on the quota functionality.
    pub fn allocate_quota(
        &self,
        service_name: impl Into<std::string::String>,
    ) -> super::builder::quota_controller::AllocateQuota {
        super::builder::quota_controller::AllocateQuota::new(self.inner.clone())
            .set_service_name(service_name.into())
    }
}

/// Implements a client for the Service Control API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_api_servicecontrol_v1::client::ServiceController;
/// let client = ServiceController::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// [Google Service Control API](/service-control/overview)
///
/// Lets clients check and report operations against a [managed
/// service](https://cloud.google.com/service-management/reference/rpc/google.api/servicemanagement.v1#google.api.servicemanagement.v1.ManagedService).
///
/// # Configuration
///
/// To configure `ServiceController` use the `with_*` methods in the type returned
/// by [builder()][ServiceController::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://servicecontrol.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::service_controller::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::service_controller::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `ServiceController` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `ServiceController` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct ServiceController {
    inner: Arc<dyn super::stub::dynamic::ServiceController>,
}

impl ServiceController {
    /// Returns a builder for [ServiceController].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_api_servicecontrol_v1::client::ServiceController;
    /// let client = ServiceController::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::service_controller::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::service_controller::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::ServiceController + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::ServiceController>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::ServiceController> {
        super::transport::ServiceController::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::ServiceController> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::ServiceController::new)
    }

    /// Checks whether an operation on a service should be allowed to proceed
    /// based on the configuration of the service and related policies. It must be
    /// called before the operation is executed.
    ///
    /// If feasible, the client should cache the check results and reuse them for
    /// 60 seconds. In case of any server errors, the client should rely on the
    /// cached results for much longer time to avoid outage.
    /// WARNING: There is general 60s delay for the configuration and policy
    /// propagation, therefore callers MUST NOT depend on the `Check` method having
    /// the latest policy information.
    ///
    /// NOTE: the [CheckRequest][google.api.servicecontrol.v1.CheckRequest] has
    /// the size limit (wire-format byte size) of 1MB.
    ///
    /// This method requires the `servicemanagement.services.check` permission
    /// on the specified service. For more information, see
    /// [Cloud IAM](https://cloud.google.com/iam).
    ///
    /// [google.api.servicecontrol.v1.CheckRequest]: crate::model::CheckRequest
    pub fn check(
        &self,
        service_name: impl Into<std::string::String>,
    ) -> super::builder::service_controller::Check {
        super::builder::service_controller::Check::new(self.inner.clone())
            .set_service_name(service_name.into())
    }

    /// Reports operation results to Google Service Control, such as logs and
    /// metrics. It should be called after an operation is completed.
    ///
    /// If feasible, the client should aggregate reporting data for up to 5
    /// seconds to reduce API traffic. Limiting aggregation to 5 seconds is to
    /// reduce data loss during client crashes. Clients should carefully choose
    /// the aggregation time window to avoid data loss risk more than 0.01%
    /// for business and compliance reasons.
    ///
    /// NOTE: the [ReportRequest][google.api.servicecontrol.v1.ReportRequest] has
    /// the size limit (wire-format byte size) of 1MB.
    ///
    /// This method requires the `servicemanagement.services.report` permission
    /// on the specified service. For more information, see
    /// [Google Cloud IAM](https://cloud.google.com/iam).
    ///
    /// [google.api.servicecontrol.v1.ReportRequest]: crate::model::ReportRequest
    pub fn report(
        &self,
        service_name: impl Into<std::string::String>,
    ) -> super::builder::service_controller::Report {
        super::builder::service_controller::Report::new(self.inner.clone())
            .set_service_name(service_name.into())
    }
}
