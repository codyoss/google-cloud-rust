// Copyright 2024 Google LLC
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

/// Defines the trait used to implement [super::client::SecretManagerService].
///
/// Application developers may need to implement this trait to mock
/// `client::SecretManagerService`.  In other use-cases, application developers only
/// use `client::SecretManagerService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait SecretManagerService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::SecretManagerService::list_secrets].
    fn list_secrets(
        &self,
        _req: crate::model::ListSecretsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListSecretsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListSecretsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::SecretManagerService::create_secret].
    fn create_secret(
        &self,
        _req: crate::model::CreateSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Secret>> + Send {
        std::future::ready::<crate::Result<crate::model::Secret>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::SecretManagerService::add_secret_version].
    fn add_secret_version(
        &self,
        _req: crate::model::AddSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SecretVersion>> + Send {
        std::future::ready::<crate::Result<crate::model::SecretVersion>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::SecretManagerService::get_secret].
    fn get_secret(
        &self,
        _req: crate::model::GetSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Secret>> + Send {
        std::future::ready::<crate::Result<crate::model::Secret>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::SecretManagerService::update_secret].
    fn update_secret(
        &self,
        _req: crate::model::UpdateSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Secret>> + Send {
        std::future::ready::<crate::Result<crate::model::Secret>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::SecretManagerService::delete_secret].
    fn delete_secret(
        &self,
        _req: crate::model::DeleteSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SecretManagerService::list_secret_versions].
    fn list_secret_versions(
        &self,
        _req: crate::model::ListSecretVersionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListSecretVersionsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListSecretVersionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::SecretManagerService::get_secret_version].
    fn get_secret_version(
        &self,
        _req: crate::model::GetSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SecretVersion>> + Send {
        std::future::ready::<crate::Result<crate::model::SecretVersion>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::SecretManagerService::access_secret_version].
    fn access_secret_version(
        &self,
        _req: crate::model::AccessSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AccessSecretVersionResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::AccessSecretVersionResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::SecretManagerService::disable_secret_version].
    fn disable_secret_version(
        &self,
        _req: crate::model::DisableSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SecretVersion>> + Send {
        std::future::ready::<crate::Result<crate::model::SecretVersion>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::SecretManagerService::enable_secret_version].
    fn enable_secret_version(
        &self,
        _req: crate::model::EnableSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SecretVersion>> + Send {
        std::future::ready::<crate::Result<crate::model::SecretVersion>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::SecretManagerService::destroy_secret_version].
    fn destroy_secret_version(
        &self,
        _req: crate::model::DestroySecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SecretVersion>> + Send {
        std::future::ready::<crate::Result<crate::model::SecretVersion>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::SecretManagerService::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::SecretManagerService::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::SecretManagerService::test_iam_permissions].
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

    /// Implements [super::client::SecretManagerService::list_locations].
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

    /// Implements [super::client::SecretManagerService::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }
}
