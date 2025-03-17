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
/// # Service Description
///
/// Manages Stackdriver dashboards. A dashboard is an arrangement of data display
/// widgets in a specific layout.
///
/// # Configuration
///
/// `DashboardsService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `DashboardsService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `DashboardsService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct DashboardsService {
    inner: Arc<dyn super::stubs::dynamic::DashboardsService>,
}

impl DashboardsService {
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
        T: super::stubs::DashboardsService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stubs::dynamic::DashboardsService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::DashboardsService> {
        super::transport::DashboardsService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::DashboardsService> {
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
    ) -> super::builders::dashboards_service::CreateDashboard {
        super::builders::dashboards_service::CreateDashboard::new(self.inner.clone())
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
    ) -> super::builders::dashboards_service::ListDashboards {
        super::builders::dashboards_service::ListDashboards::new(self.inner.clone())
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
    ) -> super::builders::dashboards_service::GetDashboard {
        super::builders::dashboards_service::GetDashboard::new(self.inner.clone())
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
    ) -> super::builders::dashboards_service::DeleteDashboard {
        super::builders::dashboards_service::DeleteDashboard::new(self.inner.clone())
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
    ) -> super::builders::dashboards_service::UpdateDashboard {
        super::builders::dashboards_service::UpdateDashboard::new(self.inner.clone())
            .set_dashboard(dashboard.into())
    }
}
