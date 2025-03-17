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
use crate::Result;

/// Implements a [SecretManagerService](super::stubs::SecretManagerService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct SecretManagerService<T>
where T: super::stubs::SecretManagerService + std::fmt::Debug + Send + Sync {
    inner: T,
}

impl<T> SecretManagerService<T>
where T: super::stubs::SecretManagerService + std::fmt::Debug + Send + Sync {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::SecretManagerService for SecretManagerService<T>
where T: super::stubs::SecretManagerService + std::fmt::Debug + Send + Sync {
    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: crate::model::ListLocationsRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: crate::model::GetLocationRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::Location> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_secrets(
        &self,
        req: crate::model::ListSecretsRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::ListSecretsResponse> {
        self.inner.list_secrets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_secret(
        &self,
        req: crate::model::CreateSecretRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::Secret> {
        self.inner.create_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_secrets_by_project_and_location(
        &self,
        req: crate::model::ListSecretsByProjectAndLocationRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::ListSecretsResponse> {
        self.inner.list_secrets_by_project_and_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_secret_by_project_and_location(
        &self,
        req: crate::model::CreateSecretByProjectAndLocationRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::Secret> {
        self.inner.create_secret_by_project_and_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn add_secret_version(
        &self,
        req: crate::model::AddSecretVersionRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::SecretVersion> {
        self.inner.add_secret_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn add_secret_version_by_project_and_location_and_secret(
        &self,
        req: crate::model::AddSecretVersionRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::SecretVersion> {
        self.inner.add_secret_version_by_project_and_location_and_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_secret(
        &self,
        req: crate::model::GetSecretRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::Secret> {
        self.inner.get_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_secret(
        &self,
        req: crate::model::DeleteSecretRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::Empty> {
        self.inner.delete_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_secret(
        &self,
        req: crate::model::UpdateSecretRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::Secret> {
        self.inner.update_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_secret_by_project_and_location_and_secret(
        &self,
        req: crate::model::GetSecretByProjectAndLocationAndSecretRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::Secret> {
        self.inner.get_secret_by_project_and_location_and_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_secret_by_project_and_location_and_secret(
        &self,
        req: crate::model::DeleteSecretByProjectAndLocationAndSecretRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::Empty> {
        self.inner.delete_secret_by_project_and_location_and_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_secret_by_project_and_location_and_secret(
        &self,
        req: crate::model::UpdateSecretByProjectAndLocationAndSecretRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::Secret> {
        self.inner.update_secret_by_project_and_location_and_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_secret_versions(
        &self,
        req: crate::model::ListSecretVersionsRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::ListSecretVersionsResponse> {
        self.inner.list_secret_versions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_secret_versions_by_project_and_location_and_secret(
        &self,
        req: crate::model::ListSecretVersionsByProjectAndLocationAndSecretRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::ListSecretVersionsResponse> {
        self.inner.list_secret_versions_by_project_and_location_and_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_secret_version(
        &self,
        req: crate::model::GetSecretVersionRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::SecretVersion> {
        self.inner.get_secret_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        req: crate::model::GetSecretVersionByProjectAndLocationAndSecretAndVersionRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::SecretVersion> {
        self.inner.get_secret_version_by_project_and_location_and_secret_and_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn access_secret_version(
        &self,
        req: crate::model::AccessSecretVersionRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::AccessSecretVersionResponse> {
        self.inner.access_secret_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn access_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        req: crate::model::AccessSecretVersionByProjectAndLocationAndSecretAndVersionRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::AccessSecretVersionResponse> {
        self.inner.access_secret_version_by_project_and_location_and_secret_and_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn disable_secret_version(
        &self,
        req: crate::model::DisableSecretVersionRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::SecretVersion> {
        self.inner.disable_secret_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn disable_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        req: crate::model::DisableSecretVersionRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::SecretVersion> {
        self.inner.disable_secret_version_by_project_and_location_and_secret_and_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn enable_secret_version(
        &self,
        req: crate::model::EnableSecretVersionRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::SecretVersion> {
        self.inner.enable_secret_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn enable_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        req: crate::model::EnableSecretVersionRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::SecretVersion> {
        self.inner.enable_secret_version_by_project_and_location_and_secret_and_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn destroy_secret_version(
        &self,
        req: crate::model::DestroySecretVersionRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::SecretVersion> {
        self.inner.destroy_secret_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn destroy_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        req: crate::model::DestroySecretVersionRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::SecretVersion> {
        self.inner.destroy_secret_version_by_project_and_location_and_secret_and_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: crate::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy_by_project_and_location_and_secret(
        &self,
        req: crate::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::Policy> {
        self.inner.set_iam_policy_by_project_and_location_and_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: crate::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy_by_project_and_location_and_secret(
        &self,
        req: crate::model::GetIamPolicyByProjectAndLocationAndSecretRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::Policy> {
        self.inner.get_iam_policy_by_project_and_location_and_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: crate::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions_by_project_and_location_and_secret(
        &self,
        req: crate::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions
    ) -> Result<crate::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions_by_project_and_location_and_secret(req, options).await
    }

}

