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

/// A dyn-compatible, crate-private version of [super::DataMigrationService].
#[async_trait::async_trait]
pub trait DataMigrationService: std::fmt::Debug + Send + Sync {
    async fn list_migration_jobs(
        &self,
        req: crate::model::ListMigrationJobsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListMigrationJobsResponse>;

    async fn get_migration_job(
        &self,
        req: crate::model::GetMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MigrationJob>;

    async fn create_migration_job(
        &self,
        req: crate::model::CreateMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_migration_job(
        &self,
        req: crate::model::UpdateMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_migration_job(
        &self,
        req: crate::model::DeleteMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn start_migration_job(
        &self,
        req: crate::model::StartMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn stop_migration_job(
        &self,
        req: crate::model::StopMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn resume_migration_job(
        &self,
        req: crate::model::ResumeMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn promote_migration_job(
        &self,
        req: crate::model::PromoteMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn verify_migration_job(
        &self,
        req: crate::model::VerifyMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn restart_migration_job(
        &self,
        req: crate::model::RestartMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn generate_ssh_script(
        &self,
        req: crate::model::GenerateSshScriptRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SshScript>;

    async fn generate_tcp_proxy_script(
        &self,
        req: crate::model::GenerateTcpProxyScriptRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TcpProxyScript>;

    async fn list_connection_profiles(
        &self,
        req: crate::model::ListConnectionProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConnectionProfilesResponse>;

    async fn get_connection_profile(
        &self,
        req: crate::model::GetConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ConnectionProfile>;

    async fn create_connection_profile(
        &self,
        req: crate::model::CreateConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_connection_profile(
        &self,
        req: crate::model::UpdateConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_connection_profile(
        &self,
        req: crate::model::DeleteConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_private_connection(
        &self,
        req: crate::model::CreatePrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_private_connection(
        &self,
        req: crate::model::GetPrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PrivateConnection>;

    async fn list_private_connections(
        &self,
        req: crate::model::ListPrivateConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPrivateConnectionsResponse>;

    async fn delete_private_connection(
        &self,
        req: crate::model::DeletePrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_conversion_workspace(
        &self,
        req: crate::model::GetConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ConversionWorkspace>;

    async fn list_conversion_workspaces(
        &self,
        req: crate::model::ListConversionWorkspacesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConversionWorkspacesResponse>;

    async fn create_conversion_workspace(
        &self,
        req: crate::model::CreateConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_conversion_workspace(
        &self,
        req: crate::model::UpdateConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_conversion_workspace(
        &self,
        req: crate::model::DeleteConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_mapping_rule(
        &self,
        req: crate::model::CreateMappingRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MappingRule>;

    async fn delete_mapping_rule(
        &self,
        req: crate::model::DeleteMappingRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn list_mapping_rules(
        &self,
        req: crate::model::ListMappingRulesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListMappingRulesResponse>;

    async fn get_mapping_rule(
        &self,
        req: crate::model::GetMappingRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MappingRule>;

    async fn seed_conversion_workspace(
        &self,
        req: crate::model::SeedConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn import_mapping_rules(
        &self,
        req: crate::model::ImportMappingRulesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn convert_conversion_workspace(
        &self,
        req: crate::model::ConvertConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn commit_conversion_workspace(
        &self,
        req: crate::model::CommitConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn rollback_conversion_workspace(
        &self,
        req: crate::model::RollbackConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn apply_conversion_workspace(
        &self,
        req: crate::model::ApplyConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn describe_database_entities(
        &self,
        req: crate::model::DescribeDatabaseEntitiesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DescribeDatabaseEntitiesResponse>;

    async fn search_background_jobs(
        &self,
        req: crate::model::SearchBackgroundJobsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchBackgroundJobsResponse>;

    async fn describe_conversion_workspace_revisions(
        &self,
        req: crate::model::DescribeConversionWorkspaceRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DescribeConversionWorkspaceRevisionsResponse>;

    async fn fetch_static_ips(
        &self,
        req: crate::model::FetchStaticIpsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FetchStaticIpsResponse>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location>;

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::DataMigrationService] also implement [DataMigrationService].
#[async_trait::async_trait]
impl<T: super::DataMigrationService> DataMigrationService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_migration_jobs(
        &self,
        req: crate::model::ListMigrationJobsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListMigrationJobsResponse> {
        T::list_migration_jobs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_migration_job(
        &self,
        req: crate::model::GetMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MigrationJob> {
        T::get_migration_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_migration_job(
        &self,
        req: crate::model::CreateMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_migration_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_migration_job(
        &self,
        req: crate::model::UpdateMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_migration_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_migration_job(
        &self,
        req: crate::model::DeleteMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_migration_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn start_migration_job(
        &self,
        req: crate::model::StartMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::start_migration_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn stop_migration_job(
        &self,
        req: crate::model::StopMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::stop_migration_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn resume_migration_job(
        &self,
        req: crate::model::ResumeMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::resume_migration_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn promote_migration_job(
        &self,
        req: crate::model::PromoteMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::promote_migration_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn verify_migration_job(
        &self,
        req: crate::model::VerifyMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::verify_migration_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn restart_migration_job(
        &self,
        req: crate::model::RestartMigrationJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::restart_migration_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn generate_ssh_script(
        &self,
        req: crate::model::GenerateSshScriptRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SshScript> {
        T::generate_ssh_script(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn generate_tcp_proxy_script(
        &self,
        req: crate::model::GenerateTcpProxyScriptRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TcpProxyScript> {
        T::generate_tcp_proxy_script(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_connection_profiles(
        &self,
        req: crate::model::ListConnectionProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConnectionProfilesResponse> {
        T::list_connection_profiles(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_connection_profile(
        &self,
        req: crate::model::GetConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ConnectionProfile> {
        T::get_connection_profile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_connection_profile(
        &self,
        req: crate::model::CreateConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_connection_profile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_connection_profile(
        &self,
        req: crate::model::UpdateConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_connection_profile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_connection_profile(
        &self,
        req: crate::model::DeleteConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_connection_profile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_private_connection(
        &self,
        req: crate::model::CreatePrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_private_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_private_connection(
        &self,
        req: crate::model::GetPrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PrivateConnection> {
        T::get_private_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_private_connections(
        &self,
        req: crate::model::ListPrivateConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPrivateConnectionsResponse> {
        T::list_private_connections(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_private_connection(
        &self,
        req: crate::model::DeletePrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_private_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_conversion_workspace(
        &self,
        req: crate::model::GetConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ConversionWorkspace> {
        T::get_conversion_workspace(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_conversion_workspaces(
        &self,
        req: crate::model::ListConversionWorkspacesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConversionWorkspacesResponse> {
        T::list_conversion_workspaces(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_conversion_workspace(
        &self,
        req: crate::model::CreateConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_conversion_workspace(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_conversion_workspace(
        &self,
        req: crate::model::UpdateConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_conversion_workspace(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_conversion_workspace(
        &self,
        req: crate::model::DeleteConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_conversion_workspace(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_mapping_rule(
        &self,
        req: crate::model::CreateMappingRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MappingRule> {
        T::create_mapping_rule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_mapping_rule(
        &self,
        req: crate::model::DeleteMappingRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_mapping_rule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_mapping_rules(
        &self,
        req: crate::model::ListMappingRulesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListMappingRulesResponse> {
        T::list_mapping_rules(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_mapping_rule(
        &self,
        req: crate::model::GetMappingRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MappingRule> {
        T::get_mapping_rule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn seed_conversion_workspace(
        &self,
        req: crate::model::SeedConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::seed_conversion_workspace(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn import_mapping_rules(
        &self,
        req: crate::model::ImportMappingRulesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::import_mapping_rules(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn convert_conversion_workspace(
        &self,
        req: crate::model::ConvertConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::convert_conversion_workspace(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn commit_conversion_workspace(
        &self,
        req: crate::model::CommitConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::commit_conversion_workspace(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn rollback_conversion_workspace(
        &self,
        req: crate::model::RollbackConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::rollback_conversion_workspace(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn apply_conversion_workspace(
        &self,
        req: crate::model::ApplyConversionWorkspaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::apply_conversion_workspace(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn describe_database_entities(
        &self,
        req: crate::model::DescribeDatabaseEntitiesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DescribeDatabaseEntitiesResponse> {
        T::describe_database_entities(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn search_background_jobs(
        &self,
        req: crate::model::SearchBackgroundJobsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchBackgroundJobsResponse> {
        T::search_background_jobs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn describe_conversion_workspace_revisions(
        &self,
        req: crate::model::DescribeConversionWorkspaceRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DescribeConversionWorkspaceRevisionsResponse> {
        T::describe_conversion_workspace_revisions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn fetch_static_ips(
        &self,
        req: crate::model::FetchStaticIpsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FetchStaticIpsResponse> {
        T::fetch_static_ips(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location> {
        T::get_location(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse> {
        T::test_iam_permissions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::cancel_operation(self, req, options).await
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
