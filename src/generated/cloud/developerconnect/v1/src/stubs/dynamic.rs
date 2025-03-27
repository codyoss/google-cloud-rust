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

/// A dyn-compatible, crate-private version of [super::DeveloperConnect].
#[async_trait::async_trait]
pub trait DeveloperConnect: std::fmt::Debug + Send + Sync {
    async fn list_connections(
        &self,
        req: crate::model::ListConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConnectionsResponse>;

    async fn get_connection(
        &self,
        req: crate::model::GetConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Connection>;

    async fn create_connection(
        &self,
        req: crate::model::CreateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_connection(
        &self,
        req: crate::model::UpdateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_connection(
        &self,
        req: crate::model::DeleteConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_git_repository_link(
        &self,
        req: crate::model::CreateGitRepositoryLinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_git_repository_link(
        &self,
        req: crate::model::DeleteGitRepositoryLinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_git_repository_links(
        &self,
        req: crate::model::ListGitRepositoryLinksRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListGitRepositoryLinksResponse>;

    async fn get_git_repository_link(
        &self,
        req: crate::model::GetGitRepositoryLinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GitRepositoryLink>;

    async fn fetch_read_write_token(
        &self,
        req: crate::model::FetchReadWriteTokenRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FetchReadWriteTokenResponse>;

    async fn fetch_read_token(
        &self,
        req: crate::model::FetchReadTokenRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FetchReadTokenResponse>;

    async fn fetch_linkable_git_repositories(
        &self,
        req: crate::model::FetchLinkableGitRepositoriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FetchLinkableGitRepositoriesResponse>;

    async fn fetch_git_hub_installations(
        &self,
        req: crate::model::FetchGitHubInstallationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FetchGitHubInstallationsResponse>;

    async fn fetch_git_refs(
        &self,
        req: crate::model::FetchGitRefsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FetchGitRefsResponse>;

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

/// All implementations of [super::DeveloperConnect] also implement [DeveloperConnect].
#[async_trait::async_trait]
impl<T: super::DeveloperConnect> DeveloperConnect for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_connections(
        &self,
        req: crate::model::ListConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConnectionsResponse> {
        T::list_connections(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_connection(
        &self,
        req: crate::model::GetConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Connection> {
        T::get_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_connection(
        &self,
        req: crate::model::CreateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_connection(
        &self,
        req: crate::model::UpdateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_connection(
        &self,
        req: crate::model::DeleteConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_git_repository_link(
        &self,
        req: crate::model::CreateGitRepositoryLinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_git_repository_link(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_git_repository_link(
        &self,
        req: crate::model::DeleteGitRepositoryLinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_git_repository_link(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_git_repository_links(
        &self,
        req: crate::model::ListGitRepositoryLinksRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListGitRepositoryLinksResponse> {
        T::list_git_repository_links(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_git_repository_link(
        &self,
        req: crate::model::GetGitRepositoryLinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GitRepositoryLink> {
        T::get_git_repository_link(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn fetch_read_write_token(
        &self,
        req: crate::model::FetchReadWriteTokenRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FetchReadWriteTokenResponse> {
        T::fetch_read_write_token(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn fetch_read_token(
        &self,
        req: crate::model::FetchReadTokenRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FetchReadTokenResponse> {
        T::fetch_read_token(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn fetch_linkable_git_repositories(
        &self,
        req: crate::model::FetchLinkableGitRepositoriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FetchLinkableGitRepositoriesResponse> {
        T::fetch_linkable_git_repositories(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn fetch_git_hub_installations(
        &self,
        req: crate::model::FetchGitHubInstallationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FetchGitHubInstallationsResponse> {
        T::fetch_git_hub_installations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn fetch_git_refs(
        &self,
        req: crate::model::FetchGitRefsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FetchGitRefsResponse> {
        T::fetch_git_refs(self, req, options).await
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
