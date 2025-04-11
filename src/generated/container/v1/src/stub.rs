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

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::ClusterManager].
///
/// Application developers may need to implement this trait to mock
/// `client::ClusterManager`.  In other use-cases, application developers only
/// use `client::ClusterManager` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait ClusterManager: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::ClusterManager::list_clusters].
    fn list_clusters(
        &self,
        _req: crate::model::ListClustersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListClustersResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListClustersResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::ClusterManager::get_cluster].
    fn get_cluster(
        &self,
        _req: crate::model::GetClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Cluster>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Cluster>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::create_cluster].
    fn create_cluster(
        &self,
        _req: crate::model::CreateClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::update_cluster].
    fn update_cluster(
        &self,
        _req: crate::model::UpdateClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::update_node_pool].
    fn update_node_pool(
        &self,
        _req: crate::model::UpdateNodePoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::set_node_pool_autoscaling].
    fn set_node_pool_autoscaling(
        &self,
        _req: crate::model::SetNodePoolAutoscalingRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::set_logging_service].
    fn set_logging_service(
        &self,
        _req: crate::model::SetLoggingServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::set_monitoring_service].
    fn set_monitoring_service(
        &self,
        _req: crate::model::SetMonitoringServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::set_addons_config].
    fn set_addons_config(
        &self,
        _req: crate::model::SetAddonsConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::set_locations].
    fn set_locations(
        &self,
        _req: crate::model::SetLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::update_master].
    fn update_master(
        &self,
        _req: crate::model::UpdateMasterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::set_master_auth].
    fn set_master_auth(
        &self,
        _req: crate::model::SetMasterAuthRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::delete_cluster].
    fn delete_cluster(
        &self,
        _req: crate::model::DeleteClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::list_operations].
    fn list_operations(
        &self,
        _req: crate::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListOperationsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListOperationsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::ClusterManager::get_operation].
    fn get_operation(
        &self,
        _req: crate::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::cancel_operation].
    fn cancel_operation(
        &self,
        _req: crate::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ClusterManager::get_server_config].
    fn get_server_config(
        &self,
        _req: crate::model::GetServerConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ServerConfig>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::ServerConfig>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::ClusterManager::get_json_web_keys].
    fn get_json_web_keys(
        &self,
        _req: crate::model::GetJSONWebKeysRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::GetJSONWebKeysResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::GetJSONWebKeysResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::ClusterManager::list_node_pools].
    fn list_node_pools(
        &self,
        _req: crate::model::ListNodePoolsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListNodePoolsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListNodePoolsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::ClusterManager::get_node_pool].
    fn get_node_pool(
        &self,
        _req: crate::model::GetNodePoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::NodePool>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::NodePool>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::create_node_pool].
    fn create_node_pool(
        &self,
        _req: crate::model::CreateNodePoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::delete_node_pool].
    fn delete_node_pool(
        &self,
        _req: crate::model::DeleteNodePoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::complete_node_pool_upgrade].
    fn complete_node_pool_upgrade(
        &self,
        _req: crate::model::CompleteNodePoolUpgradeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ClusterManager::rollback_node_pool_upgrade].
    fn rollback_node_pool_upgrade(
        &self,
        _req: crate::model::RollbackNodePoolUpgradeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::set_node_pool_management].
    fn set_node_pool_management(
        &self,
        _req: crate::model::SetNodePoolManagementRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::set_labels].
    fn set_labels(
        &self,
        _req: crate::model::SetLabelsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::set_legacy_abac].
    fn set_legacy_abac(
        &self,
        _req: crate::model::SetLegacyAbacRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::start_ip_rotation].
    fn start_ip_rotation(
        &self,
        _req: crate::model::StartIPRotationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::complete_ip_rotation].
    fn complete_ip_rotation(
        &self,
        _req: crate::model::CompleteIPRotationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::set_node_pool_size].
    fn set_node_pool_size(
        &self,
        _req: crate::model::SetNodePoolSizeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::set_network_policy].
    fn set_network_policy(
        &self,
        _req: crate::model::SetNetworkPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::set_maintenance_policy].
    fn set_maintenance_policy(
        &self,
        _req: crate::model::SetMaintenancePolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Operation>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ClusterManager::list_usable_subnetworks].
    fn list_usable_subnetworks(
        &self,
        _req: crate::model::ListUsableSubnetworksRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<
            gax::response::Response<crate::model::ListUsableSubnetworksResponse>,
        >,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListUsableSubnetworksResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::ClusterManager::check_autopilot_compatibility].
    fn check_autopilot_compatibility(
        &self,
        _req: crate::model::CheckAutopilotCompatibilityRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<
            gax::response::Response<crate::model::CheckAutopilotCompatibilityResponse>,
        >,
    > + Send {
        std::future::ready::<
            crate::Result<
                gax::response::Response<crate::model::CheckAutopilotCompatibilityResponse>,
            >,
        >(Err(Error::other("unimplemented")))
    }
}
