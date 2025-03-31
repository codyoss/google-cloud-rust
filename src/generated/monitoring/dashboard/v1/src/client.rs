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

/// Implements a client for the Cloud Monitoring API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_monitoring_dashboard_v1::client::DashboardsService;
/// let client = DashboardsService::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Manages Stackdriver dashboards. A dashboard is an arrangement of data display
/// widgets in a specific layout.
///
/// # Configuration
///
/// To configure `DashboardsService` use the `with_*` methods in the type returned
/// by [builder()][DashboardsService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://monitoring.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::dashboards_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::dashboards_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `DashboardsService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `DashboardsService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct DashboardsService {
    inner: Arc<dyn super::stub::dynamic::DashboardsService>,
}

impl DashboardsService {
    /// Returns a builder for [DashboardsService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_monitoring_dashboard_v1::client::DashboardsService;
    /// let client = DashboardsService::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::dashboards_service::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::dashboards_service::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::DashboardsService + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::DashboardsService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::DashboardsService> {
        super::transport::DashboardsService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::DashboardsService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::DashboardsService::new)
    }

    /// Creates a new custom dashboard. For examples on how you can use this API to
    /// create dashboards, see [Managing dashboards by
    /// API](https://cloud.google.com/monitoring/dashboards/api-dashboard). This
    /// method requires the `monitoring.dashboards.create` permission on the
    /// specified project. For more information about permissions, see [Cloud
    /// Identity and Access Management](https://cloud.google.com/iam).
    pub fn create_dashboard(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dashboards_service::CreateDashboard {
        super::builder::dashboards_service::CreateDashboard::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists the existing dashboards.
    ///
    /// This method requires the `monitoring.dashboards.list` permission
    /// on the specified project. For more information, see
    /// [Cloud Identity and Access Management](https://cloud.google.com/iam).
    pub fn list_dashboards(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dashboards_service::ListDashboards {
        super::builder::dashboards_service::ListDashboards::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Fetches a specific dashboard.
    ///
    /// This method requires the `monitoring.dashboards.get` permission
    /// on the specified dashboard. For more information, see
    /// [Cloud Identity and Access Management](https://cloud.google.com/iam).
    pub fn get_dashboard(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dashboards_service::GetDashboard {
        super::builder::dashboards_service::GetDashboard::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes an existing custom dashboard.
    ///
    /// This method requires the `monitoring.dashboards.delete` permission
    /// on the specified dashboard. For more information, see
    /// [Cloud Identity and Access Management](https://cloud.google.com/iam).
    pub fn delete_dashboard(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dashboards_service::DeleteDashboard {
        super::builder::dashboards_service::DeleteDashboard::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Replaces an existing custom dashboard with a new definition.
    ///
    /// This method requires the `monitoring.dashboards.update` permission
    /// on the specified dashboard. For more information, see
    /// [Cloud Identity and Access Management](https://cloud.google.com/iam).
    pub fn update_dashboard(
        &self,
        dashboard: impl Into<crate::model::Dashboard>,
    ) -> super::builder::dashboards_service::UpdateDashboard {
        super::builder::dashboards_service::UpdateDashboard::new(self.inner.clone())
            .set_dashboard(dashboard.into())
    }
}
