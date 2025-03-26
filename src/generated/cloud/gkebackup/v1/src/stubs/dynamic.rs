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

/// A dyn-compatible, crate-private version of [super::BackupForGKE].
#[async_trait::async_trait]
pub trait BackupForGKE: std::fmt::Debug + Send + Sync {
    async fn create_backup_plan(
        &self,
        req: crate::model::CreateBackupPlanRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_backup_plans(
        &self,
        req: crate::model::ListBackupPlansRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBackupPlansResponse>;

    async fn get_backup_plan(
        &self,
        req: crate::model::GetBackupPlanRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BackupPlan>;

    async fn update_backup_plan(
        &self,
        req: crate::model::UpdateBackupPlanRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_backup_plan(
        &self,
        req: crate::model::DeleteBackupPlanRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_backup(
        &self,
        req: crate::model::CreateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_backups(
        &self,
        req: crate::model::ListBackupsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBackupsResponse>;

    async fn get_backup(
        &self,
        req: crate::model::GetBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Backup>;

    async fn update_backup(
        &self,
        req: crate::model::UpdateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_backup(
        &self,
        req: crate::model::DeleteBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_volume_backups(
        &self,
        req: crate::model::ListVolumeBackupsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListVolumeBackupsResponse>;

    async fn get_volume_backup(
        &self,
        req: crate::model::GetVolumeBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VolumeBackup>;

    async fn create_restore_plan(
        &self,
        req: crate::model::CreateRestorePlanRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_restore_plans(
        &self,
        req: crate::model::ListRestorePlansRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRestorePlansResponse>;

    async fn get_restore_plan(
        &self,
        req: crate::model::GetRestorePlanRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::RestorePlan>;

    async fn update_restore_plan(
        &self,
        req: crate::model::UpdateRestorePlanRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_restore_plan(
        &self,
        req: crate::model::DeleteRestorePlanRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_restore(
        &self,
        req: crate::model::CreateRestoreRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_restores(
        &self,
        req: crate::model::ListRestoresRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRestoresResponse>;

    async fn get_restore(
        &self,
        req: crate::model::GetRestoreRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Restore>;

    async fn update_restore(
        &self,
        req: crate::model::UpdateRestoreRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_restore(
        &self,
        req: crate::model::DeleteRestoreRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_volume_restores(
        &self,
        req: crate::model::ListVolumeRestoresRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListVolumeRestoresResponse>;

    async fn get_volume_restore(
        &self,
        req: crate::model::GetVolumeRestoreRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VolumeRestore>;

    async fn get_backup_index_download_url(
        &self,
        req: crate::model::GetBackupIndexDownloadUrlRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GetBackupIndexDownloadUrlResponse>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location>;

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::BackupForGKE] also implement [BackupForGKE].
#[async_trait::async_trait]
impl<T: super::BackupForGKE> BackupForGKE for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_backup_plan(
        &self,
        req: crate::model::CreateBackupPlanRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_backup_plan(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_backup_plans(
        &self,
        req: crate::model::ListBackupPlansRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBackupPlansResponse> {
        T::list_backup_plans(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_backup_plan(
        &self,
        req: crate::model::GetBackupPlanRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BackupPlan> {
        T::get_backup_plan(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_backup_plan(
        &self,
        req: crate::model::UpdateBackupPlanRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_backup_plan(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_backup_plan(
        &self,
        req: crate::model::DeleteBackupPlanRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_backup_plan(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_backup(
        &self,
        req: crate::model::CreateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_backup(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_backups(
        &self,
        req: crate::model::ListBackupsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBackupsResponse> {
        T::list_backups(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_backup(
        &self,
        req: crate::model::GetBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Backup> {
        T::get_backup(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_backup(
        &self,
        req: crate::model::UpdateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_backup(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_backup(
        &self,
        req: crate::model::DeleteBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_backup(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_volume_backups(
        &self,
        req: crate::model::ListVolumeBackupsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListVolumeBackupsResponse> {
        T::list_volume_backups(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_volume_backup(
        &self,
        req: crate::model::GetVolumeBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VolumeBackup> {
        T::get_volume_backup(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_restore_plan(
        &self,
        req: crate::model::CreateRestorePlanRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_restore_plan(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_restore_plans(
        &self,
        req: crate::model::ListRestorePlansRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRestorePlansResponse> {
        T::list_restore_plans(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_restore_plan(
        &self,
        req: crate::model::GetRestorePlanRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::RestorePlan> {
        T::get_restore_plan(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_restore_plan(
        &self,
        req: crate::model::UpdateRestorePlanRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_restore_plan(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_restore_plan(
        &self,
        req: crate::model::DeleteRestorePlanRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_restore_plan(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_restore(
        &self,
        req: crate::model::CreateRestoreRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_restore(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_restores(
        &self,
        req: crate::model::ListRestoresRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRestoresResponse> {
        T::list_restores(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_restore(
        &self,
        req: crate::model::GetRestoreRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Restore> {
        T::get_restore(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_restore(
        &self,
        req: crate::model::UpdateRestoreRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_restore(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_restore(
        &self,
        req: crate::model::DeleteRestoreRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_restore(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_volume_restores(
        &self,
        req: crate::model::ListVolumeRestoresRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListVolumeRestoresResponse> {
        T::list_volume_restores(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_volume_restore(
        &self,
        req: crate::model::GetVolumeRestoreRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VolumeRestore> {
        T::get_volume_restore(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_backup_index_download_url(
        &self,
        req: crate::model::GetBackupIndexDownloadUrlRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GetBackupIndexDownloadUrlResponse> {
        T::get_backup_index_download_url(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location> {
        T::get_location(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse> {
        T::test_iam_permissions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::cancel_operation(self, req, options).await
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
