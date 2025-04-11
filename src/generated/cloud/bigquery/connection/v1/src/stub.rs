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

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::ConnectionService].
///
/// Application developers may need to implement this trait to mock
/// `client::ConnectionService`.  In other use-cases, application developers only
/// use `client::ConnectionService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait ConnectionService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::ConnectionService::create_connection].
    fn create_connection(
        &self,
        _req: crate::model::CreateConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Connection>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Connection>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ConnectionService::get_connection].
    fn get_connection(
        &self,
        _req: crate::model::GetConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Connection>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Connection>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ConnectionService::list_connections].
    fn list_connections(
        &self,
        _req: crate::model::ListConnectionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListConnectionsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListConnectionsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::ConnectionService::update_connection].
    fn update_connection(
        &self,
        _req: crate::model::UpdateConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Connection>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Connection>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ConnectionService::delete_connection].
    fn delete_connection(
        &self,
        _req: crate::model::DeleteConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ConnectionService::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<iam_v1::model::Policy>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ConnectionService::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<iam_v1::model::Policy>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ConnectionService::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>,
        >(Err(Error::other("unimplemented")))
    }
}
