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

/// A dyn-compatible, crate-private version of [super::SecureSourceManager].
#[async_trait::async_trait]
pub trait SecureSourceManager: std::fmt::Debug + Send + Sync {
    async fn list_instances(
        &self,
        req: crate::model::ListInstancesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListInstancesResponse>;

    async fn get_instance(
        &self,
        req: crate::model::GetInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Instance>;

    async fn create_instance(
        &self,
        req: crate::model::CreateInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_instance(
        &self,
        req: crate::model::DeleteInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_repositories(
        &self,
        req: crate::model::ListRepositoriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRepositoriesResponse>;

    async fn get_repository(
        &self,
        req: crate::model::GetRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Repository>;

    async fn create_repository(
        &self,
        req: crate::model::CreateRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_repository(
        &self,
        req: crate::model::DeleteRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_iam_policy_repo(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn set_iam_policy_repo(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn test_iam_permissions_repo(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;

    async fn create_branch_rule(
        &self,
        req: crate::model::CreateBranchRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_branch_rules(
        &self,
        req: crate::model::ListBranchRulesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBranchRulesResponse>;

    async fn get_branch_rule(
        &self,
        req: crate::model::GetBranchRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BranchRule>;

    async fn update_branch_rule(
        &self,
        req: crate::model::UpdateBranchRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_branch_rule(
        &self,
        req: crate::model::DeleteBranchRuleRequest,
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

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::SecureSourceManager] also implement [SecureSourceManager].
#[async_trait::async_trait]
impl<T: super::SecureSourceManager> SecureSourceManager for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_instances(
        &self,
        req: crate::model::ListInstancesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListInstancesResponse> {
        T::list_instances(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_instance(
        &self,
        req: crate::model::GetInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Instance> {
        T::get_instance(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_instance(
        &self,
        req: crate::model::CreateInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_instance(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_instance(
        &self,
        req: crate::model::DeleteInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_instance(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_repositories(
        &self,
        req: crate::model::ListRepositoriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRepositoriesResponse> {
        T::list_repositories(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_repository(
        &self,
        req: crate::model::GetRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Repository> {
        T::get_repository(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_repository(
        &self,
        req: crate::model::CreateRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_repository(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_repository(
        &self,
        req: crate::model::DeleteRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_repository(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy_repo(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::get_iam_policy_repo(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy_repo(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::set_iam_policy_repo(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions_repo(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse> {
        T::test_iam_permissions_repo(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_branch_rule(
        &self,
        req: crate::model::CreateBranchRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_branch_rule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_branch_rules(
        &self,
        req: crate::model::ListBranchRulesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBranchRulesResponse> {
        T::list_branch_rules(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_branch_rule(
        &self,
        req: crate::model::GetBranchRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BranchRule> {
        T::get_branch_rule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_branch_rule(
        &self,
        req: crate::model::UpdateBranchRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_branch_rule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_branch_rule(
        &self,
        req: crate::model::DeleteBranchRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_branch_rule(self, req, options).await
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
