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

/// A dyn-compatible, crate-private version of [super::NetworkSecurity].
#[async_trait::async_trait]
pub trait NetworkSecurity: std::fmt::Debug + Send + Sync {
    async fn list_authorization_policies(
        &self,
        req: crate::model::ListAuthorizationPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListAuthorizationPoliciesResponse>;

    async fn get_authorization_policy(
        &self,
        req: crate::model::GetAuthorizationPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AuthorizationPolicy>;

    async fn create_authorization_policy(
        &self,
        req: crate::model::CreateAuthorizationPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_authorization_policy(
        &self,
        req: crate::model::UpdateAuthorizationPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_authorization_policy(
        &self,
        req: crate::model::DeleteAuthorizationPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_server_tls_policies(
        &self,
        req: crate::model::ListServerTlsPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListServerTlsPoliciesResponse>;

    async fn get_server_tls_policy(
        &self,
        req: crate::model::GetServerTlsPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ServerTlsPolicy>;

    async fn create_server_tls_policy(
        &self,
        req: crate::model::CreateServerTlsPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_server_tls_policy(
        &self,
        req: crate::model::UpdateServerTlsPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_server_tls_policy(
        &self,
        req: crate::model::DeleteServerTlsPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_client_tls_policies(
        &self,
        req: crate::model::ListClientTlsPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListClientTlsPoliciesResponse>;

    async fn get_client_tls_policy(
        &self,
        req: crate::model::GetClientTlsPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ClientTlsPolicy>;

    async fn create_client_tls_policy(
        &self,
        req: crate::model::CreateClientTlsPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_client_tls_policy(
        &self,
        req: crate::model::UpdateClientTlsPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_client_tls_policy(
        &self,
        req: crate::model::DeleteClientTlsPolicyRequest,
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

/// All implementations of [super::NetworkSecurity] also implement [NetworkSecurity].
#[async_trait::async_trait]
impl<T: super::NetworkSecurity> NetworkSecurity for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_authorization_policies(
        &self,
        req: crate::model::ListAuthorizationPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListAuthorizationPoliciesResponse> {
        T::list_authorization_policies(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_authorization_policy(
        &self,
        req: crate::model::GetAuthorizationPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AuthorizationPolicy> {
        T::get_authorization_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_authorization_policy(
        &self,
        req: crate::model::CreateAuthorizationPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_authorization_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_authorization_policy(
        &self,
        req: crate::model::UpdateAuthorizationPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_authorization_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_authorization_policy(
        &self,
        req: crate::model::DeleteAuthorizationPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_authorization_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_server_tls_policies(
        &self,
        req: crate::model::ListServerTlsPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListServerTlsPoliciesResponse> {
        T::list_server_tls_policies(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_server_tls_policy(
        &self,
        req: crate::model::GetServerTlsPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ServerTlsPolicy> {
        T::get_server_tls_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_server_tls_policy(
        &self,
        req: crate::model::CreateServerTlsPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_server_tls_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_server_tls_policy(
        &self,
        req: crate::model::UpdateServerTlsPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_server_tls_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_server_tls_policy(
        &self,
        req: crate::model::DeleteServerTlsPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_server_tls_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_client_tls_policies(
        &self,
        req: crate::model::ListClientTlsPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListClientTlsPoliciesResponse> {
        T::list_client_tls_policies(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_client_tls_policy(
        &self,
        req: crate::model::GetClientTlsPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ClientTlsPolicy> {
        T::get_client_tls_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_client_tls_policy(
        &self,
        req: crate::model::CreateClientTlsPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_client_tls_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_client_tls_policy(
        &self,
        req: crate::model::UpdateClientTlsPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_client_tls_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_client_tls_policy(
        &self,
        req: crate::model::DeleteClientTlsPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_client_tls_policy(self, req, options).await
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
