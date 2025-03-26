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

/// A dyn-compatible, crate-private version of [super::PolicyBindings].
#[async_trait::async_trait]
pub trait PolicyBindings: std::fmt::Debug + Send + Sync {
    async fn create_policy_binding(
        &self,
        req: crate::model::CreatePolicyBindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_policy_binding(
        &self,
        req: crate::model::GetPolicyBindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PolicyBinding>;

    async fn update_policy_binding(
        &self,
        req: crate::model::UpdatePolicyBindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_policy_binding(
        &self,
        req: crate::model::DeletePolicyBindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_policy_bindings(
        &self,
        req: crate::model::ListPolicyBindingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPolicyBindingsResponse>;

    async fn search_target_policy_bindings(
        &self,
        req: crate::model::SearchTargetPolicyBindingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchTargetPolicyBindingsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::PolicyBindings] also implement [PolicyBindings].
#[async_trait::async_trait]
impl<T: super::PolicyBindings> PolicyBindings for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_policy_binding(
        &self,
        req: crate::model::CreatePolicyBindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_policy_binding(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_policy_binding(
        &self,
        req: crate::model::GetPolicyBindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PolicyBinding> {
        T::get_policy_binding(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_policy_binding(
        &self,
        req: crate::model::UpdatePolicyBindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_policy_binding(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_policy_binding(
        &self,
        req: crate::model::DeletePolicyBindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_policy_binding(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_policy_bindings(
        &self,
        req: crate::model::ListPolicyBindingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPolicyBindingsResponse> {
        T::list_policy_bindings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn search_target_policy_bindings(
        &self,
        req: crate::model::SearchTargetPolicyBindingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchTargetPolicyBindingsResponse> {
        T::search_target_policy_bindings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
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

/// A dyn-compatible, crate-private version of [super::PrincipalAccessBoundaryPolicies].
#[async_trait::async_trait]
pub trait PrincipalAccessBoundaryPolicies: std::fmt::Debug + Send + Sync {
    async fn create_principal_access_boundary_policy(
        &self,
        req: crate::model::CreatePrincipalAccessBoundaryPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_principal_access_boundary_policy(
        &self,
        req: crate::model::GetPrincipalAccessBoundaryPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PrincipalAccessBoundaryPolicy>;

    async fn update_principal_access_boundary_policy(
        &self,
        req: crate::model::UpdatePrincipalAccessBoundaryPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_principal_access_boundary_policy(
        &self,
        req: crate::model::DeletePrincipalAccessBoundaryPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_principal_access_boundary_policies(
        &self,
        req: crate::model::ListPrincipalAccessBoundaryPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPrincipalAccessBoundaryPoliciesResponse>;

    async fn search_principal_access_boundary_policy_bindings(
        &self,
        req: crate::model::SearchPrincipalAccessBoundaryPolicyBindingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchPrincipalAccessBoundaryPolicyBindingsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::PrincipalAccessBoundaryPolicies] also implement [PrincipalAccessBoundaryPolicies].
#[async_trait::async_trait]
impl<T: super::PrincipalAccessBoundaryPolicies> PrincipalAccessBoundaryPolicies for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_principal_access_boundary_policy(
        &self,
        req: crate::model::CreatePrincipalAccessBoundaryPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_principal_access_boundary_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_principal_access_boundary_policy(
        &self,
        req: crate::model::GetPrincipalAccessBoundaryPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PrincipalAccessBoundaryPolicy> {
        T::get_principal_access_boundary_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_principal_access_boundary_policy(
        &self,
        req: crate::model::UpdatePrincipalAccessBoundaryPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_principal_access_boundary_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_principal_access_boundary_policy(
        &self,
        req: crate::model::DeletePrincipalAccessBoundaryPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_principal_access_boundary_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_principal_access_boundary_policies(
        &self,
        req: crate::model::ListPrincipalAccessBoundaryPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPrincipalAccessBoundaryPoliciesResponse> {
        T::list_principal_access_boundary_policies(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn search_principal_access_boundary_policy_bindings(
        &self,
        req: crate::model::SearchPrincipalAccessBoundaryPolicyBindingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchPrincipalAccessBoundaryPolicyBindingsResponse> {
        T::search_principal_access_boundary_policy_bindings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
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
