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

/// A dyn-compatible, crate-private version of [super::SecretManagerService].
#[async_trait::async_trait]
pub trait SecretManagerService: std::fmt::Debug + Send + Sync {
    async fn list_secrets(
        &self,
        req: crate::model::ListSecretsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListSecretsResponse>>;

    async fn create_secret(
        &self,
        req: crate::model::CreateSecretRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Secret>>;

    async fn add_secret_version(
        &self,
        req: crate::model::AddSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SecretVersion>>;

    async fn get_secret(
        &self,
        req: crate::model::GetSecretRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Secret>>;

    async fn update_secret(
        &self,
        req: crate::model::UpdateSecretRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Secret>>;

    async fn delete_secret(
        &self,
        req: crate::model::DeleteSecretRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn list_secret_versions(
        &self,
        req: crate::model::ListSecretVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListSecretVersionsResponse>>;

    async fn get_secret_version(
        &self,
        req: crate::model::GetSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SecretVersion>>;

    async fn access_secret_version(
        &self,
        req: crate::model::AccessSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AccessSecretVersionResponse>>;

    async fn disable_secret_version(
        &self,
        req: crate::model::DisableSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SecretVersion>>;

    async fn enable_secret_version(
        &self,
        req: crate::model::EnableSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SecretVersion>>;

    async fn destroy_secret_version(
        &self,
        req: crate::model::DestroySecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SecretVersion>>;

    async fn set_iam_policy(
        &self,
        req: iam::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam::model::Policy>>;

    async fn get_iam_policy(
        &self,
        req: iam::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam::model::Policy>>;

    async fn test_iam_permissions(
        &self,
        req: iam::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam::model::TestIamPermissionsResponse>>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::ListLocationsResponse>>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::Location>>;

}

/// All implementations of [super::SecretManagerService] also implement [SecretManagerService].
#[async_trait::async_trait]
impl<T: super::SecretManagerService> SecretManagerService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_secrets(
        &self,
        req: crate::model::ListSecretsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListSecretsResponse>> {
        T::list_secrets(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_secret(
        &self,
        req: crate::model::CreateSecretRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Secret>> {
        T::create_secret(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn add_secret_version(
        &self,
        req: crate::model::AddSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SecretVersion>> {
        T::add_secret_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_secret(
        &self,
        req: crate::model::GetSecretRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Secret>> {
        T::get_secret(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_secret(
        &self,
        req: crate::model::UpdateSecretRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Secret>> {
        T::update_secret(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_secret(
        &self,
        req: crate::model::DeleteSecretRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_secret(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_secret_versions(
        &self,
        req: crate::model::ListSecretVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListSecretVersionsResponse>> {
        T::list_secret_versions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_secret_version(
        &self,
        req: crate::model::GetSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SecretVersion>> {
        T::get_secret_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn access_secret_version(
        &self,
        req: crate::model::AccessSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AccessSecretVersionResponse>> {
        T::access_secret_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn disable_secret_version(
        &self,
        req: crate::model::DisableSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SecretVersion>> {
        T::disable_secret_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn enable_secret_version(
        &self,
        req: crate::model::EnableSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SecretVersion>> {
        T::enable_secret_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn destroy_secret_version(
        &self,
        req: crate::model::DestroySecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SecretVersion>> {
        T::destroy_secret_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: iam::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam::model::Policy>> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam::model::Policy>> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam::model::TestIamPermissionsResponse>> {
        T::test_iam_permissions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::ListLocationsResponse>> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::Location>> {
        T::get_location(self, req, options).await
    }

}
