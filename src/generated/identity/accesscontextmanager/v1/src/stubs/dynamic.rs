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

/// A dyn-compatible, crate-private version of [super::AccessContextManager].
#[async_trait::async_trait]
pub trait AccessContextManager: std::fmt::Debug + Send + Sync {
    async fn list_access_policies(
        &self,
        req: crate::model::ListAccessPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListAccessPoliciesResponse>;

    async fn get_access_policy(
        &self,
        req: crate::model::GetAccessPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AccessPolicy>;

    async fn create_access_policy(
        &self,
        req: crate::model::AccessPolicy,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_access_policy(
        &self,
        req: crate::model::UpdateAccessPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_access_policy(
        &self,
        req: crate::model::DeleteAccessPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_access_levels(
        &self,
        req: crate::model::ListAccessLevelsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListAccessLevelsResponse>;

    async fn get_access_level(
        &self,
        req: crate::model::GetAccessLevelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AccessLevel>;

    async fn create_access_level(
        &self,
        req: crate::model::CreateAccessLevelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_access_level(
        &self,
        req: crate::model::UpdateAccessLevelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_access_level(
        &self,
        req: crate::model::DeleteAccessLevelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn replace_access_levels(
        &self,
        req: crate::model::ReplaceAccessLevelsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_service_perimeters(
        &self,
        req: crate::model::ListServicePerimetersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListServicePerimetersResponse>;

    async fn get_service_perimeter(
        &self,
        req: crate::model::GetServicePerimeterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ServicePerimeter>;

    async fn create_service_perimeter(
        &self,
        req: crate::model::CreateServicePerimeterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_service_perimeter(
        &self,
        req: crate::model::UpdateServicePerimeterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_service_perimeter(
        &self,
        req: crate::model::DeleteServicePerimeterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn replace_service_perimeters(
        &self,
        req: crate::model::ReplaceServicePerimetersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn commit_service_perimeters(
        &self,
        req: crate::model::CommitServicePerimetersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_gcp_user_access_bindings(
        &self,
        req: crate::model::ListGcpUserAccessBindingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListGcpUserAccessBindingsResponse>;

    async fn get_gcp_user_access_binding(
        &self,
        req: crate::model::GetGcpUserAccessBindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GcpUserAccessBinding>;

    async fn create_gcp_user_access_binding(
        &self,
        req: crate::model::CreateGcpUserAccessBindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_gcp_user_access_binding(
        &self,
        req: crate::model::UpdateGcpUserAccessBindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_gcp_user_access_binding(
        &self,
        req: crate::model::DeleteGcpUserAccessBindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

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

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [crate::stubs::AccessContextManager] also implement [AccessContextManager].
#[async_trait::async_trait]
impl<T: crate::stubs::AccessContextManager> AccessContextManager for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_access_policies(
        &self,
        req: crate::model::ListAccessPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListAccessPoliciesResponse> {
        T::list_access_policies(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_access_policy(
        &self,
        req: crate::model::GetAccessPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AccessPolicy> {
        T::get_access_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_access_policy(
        &self,
        req: crate::model::AccessPolicy,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_access_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_access_policy(
        &self,
        req: crate::model::UpdateAccessPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_access_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_access_policy(
        &self,
        req: crate::model::DeleteAccessPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_access_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_access_levels(
        &self,
        req: crate::model::ListAccessLevelsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListAccessLevelsResponse> {
        T::list_access_levels(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_access_level(
        &self,
        req: crate::model::GetAccessLevelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AccessLevel> {
        T::get_access_level(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_access_level(
        &self,
        req: crate::model::CreateAccessLevelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_access_level(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_access_level(
        &self,
        req: crate::model::UpdateAccessLevelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_access_level(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_access_level(
        &self,
        req: crate::model::DeleteAccessLevelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_access_level(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn replace_access_levels(
        &self,
        req: crate::model::ReplaceAccessLevelsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::replace_access_levels(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_service_perimeters(
        &self,
        req: crate::model::ListServicePerimetersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListServicePerimetersResponse> {
        T::list_service_perimeters(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_service_perimeter(
        &self,
        req: crate::model::GetServicePerimeterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ServicePerimeter> {
        T::get_service_perimeter(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_service_perimeter(
        &self,
        req: crate::model::CreateServicePerimeterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_service_perimeter(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_service_perimeter(
        &self,
        req: crate::model::UpdateServicePerimeterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_service_perimeter(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_service_perimeter(
        &self,
        req: crate::model::DeleteServicePerimeterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_service_perimeter(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn replace_service_perimeters(
        &self,
        req: crate::model::ReplaceServicePerimetersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::replace_service_perimeters(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn commit_service_perimeters(
        &self,
        req: crate::model::CommitServicePerimetersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::commit_service_perimeters(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_gcp_user_access_bindings(
        &self,
        req: crate::model::ListGcpUserAccessBindingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListGcpUserAccessBindingsResponse> {
        T::list_gcp_user_access_bindings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_gcp_user_access_binding(
        &self,
        req: crate::model::GetGcpUserAccessBindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GcpUserAccessBinding> {
        T::get_gcp_user_access_binding(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_gcp_user_access_binding(
        &self,
        req: crate::model::CreateGcpUserAccessBindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_gcp_user_access_binding(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_gcp_user_access_binding(
        &self,
        req: crate::model::UpdateGcpUserAccessBindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_gcp_user_access_binding(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_gcp_user_access_binding(
        &self,
        req: crate::model::DeleteGcpUserAccessBindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_gcp_user_access_binding(self, req, options).await
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
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
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
