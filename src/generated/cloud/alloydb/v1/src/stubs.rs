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

//! Traits to mock the clients in this library.
//!
//! Application developers may need to mock the clients in this library to test
//! how their application works with different (and sometimes hard to trigger)
//! client and service behavior. Such test can define mocks implementing the
//! trait(s) defined in this module, initialize the client with an instance of
//! this mock in their tests, and verify their application responds as expected.

#![allow(rustdoc::broken_intra_doc_links)]

use gax::error::Error;
use std::sync::Arc;

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::AlloyDBAdmin].
///
/// Application developers may need to implement this trait to mock
/// `client::AlloyDBAdmin`.  In other use-cases, application developers only
/// use `client::AlloyDBAdmin` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait AlloyDBAdmin: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::AlloyDBAdmin::list_clusters].
    fn list_clusters(
        &self,
        _req: crate::model::ListClustersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListClustersResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListClustersResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::get_cluster].
    fn get_cluster(
        &self,
        _req: crate::model::GetClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Cluster>> + Send {
        std::future::ready::<crate::Result<crate::model::Cluster>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::create_cluster].
    fn create_cluster(
        &self,
        _req: crate::model::CreateClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::update_cluster].
    fn update_cluster(
        &self,
        _req: crate::model::UpdateClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::delete_cluster].
    fn delete_cluster(
        &self,
        _req: crate::model::DeleteClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::promote_cluster].
    fn promote_cluster(
        &self,
        _req: crate::model::PromoteClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::switchover_cluster].
    fn switchover_cluster(
        &self,
        _req: crate::model::SwitchoverClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::restore_cluster].
    fn restore_cluster(
        &self,
        _req: crate::model::RestoreClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::create_secondary_cluster].
    fn create_secondary_cluster(
        &self,
        _req: crate::model::CreateSecondaryClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::list_instances].
    fn list_instances(
        &self,
        _req: crate::model::ListInstancesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListInstancesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListInstancesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::get_instance].
    fn get_instance(
        &self,
        _req: crate::model::GetInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Instance>> + Send {
        std::future::ready::<crate::Result<crate::model::Instance>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::create_instance].
    fn create_instance(
        &self,
        _req: crate::model::CreateInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::create_secondary_instance].
    fn create_secondary_instance(
        &self,
        _req: crate::model::CreateSecondaryInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::batch_create_instances].
    fn batch_create_instances(
        &self,
        _req: crate::model::BatchCreateInstancesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::update_instance].
    fn update_instance(
        &self,
        _req: crate::model::UpdateInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::delete_instance].
    fn delete_instance(
        &self,
        _req: crate::model::DeleteInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::failover_instance].
    fn failover_instance(
        &self,
        _req: crate::model::FailoverInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::inject_fault].
    fn inject_fault(
        &self,
        _req: crate::model::InjectFaultRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::restart_instance].
    fn restart_instance(
        &self,
        _req: crate::model::RestartInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::execute_sql].
    fn execute_sql(
        &self,
        _req: crate::model::ExecuteSqlRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ExecuteSqlResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ExecuteSqlResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::list_backups].
    fn list_backups(
        &self,
        _req: crate::model::ListBackupsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListBackupsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListBackupsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::get_backup].
    fn get_backup(
        &self,
        _req: crate::model::GetBackupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Backup>> + Send {
        std::future::ready::<crate::Result<crate::model::Backup>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::create_backup].
    fn create_backup(
        &self,
        _req: crate::model::CreateBackupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::update_backup].
    fn update_backup(
        &self,
        _req: crate::model::UpdateBackupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::delete_backup].
    fn delete_backup(
        &self,
        _req: crate::model::DeleteBackupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::list_supported_database_flags].
    fn list_supported_database_flags(
        &self,
        _req: crate::model::ListSupportedDatabaseFlagsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListSupportedDatabaseFlagsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListSupportedDatabaseFlagsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::AlloyDBAdmin::generate_client_certificate].
    fn generate_client_certificate(
        &self,
        _req: crate::model::GenerateClientCertificateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::GenerateClientCertificateResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::GenerateClientCertificateResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::AlloyDBAdmin::get_connection_info].
    fn get_connection_info(
        &self,
        _req: crate::model::GetConnectionInfoRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ConnectionInfo>> + Send {
        std::future::ready::<crate::Result<crate::model::ConnectionInfo>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::list_users].
    fn list_users(
        &self,
        _req: crate::model::ListUsersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListUsersResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListUsersResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::get_user].
    fn get_user(
        &self,
        _req: crate::model::GetUserRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::User>> + Send {
        std::future::ready::<crate::Result<crate::model::User>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::AlloyDBAdmin::create_user].
    fn create_user(
        &self,
        _req: crate::model::CreateUserRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::User>> + Send {
        std::future::ready::<crate::Result<crate::model::User>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::AlloyDBAdmin::update_user].
    fn update_user(
        &self,
        _req: crate::model::UpdateUserRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::User>> + Send {
        std::future::ready::<crate::Result<crate::model::User>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::AlloyDBAdmin::delete_user].
    fn delete_user(
        &self,
        _req: crate::model::DeleteUserRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::AlloyDBAdmin::list_databases].
    fn list_databases(
        &self,
        _req: crate::model::ListDatabasesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListDatabasesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListDatabasesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::ListLocationsResponse>> + Send
    {
        std::future::ready::<crate::Result<location::model::ListLocationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::AlloyDBAdmin::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::AlloyDBAdmin::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlloyDBAdmin::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::AlloyDBAdmin::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Returns the polling policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        Arc::new(gax::polling_policy::Aip194Strict)
    }

    /// Returns the polling backoff policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_backoff_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        Arc::new(gax::exponential_backoff::ExponentialBackoff::default())
    }
}
