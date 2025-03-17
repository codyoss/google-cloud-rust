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
use crate::Result;

/// Implements a [AppConnectionsService](super::stubs::AppConnectionsService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct AppConnectionsService<T>
where
    T: super::stubs::AppConnectionsService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> AppConnectionsService<T>
where
    T: super::stubs::AppConnectionsService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::AppConnectionsService for AppConnectionsService<T>
where
    T: super::stubs::AppConnectionsService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_app_connections(
        &self,
        req: crate::model::ListAppConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAppConnectionsResponse> {
        self.inner.list_app_connections(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_app_connection(
        &self,
        req: crate::model::GetAppConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AppConnection> {
        self.inner.get_app_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_app_connection(
        &self,
        req: crate::model::CreateAppConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_app_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_app_connection(
        &self,
        req: crate::model::UpdateAppConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_app_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_app_connection(
        &self,
        req: crate::model::DeleteAppConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_app_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn resolve_app_connections(
        &self,
        req: crate::model::ResolveAppConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ResolveAppConnectionsResponse> {
        self.inner.resolve_app_connections(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_operation(req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy> {
        self.inner.get_polling_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
