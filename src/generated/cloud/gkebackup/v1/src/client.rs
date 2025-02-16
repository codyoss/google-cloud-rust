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
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Backup for GKE API.
///
/// # Service Description
///
/// BackupForGKE allows Kubernetes administrators to configure, execute, and
/// manage backup and restore operations for their GKE clusters.
///
/// # Configuration
///
/// `BackupForGKE` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `BackupForGKE` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `BackupForGKE` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct BackupForGKE {
    inner: Arc<dyn crate::stubs::dynamic::BackupForGKE>,
}

impl BackupForGKE {
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
        T: crate::stubs::BackupForGKE + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::BackupForGKE>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::BackupForGKE> {
        crate::transport::BackupForGKE::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::BackupForGKE> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::BackupForGKE::new)
    }

    /// Creates a new BackupPlan in a given location.
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
    pub fn create_backup_plan(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::CreateBackupPlan {
        crate::builders::backup_for_gke::CreateBackupPlan::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists BackupPlans in a given location.
    pub fn list_backup_plans(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::ListBackupPlans {
        crate::builders::backup_for_gke::ListBackupPlans::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Retrieve the details of a single BackupPlan.
    pub fn get_backup_plan(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::GetBackupPlan {
        crate::builders::backup_for_gke::GetBackupPlan::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Update a BackupPlan.
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
    pub fn update_backup_plan(
        &self,
        backup_plan: impl Into<crate::model::BackupPlan>,
    ) -> crate::builders::backup_for_gke::UpdateBackupPlan {
        crate::builders::backup_for_gke::UpdateBackupPlan::new(self.inner.clone())
            .set_backup_plan(backup_plan.into())
    }

    /// Deletes an existing BackupPlan.
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
    pub fn delete_backup_plan(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::DeleteBackupPlan {
        crate::builders::backup_for_gke::DeleteBackupPlan::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a Backup for the given BackupPlan.
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
    pub fn create_backup(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::CreateBackup {
        crate::builders::backup_for_gke::CreateBackup::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists the Backups for a given BackupPlan.
    pub fn list_backups(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::ListBackups {
        crate::builders::backup_for_gke::ListBackups::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Retrieve the details of a single Backup.
    pub fn get_backup(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::GetBackup {
        crate::builders::backup_for_gke::GetBackup::new(self.inner.clone()).set_name(name.into())
    }

    /// Update a Backup.
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
    pub fn update_backup(
        &self,
        backup: impl Into<crate::model::Backup>,
    ) -> crate::builders::backup_for_gke::UpdateBackup {
        crate::builders::backup_for_gke::UpdateBackup::new(self.inner.clone())
            .set_backup(backup.into())
    }

    /// Deletes an existing Backup.
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
    pub fn delete_backup(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::DeleteBackup {
        crate::builders::backup_for_gke::DeleteBackup::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists the VolumeBackups for a given Backup.
    pub fn list_volume_backups(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::ListVolumeBackups {
        crate::builders::backup_for_gke::ListVolumeBackups::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Retrieve the details of a single VolumeBackup.
    pub fn get_volume_backup(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::GetVolumeBackup {
        crate::builders::backup_for_gke::GetVolumeBackup::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new RestorePlan in a given location.
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
    pub fn create_restore_plan(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::CreateRestorePlan {
        crate::builders::backup_for_gke::CreateRestorePlan::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists RestorePlans in a given location.
    pub fn list_restore_plans(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::ListRestorePlans {
        crate::builders::backup_for_gke::ListRestorePlans::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Retrieve the details of a single RestorePlan.
    pub fn get_restore_plan(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::GetRestorePlan {
        crate::builders::backup_for_gke::GetRestorePlan::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Update a RestorePlan.
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
    pub fn update_restore_plan(
        &self,
        restore_plan: impl Into<crate::model::RestorePlan>,
    ) -> crate::builders::backup_for_gke::UpdateRestorePlan {
        crate::builders::backup_for_gke::UpdateRestorePlan::new(self.inner.clone())
            .set_restore_plan(restore_plan.into())
    }

    /// Deletes an existing RestorePlan.
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
    pub fn delete_restore_plan(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::DeleteRestorePlan {
        crate::builders::backup_for_gke::DeleteRestorePlan::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new Restore for the given RestorePlan.
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
    pub fn create_restore(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::CreateRestore {
        crate::builders::backup_for_gke::CreateRestore::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists the Restores for a given RestorePlan.
    pub fn list_restores(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::ListRestores {
        crate::builders::backup_for_gke::ListRestores::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Retrieves the details of a single Restore.
    pub fn get_restore(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::GetRestore {
        crate::builders::backup_for_gke::GetRestore::new(self.inner.clone()).set_name(name.into())
    }

    /// Update a Restore.
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
    pub fn update_restore(
        &self,
        restore: impl Into<crate::model::Restore>,
    ) -> crate::builders::backup_for_gke::UpdateRestore {
        crate::builders::backup_for_gke::UpdateRestore::new(self.inner.clone())
            .set_restore(restore.into())
    }

    /// Deletes an existing Restore.
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
    pub fn delete_restore(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::DeleteRestore {
        crate::builders::backup_for_gke::DeleteRestore::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists the VolumeRestores for a given Restore.
    pub fn list_volume_restores(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::ListVolumeRestores {
        crate::builders::backup_for_gke::ListVolumeRestores::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Retrieve the details of a single VolumeRestore.
    pub fn get_volume_restore(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::GetVolumeRestore {
        crate::builders::backup_for_gke::GetVolumeRestore::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Retrieve the link to the backupIndex.
    pub fn get_backup_index_download_url(
        &self,
        backup: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::GetBackupIndexDownloadUrl {
        crate::builders::backup_for_gke::GetBackupIndexDownloadUrl::new(self.inner.clone())
            .set_backup(backup.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::ListLocations {
        crate::builders::backup_for_gke::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::GetLocation {
        crate::builders::backup_for_gke::GetLocation::new(self.inner.clone()).set_name(name.into())
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::SetIamPolicy {
        crate::builders::backup_for_gke::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::GetIamPolicy {
        crate::builders::backup_for_gke::GetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
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
    ) -> crate::builders::backup_for_gke::TestIamPermissions {
        crate::builders::backup_for_gke::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::ListOperations {
        crate::builders::backup_for_gke::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::GetOperation {
        crate::builders::backup_for_gke::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::DeleteOperation {
        crate::builders::backup_for_gke::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::backup_for_gke::CancelOperation {
        crate::builders::backup_for_gke::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
