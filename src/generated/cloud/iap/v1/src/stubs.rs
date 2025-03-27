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

/// Defines the trait used to implement [super::client::IdentityAwareProxyAdminService].
///
/// Application developers may need to implement this trait to mock
/// `client::IdentityAwareProxyAdminService`.  In other use-cases, application developers only
/// use `client::IdentityAwareProxyAdminService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait IdentityAwareProxyAdminService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::IdentityAwareProxyAdminService::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::IdentityAwareProxyAdminService::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::IdentityAwareProxyAdminService::test_iam_permissions].
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

    /// Implements [super::client::IdentityAwareProxyAdminService::get_iap_settings].
    fn get_iap_settings(
        &self,
        _req: crate::model::GetIapSettingsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::IapSettings>> + Send {
        std::future::ready::<crate::Result<crate::model::IapSettings>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::IdentityAwareProxyAdminService::update_iap_settings].
    fn update_iap_settings(
        &self,
        _req: crate::model::UpdateIapSettingsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::IapSettings>> + Send {
        std::future::ready::<crate::Result<crate::model::IapSettings>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::IdentityAwareProxyAdminService::list_tunnel_dest_groups].
    fn list_tunnel_dest_groups(
        &self,
        _req: crate::model::ListTunnelDestGroupsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListTunnelDestGroupsResponse>>
    + Send {
        std::future::ready::<crate::Result<crate::model::ListTunnelDestGroupsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::IdentityAwareProxyAdminService::create_tunnel_dest_group].
    fn create_tunnel_dest_group(
        &self,
        _req: crate::model::CreateTunnelDestGroupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TunnelDestGroup>> + Send
    {
        std::future::ready::<crate::Result<crate::model::TunnelDestGroup>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::IdentityAwareProxyAdminService::get_tunnel_dest_group].
    fn get_tunnel_dest_group(
        &self,
        _req: crate::model::GetTunnelDestGroupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TunnelDestGroup>> + Send
    {
        std::future::ready::<crate::Result<crate::model::TunnelDestGroup>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::IdentityAwareProxyAdminService::delete_tunnel_dest_group].
    fn delete_tunnel_dest_group(
        &self,
        _req: crate::model::DeleteTunnelDestGroupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::IdentityAwareProxyAdminService::update_tunnel_dest_group].
    fn update_tunnel_dest_group(
        &self,
        _req: crate::model::UpdateTunnelDestGroupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TunnelDestGroup>> + Send
    {
        std::future::ready::<crate::Result<crate::model::TunnelDestGroup>>(Err(Error::other(
            "unimplemented",
        )))
    }
}

/// Defines the trait used to implement [super::client::IdentityAwareProxyOAuthService].
///
/// Application developers may need to implement this trait to mock
/// `client::IdentityAwareProxyOAuthService`.  In other use-cases, application developers only
/// use `client::IdentityAwareProxyOAuthService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait IdentityAwareProxyOAuthService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::IdentityAwareProxyOAuthService::list_brands].
    fn list_brands(
        &self,
        _req: crate::model::ListBrandsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListBrandsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListBrandsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::IdentityAwareProxyOAuthService::create_brand].
    fn create_brand(
        &self,
        _req: crate::model::CreateBrandRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Brand>> + Send {
        std::future::ready::<crate::Result<crate::model::Brand>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::IdentityAwareProxyOAuthService::get_brand].
    fn get_brand(
        &self,
        _req: crate::model::GetBrandRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Brand>> + Send {
        std::future::ready::<crate::Result<crate::model::Brand>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::IdentityAwareProxyOAuthService::create_identity_aware_proxy_client].
    fn create_identity_aware_proxy_client(
        &self,
        _req: crate::model::CreateIdentityAwareProxyClientRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::IdentityAwareProxyClient>> + Send
    {
        std::future::ready::<crate::Result<crate::model::IdentityAwareProxyClient>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::IdentityAwareProxyOAuthService::list_identity_aware_proxy_clients].
    fn list_identity_aware_proxy_clients(
        &self,
        _req: crate::model::ListIdentityAwareProxyClientsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListIdentityAwareProxyClientsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListIdentityAwareProxyClientsResponse>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::IdentityAwareProxyOAuthService::get_identity_aware_proxy_client].
    fn get_identity_aware_proxy_client(
        &self,
        _req: crate::model::GetIdentityAwareProxyClientRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::IdentityAwareProxyClient>> + Send
    {
        std::future::ready::<crate::Result<crate::model::IdentityAwareProxyClient>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::IdentityAwareProxyOAuthService::reset_identity_aware_proxy_client_secret].
    fn reset_identity_aware_proxy_client_secret(
        &self,
        _req: crate::model::ResetIdentityAwareProxyClientSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::IdentityAwareProxyClient>> + Send
    {
        std::future::ready::<crate::Result<crate::model::IdentityAwareProxyClient>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::IdentityAwareProxyOAuthService::delete_identity_aware_proxy_client].
    fn delete_identity_aware_proxy_client(
        &self,
        _req: crate::model::DeleteIdentityAwareProxyClientRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }
}
