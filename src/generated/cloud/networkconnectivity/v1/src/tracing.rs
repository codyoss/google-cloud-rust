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

/// Implements a [HubService](super::stubs::HubService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct HubService<T>
where
    T: super::stubs::HubService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> HubService<T>
where
    T: super::stubs::HubService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::HubService for HubService<T>
where
    T: super::stubs::HubService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_hubs(
        &self,
        req: crate::model::ListHubsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListHubsResponse> {
        self.inner.list_hubs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_hub(
        &self,
        req: crate::model::GetHubRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Hub> {
        self.inner.get_hub(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_hub(
        &self,
        req: crate::model::CreateHubRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_hub(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_hub(
        &self,
        req: crate::model::UpdateHubRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_hub(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_hub(
        &self,
        req: crate::model::DeleteHubRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_hub(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_hub_spokes(
        &self,
        req: crate::model::ListHubSpokesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListHubSpokesResponse> {
        self.inner.list_hub_spokes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn query_hub_status(
        &self,
        req: crate::model::QueryHubStatusRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::QueryHubStatusResponse> {
        self.inner.query_hub_status(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_spokes(
        &self,
        req: crate::model::ListSpokesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSpokesResponse> {
        self.inner.list_spokes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_spoke(
        &self,
        req: crate::model::GetSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Spoke> {
        self.inner.get_spoke(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_spoke(
        &self,
        req: crate::model::CreateSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_spoke(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_spoke(
        &self,
        req: crate::model::UpdateSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_spoke(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn reject_hub_spoke(
        &self,
        req: crate::model::RejectHubSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.reject_hub_spoke(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn accept_hub_spoke(
        &self,
        req: crate::model::AcceptHubSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.accept_hub_spoke(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_spoke(
        &self,
        req: crate::model::DeleteSpokeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_spoke(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_route_table(
        &self,
        req: crate::model::GetRouteTableRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::RouteTable> {
        self.inner.get_route_table(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_route(
        &self,
        req: crate::model::GetRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Route> {
        self.inner.get_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_routes(
        &self,
        req: crate::model::ListRoutesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListRoutesResponse> {
        self.inner.list_routes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_route_tables(
        &self,
        req: crate::model::ListRouteTablesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListRouteTablesResponse> {
        self.inner.list_route_tables(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_group(
        &self,
        req: crate::model::GetGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Group> {
        self.inner.get_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_groups(
        &self,
        req: crate::model::ListGroupsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListGroupsResponse> {
        self.inner.list_groups(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_group(
        &self,
        req: crate::model::UpdateGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_operation(req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy> {
        self.inner.get_polling_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}

/// Implements a [PolicyBasedRoutingService](super::stubs::PolicyBasedRoutingService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct PolicyBasedRoutingService<T>
where
    T: super::stubs::PolicyBasedRoutingService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> PolicyBasedRoutingService<T>
where
    T: super::stubs::PolicyBasedRoutingService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::PolicyBasedRoutingService for PolicyBasedRoutingService<T>
where
    T: super::stubs::PolicyBasedRoutingService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_policy_based_routes(
        &self,
        req: crate::model::ListPolicyBasedRoutesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListPolicyBasedRoutesResponse> {
        self.inner.list_policy_based_routes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_policy_based_route(
        &self,
        req: crate::model::GetPolicyBasedRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::PolicyBasedRoute> {
        self.inner.get_policy_based_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_policy_based_route(
        &self,
        req: crate::model::CreatePolicyBasedRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_policy_based_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_policy_based_route(
        &self,
        req: crate::model::DeletePolicyBasedRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_policy_based_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_operation(req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy> {
        self.inner.get_polling_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
