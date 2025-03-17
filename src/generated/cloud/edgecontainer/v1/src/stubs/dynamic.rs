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

/// A dyn-compatible, crate-private version of [super::EdgeContainer].
#[async_trait::async_trait]
pub trait EdgeContainer: std::fmt::Debug + Send + Sync {
    async fn list_clusters(
        &self,
        req: crate::model::ListClustersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListClustersResponse>;

    async fn get_cluster(
        &self,
        req: crate::model::GetClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Cluster>;

    async fn create_cluster(
        &self,
        req: crate::model::CreateClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_cluster(
        &self,
        req: crate::model::UpdateClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn upgrade_cluster(
        &self,
        req: crate::model::UpgradeClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_cluster(
        &self,
        req: crate::model::DeleteClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn generate_access_token(
        &self,
        req: crate::model::GenerateAccessTokenRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GenerateAccessTokenResponse>;

    async fn generate_offline_credential(
        &self,
        req: crate::model::GenerateOfflineCredentialRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GenerateOfflineCredentialResponse>;

    async fn list_node_pools(
        &self,
        req: crate::model::ListNodePoolsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListNodePoolsResponse>;

    async fn get_node_pool(
        &self,
        req: crate::model::GetNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::NodePool>;

    async fn create_node_pool(
        &self,
        req: crate::model::CreateNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_node_pool(
        &self,
        req: crate::model::UpdateNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_node_pool(
        &self,
        req: crate::model::DeleteNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_machines(
        &self,
        req: crate::model::ListMachinesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListMachinesResponse>;

    async fn get_machine(
        &self,
        req: crate::model::GetMachineRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Machine>;

    async fn list_vpn_connections(
        &self,
        req: crate::model::ListVpnConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListVpnConnectionsResponse>;

    async fn get_vpn_connection(
        &self,
        req: crate::model::GetVpnConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VpnConnection>;

    async fn create_vpn_connection(
        &self,
        req: crate::model::CreateVpnConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_vpn_connection(
        &self,
        req: crate::model::DeleteVpnConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_server_config(
        &self,
        req: crate::model::GetServerConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ServerConfig>;

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

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::EdgeContainer] also implement [EdgeContainer].
#[async_trait::async_trait]
impl<T: super::EdgeContainer> EdgeContainer for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_clusters(
        &self,
        req: crate::model::ListClustersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListClustersResponse> {
        T::list_clusters(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_cluster(
        &self,
        req: crate::model::GetClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Cluster> {
        T::get_cluster(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_cluster(
        &self,
        req: crate::model::CreateClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_cluster(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_cluster(
        &self,
        req: crate::model::UpdateClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_cluster(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn upgrade_cluster(
        &self,
        req: crate::model::UpgradeClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::upgrade_cluster(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_cluster(
        &self,
        req: crate::model::DeleteClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_cluster(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn generate_access_token(
        &self,
        req: crate::model::GenerateAccessTokenRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GenerateAccessTokenResponse> {
        T::generate_access_token(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn generate_offline_credential(
        &self,
        req: crate::model::GenerateOfflineCredentialRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GenerateOfflineCredentialResponse> {
        T::generate_offline_credential(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_node_pools(
        &self,
        req: crate::model::ListNodePoolsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListNodePoolsResponse> {
        T::list_node_pools(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_node_pool(
        &self,
        req: crate::model::GetNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::NodePool> {
        T::get_node_pool(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_node_pool(
        &self,
        req: crate::model::CreateNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_node_pool(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_node_pool(
        &self,
        req: crate::model::UpdateNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_node_pool(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_node_pool(
        &self,
        req: crate::model::DeleteNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_node_pool(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_machines(
        &self,
        req: crate::model::ListMachinesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListMachinesResponse> {
        T::list_machines(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_machine(
        &self,
        req: crate::model::GetMachineRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Machine> {
        T::get_machine(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_vpn_connections(
        &self,
        req: crate::model::ListVpnConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListVpnConnectionsResponse> {
        T::list_vpn_connections(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_vpn_connection(
        &self,
        req: crate::model::GetVpnConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VpnConnection> {
        T::get_vpn_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_vpn_connection(
        &self,
        req: crate::model::CreateVpnConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_vpn_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_vpn_connection(
        &self,
        req: crate::model::DeleteVpnConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_vpn_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_server_config(
        &self,
        req: crate::model::GetServerConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ServerConfig> {
        T::get_server_config(self, req, options).await
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

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        T::get_polling_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}
