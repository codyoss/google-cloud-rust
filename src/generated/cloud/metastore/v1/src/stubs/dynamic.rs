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

/// A dyn-compatible, crate-private version of [super::DataprocMetastore].
#[async_trait::async_trait]
pub trait DataprocMetastore: std::fmt::Debug + Send + Sync {
    async fn list_services(
        &self,
        req: crate::model::ListServicesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListServicesResponse>;

    async fn get_service(
        &self,
        req: crate::model::GetServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Service>;

    async fn create_service(
        &self,
        req: crate::model::CreateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_service(
        &self,
        req: crate::model::UpdateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_service(
        &self,
        req: crate::model::DeleteServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_metadata_imports(
        &self,
        req: crate::model::ListMetadataImportsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListMetadataImportsResponse>;

    async fn get_metadata_import(
        &self,
        req: crate::model::GetMetadataImportRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MetadataImport>;

    async fn create_metadata_import(
        &self,
        req: crate::model::CreateMetadataImportRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_metadata_import(
        &self,
        req: crate::model::UpdateMetadataImportRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn export_metadata(
        &self,
        req: crate::model::ExportMetadataRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn restore_service(
        &self,
        req: crate::model::RestoreServiceRequest,
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

    async fn create_backup(
        &self,
        req: crate::model::CreateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_backup(
        &self,
        req: crate::model::DeleteBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn query_metadata(
        &self,
        req: crate::model::QueryMetadataRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn move_table_to_database(
        &self,
        req: crate::model::MoveTableToDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn alter_metadata_resource_location(
        &self,
        req: crate::model::AlterMetadataResourceLocationRequest,
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

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [crate::stubs::DataprocMetastore] also implement [DataprocMetastore].
#[async_trait::async_trait]
impl<T: crate::stubs::DataprocMetastore> DataprocMetastore for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_services(
        &self,
        req: crate::model::ListServicesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListServicesResponse> {
        T::list_services(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_service(
        &self,
        req: crate::model::GetServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Service> {
        T::get_service(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_service(
        &self,
        req: crate::model::CreateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_service(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_service(
        &self,
        req: crate::model::UpdateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_service(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_service(
        &self,
        req: crate::model::DeleteServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_service(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_metadata_imports(
        &self,
        req: crate::model::ListMetadataImportsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListMetadataImportsResponse> {
        T::list_metadata_imports(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_metadata_import(
        &self,
        req: crate::model::GetMetadataImportRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MetadataImport> {
        T::get_metadata_import(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_metadata_import(
        &self,
        req: crate::model::CreateMetadataImportRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_metadata_import(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_metadata_import(
        &self,
        req: crate::model::UpdateMetadataImportRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_metadata_import(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn export_metadata(
        &self,
        req: crate::model::ExportMetadataRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::export_metadata(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn restore_service(
        &self,
        req: crate::model::RestoreServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::restore_service(self, req, options).await
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
    async fn create_backup(
        &self,
        req: crate::model::CreateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_backup(self, req, options).await
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
    async fn query_metadata(
        &self,
        req: crate::model::QueryMetadataRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::query_metadata(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn move_table_to_database(
        &self,
        req: crate::model::MoveTableToDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::move_table_to_database(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn alter_metadata_resource_location(
        &self,
        req: crate::model::AlterMetadataResourceLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::alter_metadata_resource_location(self, req, options).await
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

/// A dyn-compatible, crate-private version of [super::DataprocMetastoreFederation].
#[async_trait::async_trait]
pub trait DataprocMetastoreFederation: std::fmt::Debug + Send + Sync {
    async fn list_federations(
        &self,
        req: crate::model::ListFederationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListFederationsResponse>;

    async fn get_federation(
        &self,
        req: crate::model::GetFederationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Federation>;

    async fn create_federation(
        &self,
        req: crate::model::CreateFederationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_federation(
        &self,
        req: crate::model::UpdateFederationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_federation(
        &self,
        req: crate::model::DeleteFederationRequest,
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

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [crate::stubs::DataprocMetastoreFederation] also implement [DataprocMetastoreFederation].
#[async_trait::async_trait]
impl<T: crate::stubs::DataprocMetastoreFederation> DataprocMetastoreFederation for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_federations(
        &self,
        req: crate::model::ListFederationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListFederationsResponse> {
        T::list_federations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_federation(
        &self,
        req: crate::model::GetFederationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Federation> {
        T::get_federation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_federation(
        &self,
        req: crate::model::CreateFederationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_federation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_federation(
        &self,
        req: crate::model::UpdateFederationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_federation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_federation(
        &self,
        req: crate::model::DeleteFederationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_federation(self, req, options).await
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
