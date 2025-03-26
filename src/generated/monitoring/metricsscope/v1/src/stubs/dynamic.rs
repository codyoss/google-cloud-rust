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

use std::sync::Arc;

/// A dyn-compatible, crate-private version of [super::MetricsScopes].
#[async_trait::async_trait]
pub trait MetricsScopes: std::fmt::Debug + Send + Sync {
    async fn get_metrics_scope(
        &self,
        req: crate::model::GetMetricsScopeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MetricsScope>;

    async fn list_metrics_scopes_by_monitored_project(
        &self,
        req: crate::model::ListMetricsScopesByMonitoredProjectRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListMetricsScopesByMonitoredProjectResponse>;

    async fn create_monitored_project(
        &self,
        req: crate::model::CreateMonitoredProjectRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_monitored_project(
        &self,
        req: crate::model::DeleteMonitoredProjectRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::MetricsScopes] also implement [MetricsScopes].
#[async_trait::async_trait]
impl<T: super::MetricsScopes> MetricsScopes for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn get_metrics_scope(
        &self,
        req: crate::model::GetMetricsScopeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MetricsScope> {
        T::get_metrics_scope(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_metrics_scopes_by_monitored_project(
        &self,
        req: crate::model::ListMetricsScopesByMonitoredProjectRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListMetricsScopesByMonitoredProjectResponse> {
        T::list_metrics_scopes_by_monitored_project(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_monitored_project(
        &self,
        req: crate::model::CreateMonitoredProjectRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_monitored_project(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_monitored_project(
        &self,
        req: crate::model::DeleteMonitoredProjectRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_monitored_project(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        T::get_polling_error_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}
