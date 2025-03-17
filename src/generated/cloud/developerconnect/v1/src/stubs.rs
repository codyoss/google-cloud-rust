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

/// Defines the trait used to implement [super::client::DeveloperConnect].
///
/// Application developers may need to implement this trait to mock
/// `client::DeveloperConnect`.  In other use-cases, application developers only
/// use `client::DeveloperConnect` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait DeveloperConnect: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::DeveloperConnect::list_connections].
    fn list_connections(
        &self,
        _req: crate::model::ListConnectionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListConnectionsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListConnectionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::DeveloperConnect::get_connection].
    fn get_connection(
        &self,
        _req: crate::model::GetConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Connection>> + Send {
        std::future::ready::<crate::Result<crate::model::Connection>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DeveloperConnect::create_connection].
    fn create_connection(
        &self,
        _req: crate::model::CreateConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DeveloperConnect::update_connection].
    fn update_connection(
        &self,
        _req: crate::model::UpdateConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DeveloperConnect::delete_connection].
    fn delete_connection(
        &self,
        _req: crate::model::DeleteConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DeveloperConnect::create_git_repository_link].
    fn create_git_repository_link(
        &self,
        _req: crate::model::CreateGitRepositoryLinkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DeveloperConnect::delete_git_repository_link].
    fn delete_git_repository_link(
        &self,
        _req: crate::model::DeleteGitRepositoryLinkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DeveloperConnect::list_git_repository_links].
    fn list_git_repository_links(
        &self,
        _req: crate::model::ListGitRepositoryLinksRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListGitRepositoryLinksResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::ListGitRepositoryLinksResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::DeveloperConnect::get_git_repository_link].
    fn get_git_repository_link(
        &self,
        _req: crate::model::GetGitRepositoryLinkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::GitRepositoryLink>> + Send
    {
        std::future::ready::<crate::Result<crate::model::GitRepositoryLink>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DeveloperConnect::fetch_read_write_token].
    fn fetch_read_write_token(
        &self,
        _req: crate::model::FetchReadWriteTokenRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::FetchReadWriteTokenResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::FetchReadWriteTokenResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::DeveloperConnect::fetch_read_token].
    fn fetch_read_token(
        &self,
        _req: crate::model::FetchReadTokenRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::FetchReadTokenResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::FetchReadTokenResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::DeveloperConnect::fetch_linkable_git_repositories].
    fn fetch_linkable_git_repositories(
        &self,
        _req: crate::model::FetchLinkableGitRepositoriesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::FetchLinkableGitRepositoriesResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::FetchLinkableGitRepositoriesResponse>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::DeveloperConnect::fetch_git_hub_installations].
    fn fetch_git_hub_installations(
        &self,
        _req: crate::model::FetchGitHubInstallationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::FetchGitHubInstallationsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::FetchGitHubInstallationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::DeveloperConnect::fetch_git_refs].
    fn fetch_git_refs(
        &self,
        _req: crate::model::FetchGitRefsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::FetchGitRefsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::FetchGitRefsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DeveloperConnect::list_locations].
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

    /// Implements [super::client::DeveloperConnect::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DeveloperConnect::list_operations].
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

    /// Implements [super::client::DeveloperConnect::get_operation].
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

    /// Implements [super::client::DeveloperConnect::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::DeveloperConnect::cancel_operation].
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
