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

/// A dyn-compatible, crate-private version of [super::ArtifactRegistry].
#[async_trait::async_trait]
pub trait ArtifactRegistry: std::fmt::Debug + Send + Sync {
    async fn list_docker_images(
        &self,
        req: crate::model::ListDockerImagesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDockerImagesResponse>>;

    async fn get_docker_image(
        &self,
        req: crate::model::GetDockerImageRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DockerImage>>;

    async fn list_maven_artifacts(
        &self,
        req: crate::model::ListMavenArtifactsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListMavenArtifactsResponse>>;

    async fn get_maven_artifact(
        &self,
        req: crate::model::GetMavenArtifactRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::MavenArtifact>>;

    async fn list_npm_packages(
        &self,
        req: crate::model::ListNpmPackagesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListNpmPackagesResponse>>;

    async fn get_npm_package(
        &self,
        req: crate::model::GetNpmPackageRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::NpmPackage>>;

    async fn list_python_packages(
        &self,
        req: crate::model::ListPythonPackagesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListPythonPackagesResponse>>;

    async fn get_python_package(
        &self,
        req: crate::model::GetPythonPackageRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::PythonPackage>>;

    async fn import_apt_artifacts(
        &self,
        req: crate::model::ImportAptArtifactsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn import_yum_artifacts(
        &self,
        req: crate::model::ImportYumArtifactsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn list_repositories(
        &self,
        req: crate::model::ListRepositoriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListRepositoriesResponse>>;

    async fn get_repository(
        &self,
        req: crate::model::GetRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Repository>>;

    async fn create_repository(
        &self,
        req: crate::model::CreateRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn update_repository(
        &self,
        req: crate::model::UpdateRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Repository>>;

    async fn delete_repository(
        &self,
        req: crate::model::DeleteRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn list_packages(
        &self,
        req: crate::model::ListPackagesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListPackagesResponse>>;

    async fn get_package(
        &self,
        req: crate::model::GetPackageRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Package>>;

    async fn delete_package(
        &self,
        req: crate::model::DeletePackageRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn list_versions(
        &self,
        req: crate::model::ListVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListVersionsResponse>>;

    async fn get_version(
        &self,
        req: crate::model::GetVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Version>>;

    async fn delete_version(
        &self,
        req: crate::model::DeleteVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn batch_delete_versions(
        &self,
        req: crate::model::BatchDeleteVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn update_version(
        &self,
        req: crate::model::UpdateVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Version>>;

    async fn list_files(
        &self,
        req: crate::model::ListFilesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListFilesResponse>>;

    async fn get_file(
        &self,
        req: crate::model::GetFileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::File>>;

    async fn delete_file(
        &self,
        req: crate::model::DeleteFileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn update_file(
        &self,
        req: crate::model::UpdateFileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::File>>;

    async fn list_tags(
        &self,
        req: crate::model::ListTagsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListTagsResponse>>;

    async fn get_tag(
        &self,
        req: crate::model::GetTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Tag>>;

    async fn create_tag(
        &self,
        req: crate::model::CreateTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Tag>>;

    async fn update_tag(
        &self,
        req: crate::model::UpdateTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Tag>>;

    async fn delete_tag(
        &self,
        req: crate::model::DeleteTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn create_rule(
        &self,
        req: crate::model::CreateRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Rule>>;

    async fn list_rules(
        &self,
        req: crate::model::ListRulesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListRulesResponse>>;

    async fn get_rule(
        &self,
        req: crate::model::GetRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Rule>>;

    async fn update_rule(
        &self,
        req: crate::model::UpdateRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Rule>>;

    async fn delete_rule(
        &self,
        req: crate::model::DeleteRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

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

    async fn get_project_settings(
        &self,
        req: crate::model::GetProjectSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ProjectSettings>>;

    async fn update_project_settings(
        &self,
        req: crate::model::UpdateProjectSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ProjectSettings>>;

    async fn get_vpcsc_config(
        &self,
        req: crate::model::GetVPCSCConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::VPCSCConfig>>;

    async fn update_vpcsc_config(
        &self,
        req: crate::model::UpdateVPCSCConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::VPCSCConfig>>;

    async fn update_package(
        &self,
        req: crate::model::UpdatePackageRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Package>>;

    async fn list_attachments(
        &self,
        req: crate::model::ListAttachmentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListAttachmentsResponse>>;

    async fn get_attachment(
        &self,
        req: crate::model::GetAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Attachment>>;

    async fn create_attachment(
        &self,
        req: crate::model::CreateAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_attachment(
        &self,
        req: crate::model::DeleteAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

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

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::ArtifactRegistry] also implement [ArtifactRegistry].
#[async_trait::async_trait]
impl<T: super::ArtifactRegistry> ArtifactRegistry for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_docker_images(
        &self,
        req: crate::model::ListDockerImagesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDockerImagesResponse>> {
        T::list_docker_images(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_docker_image(
        &self,
        req: crate::model::GetDockerImageRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DockerImage>> {
        T::get_docker_image(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_maven_artifacts(
        &self,
        req: crate::model::ListMavenArtifactsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListMavenArtifactsResponse>> {
        T::list_maven_artifacts(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_maven_artifact(
        &self,
        req: crate::model::GetMavenArtifactRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::MavenArtifact>> {
        T::get_maven_artifact(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_npm_packages(
        &self,
        req: crate::model::ListNpmPackagesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListNpmPackagesResponse>> {
        T::list_npm_packages(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_npm_package(
        &self,
        req: crate::model::GetNpmPackageRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::NpmPackage>> {
        T::get_npm_package(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_python_packages(
        &self,
        req: crate::model::ListPythonPackagesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListPythonPackagesResponse>> {
        T::list_python_packages(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_python_package(
        &self,
        req: crate::model::GetPythonPackageRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::PythonPackage>> {
        T::get_python_package(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn import_apt_artifacts(
        &self,
        req: crate::model::ImportAptArtifactsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::import_apt_artifacts(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn import_yum_artifacts(
        &self,
        req: crate::model::ImportYumArtifactsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::import_yum_artifacts(self, req, options).await
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
    async fn get_repository(
        &self,
        req: crate::model::GetRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Repository>> {
        T::get_repository(self, req, options).await
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
    async fn update_repository(
        &self,
        req: crate::model::UpdateRepositoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Repository>> {
        T::update_repository(self, req, options).await
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
    async fn list_packages(
        &self,
        req: crate::model::ListPackagesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListPackagesResponse>> {
        T::list_packages(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_package(
        &self,
        req: crate::model::GetPackageRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Package>> {
        T::get_package(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_package(
        &self,
        req: crate::model::DeletePackageRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_package(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_versions(
        &self,
        req: crate::model::ListVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListVersionsResponse>> {
        T::list_versions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_version(
        &self,
        req: crate::model::GetVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Version>> {
        T::get_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_version(
        &self,
        req: crate::model::DeleteVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn batch_delete_versions(
        &self,
        req: crate::model::BatchDeleteVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::batch_delete_versions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_version(
        &self,
        req: crate::model::UpdateVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Version>> {
        T::update_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_files(
        &self,
        req: crate::model::ListFilesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListFilesResponse>> {
        T::list_files(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_file(
        &self,
        req: crate::model::GetFileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::File>> {
        T::get_file(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_file(
        &self,
        req: crate::model::DeleteFileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_file(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_file(
        &self,
        req: crate::model::UpdateFileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::File>> {
        T::update_file(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_tags(
        &self,
        req: crate::model::ListTagsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListTagsResponse>> {
        T::list_tags(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_tag(
        &self,
        req: crate::model::GetTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Tag>> {
        T::get_tag(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_tag(
        &self,
        req: crate::model::CreateTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Tag>> {
        T::create_tag(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_tag(
        &self,
        req: crate::model::UpdateTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Tag>> {
        T::update_tag(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_tag(
        &self,
        req: crate::model::DeleteTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_tag(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_rule(
        &self,
        req: crate::model::CreateRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Rule>> {
        T::create_rule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_rules(
        &self,
        req: crate::model::ListRulesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListRulesResponse>> {
        T::list_rules(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_rule(
        &self,
        req: crate::model::GetRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Rule>> {
        T::get_rule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_rule(
        &self,
        req: crate::model::UpdateRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Rule>> {
        T::update_rule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_rule(
        &self,
        req: crate::model::DeleteRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_rule(self, req, options).await
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
    async fn get_project_settings(
        &self,
        req: crate::model::GetProjectSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ProjectSettings>> {
        T::get_project_settings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_project_settings(
        &self,
        req: crate::model::UpdateProjectSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ProjectSettings>> {
        T::update_project_settings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_vpcsc_config(
        &self,
        req: crate::model::GetVPCSCConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::VPCSCConfig>> {
        T::get_vpcsc_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_vpcsc_config(
        &self,
        req: crate::model::UpdateVPCSCConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::VPCSCConfig>> {
        T::update_vpcsc_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_package(
        &self,
        req: crate::model::UpdatePackageRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Package>> {
        T::update_package(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_attachments(
        &self,
        req: crate::model::ListAttachmentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListAttachmentsResponse>> {
        T::list_attachments(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_attachment(
        &self,
        req: crate::model::GetAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Attachment>> {
        T::get_attachment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_attachment(
        &self,
        req: crate::model::CreateAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_attachment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_attachment(
        &self,
        req: crate::model::DeleteAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_attachment(self, req, options).await
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

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
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
