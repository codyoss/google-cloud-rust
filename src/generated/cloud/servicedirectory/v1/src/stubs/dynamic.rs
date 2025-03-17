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

/// A dyn-compatible, crate-private version of [super::LookupService].
#[async_trait::async_trait]
pub trait LookupService: std::fmt::Debug + Send + Sync {
    async fn resolve_service(
        &self,
        req: crate::model::ResolveServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ResolveServiceResponse>;

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
}

/// All implementations of [super::LookupService] also implement [LookupService].
#[async_trait::async_trait]
impl<T: super::LookupService> LookupService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn resolve_service(
        &self,
        req: crate::model::ResolveServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ResolveServiceResponse> {
        T::resolve_service(self, req, options).await
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
}

/// A dyn-compatible, crate-private version of [super::RegistrationService].
#[async_trait::async_trait]
pub trait RegistrationService: std::fmt::Debug + Send + Sync {
    async fn create_namespace(
        &self,
        req: crate::model::CreateNamespaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Namespace>;

    async fn list_namespaces(
        &self,
        req: crate::model::ListNamespacesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListNamespacesResponse>;

    async fn get_namespace(
        &self,
        req: crate::model::GetNamespaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Namespace>;

    async fn update_namespace(
        &self,
        req: crate::model::UpdateNamespaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Namespace>;

    async fn delete_namespace(
        &self,
        req: crate::model::DeleteNamespaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn create_service(
        &self,
        req: crate::model::CreateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Service>;

    async fn list_services(
        &self,
        req: crate::model::ListServicesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListServicesResponse>;

    async fn get_service(
        &self,
        req: crate::model::GetServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Service>;

    async fn update_service(
        &self,
        req: crate::model::UpdateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Service>;

    async fn delete_service(
        &self,
        req: crate::model::DeleteServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn create_endpoint(
        &self,
        req: crate::model::CreateEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Endpoint>;

    async fn list_endpoints(
        &self,
        req: crate::model::ListEndpointsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListEndpointsResponse>;

    async fn get_endpoint(
        &self,
        req: crate::model::GetEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Endpoint>;

    async fn update_endpoint(
        &self,
        req: crate::model::UpdateEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Endpoint>;

    async fn delete_endpoint(
        &self,
        req: crate::model::DeleteEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;

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
}

/// All implementations of [super::RegistrationService] also implement [RegistrationService].
#[async_trait::async_trait]
impl<T: super::RegistrationService> RegistrationService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_namespace(
        &self,
        req: crate::model::CreateNamespaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Namespace> {
        T::create_namespace(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_namespaces(
        &self,
        req: crate::model::ListNamespacesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListNamespacesResponse> {
        T::list_namespaces(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_namespace(
        &self,
        req: crate::model::GetNamespaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Namespace> {
        T::get_namespace(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_namespace(
        &self,
        req: crate::model::UpdateNamespaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Namespace> {
        T::update_namespace(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_namespace(
        &self,
        req: crate::model::DeleteNamespaceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_namespace(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_service(
        &self,
        req: crate::model::CreateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Service> {
        T::create_service(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_services(
        &self,
        req: crate::model::ListServicesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListServicesResponse> {
        T::list_services(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_service(
        &self,
        req: crate::model::GetServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Service> {
        T::get_service(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_service(
        &self,
        req: crate::model::UpdateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Service> {
        T::update_service(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_service(
        &self,
        req: crate::model::DeleteServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_service(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_endpoint(
        &self,
        req: crate::model::CreateEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Endpoint> {
        T::create_endpoint(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_endpoints(
        &self,
        req: crate::model::ListEndpointsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListEndpointsResponse> {
        T::list_endpoints(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_endpoint(
        &self,
        req: crate::model::GetEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Endpoint> {
        T::get_endpoint(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_endpoint(
        &self,
        req: crate::model::UpdateEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Endpoint> {
        T::update_endpoint(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_endpoint(
        &self,
        req: crate::model::DeleteEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_endpoint(self, req, options).await
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
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::set_iam_policy(self, req, options).await
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
}
