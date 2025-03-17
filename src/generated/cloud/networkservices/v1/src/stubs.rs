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

/// Defines the trait used to implement [super::client::DepService].
///
/// Application developers may need to implement this trait to mock
/// `client::DepService`.  In other use-cases, application developers only
/// use `client::DepService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait DepService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::DepService::list_lb_traffic_extensions].
    fn list_lb_traffic_extensions(
        &self,
        _req: crate::model::ListLbTrafficExtensionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListLbTrafficExtensionsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListLbTrafficExtensionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::DepService::get_lb_traffic_extension].
    fn get_lb_traffic_extension(
        &self,
        _req: crate::model::GetLbTrafficExtensionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::LbTrafficExtension>> + Send
    {
        std::future::ready::<crate::Result<crate::model::LbTrafficExtension>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DepService::create_lb_traffic_extension].
    fn create_lb_traffic_extension(
        &self,
        _req: crate::model::CreateLbTrafficExtensionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DepService::update_lb_traffic_extension].
    fn update_lb_traffic_extension(
        &self,
        _req: crate::model::UpdateLbTrafficExtensionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DepService::delete_lb_traffic_extension].
    fn delete_lb_traffic_extension(
        &self,
        _req: crate::model::DeleteLbTrafficExtensionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DepService::list_lb_route_extensions].
    fn list_lb_route_extensions(
        &self,
        _req: crate::model::ListLbRouteExtensionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListLbRouteExtensionsResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::ListLbRouteExtensionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::DepService::get_lb_route_extension].
    fn get_lb_route_extension(
        &self,
        _req: crate::model::GetLbRouteExtensionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::LbRouteExtension>> + Send
    {
        std::future::ready::<crate::Result<crate::model::LbRouteExtension>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DepService::create_lb_route_extension].
    fn create_lb_route_extension(
        &self,
        _req: crate::model::CreateLbRouteExtensionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DepService::update_lb_route_extension].
    fn update_lb_route_extension(
        &self,
        _req: crate::model::UpdateLbRouteExtensionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DepService::delete_lb_route_extension].
    fn delete_lb_route_extension(
        &self,
        _req: crate::model::DeleteLbRouteExtensionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DepService::list_locations].
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

    /// Implements [super::client::DepService::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DepService::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DepService::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DepService::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::TestIamPermissionsResponse>> + Send
    {
        std::future::ready::<crate::Result<iam_v1::model::TestIamPermissionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::DepService::list_operations].
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

    /// Implements [super::client::DepService::get_operation].
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

    /// Implements [super::client::DepService::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::DepService::cancel_operation].
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

/// Defines the trait used to implement [super::client::NetworkServices].
///
/// Application developers may need to implement this trait to mock
/// `client::NetworkServices`.  In other use-cases, application developers only
/// use `client::NetworkServices` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait NetworkServices: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::NetworkServices::list_endpoint_policies].
    fn list_endpoint_policies(
        &self,
        _req: crate::model::ListEndpointPoliciesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListEndpointPoliciesResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::ListEndpointPoliciesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::NetworkServices::get_endpoint_policy].
    fn get_endpoint_policy(
        &self,
        _req: crate::model::GetEndpointPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::EndpointPolicy>> + Send {
        std::future::ready::<crate::Result<crate::model::EndpointPolicy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::create_endpoint_policy].
    fn create_endpoint_policy(
        &self,
        _req: crate::model::CreateEndpointPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::update_endpoint_policy].
    fn update_endpoint_policy(
        &self,
        _req: crate::model::UpdateEndpointPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::delete_endpoint_policy].
    fn delete_endpoint_policy(
        &self,
        _req: crate::model::DeleteEndpointPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::list_gateways].
    fn list_gateways(
        &self,
        _req: crate::model::ListGatewaysRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListGatewaysResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListGatewaysResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::get_gateway].
    fn get_gateway(
        &self,
        _req: crate::model::GetGatewayRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Gateway>> + Send {
        std::future::ready::<crate::Result<crate::model::Gateway>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::create_gateway].
    fn create_gateway(
        &self,
        _req: crate::model::CreateGatewayRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::update_gateway].
    fn update_gateway(
        &self,
        _req: crate::model::UpdateGatewayRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::delete_gateway].
    fn delete_gateway(
        &self,
        _req: crate::model::DeleteGatewayRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::list_grpc_routes].
    fn list_grpc_routes(
        &self,
        _req: crate::model::ListGrpcRoutesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListGrpcRoutesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListGrpcRoutesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::NetworkServices::get_grpc_route].
    fn get_grpc_route(
        &self,
        _req: crate::model::GetGrpcRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::GrpcRoute>> + Send {
        std::future::ready::<crate::Result<crate::model::GrpcRoute>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::create_grpc_route].
    fn create_grpc_route(
        &self,
        _req: crate::model::CreateGrpcRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::update_grpc_route].
    fn update_grpc_route(
        &self,
        _req: crate::model::UpdateGrpcRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::delete_grpc_route].
    fn delete_grpc_route(
        &self,
        _req: crate::model::DeleteGrpcRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::list_http_routes].
    fn list_http_routes(
        &self,
        _req: crate::model::ListHttpRoutesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListHttpRoutesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListHttpRoutesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::NetworkServices::get_http_route].
    fn get_http_route(
        &self,
        _req: crate::model::GetHttpRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::HttpRoute>> + Send {
        std::future::ready::<crate::Result<crate::model::HttpRoute>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::create_http_route].
    fn create_http_route(
        &self,
        _req: crate::model::CreateHttpRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::update_http_route].
    fn update_http_route(
        &self,
        _req: crate::model::UpdateHttpRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::delete_http_route].
    fn delete_http_route(
        &self,
        _req: crate::model::DeleteHttpRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::list_tcp_routes].
    fn list_tcp_routes(
        &self,
        _req: crate::model::ListTcpRoutesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListTcpRoutesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListTcpRoutesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::get_tcp_route].
    fn get_tcp_route(
        &self,
        _req: crate::model::GetTcpRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TcpRoute>> + Send {
        std::future::ready::<crate::Result<crate::model::TcpRoute>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::create_tcp_route].
    fn create_tcp_route(
        &self,
        _req: crate::model::CreateTcpRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::update_tcp_route].
    fn update_tcp_route(
        &self,
        _req: crate::model::UpdateTcpRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::delete_tcp_route].
    fn delete_tcp_route(
        &self,
        _req: crate::model::DeleteTcpRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::list_tls_routes].
    fn list_tls_routes(
        &self,
        _req: crate::model::ListTlsRoutesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListTlsRoutesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListTlsRoutesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::get_tls_route].
    fn get_tls_route(
        &self,
        _req: crate::model::GetTlsRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TlsRoute>> + Send {
        std::future::ready::<crate::Result<crate::model::TlsRoute>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::create_tls_route].
    fn create_tls_route(
        &self,
        _req: crate::model::CreateTlsRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::update_tls_route].
    fn update_tls_route(
        &self,
        _req: crate::model::UpdateTlsRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::delete_tls_route].
    fn delete_tls_route(
        &self,
        _req: crate::model::DeleteTlsRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::list_service_bindings].
    fn list_service_bindings(
        &self,
        _req: crate::model::ListServiceBindingsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListServiceBindingsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListServiceBindingsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::NetworkServices::get_service_binding].
    fn get_service_binding(
        &self,
        _req: crate::model::GetServiceBindingRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ServiceBinding>> + Send {
        std::future::ready::<crate::Result<crate::model::ServiceBinding>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::create_service_binding].
    fn create_service_binding(
        &self,
        _req: crate::model::CreateServiceBindingRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::delete_service_binding].
    fn delete_service_binding(
        &self,
        _req: crate::model::DeleteServiceBindingRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::list_meshes].
    fn list_meshes(
        &self,
        _req: crate::model::ListMeshesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListMeshesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListMeshesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::get_mesh].
    fn get_mesh(
        &self,
        _req: crate::model::GetMeshRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Mesh>> + Send {
        std::future::ready::<crate::Result<crate::model::Mesh>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::NetworkServices::create_mesh].
    fn create_mesh(
        &self,
        _req: crate::model::CreateMeshRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::update_mesh].
    fn update_mesh(
        &self,
        _req: crate::model::UpdateMeshRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::delete_mesh].
    fn delete_mesh(
        &self,
        _req: crate::model::DeleteMeshRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::list_locations].
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

    /// Implements [super::client::NetworkServices::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NetworkServices::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::TestIamPermissionsResponse>> + Send
    {
        std::future::ready::<crate::Result<iam_v1::model::TestIamPermissionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::NetworkServices::list_operations].
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

    /// Implements [super::client::NetworkServices::get_operation].
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

    /// Implements [super::client::NetworkServices::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::NetworkServices::cancel_operation].
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
