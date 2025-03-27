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

/// Defines the trait used to implement [super::client::OracleDatabase].
///
/// Application developers may need to implement this trait to mock
/// `client::OracleDatabase`.  In other use-cases, application developers only
/// use `client::OracleDatabase` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait OracleDatabase: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::OracleDatabase::list_cloud_exadata_infrastructures].
    fn list_cloud_exadata_infrastructures(
        &self,
        _req: crate::model::ListCloudExadataInfrastructuresRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListCloudExadataInfrastructuresResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListCloudExadataInfrastructuresResponse>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::OracleDatabase::get_cloud_exadata_infrastructure].
    fn get_cloud_exadata_infrastructure(
        &self,
        _req: crate::model::GetCloudExadataInfrastructureRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::CloudExadataInfrastructure>> + Send
    {
        std::future::ready::<crate::Result<crate::model::CloudExadataInfrastructure>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::OracleDatabase::create_cloud_exadata_infrastructure].
    fn create_cloud_exadata_infrastructure(
        &self,
        _req: crate::model::CreateCloudExadataInfrastructureRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OracleDatabase::delete_cloud_exadata_infrastructure].
    fn delete_cloud_exadata_infrastructure(
        &self,
        _req: crate::model::DeleteCloudExadataInfrastructureRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OracleDatabase::list_cloud_vm_clusters].
    fn list_cloud_vm_clusters(
        &self,
        _req: crate::model::ListCloudVmClustersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListCloudVmClustersResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListCloudVmClustersResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::OracleDatabase::get_cloud_vm_cluster].
    fn get_cloud_vm_cluster(
        &self,
        _req: crate::model::GetCloudVmClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::CloudVmCluster>> + Send {
        std::future::ready::<crate::Result<crate::model::CloudVmCluster>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OracleDatabase::create_cloud_vm_cluster].
    fn create_cloud_vm_cluster(
        &self,
        _req: crate::model::CreateCloudVmClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OracleDatabase::delete_cloud_vm_cluster].
    fn delete_cloud_vm_cluster(
        &self,
        _req: crate::model::DeleteCloudVmClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OracleDatabase::list_entitlements].
    fn list_entitlements(
        &self,
        _req: crate::model::ListEntitlementsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListEntitlementsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListEntitlementsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::OracleDatabase::list_db_servers].
    fn list_db_servers(
        &self,
        _req: crate::model::ListDbServersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListDbServersResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListDbServersResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OracleDatabase::list_db_nodes].
    fn list_db_nodes(
        &self,
        _req: crate::model::ListDbNodesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListDbNodesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListDbNodesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OracleDatabase::list_gi_versions].
    fn list_gi_versions(
        &self,
        _req: crate::model::ListGiVersionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListGiVersionsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListGiVersionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::OracleDatabase::list_db_system_shapes].
    fn list_db_system_shapes(
        &self,
        _req: crate::model::ListDbSystemShapesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListDbSystemShapesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListDbSystemShapesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::OracleDatabase::list_autonomous_databases].
    fn list_autonomous_databases(
        &self,
        _req: crate::model::ListAutonomousDatabasesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListAutonomousDatabasesResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListAutonomousDatabasesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::OracleDatabase::get_autonomous_database].
    fn get_autonomous_database(
        &self,
        _req: crate::model::GetAutonomousDatabaseRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AutonomousDatabase>> + Send
    {
        std::future::ready::<crate::Result<crate::model::AutonomousDatabase>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OracleDatabase::create_autonomous_database].
    fn create_autonomous_database(
        &self,
        _req: crate::model::CreateAutonomousDatabaseRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OracleDatabase::delete_autonomous_database].
    fn delete_autonomous_database(
        &self,
        _req: crate::model::DeleteAutonomousDatabaseRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OracleDatabase::restore_autonomous_database].
    fn restore_autonomous_database(
        &self,
        _req: crate::model::RestoreAutonomousDatabaseRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OracleDatabase::generate_autonomous_database_wallet].
    fn generate_autonomous_database_wallet(
        &self,
        _req: crate::model::GenerateAutonomousDatabaseWalletRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::GenerateAutonomousDatabaseWalletResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::GenerateAutonomousDatabaseWalletResponse>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::OracleDatabase::list_autonomous_db_versions].
    fn list_autonomous_db_versions(
        &self,
        _req: crate::model::ListAutonomousDbVersionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListAutonomousDbVersionsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListAutonomousDbVersionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::OracleDatabase::list_autonomous_database_character_sets].
    fn list_autonomous_database_character_sets(
        &self,
        _req: crate::model::ListAutonomousDatabaseCharacterSetsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListAutonomousDatabaseCharacterSetsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListAutonomousDatabaseCharacterSetsResponse>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::OracleDatabase::list_autonomous_database_backups].
    fn list_autonomous_database_backups(
        &self,
        _req: crate::model::ListAutonomousDatabaseBackupsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListAutonomousDatabaseBackupsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListAutonomousDatabaseBackupsResponse>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::OracleDatabase::list_locations].
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

    /// Implements [super::client::OracleDatabase::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OracleDatabase::list_operations].
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

    /// Implements [super::client::OracleDatabase::get_operation].
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

    /// Implements [super::client::OracleDatabase::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::OracleDatabase::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }

    /// Returns the polling error policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_error_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        Arc::new(gax::polling_error_policy::Aip194Strict)
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
