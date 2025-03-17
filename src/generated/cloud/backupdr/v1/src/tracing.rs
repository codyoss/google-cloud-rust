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

/// Implements a [BackupDR](super::stubs::BackupDR) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct BackupDR<T>
where
    T: super::stubs::BackupDR + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> BackupDR<T>
where
    T: super::stubs::BackupDR + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::BackupDR for BackupDR<T>
where
    T: super::stubs::BackupDR + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_management_servers(
        &self,
        req: crate::model::ListManagementServersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListManagementServersResponse> {
        self.inner.list_management_servers(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_management_server(
        &self,
        req: crate::model::GetManagementServerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ManagementServer> {
        self.inner.get_management_server(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_management_server(
        &self,
        req: crate::model::CreateManagementServerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_management_server(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_management_server(
        &self,
        req: crate::model::DeleteManagementServerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_management_server(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_backup_vault(
        &self,
        req: crate::model::CreateBackupVaultRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_backup_vault(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_backup_vaults(
        &self,
        req: crate::model::ListBackupVaultsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListBackupVaultsResponse> {
        self.inner.list_backup_vaults(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn fetch_usable_backup_vaults(
        &self,
        req: crate::model::FetchUsableBackupVaultsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::FetchUsableBackupVaultsResponse> {
        self.inner.fetch_usable_backup_vaults(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_backup_vault(
        &self,
        req: crate::model::GetBackupVaultRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::BackupVault> {
        self.inner.get_backup_vault(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_backup_vault(
        &self,
        req: crate::model::UpdateBackupVaultRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_backup_vault(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_backup_vault(
        &self,
        req: crate::model::DeleteBackupVaultRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_backup_vault(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_data_sources(
        &self,
        req: crate::model::ListDataSourcesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDataSourcesResponse> {
        self.inner.list_data_sources(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_data_source(
        &self,
        req: crate::model::GetDataSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DataSource> {
        self.inner.get_data_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_data_source(
        &self,
        req: crate::model::UpdateDataSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_data_source(req, options).await
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
    async fn update_backup(
        &self,
        req: crate::model::UpdateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_backup(req, options).await
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
    async fn restore_backup(
        &self,
        req: crate::model::RestoreBackupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.restore_backup(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_backup_plan(
        &self,
        req: crate::model::CreateBackupPlanRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_backup_plan(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_backup_plan(
        &self,
        req: crate::model::GetBackupPlanRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::BackupPlan> {
        self.inner.get_backup_plan(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_backup_plans(
        &self,
        req: crate::model::ListBackupPlansRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListBackupPlansResponse> {
        self.inner.list_backup_plans(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_backup_plan(
        &self,
        req: crate::model::DeleteBackupPlanRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_backup_plan(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_backup_plan_association(
        &self,
        req: crate::model::CreateBackupPlanAssociationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner
            .create_backup_plan_association(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn get_backup_plan_association(
        &self,
        req: crate::model::GetBackupPlanAssociationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::BackupPlanAssociation> {
        self.inner.get_backup_plan_association(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_backup_plan_associations(
        &self,
        req: crate::model::ListBackupPlanAssociationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListBackupPlanAssociationsResponse> {
        self.inner.list_backup_plan_associations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_backup_plan_association(
        &self,
        req: crate::model::DeleteBackupPlanAssociationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner
            .delete_backup_plan_association(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn trigger_backup(
        &self,
        req: crate::model::TriggerBackupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.trigger_backup(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn initialize_service(
        &self,
        req: crate::model::InitializeServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.initialize_service(req, options).await
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
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
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
