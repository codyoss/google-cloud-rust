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

/// A dyn-compatible, crate-private version of [super::AlloyDBAdmin].
#[async_trait::async_trait]
pub trait AlloyDBAdmin: std::fmt::Debug + Send + Sync {
    async fn list_clusters(
        &self,
        req: crate::model::ListClustersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListClustersResponse>>;

    async fn get_cluster(
        &self,
        req: crate::model::GetClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Cluster>>;

    async fn create_cluster(
        &self,
        req: crate::model::CreateClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn update_cluster(
        &self,
        req: crate::model::UpdateClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_cluster(
        &self,
        req: crate::model::DeleteClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn promote_cluster(
        &self,
        req: crate::model::PromoteClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn switchover_cluster(
        &self,
        req: crate::model::SwitchoverClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn restore_cluster(
        &self,
        req: crate::model::RestoreClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn create_secondary_cluster(
        &self,
        req: crate::model::CreateSecondaryClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn list_instances(
        &self,
        req: crate::model::ListInstancesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListInstancesResponse>>;

    async fn get_instance(
        &self,
        req: crate::model::GetInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Instance>>;

    async fn create_instance(
        &self,
        req: crate::model::CreateInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn create_secondary_instance(
        &self,
        req: crate::model::CreateSecondaryInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn batch_create_instances(
        &self,
        req: crate::model::BatchCreateInstancesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn update_instance(
        &self,
        req: crate::model::UpdateInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_instance(
        &self,
        req: crate::model::DeleteInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn failover_instance(
        &self,
        req: crate::model::FailoverInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn inject_fault(
        &self,
        req: crate::model::InjectFaultRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn restart_instance(
        &self,
        req: crate::model::RestartInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn execute_sql(
        &self,
        req: crate::model::ExecuteSqlRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ExecuteSqlResponse>>;

    async fn list_backups(
        &self,
        req: crate::model::ListBackupsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListBackupsResponse>>;

    async fn get_backup(
        &self,
        req: crate::model::GetBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Backup>>;

    async fn create_backup(
        &self,
        req: crate::model::CreateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn update_backup(
        &self,
        req: crate::model::UpdateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_backup(
        &self,
        req: crate::model::DeleteBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn list_supported_database_flags(
        &self,
        req: crate::model::ListSupportedDatabaseFlagsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListSupportedDatabaseFlagsResponse>>;

    async fn generate_client_certificate(
        &self,
        req: crate::model::GenerateClientCertificateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::GenerateClientCertificateResponse>>;

    async fn get_connection_info(
        &self,
        req: crate::model::GetConnectionInfoRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ConnectionInfo>>;

    async fn list_users(
        &self,
        req: crate::model::ListUsersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListUsersResponse>>;

    async fn get_user(
        &self,
        req: crate::model::GetUserRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::User>>;

    async fn create_user(
        &self,
        req: crate::model::CreateUserRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::User>>;

    async fn update_user(
        &self,
        req: crate::model::UpdateUserRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::User>>;

    async fn delete_user(
        &self,
        req: crate::model::DeleteUserRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn list_databases(
        &self,
        req: crate::model::ListDatabasesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDatabasesResponse>>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::ListLocationsResponse>>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::Location>>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::AlloyDBAdmin] also implement [AlloyDBAdmin].
#[async_trait::async_trait]
impl<T: super::AlloyDBAdmin> AlloyDBAdmin for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_clusters(
        &self,
        req: crate::model::ListClustersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListClustersResponse>> {
        T::list_clusters(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_cluster(
        &self,
        req: crate::model::GetClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Cluster>> {
        T::get_cluster(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_cluster(
        &self,
        req: crate::model::CreateClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_cluster(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_cluster(
        &self,
        req: crate::model::UpdateClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::update_cluster(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_cluster(
        &self,
        req: crate::model::DeleteClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_cluster(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn promote_cluster(
        &self,
        req: crate::model::PromoteClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::promote_cluster(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn switchover_cluster(
        &self,
        req: crate::model::SwitchoverClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::switchover_cluster(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn restore_cluster(
        &self,
        req: crate::model::RestoreClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::restore_cluster(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_secondary_cluster(
        &self,
        req: crate::model::CreateSecondaryClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_secondary_cluster(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_instances(
        &self,
        req: crate::model::ListInstancesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListInstancesResponse>> {
        T::list_instances(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_instance(
        &self,
        req: crate::model::GetInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Instance>> {
        T::get_instance(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_instance(
        &self,
        req: crate::model::CreateInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_instance(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_secondary_instance(
        &self,
        req: crate::model::CreateSecondaryInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_secondary_instance(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn batch_create_instances(
        &self,
        req: crate::model::BatchCreateInstancesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::batch_create_instances(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_instance(
        &self,
        req: crate::model::UpdateInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::update_instance(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_instance(
        &self,
        req: crate::model::DeleteInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_instance(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn failover_instance(
        &self,
        req: crate::model::FailoverInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::failover_instance(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn inject_fault(
        &self,
        req: crate::model::InjectFaultRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::inject_fault(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn restart_instance(
        &self,
        req: crate::model::RestartInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::restart_instance(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn execute_sql(
        &self,
        req: crate::model::ExecuteSqlRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ExecuteSqlResponse>> {
        T::execute_sql(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_backups(
        &self,
        req: crate::model::ListBackupsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListBackupsResponse>> {
        T::list_backups(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_backup(
        &self,
        req: crate::model::GetBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Backup>> {
        T::get_backup(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_backup(
        &self,
        req: crate::model::CreateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_backup(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_backup(
        &self,
        req: crate::model::UpdateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::update_backup(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_backup(
        &self,
        req: crate::model::DeleteBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_backup(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_supported_database_flags(
        &self,
        req: crate::model::ListSupportedDatabaseFlagsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListSupportedDatabaseFlagsResponse>>
    {
        T::list_supported_database_flags(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn generate_client_certificate(
        &self,
        req: crate::model::GenerateClientCertificateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::GenerateClientCertificateResponse>>
    {
        T::generate_client_certificate(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_connection_info(
        &self,
        req: crate::model::GetConnectionInfoRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ConnectionInfo>> {
        T::get_connection_info(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_users(
        &self,
        req: crate::model::ListUsersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListUsersResponse>> {
        T::list_users(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_user(
        &self,
        req: crate::model::GetUserRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::User>> {
        T::get_user(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_user(
        &self,
        req: crate::model::CreateUserRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::User>> {
        T::create_user(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_user(
        &self,
        req: crate::model::UpdateUserRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::User>> {
        T::update_user(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_user(
        &self,
        req: crate::model::DeleteUserRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_user(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_databases(
        &self,
        req: crate::model::ListDatabasesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDatabasesResponse>> {
        T::list_databases(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::ListLocationsResponse>> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::Location>> {
        T::get_location(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
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
