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

/// Implements a [ArtifactRegistry](super::stub::ArtifactRegistry) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct ArtifactRegistry<T>
where
    T: super::stub::ArtifactRegistry + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> ArtifactRegistry<T>
where
    T: super::stub::ArtifactRegistry + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::ArtifactRegistry for ArtifactRegistry<T>
where
    T: super::stub::ArtifactRegistry + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_docker_images(
        &self,
        req: crate::model::ListDockerImagesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDockerImagesResponse>> {
        self.inner.list_docker_images(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_docker_image(
        &self,
        req: crate::model::GetDockerImageRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::DockerImage>> {
        self.inner.get_docker_image(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_maven_artifacts(
        &self,
        req: crate::model::ListMavenArtifactsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListMavenArtifactsResponse>> {
        self.inner.list_maven_artifacts(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_maven_artifact(
        &self,
        req: crate::model::GetMavenArtifactRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::MavenArtifact>> {
        self.inner.get_maven_artifact(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_npm_packages(
        &self,
        req: crate::model::ListNpmPackagesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListNpmPackagesResponse>> {
        self.inner.list_npm_packages(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_npm_package(
        &self,
        req: crate::model::GetNpmPackageRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::NpmPackage>> {
        self.inner.get_npm_package(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_python_packages(
        &self,
        req: crate::model::ListPythonPackagesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListPythonPackagesResponse>> {
        self.inner.list_python_packages(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_python_package(
        &self,
        req: crate::model::GetPythonPackageRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::PythonPackage>> {
        self.inner.get_python_package(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn import_apt_artifacts(
        &self,
        req: crate::model::ImportAptArtifactsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.import_apt_artifacts(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn import_yum_artifacts(
        &self,
        req: crate::model::ImportYumArtifactsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.import_yum_artifacts(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_repositories(
        &self,
        req: crate::model::ListRepositoriesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListRepositoriesResponse>> {
        self.inner.list_repositories(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_repository(
        &self,
        req: crate::model::GetRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Repository>> {
        self.inner.get_repository(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_repository(
        &self,
        req: crate::model::CreateRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_repository(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_repository(
        &self,
        req: crate::model::UpdateRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Repository>> {
        self.inner.update_repository(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_repository(
        &self,
        req: crate::model::DeleteRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_repository(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_packages(
        &self,
        req: crate::model::ListPackagesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListPackagesResponse>> {
        self.inner.list_packages(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_package(
        &self,
        req: crate::model::GetPackageRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Package>> {
        self.inner.get_package(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_package(
        &self,
        req: crate::model::DeletePackageRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_package(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_versions(
        &self,
        req: crate::model::ListVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListVersionsResponse>> {
        self.inner.list_versions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_version(
        &self,
        req: crate::model::GetVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Version>> {
        self.inner.get_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_version(
        &self,
        req: crate::model::DeleteVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn batch_delete_versions(
        &self,
        req: crate::model::BatchDeleteVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.batch_delete_versions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_version(
        &self,
        req: crate::model::UpdateVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Version>> {
        self.inner.update_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_files(
        &self,
        req: crate::model::ListFilesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListFilesResponse>> {
        self.inner.list_files(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_file(
        &self,
        req: crate::model::GetFileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::File>> {
        self.inner.get_file(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_file(
        &self,
        req: crate::model::DeleteFileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_file(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_file(
        &self,
        req: crate::model::UpdateFileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::File>> {
        self.inner.update_file(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_tags(
        &self,
        req: crate::model::ListTagsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListTagsResponse>> {
        self.inner.list_tags(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_tag(
        &self,
        req: crate::model::GetTagRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Tag>> {
        self.inner.get_tag(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_tag(
        &self,
        req: crate::model::CreateTagRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Tag>> {
        self.inner.create_tag(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_tag(
        &self,
        req: crate::model::UpdateTagRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Tag>> {
        self.inner.update_tag(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_tag(
        &self,
        req: crate::model::DeleteTagRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_tag(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_rule(
        &self,
        req: crate::model::CreateRuleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Rule>> {
        self.inner.create_rule(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_rules(
        &self,
        req: crate::model::ListRulesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListRulesResponse>> {
        self.inner.list_rules(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_rule(
        &self,
        req: crate::model::GetRuleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Rule>> {
        self.inner.get_rule(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_rule(
        &self,
        req: crate::model::UpdateRuleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Rule>> {
        self.inner.update_rule(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_rule(
        &self,
        req: crate::model::DeleteRuleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_rule(req, options).await
    }

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
    async fn get_project_settings(
        &self,
        req: crate::model::GetProjectSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ProjectSettings>> {
        self.inner.get_project_settings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_project_settings(
        &self,
        req: crate::model::UpdateProjectSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ProjectSettings>> {
        self.inner.update_project_settings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_vpcsc_config(
        &self,
        req: crate::model::GetVPCSCConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::VPCSCConfig>> {
        self.inner.get_vpcsc_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_vpcsc_config(
        &self,
        req: crate::model::UpdateVPCSCConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::VPCSCConfig>> {
        self.inner.update_vpcsc_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_package(
        &self,
        req: crate::model::UpdatePackageRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Package>> {
        self.inner.update_package(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_attachments(
        &self,
        req: crate::model::ListAttachmentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListAttachmentsResponse>> {
        self.inner.list_attachments(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_attachment(
        &self,
        req: crate::model::GetAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Attachment>> {
        self.inner.get_attachment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_attachment(
        &self,
        req: crate::model::CreateAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_attachment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_attachment(
        &self,
        req: crate::model::DeleteAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_attachment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::ListLocationsResponse>> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::Location>> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.get_operation(req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        self.inner.get_polling_error_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
