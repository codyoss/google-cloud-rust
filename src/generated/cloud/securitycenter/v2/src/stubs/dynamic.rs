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

/// A dyn-compatible, crate-private version of [super::SecurityCenter].
#[async_trait::async_trait]
pub trait SecurityCenter: std::fmt::Debug + Send + Sync {
    async fn batch_create_resource_value_configs(
        &self,
        req: crate::model::BatchCreateResourceValueConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BatchCreateResourceValueConfigsResponse>;

    async fn bulk_mute_findings(
        &self,
        req: crate::model::BulkMuteFindingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_big_query_export(
        &self,
        req: crate::model::CreateBigQueryExportRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BigQueryExport>;

    async fn create_finding(
        &self,
        req: crate::model::CreateFindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Finding>;

    async fn create_mute_config(
        &self,
        req: crate::model::CreateMuteConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MuteConfig>;

    async fn create_notification_config(
        &self,
        req: crate::model::CreateNotificationConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::NotificationConfig>;

    async fn create_source(
        &self,
        req: crate::model::CreateSourceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Source>;

    async fn delete_big_query_export(
        &self,
        req: crate::model::DeleteBigQueryExportRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn delete_mute_config(
        &self,
        req: crate::model::DeleteMuteConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn delete_notification_config(
        &self,
        req: crate::model::DeleteNotificationConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn delete_resource_value_config(
        &self,
        req: crate::model::DeleteResourceValueConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn get_big_query_export(
        &self,
        req: crate::model::GetBigQueryExportRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BigQueryExport>;

    async fn get_simulation(
        &self,
        req: crate::model::GetSimulationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Simulation>;

    async fn get_valued_resource(
        &self,
        req: crate::model::GetValuedResourceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ValuedResource>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn get_mute_config(
        &self,
        req: crate::model::GetMuteConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MuteConfig>;

    async fn get_notification_config(
        &self,
        req: crate::model::GetNotificationConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::NotificationConfig>;

    async fn get_resource_value_config(
        &self,
        req: crate::model::GetResourceValueConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ResourceValueConfig>;

    async fn get_source(
        &self,
        req: crate::model::GetSourceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Source>;

    async fn group_findings(
        &self,
        req: crate::model::GroupFindingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GroupFindingsResponse>;

    async fn list_attack_paths(
        &self,
        req: crate::model::ListAttackPathsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListAttackPathsResponse>;

    async fn list_big_query_exports(
        &self,
        req: crate::model::ListBigQueryExportsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBigQueryExportsResponse>;

    async fn list_findings(
        &self,
        req: crate::model::ListFindingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListFindingsResponse>;

    async fn list_mute_configs(
        &self,
        req: crate::model::ListMuteConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListMuteConfigsResponse>;

    async fn list_notification_configs(
        &self,
        req: crate::model::ListNotificationConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListNotificationConfigsResponse>;

    async fn list_resource_value_configs(
        &self,
        req: crate::model::ListResourceValueConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListResourceValueConfigsResponse>;

    async fn list_sources(
        &self,
        req: crate::model::ListSourcesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListSourcesResponse>;

    async fn list_valued_resources(
        &self,
        req: crate::model::ListValuedResourcesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListValuedResourcesResponse>;

    async fn set_finding_state(
        &self,
        req: crate::model::SetFindingStateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Finding>;

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn set_mute(
        &self,
        req: crate::model::SetMuteRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Finding>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;

    async fn update_big_query_export(
        &self,
        req: crate::model::UpdateBigQueryExportRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BigQueryExport>;

    async fn update_external_system(
        &self,
        req: crate::model::UpdateExternalSystemRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ExternalSystem>;

    async fn update_finding(
        &self,
        req: crate::model::UpdateFindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Finding>;

    async fn update_mute_config(
        &self,
        req: crate::model::UpdateMuteConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MuteConfig>;

    async fn update_notification_config(
        &self,
        req: crate::model::UpdateNotificationConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::NotificationConfig>;

    async fn update_resource_value_config(
        &self,
        req: crate::model::UpdateResourceValueConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ResourceValueConfig>;

    async fn update_security_marks(
        &self,
        req: crate::model::UpdateSecurityMarksRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SecurityMarks>;

    async fn update_source(
        &self,
        req: crate::model::UpdateSourceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Source>;

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

/// All implementations of [super::SecurityCenter] also implement [SecurityCenter].
#[async_trait::async_trait]
impl<T: super::SecurityCenter> SecurityCenter for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn batch_create_resource_value_configs(
        &self,
        req: crate::model::BatchCreateResourceValueConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BatchCreateResourceValueConfigsResponse> {
        T::batch_create_resource_value_configs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn bulk_mute_findings(
        &self,
        req: crate::model::BulkMuteFindingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::bulk_mute_findings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_big_query_export(
        &self,
        req: crate::model::CreateBigQueryExportRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BigQueryExport> {
        T::create_big_query_export(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_finding(
        &self,
        req: crate::model::CreateFindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Finding> {
        T::create_finding(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_mute_config(
        &self,
        req: crate::model::CreateMuteConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MuteConfig> {
        T::create_mute_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_notification_config(
        &self,
        req: crate::model::CreateNotificationConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::NotificationConfig> {
        T::create_notification_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_source(
        &self,
        req: crate::model::CreateSourceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Source> {
        T::create_source(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_big_query_export(
        &self,
        req: crate::model::DeleteBigQueryExportRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_big_query_export(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_mute_config(
        &self,
        req: crate::model::DeleteMuteConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_mute_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_notification_config(
        &self,
        req: crate::model::DeleteNotificationConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_notification_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_resource_value_config(
        &self,
        req: crate::model::DeleteResourceValueConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_resource_value_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_big_query_export(
        &self,
        req: crate::model::GetBigQueryExportRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BigQueryExport> {
        T::get_big_query_export(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_simulation(
        &self,
        req: crate::model::GetSimulationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Simulation> {
        T::get_simulation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_valued_resource(
        &self,
        req: crate::model::GetValuedResourceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ValuedResource> {
        T::get_valued_resource(self, req, options).await
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
    async fn get_mute_config(
        &self,
        req: crate::model::GetMuteConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MuteConfig> {
        T::get_mute_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_notification_config(
        &self,
        req: crate::model::GetNotificationConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::NotificationConfig> {
        T::get_notification_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_resource_value_config(
        &self,
        req: crate::model::GetResourceValueConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ResourceValueConfig> {
        T::get_resource_value_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_source(
        &self,
        req: crate::model::GetSourceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Source> {
        T::get_source(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn group_findings(
        &self,
        req: crate::model::GroupFindingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GroupFindingsResponse> {
        T::group_findings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_attack_paths(
        &self,
        req: crate::model::ListAttackPathsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListAttackPathsResponse> {
        T::list_attack_paths(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_big_query_exports(
        &self,
        req: crate::model::ListBigQueryExportsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBigQueryExportsResponse> {
        T::list_big_query_exports(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_findings(
        &self,
        req: crate::model::ListFindingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListFindingsResponse> {
        T::list_findings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_mute_configs(
        &self,
        req: crate::model::ListMuteConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListMuteConfigsResponse> {
        T::list_mute_configs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_notification_configs(
        &self,
        req: crate::model::ListNotificationConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListNotificationConfigsResponse> {
        T::list_notification_configs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_resource_value_configs(
        &self,
        req: crate::model::ListResourceValueConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListResourceValueConfigsResponse> {
        T::list_resource_value_configs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_sources(
        &self,
        req: crate::model::ListSourcesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListSourcesResponse> {
        T::list_sources(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_valued_resources(
        &self,
        req: crate::model::ListValuedResourcesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListValuedResourcesResponse> {
        T::list_valued_resources(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_finding_state(
        &self,
        req: crate::model::SetFindingStateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Finding> {
        T::set_finding_state(self, req, options).await
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
    async fn set_mute(
        &self,
        req: crate::model::SetMuteRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Finding> {
        T::set_mute(self, req, options).await
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
    async fn update_big_query_export(
        &self,
        req: crate::model::UpdateBigQueryExportRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BigQueryExport> {
        T::update_big_query_export(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_external_system(
        &self,
        req: crate::model::UpdateExternalSystemRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ExternalSystem> {
        T::update_external_system(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_finding(
        &self,
        req: crate::model::UpdateFindingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Finding> {
        T::update_finding(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_mute_config(
        &self,
        req: crate::model::UpdateMuteConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MuteConfig> {
        T::update_mute_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_notification_config(
        &self,
        req: crate::model::UpdateNotificationConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::NotificationConfig> {
        T::update_notification_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_resource_value_config(
        &self,
        req: crate::model::UpdateResourceValueConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ResourceValueConfig> {
        T::update_resource_value_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_security_marks(
        &self,
        req: crate::model::UpdateSecurityMarksRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SecurityMarks> {
        T::update_security_marks(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_source(
        &self,
        req: crate::model::UpdateSourceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Source> {
        T::update_source(self, req, options).await
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
