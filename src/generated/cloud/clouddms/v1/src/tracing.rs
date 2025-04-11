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

/// Implements a [DataMigrationService](super::stub::DataMigrationService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct DataMigrationService<T>
where
    T: super::stub::DataMigrationService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> DataMigrationService<T>
where
    T: super::stub::DataMigrationService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::DataMigrationService for DataMigrationService<T>
where
    T: super::stub::DataMigrationService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_migration_jobs(
        &self,
        req: crate::model::ListMigrationJobsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListMigrationJobsResponse>> {
        self.inner.list_migration_jobs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_migration_job(
        &self,
        req: crate::model::GetMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::MigrationJob>> {
        self.inner.get_migration_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_migration_job(
        &self,
        req: crate::model::CreateMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_migration_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_migration_job(
        &self,
        req: crate::model::UpdateMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_migration_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_migration_job(
        &self,
        req: crate::model::DeleteMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_migration_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn start_migration_job(
        &self,
        req: crate::model::StartMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.start_migration_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn stop_migration_job(
        &self,
        req: crate::model::StopMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.stop_migration_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn resume_migration_job(
        &self,
        req: crate::model::ResumeMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.resume_migration_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn promote_migration_job(
        &self,
        req: crate::model::PromoteMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.promote_migration_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn verify_migration_job(
        &self,
        req: crate::model::VerifyMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.verify_migration_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn restart_migration_job(
        &self,
        req: crate::model::RestartMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.restart_migration_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn generate_ssh_script(
        &self,
        req: crate::model::GenerateSshScriptRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SshScript>> {
        self.inner.generate_ssh_script(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn generate_tcp_proxy_script(
        &self,
        req: crate::model::GenerateTcpProxyScriptRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::TcpProxyScript>> {
        self.inner.generate_tcp_proxy_script(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_connection_profiles(
        &self,
        req: crate::model::ListConnectionProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListConnectionProfilesResponse>> {
        self.inner.list_connection_profiles(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_connection_profile(
        &self,
        req: crate::model::GetConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ConnectionProfile>> {
        self.inner.get_connection_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_connection_profile(
        &self,
        req: crate::model::CreateConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_connection_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_connection_profile(
        &self,
        req: crate::model::UpdateConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_connection_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_connection_profile(
        &self,
        req: crate::model::DeleteConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_connection_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_private_connection(
        &self,
        req: crate::model::CreatePrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_private_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_private_connection(
        &self,
        req: crate::model::GetPrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::PrivateConnection>> {
        self.inner.get_private_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_private_connections(
        &self,
        req: crate::model::ListPrivateConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListPrivateConnectionsResponse>> {
        self.inner.list_private_connections(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_private_connection(
        &self,
        req: crate::model::DeletePrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_private_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_conversion_workspace(
        &self,
        req: crate::model::GetConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ConversionWorkspace>> {
        self.inner.get_conversion_workspace(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_conversion_workspaces(
        &self,
        req: crate::model::ListConversionWorkspacesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListConversionWorkspacesResponse>> {
        self.inner.list_conversion_workspaces(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_conversion_workspace(
        &self,
        req: crate::model::CreateConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_conversion_workspace(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_conversion_workspace(
        &self,
        req: crate::model::UpdateConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_conversion_workspace(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_conversion_workspace(
        &self,
        req: crate::model::DeleteConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_conversion_workspace(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_mapping_rule(
        &self,
        req: crate::model::CreateMappingRuleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::MappingRule>> {
        self.inner.create_mapping_rule(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_mapping_rule(
        &self,
        req: crate::model::DeleteMappingRuleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_mapping_rule(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_mapping_rules(
        &self,
        req: crate::model::ListMappingRulesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListMappingRulesResponse>> {
        self.inner.list_mapping_rules(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_mapping_rule(
        &self,
        req: crate::model::GetMappingRuleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::MappingRule>> {
        self.inner.get_mapping_rule(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn seed_conversion_workspace(
        &self,
        req: crate::model::SeedConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.seed_conversion_workspace(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn import_mapping_rules(
        &self,
        req: crate::model::ImportMappingRulesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.import_mapping_rules(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn convert_conversion_workspace(
        &self,
        req: crate::model::ConvertConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.convert_conversion_workspace(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn commit_conversion_workspace(
        &self,
        req: crate::model::CommitConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.commit_conversion_workspace(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn rollback_conversion_workspace(
        &self,
        req: crate::model::RollbackConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.rollback_conversion_workspace(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn apply_conversion_workspace(
        &self,
        req: crate::model::ApplyConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.apply_conversion_workspace(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn describe_database_entities(
        &self,
        req: crate::model::DescribeDatabaseEntitiesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::DescribeDatabaseEntitiesResponse>> {
        self.inner.describe_database_entities(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn search_background_jobs(
        &self,
        req: crate::model::SearchBackgroundJobsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SearchBackgroundJobsResponse>> {
        self.inner.search_background_jobs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn describe_conversion_workspace_revisions(
        &self,
        req: crate::model::DescribeConversionWorkspaceRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::DescribeConversionWorkspaceRevisionsResponse>>
    {
        self.inner
            .describe_conversion_workspace_revisions(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn fetch_static_ips(
        &self,
        req: crate::model::FetchStaticIpsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::FetchStaticIpsResponse>> {
        self.inner.fetch_static_ips(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::ListLocationsResponse>> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::Location>> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>> {
        self.inner.test_iam_permissions(req, options).await
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

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.cancel_operation(req, options).await
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
