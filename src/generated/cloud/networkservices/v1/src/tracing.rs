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

/// Implements a [DepService](super::stubs::DepService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct DepService<T>
where
    T: super::stubs::DepService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> DepService<T>
where
    T: super::stubs::DepService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::DepService for DepService<T>
where
    T: super::stubs::DepService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_lb_traffic_extensions(
        &self,
        req: crate::model::ListLbTrafficExtensionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListLbTrafficExtensionsResponse> {
        self.inner.list_lb_traffic_extensions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_lb_traffic_extension(
        &self,
        req: crate::model::GetLbTrafficExtensionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::LbTrafficExtension> {
        self.inner.get_lb_traffic_extension(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_lb_traffic_extension(
        &self,
        req: crate::model::CreateLbTrafficExtensionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_lb_traffic_extension(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_lb_traffic_extension(
        &self,
        req: crate::model::UpdateLbTrafficExtensionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_lb_traffic_extension(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_lb_traffic_extension(
        &self,
        req: crate::model::DeleteLbTrafficExtensionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_lb_traffic_extension(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_lb_route_extensions(
        &self,
        req: crate::model::ListLbRouteExtensionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListLbRouteExtensionsResponse> {
        self.inner.list_lb_route_extensions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_lb_route_extension(
        &self,
        req: crate::model::GetLbRouteExtensionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::LbRouteExtension> {
        self.inner.get_lb_route_extension(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_lb_route_extension(
        &self,
        req: crate::model::CreateLbRouteExtensionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_lb_route_extension(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_lb_route_extension(
        &self,
        req: crate::model::UpdateLbRouteExtensionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_lb_route_extension(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_lb_route_extension(
        &self,
        req: crate::model::DeleteLbRouteExtensionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_lb_route_extension(req, options).await
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

/// Implements a [NetworkServices](super::stubs::NetworkServices) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct NetworkServices<T>
where
    T: super::stubs::NetworkServices + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> NetworkServices<T>
where
    T: super::stubs::NetworkServices + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::NetworkServices for NetworkServices<T>
where
    T: super::stubs::NetworkServices + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_endpoint_policies(
        &self,
        req: crate::model::ListEndpointPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListEndpointPoliciesResponse> {
        self.inner.list_endpoint_policies(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_endpoint_policy(
        &self,
        req: crate::model::GetEndpointPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::EndpointPolicy> {
        self.inner.get_endpoint_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_endpoint_policy(
        &self,
        req: crate::model::CreateEndpointPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_endpoint_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_endpoint_policy(
        &self,
        req: crate::model::UpdateEndpointPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_endpoint_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_endpoint_policy(
        &self,
        req: crate::model::DeleteEndpointPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_endpoint_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_gateways(
        &self,
        req: crate::model::ListGatewaysRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListGatewaysResponse> {
        self.inner.list_gateways(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_gateway(
        &self,
        req: crate::model::GetGatewayRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Gateway> {
        self.inner.get_gateway(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_gateway(
        &self,
        req: crate::model::CreateGatewayRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_gateway(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_gateway(
        &self,
        req: crate::model::UpdateGatewayRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_gateway(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_gateway(
        &self,
        req: crate::model::DeleteGatewayRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_gateway(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_grpc_routes(
        &self,
        req: crate::model::ListGrpcRoutesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListGrpcRoutesResponse> {
        self.inner.list_grpc_routes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_grpc_route(
        &self,
        req: crate::model::GetGrpcRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GrpcRoute> {
        self.inner.get_grpc_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_grpc_route(
        &self,
        req: crate::model::CreateGrpcRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_grpc_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_grpc_route(
        &self,
        req: crate::model::UpdateGrpcRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_grpc_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_grpc_route(
        &self,
        req: crate::model::DeleteGrpcRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_grpc_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_http_routes(
        &self,
        req: crate::model::ListHttpRoutesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListHttpRoutesResponse> {
        self.inner.list_http_routes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_http_route(
        &self,
        req: crate::model::GetHttpRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::HttpRoute> {
        self.inner.get_http_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_http_route(
        &self,
        req: crate::model::CreateHttpRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_http_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_http_route(
        &self,
        req: crate::model::UpdateHttpRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_http_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_http_route(
        &self,
        req: crate::model::DeleteHttpRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_http_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_tcp_routes(
        &self,
        req: crate::model::ListTcpRoutesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTcpRoutesResponse> {
        self.inner.list_tcp_routes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_tcp_route(
        &self,
        req: crate::model::GetTcpRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TcpRoute> {
        self.inner.get_tcp_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_tcp_route(
        &self,
        req: crate::model::CreateTcpRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_tcp_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_tcp_route(
        &self,
        req: crate::model::UpdateTcpRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_tcp_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_tcp_route(
        &self,
        req: crate::model::DeleteTcpRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_tcp_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_tls_routes(
        &self,
        req: crate::model::ListTlsRoutesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTlsRoutesResponse> {
        self.inner.list_tls_routes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_tls_route(
        &self,
        req: crate::model::GetTlsRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TlsRoute> {
        self.inner.get_tls_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_tls_route(
        &self,
        req: crate::model::CreateTlsRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_tls_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_tls_route(
        &self,
        req: crate::model::UpdateTlsRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_tls_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_tls_route(
        &self,
        req: crate::model::DeleteTlsRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_tls_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_service_bindings(
        &self,
        req: crate::model::ListServiceBindingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListServiceBindingsResponse> {
        self.inner.list_service_bindings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_service_binding(
        &self,
        req: crate::model::GetServiceBindingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ServiceBinding> {
        self.inner.get_service_binding(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_service_binding(
        &self,
        req: crate::model::CreateServiceBindingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_service_binding(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_service_binding(
        &self,
        req: crate::model::DeleteServiceBindingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_service_binding(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_meshes(
        &self,
        req: crate::model::ListMeshesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListMeshesResponse> {
        self.inner.list_meshes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_mesh(
        &self,
        req: crate::model::GetMeshRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Mesh> {
        self.inner.get_mesh(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_mesh(
        &self,
        req: crate::model::CreateMeshRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_mesh(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_mesh(
        &self,
        req: crate::model::UpdateMeshRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_mesh(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_mesh(
        &self,
        req: crate::model::DeleteMeshRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_mesh(req, options).await
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
