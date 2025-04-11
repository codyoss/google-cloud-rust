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

/// Implements a [Environments](super::stub::Environments) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Environments<T>
where
    T: super::stub::Environments + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Environments<T>
where
    T: super::stub::Environments + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::Environments for Environments<T>
where
    T: super::stub::Environments + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_environment(
        &self,
        req: crate::model::CreateEnvironmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_environment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_environment(
        &self,
        req: crate::model::GetEnvironmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Environment>> {
        self.inner.get_environment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_environments(
        &self,
        req: crate::model::ListEnvironmentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListEnvironmentsResponse>> {
        self.inner.list_environments(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_environment(
        &self,
        req: crate::model::UpdateEnvironmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_environment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_environment(
        &self,
        req: crate::model::DeleteEnvironmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_environment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn execute_airflow_command(
        &self,
        req: crate::model::ExecuteAirflowCommandRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ExecuteAirflowCommandResponse>> {
        self.inner.execute_airflow_command(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn stop_airflow_command(
        &self,
        req: crate::model::StopAirflowCommandRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::StopAirflowCommandResponse>> {
        self.inner.stop_airflow_command(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn poll_airflow_command(
        &self,
        req: crate::model::PollAirflowCommandRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::PollAirflowCommandResponse>> {
        self.inner.poll_airflow_command(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_workloads(
        &self,
        req: crate::model::ListWorkloadsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListWorkloadsResponse>> {
        self.inner.list_workloads(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn check_upgrade(
        &self,
        req: crate::model::CheckUpgradeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.check_upgrade(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_user_workloads_secret(
        &self,
        req: crate::model::CreateUserWorkloadsSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::UserWorkloadsSecret>> {
        self.inner.create_user_workloads_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_user_workloads_secret(
        &self,
        req: crate::model::GetUserWorkloadsSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::UserWorkloadsSecret>> {
        self.inner.get_user_workloads_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_user_workloads_secrets(
        &self,
        req: crate::model::ListUserWorkloadsSecretsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListUserWorkloadsSecretsResponse>> {
        self.inner.list_user_workloads_secrets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_user_workloads_secret(
        &self,
        req: crate::model::UpdateUserWorkloadsSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::UserWorkloadsSecret>> {
        self.inner.update_user_workloads_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_user_workloads_secret(
        &self,
        req: crate::model::DeleteUserWorkloadsSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_user_workloads_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_user_workloads_config_map(
        &self,
        req: crate::model::CreateUserWorkloadsConfigMapRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::UserWorkloadsConfigMap>> {
        self.inner
            .create_user_workloads_config_map(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn get_user_workloads_config_map(
        &self,
        req: crate::model::GetUserWorkloadsConfigMapRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::UserWorkloadsConfigMap>> {
        self.inner.get_user_workloads_config_map(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_user_workloads_config_maps(
        &self,
        req: crate::model::ListUserWorkloadsConfigMapsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListUserWorkloadsConfigMapsResponse>> {
        self.inner
            .list_user_workloads_config_maps(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn update_user_workloads_config_map(
        &self,
        req: crate::model::UpdateUserWorkloadsConfigMapRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::UserWorkloadsConfigMap>> {
        self.inner
            .update_user_workloads_config_map(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn delete_user_workloads_config_map(
        &self,
        req: crate::model::DeleteUserWorkloadsConfigMapRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner
            .delete_user_workloads_config_map(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn save_snapshot(
        &self,
        req: crate::model::SaveSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.save_snapshot(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn load_snapshot(
        &self,
        req: crate::model::LoadSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.load_snapshot(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn database_failover(
        &self,
        req: crate::model::DatabaseFailoverRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.database_failover(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn fetch_database_properties(
        &self,
        req: crate::model::FetchDatabasePropertiesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::FetchDatabasePropertiesResponse>> {
        self.inner.fetch_database_properties(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_operation(req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        self.inner.get_polling_error_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}

/// Implements a [ImageVersions](super::stub::ImageVersions) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct ImageVersions<T>
where
    T: super::stub::ImageVersions + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> ImageVersions<T>
where
    T: super::stub::ImageVersions + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::ImageVersions for ImageVersions<T>
where
    T: super::stub::ImageVersions + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_image_versions(
        &self,
        req: crate::model::ListImageVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListImageVersionsResponse>> {
        self.inner.list_image_versions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_operation(req, options).await
    }
}
