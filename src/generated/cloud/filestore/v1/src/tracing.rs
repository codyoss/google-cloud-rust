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

/// Implements a [CloudFilestoreManager](super::stubs::CloudFilestoreManager) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct CloudFilestoreManager<T>
where
    T: super::stubs::CloudFilestoreManager + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> CloudFilestoreManager<T>
where
    T: super::stubs::CloudFilestoreManager + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::CloudFilestoreManager for CloudFilestoreManager<T>
where
    T: super::stubs::CloudFilestoreManager + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_instances(
        &self,
        req: crate::model::ListInstancesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListInstancesResponse> {
        self.inner.list_instances(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_instance(
        &self,
        req: crate::model::GetInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Instance> {
        self.inner.get_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_instance(
        &self,
        req: crate::model::CreateInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_instance(
        &self,
        req: crate::model::UpdateInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn restore_instance(
        &self,
        req: crate::model::RestoreInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.restore_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn revert_instance(
        &self,
        req: crate::model::RevertInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.revert_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_instance(
        &self,
        req: crate::model::DeleteInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_snapshots(
        &self,
        req: crate::model::ListSnapshotsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSnapshotsResponse> {
        self.inner.list_snapshots(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_snapshot(
        &self,
        req: crate::model::GetSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Snapshot> {
        self.inner.get_snapshot(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_snapshot(
        &self,
        req: crate::model::CreateSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_snapshot(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_snapshot(
        &self,
        req: crate::model::DeleteSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_snapshot(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_snapshot(
        &self,
        req: crate::model::UpdateSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_snapshot(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_backups(
        &self,
        req: crate::model::ListBackupsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListBackupsResponse> {
        self.inner.list_backups(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_backup(
        &self,
        req: crate::model::GetBackupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Backup> {
        self.inner.get_backup(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_backup(
        &self,
        req: crate::model::CreateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_backup(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_backup(
        &self,
        req: crate::model::DeleteBackupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_backup(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_backup(
        &self,
        req: crate::model::UpdateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_backup(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn promote_replica(
        &self,
        req: crate::model::PromoteReplicaRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.promote_replica(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_operation(req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy> {
        self.inner.get_polling_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
