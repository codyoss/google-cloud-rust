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

/// A dyn-compatible, crate-private version of [super::NetApp].
#[async_trait::async_trait]
pub trait NetApp: std::fmt::Debug + Send + Sync {
    async fn list_storage_pools(
        &self,
        req: crate::model::ListStoragePoolsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListStoragePoolsResponse>;

    async fn create_storage_pool(
        &self,
        req: crate::model::CreateStoragePoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_storage_pool(
        &self,
        req: crate::model::GetStoragePoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::StoragePool>;

    async fn update_storage_pool(
        &self,
        req: crate::model::UpdateStoragePoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_storage_pool(
        &self,
        req: crate::model::DeleteStoragePoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn validate_directory_service(
        &self,
        req: crate::model::ValidateDirectoryServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn switch_active_replica_zone(
        &self,
        req: crate::model::SwitchActiveReplicaZoneRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_volumes(
        &self,
        req: crate::model::ListVolumesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListVolumesResponse>;

    async fn get_volume(
        &self,
        req: crate::model::GetVolumeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Volume>;

    async fn create_volume(
        &self,
        req: crate::model::CreateVolumeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_volume(
        &self,
        req: crate::model::UpdateVolumeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_volume(
        &self,
        req: crate::model::DeleteVolumeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn revert_volume(
        &self,
        req: crate::model::RevertVolumeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_snapshots(
        &self,
        req: crate::model::ListSnapshotsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListSnapshotsResponse>;

    async fn get_snapshot(
        &self,
        req: crate::model::GetSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Snapshot>;

    async fn create_snapshot(
        &self,
        req: crate::model::CreateSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_snapshot(
        &self,
        req: crate::model::DeleteSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_snapshot(
        &self,
        req: crate::model::UpdateSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_active_directories(
        &self,
        req: crate::model::ListActiveDirectoriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListActiveDirectoriesResponse>;

    async fn get_active_directory(
        &self,
        req: crate::model::GetActiveDirectoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ActiveDirectory>;

    async fn create_active_directory(
        &self,
        req: crate::model::CreateActiveDirectoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_active_directory(
        &self,
        req: crate::model::UpdateActiveDirectoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_active_directory(
        &self,
        req: crate::model::DeleteActiveDirectoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_kms_configs(
        &self,
        req: crate::model::ListKmsConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListKmsConfigsResponse>;

    async fn create_kms_config(
        &self,
        req: crate::model::CreateKmsConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_kms_config(
        &self,
        req: crate::model::GetKmsConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::KmsConfig>;

    async fn update_kms_config(
        &self,
        req: crate::model::UpdateKmsConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn encrypt_volumes(
        &self,
        req: crate::model::EncryptVolumesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn verify_kms_config(
        &self,
        req: crate::model::VerifyKmsConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VerifyKmsConfigResponse>;

    async fn delete_kms_config(
        &self,
        req: crate::model::DeleteKmsConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_replications(
        &self,
        req: crate::model::ListReplicationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListReplicationsResponse>;

    async fn get_replication(
        &self,
        req: crate::model::GetReplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Replication>;

    async fn create_replication(
        &self,
        req: crate::model::CreateReplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_replication(
        &self,
        req: crate::model::DeleteReplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_replication(
        &self,
        req: crate::model::UpdateReplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn stop_replication(
        &self,
        req: crate::model::StopReplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn resume_replication(
        &self,
        req: crate::model::ResumeReplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn reverse_replication_direction(
        &self,
        req: crate::model::ReverseReplicationDirectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn establish_peering(
        &self,
        req: crate::model::EstablishPeeringRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn sync_replication(
        &self,
        req: crate::model::SyncReplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_backup_vault(
        &self,
        req: crate::model::CreateBackupVaultRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_backup_vault(
        &self,
        req: crate::model::GetBackupVaultRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BackupVault>;

    async fn list_backup_vaults(
        &self,
        req: crate::model::ListBackupVaultsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBackupVaultsResponse>;

    async fn update_backup_vault(
        &self,
        req: crate::model::UpdateBackupVaultRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_backup_vault(
        &self,
        req: crate::model::DeleteBackupVaultRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_backup(
        &self,
        req: crate::model::CreateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_backup(
        &self,
        req: crate::model::GetBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Backup>;

    async fn list_backups(
        &self,
        req: crate::model::ListBackupsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBackupsResponse>;

    async fn delete_backup(
        &self,
        req: crate::model::DeleteBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_backup(
        &self,
        req: crate::model::UpdateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_backup_policy(
        &self,
        req: crate::model::CreateBackupPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_backup_policy(
        &self,
        req: crate::model::GetBackupPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BackupPolicy>;

    async fn list_backup_policies(
        &self,
        req: crate::model::ListBackupPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBackupPoliciesResponse>;

    async fn update_backup_policy(
        &self,
        req: crate::model::UpdateBackupPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_backup_policy(
        &self,
        req: crate::model::DeleteBackupPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_quota_rules(
        &self,
        req: crate::model::ListQuotaRulesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListQuotaRulesResponse>;

    async fn get_quota_rule(
        &self,
        req: crate::model::GetQuotaRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::QuotaRule>;

    async fn create_quota_rule(
        &self,
        req: crate::model::CreateQuotaRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_quota_rule(
        &self,
        req: crate::model::UpdateQuotaRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_quota_rule(
        &self,
        req: crate::model::DeleteQuotaRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

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

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [crate::stubs::NetApp] also implement [NetApp].
#[async_trait::async_trait]
impl<T: crate::stubs::NetApp> NetApp for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_storage_pools(
        &self,
        req: crate::model::ListStoragePoolsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListStoragePoolsResponse> {
        T::list_storage_pools(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_storage_pool(
        &self,
        req: crate::model::CreateStoragePoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_storage_pool(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_storage_pool(
        &self,
        req: crate::model::GetStoragePoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::StoragePool> {
        T::get_storage_pool(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_storage_pool(
        &self,
        req: crate::model::UpdateStoragePoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_storage_pool(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_storage_pool(
        &self,
        req: crate::model::DeleteStoragePoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_storage_pool(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn validate_directory_service(
        &self,
        req: crate::model::ValidateDirectoryServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::validate_directory_service(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn switch_active_replica_zone(
        &self,
        req: crate::model::SwitchActiveReplicaZoneRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::switch_active_replica_zone(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_volumes(
        &self,
        req: crate::model::ListVolumesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListVolumesResponse> {
        T::list_volumes(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_volume(
        &self,
        req: crate::model::GetVolumeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Volume> {
        T::get_volume(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_volume(
        &self,
        req: crate::model::CreateVolumeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_volume(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_volume(
        &self,
        req: crate::model::UpdateVolumeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_volume(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_volume(
        &self,
        req: crate::model::DeleteVolumeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_volume(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn revert_volume(
        &self,
        req: crate::model::RevertVolumeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::revert_volume(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_snapshots(
        &self,
        req: crate::model::ListSnapshotsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListSnapshotsResponse> {
        T::list_snapshots(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_snapshot(
        &self,
        req: crate::model::GetSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Snapshot> {
        T::get_snapshot(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_snapshot(
        &self,
        req: crate::model::CreateSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_snapshot(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_snapshot(
        &self,
        req: crate::model::DeleteSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_snapshot(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_snapshot(
        &self,
        req: crate::model::UpdateSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_snapshot(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_active_directories(
        &self,
        req: crate::model::ListActiveDirectoriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListActiveDirectoriesResponse> {
        T::list_active_directories(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_active_directory(
        &self,
        req: crate::model::GetActiveDirectoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ActiveDirectory> {
        T::get_active_directory(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_active_directory(
        &self,
        req: crate::model::CreateActiveDirectoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_active_directory(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_active_directory(
        &self,
        req: crate::model::UpdateActiveDirectoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_active_directory(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_active_directory(
        &self,
        req: crate::model::DeleteActiveDirectoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_active_directory(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_kms_configs(
        &self,
        req: crate::model::ListKmsConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListKmsConfigsResponse> {
        T::list_kms_configs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_kms_config(
        &self,
        req: crate::model::CreateKmsConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_kms_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_kms_config(
        &self,
        req: crate::model::GetKmsConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::KmsConfig> {
        T::get_kms_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_kms_config(
        &self,
        req: crate::model::UpdateKmsConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_kms_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn encrypt_volumes(
        &self,
        req: crate::model::EncryptVolumesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::encrypt_volumes(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn verify_kms_config(
        &self,
        req: crate::model::VerifyKmsConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VerifyKmsConfigResponse> {
        T::verify_kms_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_kms_config(
        &self,
        req: crate::model::DeleteKmsConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_kms_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_replications(
        &self,
        req: crate::model::ListReplicationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListReplicationsResponse> {
        T::list_replications(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_replication(
        &self,
        req: crate::model::GetReplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Replication> {
        T::get_replication(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_replication(
        &self,
        req: crate::model::CreateReplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_replication(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_replication(
        &self,
        req: crate::model::DeleteReplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_replication(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_replication(
        &self,
        req: crate::model::UpdateReplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_replication(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn stop_replication(
        &self,
        req: crate::model::StopReplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::stop_replication(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn resume_replication(
        &self,
        req: crate::model::ResumeReplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::resume_replication(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn reverse_replication_direction(
        &self,
        req: crate::model::ReverseReplicationDirectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::reverse_replication_direction(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn establish_peering(
        &self,
        req: crate::model::EstablishPeeringRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::establish_peering(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn sync_replication(
        &self,
        req: crate::model::SyncReplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::sync_replication(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_backup_vault(
        &self,
        req: crate::model::CreateBackupVaultRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_backup_vault(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_backup_vault(
        &self,
        req: crate::model::GetBackupVaultRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BackupVault> {
        T::get_backup_vault(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_backup_vaults(
        &self,
        req: crate::model::ListBackupVaultsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBackupVaultsResponse> {
        T::list_backup_vaults(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_backup_vault(
        &self,
        req: crate::model::UpdateBackupVaultRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_backup_vault(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_backup_vault(
        &self,
        req: crate::model::DeleteBackupVaultRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_backup_vault(self, req, options).await
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
    async fn get_backup(
        &self,
        req: crate::model::GetBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Backup> {
        T::get_backup(self, req, options).await
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
    async fn delete_backup(
        &self,
        req: crate::model::DeleteBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_backup(self, req, options).await
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
    async fn create_backup_policy(
        &self,
        req: crate::model::CreateBackupPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_backup_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_backup_policy(
        &self,
        req: crate::model::GetBackupPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BackupPolicy> {
        T::get_backup_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_backup_policies(
        &self,
        req: crate::model::ListBackupPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBackupPoliciesResponse> {
        T::list_backup_policies(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_backup_policy(
        &self,
        req: crate::model::UpdateBackupPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_backup_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_backup_policy(
        &self,
        req: crate::model::DeleteBackupPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_backup_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_quota_rules(
        &self,
        req: crate::model::ListQuotaRulesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListQuotaRulesResponse> {
        T::list_quota_rules(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_quota_rule(
        &self,
        req: crate::model::GetQuotaRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::QuotaRule> {
        T::get_quota_rule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_quota_rule(
        &self,
        req: crate::model::CreateQuotaRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_quota_rule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_quota_rule(
        &self,
        req: crate::model::UpdateQuotaRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_quota_rule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_quota_rule(
        &self,
        req: crate::model::DeleteQuotaRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_quota_rule(self, req, options).await
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

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        T::get_polling_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}
