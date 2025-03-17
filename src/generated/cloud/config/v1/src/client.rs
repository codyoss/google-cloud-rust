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
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Infrastructure Manager API.
///
/// # Service Description
///
/// Infrastructure Manager is a managed service that automates the deployment and
/// management of Google Cloud infrastructure resources.
///
/// # Configuration
///
/// `Config` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `Config` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Config` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Config {
    inner: Arc<dyn super::stubs::dynamic::Config>,
}

impl Config {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stubs::Config + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stubs::dynamic::Config>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::Config> {
        super::transport::Config::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::Config> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::Config::new)
    }

    /// Lists [Deployment][google.cloud.config.v1.Deployment]s in a given project
    /// and location.
    ///
    /// [google.cloud.config.v1.Deployment]: crate::model::Deployment
    pub fn list_deployments(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::config::ListDeployments {
        super::builders::config::ListDeployments::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets details about a [Deployment][google.cloud.config.v1.Deployment].
    ///
    /// [google.cloud.config.v1.Deployment]: crate::model::Deployment
    pub fn get_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::config::GetDeployment {
        super::builders::config::GetDeployment::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a [Deployment][google.cloud.config.v1.Deployment].
    ///
    /// [google.cloud.config.v1.Deployment]: crate::model::Deployment
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_deployment(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::config::CreateDeployment {
        super::builders::config::CreateDeployment::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Updates a [Deployment][google.cloud.config.v1.Deployment].
    ///
    /// [google.cloud.config.v1.Deployment]: crate::model::Deployment
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_deployment(
        &self,
        deployment: impl Into<crate::model::Deployment>,
    ) -> super::builders::config::UpdateDeployment {
        super::builders::config::UpdateDeployment::new(self.inner.clone())
            .set_deployment(deployment.into())
    }

    /// Deletes a [Deployment][google.cloud.config.v1.Deployment].
    ///
    /// [google.cloud.config.v1.Deployment]: crate::model::Deployment
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::config::DeleteDeployment {
        super::builders::config::DeleteDeployment::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists [Revision][google.cloud.config.v1.Revision]s of a deployment.
    ///
    /// [google.cloud.config.v1.Revision]: crate::model::Revision
    pub fn list_revisions(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::config::ListRevisions {
        super::builders::config::ListRevisions::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets details about a [Revision][google.cloud.config.v1.Revision].
    ///
    /// [google.cloud.config.v1.Revision]: crate::model::Revision
    pub fn get_revision(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::config::GetRevision {
        super::builders::config::GetRevision::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets details about a [Resource][google.cloud.config.v1.Resource] deployed
    /// by Infra Manager.
    ///
    /// [google.cloud.config.v1.Resource]: crate::model::Resource
    pub fn get_resource(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::config::GetResource {
        super::builders::config::GetResource::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists [Resources][google.cloud.config.v1.Resource] in a given revision.
    ///
    /// [google.cloud.config.v1.Resource]: crate::model::Resource
    pub fn list_resources(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::config::ListResources {
        super::builders::config::ListResources::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Exports Terraform state file from a given deployment.
    pub fn export_deployment_statefile(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::config::ExportDeploymentStatefile {
        super::builders::config::ExportDeploymentStatefile::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Exports Terraform state file from a given revision.
    pub fn export_revision_statefile(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::config::ExportRevisionStatefile {
        super::builders::config::ExportRevisionStatefile::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Imports Terraform state file in a given deployment. The state file does not
    /// take effect until the Deployment has been unlocked.
    pub fn import_statefile(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::config::ImportStatefile {
        super::builders::config::ImportStatefile::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Deletes Terraform state file in a given deployment.
    pub fn delete_statefile(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::config::DeleteStatefile {
        super::builders::config::DeleteStatefile::new(self.inner.clone()).set_name(name.into())
    }

    /// Locks a deployment.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn lock_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::config::LockDeployment {
        super::builders::config::LockDeployment::new(self.inner.clone()).set_name(name.into())
    }

    /// Unlocks a locked deployment.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn unlock_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::config::UnlockDeployment {
        super::builders::config::UnlockDeployment::new(self.inner.clone()).set_name(name.into())
    }

    /// Exports the lock info on a locked deployment.
    pub fn export_lock_info(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::config::ExportLockInfo {
        super::builders::config::ExportLockInfo::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a [Preview][google.cloud.config.v1.Preview].
    ///
    /// [google.cloud.config.v1.Preview]: crate::model::Preview
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_preview(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::config::CreatePreview {
        super::builders::config::CreatePreview::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets details about a [Preview][google.cloud.config.v1.Preview].
    ///
    /// [google.cloud.config.v1.Preview]: crate::model::Preview
    pub fn get_preview(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::config::GetPreview {
        super::builders::config::GetPreview::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists [Preview][google.cloud.config.v1.Preview]s in a given project and
    /// location.
    ///
    /// [google.cloud.config.v1.Preview]: crate::model::Preview
    pub fn list_previews(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::config::ListPreviews {
        super::builders::config::ListPreviews::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Deletes a [Preview][google.cloud.config.v1.Preview].
    ///
    /// [google.cloud.config.v1.Preview]: crate::model::Preview
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_preview(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::config::DeletePreview {
        super::builders::config::DeletePreview::new(self.inner.clone()).set_name(name.into())
    }

    /// Export [Preview][google.cloud.config.v1.Preview] results.
    ///
    /// [google.cloud.config.v1.Preview]: crate::model::Preview
    pub fn export_preview_result(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::config::ExportPreviewResult {
        super::builders::config::ExportPreviewResult::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists [TerraformVersion][google.cloud.config.v1.TerraformVersion]s in a
    /// given project and location.
    ///
    /// [google.cloud.config.v1.TerraformVersion]: crate::model::TerraformVersion
    pub fn list_terraform_versions(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::config::ListTerraformVersions {
        super::builders::config::ListTerraformVersions::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details about a
    /// [TerraformVersion][google.cloud.config.v1.TerraformVersion].
    ///
    /// [google.cloud.config.v1.TerraformVersion]: crate::model::TerraformVersion
    pub fn get_terraform_version(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::config::GetTerraformVersion {
        super::builders::config::GetTerraformVersion::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::config::ListLocations {
        super::builders::config::ListLocations::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::config::GetLocation {
        super::builders::config::GetLocation::new(self.inner.clone()).set_name(name.into())
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builders::config::SetIamPolicy {
        super::builders::config::SetIamPolicy::new(self.inner.clone()).set_resource(resource.into())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builders::config::GetIamPolicy {
        super::builders::config::GetIamPolicy::new(self.inner.clone()).set_resource(resource.into())
    }

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    pub fn test_iam_permissions(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builders::config::TestIamPermissions {
        super::builders::config::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::config::ListOperations {
        super::builders::config::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::config::GetOperation {
        super::builders::config::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::config::DeleteOperation {
        super::builders::config::DeleteOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::config::CancelOperation {
        super::builders::config::CancelOperation::new(self.inner.clone()).set_name(name.into())
    }
}
