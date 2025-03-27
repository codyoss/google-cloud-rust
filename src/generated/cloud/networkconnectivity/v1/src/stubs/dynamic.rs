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

/// A dyn-compatible, crate-private version of [super::HubService].
#[async_trait::async_trait]
pub trait HubService: std::fmt::Debug + Send + Sync {
    async fn list_hubs(
        &self,
        req: crate::model::ListHubsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListHubsResponse>;

    async fn get_hub(
        &self,
        req: crate::model::GetHubRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Hub>;

    async fn create_hub(
        &self,
        req: crate::model::CreateHubRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_hub(
        &self,
        req: crate::model::UpdateHubRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_hub(
        &self,
        req: crate::model::DeleteHubRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_hub_spokes(
        &self,
        req: crate::model::ListHubSpokesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListHubSpokesResponse>;

    async fn query_hub_status(
        &self,
        req: crate::model::QueryHubStatusRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::QueryHubStatusResponse>;

    async fn list_spokes(
        &self,
        req: crate::model::ListSpokesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListSpokesResponse>;

    async fn get_spoke(
        &self,
        req: crate::model::GetSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Spoke>;

    async fn create_spoke(
        &self,
        req: crate::model::CreateSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_spoke(
        &self,
        req: crate::model::UpdateSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn reject_hub_spoke(
        &self,
        req: crate::model::RejectHubSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn accept_hub_spoke(
        &self,
        req: crate::model::AcceptHubSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_spoke(
        &self,
        req: crate::model::DeleteSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_route_table(
        &self,
        req: crate::model::GetRouteTableRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::RouteTable>;

    async fn get_route(
        &self,
        req: crate::model::GetRouteRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Route>;

    async fn list_routes(
        &self,
        req: crate::model::ListRoutesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRoutesResponse>;

    async fn list_route_tables(
        &self,
        req: crate::model::ListRouteTablesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRouteTablesResponse>;

    async fn get_group(
        &self,
        req: crate::model::GetGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Group>;

    async fn list_groups(
        &self,
        req: crate::model::ListGroupsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListGroupsResponse>;

    async fn update_group(
        &self,
        req: crate::model::UpdateGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

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
    ) -> crate::Result<()>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::HubService] also implement [HubService].
#[async_trait::async_trait]
impl<T: super::HubService> HubService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_hubs(
        &self,
        req: crate::model::ListHubsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListHubsResponse> {
        T::list_hubs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_hub(
        &self,
        req: crate::model::GetHubRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Hub> {
        T::get_hub(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_hub(
        &self,
        req: crate::model::CreateHubRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_hub(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_hub(
        &self,
        req: crate::model::UpdateHubRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_hub(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_hub(
        &self,
        req: crate::model::DeleteHubRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_hub(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_hub_spokes(
        &self,
        req: crate::model::ListHubSpokesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListHubSpokesResponse> {
        T::list_hub_spokes(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn query_hub_status(
        &self,
        req: crate::model::QueryHubStatusRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::QueryHubStatusResponse> {
        T::query_hub_status(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_spokes(
        &self,
        req: crate::model::ListSpokesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListSpokesResponse> {
        T::list_spokes(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_spoke(
        &self,
        req: crate::model::GetSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Spoke> {
        T::get_spoke(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_spoke(
        &self,
        req: crate::model::CreateSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_spoke(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_spoke(
        &self,
        req: crate::model::UpdateSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_spoke(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn reject_hub_spoke(
        &self,
        req: crate::model::RejectHubSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::reject_hub_spoke(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn accept_hub_spoke(
        &self,
        req: crate::model::AcceptHubSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::accept_hub_spoke(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_spoke(
        &self,
        req: crate::model::DeleteSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_spoke(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_route_table(
        &self,
        req: crate::model::GetRouteTableRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::RouteTable> {
        T::get_route_table(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_route(
        &self,
        req: crate::model::GetRouteRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Route> {
        T::get_route(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_routes(
        &self,
        req: crate::model::ListRoutesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRoutesResponse> {
        T::list_routes(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_route_tables(
        &self,
        req: crate::model::ListRouteTablesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRouteTablesResponse> {
        T::list_route_tables(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_group(
        &self,
        req: crate::model::GetGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Group> {
        T::get_group(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_groups(
        &self,
        req: crate::model::ListGroupsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListGroupsResponse> {
        T::list_groups(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_group(
        &self,
        req: crate::model::UpdateGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_group(self, req, options).await
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
    ) -> crate::Result<()> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
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

/// A dyn-compatible, crate-private version of [super::PolicyBasedRoutingService].
#[async_trait::async_trait]
pub trait PolicyBasedRoutingService: std::fmt::Debug + Send + Sync {
    async fn list_policy_based_routes(
        &self,
        req: crate::model::ListPolicyBasedRoutesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPolicyBasedRoutesResponse>;

    async fn get_policy_based_route(
        &self,
        req: crate::model::GetPolicyBasedRouteRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PolicyBasedRoute>;

    async fn create_policy_based_route(
        &self,
        req: crate::model::CreatePolicyBasedRouteRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_policy_based_route(
        &self,
        req: crate::model::DeletePolicyBasedRouteRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

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
    ) -> crate::Result<()>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::PolicyBasedRoutingService] also implement [PolicyBasedRoutingService].
#[async_trait::async_trait]
impl<T: super::PolicyBasedRoutingService> PolicyBasedRoutingService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_policy_based_routes(
        &self,
        req: crate::model::ListPolicyBasedRoutesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPolicyBasedRoutesResponse> {
        T::list_policy_based_routes(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_policy_based_route(
        &self,
        req: crate::model::GetPolicyBasedRouteRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PolicyBasedRoute> {
        T::get_policy_based_route(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_policy_based_route(
        &self,
        req: crate::model::CreatePolicyBasedRouteRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_policy_based_route(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_policy_based_route(
        &self,
        req: crate::model::DeletePolicyBasedRouteRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_policy_based_route(self, req, options).await
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
    ) -> crate::Result<()> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
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
