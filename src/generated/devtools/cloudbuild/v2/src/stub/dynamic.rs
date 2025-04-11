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

/// A dyn-compatible, crate-private version of [super::RepositoryManager].
#[async_trait::async_trait]
pub trait RepositoryManager: std::fmt::Debug + Send + Sync {
    async fn create_connection(
        &self,
        req: crate::model::CreateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn get_connection(
        &self,
        req: crate::model::GetConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Connection>>;

    async fn list_connections(
        &self,
        req: crate::model::ListConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListConnectionsResponse>>;

    async fn update_connection(
        &self,
        req: crate::model::UpdateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_connection(
        &self,
        req: crate::model::DeleteConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn create_repository(
        &self,
        req: crate::model::CreateRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn batch_create_repositories(
        &self,
        req: crate::model::BatchCreateRepositoriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn get_repository(
        &self,
        req: crate::model::GetRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Repository>>;

    async fn list_repositories(
        &self,
        req: crate::model::ListRepositoriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListRepositoriesResponse>>;

    async fn delete_repository(
        &self,
        req: crate::model::DeleteRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn fetch_read_write_token(
        &self,
        req: crate::model::FetchReadWriteTokenRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::FetchReadWriteTokenResponse>>;

    async fn fetch_read_token(
        &self,
        req: crate::model::FetchReadTokenRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::FetchReadTokenResponse>>;

    async fn fetch_linkable_repositories(
        &self,
        req: crate::model::FetchLinkableRepositoriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::FetchLinkableRepositoriesResponse>>;

    async fn fetch_git_refs(
        &self,
        req: crate::model::FetchGitRefsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::FetchGitRefsResponse>>;

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam_v1::model::Policy>>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam_v1::model::Policy>>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::RepositoryManager] also implement [RepositoryManager].
#[async_trait::async_trait]
impl<T: super::RepositoryManager> RepositoryManager for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_connection(
        &self,
        req: crate::model::CreateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_connection(
        &self,
        req: crate::model::GetConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Connection>> {
        T::get_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_connections(
        &self,
        req: crate::model::ListConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListConnectionsResponse>> {
        T::list_connections(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_connection(
        &self,
        req: crate::model::UpdateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::update_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_connection(
        &self,
        req: crate::model::DeleteConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_repository(
        &self,
        req: crate::model::CreateRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_repository(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn batch_create_repositories(
        &self,
        req: crate::model::BatchCreateRepositoriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::batch_create_repositories(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_repository(
        &self,
        req: crate::model::GetRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Repository>> {
        T::get_repository(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_repositories(
        &self,
        req: crate::model::ListRepositoriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListRepositoriesResponse>> {
        T::list_repositories(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_repository(
        &self,
        req: crate::model::DeleteRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_repository(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn fetch_read_write_token(
        &self,
        req: crate::model::FetchReadWriteTokenRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::FetchReadWriteTokenResponse>> {
        T::fetch_read_write_token(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn fetch_read_token(
        &self,
        req: crate::model::FetchReadTokenRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::FetchReadTokenResponse>> {
        T::fetch_read_token(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn fetch_linkable_repositories(
        &self,
        req: crate::model::FetchLinkableRepositoriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::FetchLinkableRepositoriesResponse>>
    {
        T::fetch_linkable_repositories(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn fetch_git_refs(
        &self,
        req: crate::model::FetchGitRefsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::FetchGitRefsResponse>> {
        T::fetch_git_refs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam_v1::model::Policy>> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam_v1::model::Policy>> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>> {
        T::test_iam_permissions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
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
