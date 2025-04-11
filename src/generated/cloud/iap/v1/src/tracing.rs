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
use crate::Result;

/// Implements a [IdentityAwareProxyAdminService](super::stub::IdentityAwareProxyAdminService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct IdentityAwareProxyAdminService<T>
where
    T: super::stub::IdentityAwareProxyAdminService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> IdentityAwareProxyAdminService<T>
where
    T: super::stub::IdentityAwareProxyAdminService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::IdentityAwareProxyAdminService for IdentityAwareProxyAdminService<T>
where
    T: super::stub::IdentityAwareProxyAdminService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>> {
        self.inner.test_iam_permissions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iap_settings(
        &self,
        req: crate::model::GetIapSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::IapSettings>> {
        self.inner.get_iap_settings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_iap_settings(
        &self,
        req: crate::model::UpdateIapSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::IapSettings>> {
        self.inner.update_iap_settings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_tunnel_dest_groups(
        &self,
        req: crate::model::ListTunnelDestGroupsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListTunnelDestGroupsResponse>> {
        self.inner.list_tunnel_dest_groups(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_tunnel_dest_group(
        &self,
        req: crate::model::CreateTunnelDestGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::TunnelDestGroup>> {
        self.inner.create_tunnel_dest_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_tunnel_dest_group(
        &self,
        req: crate::model::GetTunnelDestGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::TunnelDestGroup>> {
        self.inner.get_tunnel_dest_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_tunnel_dest_group(
        &self,
        req: crate::model::DeleteTunnelDestGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_tunnel_dest_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_tunnel_dest_group(
        &self,
        req: crate::model::UpdateTunnelDestGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::TunnelDestGroup>> {
        self.inner.update_tunnel_dest_group(req, options).await
    }
}

/// Implements a [IdentityAwareProxyOAuthService](super::stub::IdentityAwareProxyOAuthService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct IdentityAwareProxyOAuthService<T>
where
    T: super::stub::IdentityAwareProxyOAuthService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> IdentityAwareProxyOAuthService<T>
where
    T: super::stub::IdentityAwareProxyOAuthService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::IdentityAwareProxyOAuthService for IdentityAwareProxyOAuthService<T>
where
    T: super::stub::IdentityAwareProxyOAuthService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_brands(
        &self,
        req: crate::model::ListBrandsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListBrandsResponse>> {
        self.inner.list_brands(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_brand(
        &self,
        req: crate::model::CreateBrandRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Brand>> {
        self.inner.create_brand(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_brand(
        &self,
        req: crate::model::GetBrandRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Brand>> {
        self.inner.get_brand(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_identity_aware_proxy_client(
        &self,
        req: crate::model::CreateIdentityAwareProxyClientRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::IdentityAwareProxyClient>> {
        self.inner
            .create_identity_aware_proxy_client(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn list_identity_aware_proxy_clients(
        &self,
        req: crate::model::ListIdentityAwareProxyClientsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListIdentityAwareProxyClientsResponse>> {
        self.inner
            .list_identity_aware_proxy_clients(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn get_identity_aware_proxy_client(
        &self,
        req: crate::model::GetIdentityAwareProxyClientRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::IdentityAwareProxyClient>> {
        self.inner
            .get_identity_aware_proxy_client(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn reset_identity_aware_proxy_client_secret(
        &self,
        req: crate::model::ResetIdentityAwareProxyClientSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::IdentityAwareProxyClient>> {
        self.inner
            .reset_identity_aware_proxy_client_secret(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn delete_identity_aware_proxy_client(
        &self,
        req: crate::model::DeleteIdentityAwareProxyClientRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner
            .delete_identity_aware_proxy_client(req, options)
            .await
    }
}
