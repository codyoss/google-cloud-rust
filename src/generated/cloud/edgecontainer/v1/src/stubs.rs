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

/// Defines the trait used to implement [super::client::EdgeContainer].
///
/// Application developers may need to implement this trait to mock
/// `client::EdgeContainer`.  In other use-cases, application developers only
/// use `client::EdgeContainer` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait EdgeContainer: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::EdgeContainer::list_clusters].
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

    /// Implements [super::client::EdgeContainer::get_cluster].
    fn get_cluster(
        &self,
        _req: crate::model::GetClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Cluster>> + Send {
        std::future::ready::<crate::Result<crate::model::Cluster>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeContainer::create_cluster].
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

    /// Implements [super::client::EdgeContainer::update_cluster].
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

    /// Implements [super::client::EdgeContainer::upgrade_cluster].
    fn upgrade_cluster(
        &self,
        _req: crate::model::UpgradeClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeContainer::delete_cluster].
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

    /// Implements [super::client::EdgeContainer::generate_access_token].
    fn generate_access_token(
        &self,
        _req: crate::model::GenerateAccessTokenRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::GenerateAccessTokenResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::GenerateAccessTokenResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::EdgeContainer::generate_offline_credential].
    fn generate_offline_credential(
        &self,
        _req: crate::model::GenerateOfflineCredentialRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::GenerateOfflineCredentialResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::GenerateOfflineCredentialResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::EdgeContainer::list_node_pools].
    fn list_node_pools(
        &self,
        _req: crate::model::ListNodePoolsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListNodePoolsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListNodePoolsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeContainer::get_node_pool].
    fn get_node_pool(
        &self,
        _req: crate::model::GetNodePoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::NodePool>> + Send {
        std::future::ready::<crate::Result<crate::model::NodePool>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeContainer::create_node_pool].
    fn create_node_pool(
        &self,
        _req: crate::model::CreateNodePoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeContainer::update_node_pool].
    fn update_node_pool(
        &self,
        _req: crate::model::UpdateNodePoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeContainer::delete_node_pool].
    fn delete_node_pool(
        &self,
        _req: crate::model::DeleteNodePoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeContainer::list_machines].
    fn list_machines(
        &self,
        _req: crate::model::ListMachinesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListMachinesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListMachinesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeContainer::get_machine].
    fn get_machine(
        &self,
        _req: crate::model::GetMachineRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Machine>> + Send {
        std::future::ready::<crate::Result<crate::model::Machine>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeContainer::list_vpn_connections].
    fn list_vpn_connections(
        &self,
        _req: crate::model::ListVpnConnectionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListVpnConnectionsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListVpnConnectionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::EdgeContainer::get_vpn_connection].
    fn get_vpn_connection(
        &self,
        _req: crate::model::GetVpnConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::VpnConnection>> + Send {
        std::future::ready::<crate::Result<crate::model::VpnConnection>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeContainer::create_vpn_connection].
    fn create_vpn_connection(
        &self,
        _req: crate::model::CreateVpnConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeContainer::delete_vpn_connection].
    fn delete_vpn_connection(
        &self,
        _req: crate::model::DeleteVpnConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeContainer::get_server_config].
    fn get_server_config(
        &self,
        _req: crate::model::GetServerConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ServerConfig>> + Send {
        std::future::ready::<crate::Result<crate::model::ServerConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeContainer::list_locations].
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

    /// Implements [super::client::EdgeContainer::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeContainer::list_operations].
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

    /// Implements [super::client::EdgeContainer::get_operation].
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

    /// Implements [super::client::EdgeContainer::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::EdgeContainer::cancel_operation].
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
