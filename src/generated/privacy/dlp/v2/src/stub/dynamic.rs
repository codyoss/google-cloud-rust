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

/// A dyn-compatible, crate-private version of [super::DlpService].
#[async_trait::async_trait]
pub trait DlpService: std::fmt::Debug + Send + Sync {
    async fn inspect_content(
        &self,
        req: crate::model::InspectContentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::InspectContentResponse>>;

    async fn redact_image(
        &self,
        req: crate::model::RedactImageRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::RedactImageResponse>>;

    async fn deidentify_content(
        &self,
        req: crate::model::DeidentifyContentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DeidentifyContentResponse>>;

    async fn reidentify_content(
        &self,
        req: crate::model::ReidentifyContentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReidentifyContentResponse>>;

    async fn list_info_types(
        &self,
        req: crate::model::ListInfoTypesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListInfoTypesResponse>>;

    async fn create_inspect_template(
        &self,
        req: crate::model::CreateInspectTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::InspectTemplate>>;

    async fn update_inspect_template(
        &self,
        req: crate::model::UpdateInspectTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::InspectTemplate>>;

    async fn get_inspect_template(
        &self,
        req: crate::model::GetInspectTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::InspectTemplate>>;

    async fn list_inspect_templates(
        &self,
        req: crate::model::ListInspectTemplatesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListInspectTemplatesResponse>>;

    async fn delete_inspect_template(
        &self,
        req: crate::model::DeleteInspectTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn create_deidentify_template(
        &self,
        req: crate::model::CreateDeidentifyTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DeidentifyTemplate>>;

    async fn update_deidentify_template(
        &self,
        req: crate::model::UpdateDeidentifyTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DeidentifyTemplate>>;

    async fn get_deidentify_template(
        &self,
        req: crate::model::GetDeidentifyTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DeidentifyTemplate>>;

    async fn list_deidentify_templates(
        &self,
        req: crate::model::ListDeidentifyTemplatesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDeidentifyTemplatesResponse>>;

    async fn delete_deidentify_template(
        &self,
        req: crate::model::DeleteDeidentifyTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn create_job_trigger(
        &self,
        req: crate::model::CreateJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::JobTrigger>>;

    async fn update_job_trigger(
        &self,
        req: crate::model::UpdateJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::JobTrigger>>;

    async fn hybrid_inspect_job_trigger(
        &self,
        req: crate::model::HybridInspectJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::HybridInspectResponse>>;

    async fn get_job_trigger(
        &self,
        req: crate::model::GetJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::JobTrigger>>;

    async fn list_job_triggers(
        &self,
        req: crate::model::ListJobTriggersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListJobTriggersResponse>>;

    async fn delete_job_trigger(
        &self,
        req: crate::model::DeleteJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn activate_job_trigger(
        &self,
        req: crate::model::ActivateJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DlpJob>>;

    async fn create_discovery_config(
        &self,
        req: crate::model::CreateDiscoveryConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DiscoveryConfig>>;

    async fn update_discovery_config(
        &self,
        req: crate::model::UpdateDiscoveryConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DiscoveryConfig>>;

    async fn get_discovery_config(
        &self,
        req: crate::model::GetDiscoveryConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DiscoveryConfig>>;

    async fn list_discovery_configs(
        &self,
        req: crate::model::ListDiscoveryConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDiscoveryConfigsResponse>>;

    async fn delete_discovery_config(
        &self,
        req: crate::model::DeleteDiscoveryConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn create_dlp_job(
        &self,
        req: crate::model::CreateDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DlpJob>>;

    async fn list_dlp_jobs(
        &self,
        req: crate::model::ListDlpJobsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDlpJobsResponse>>;

    async fn get_dlp_job(
        &self,
        req: crate::model::GetDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DlpJob>>;

    async fn delete_dlp_job(
        &self,
        req: crate::model::DeleteDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn cancel_dlp_job(
        &self,
        req: crate::model::CancelDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn create_stored_info_type(
        &self,
        req: crate::model::CreateStoredInfoTypeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::StoredInfoType>>;

    async fn update_stored_info_type(
        &self,
        req: crate::model::UpdateStoredInfoTypeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::StoredInfoType>>;

    async fn get_stored_info_type(
        &self,
        req: crate::model::GetStoredInfoTypeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::StoredInfoType>>;

    async fn list_stored_info_types(
        &self,
        req: crate::model::ListStoredInfoTypesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListStoredInfoTypesResponse>>;

    async fn delete_stored_info_type(
        &self,
        req: crate::model::DeleteStoredInfoTypeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn list_project_data_profiles(
        &self,
        req: crate::model::ListProjectDataProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListProjectDataProfilesResponse>>;

    async fn list_table_data_profiles(
        &self,
        req: crate::model::ListTableDataProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListTableDataProfilesResponse>>;

    async fn list_column_data_profiles(
        &self,
        req: crate::model::ListColumnDataProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListColumnDataProfilesResponse>>;

    async fn get_project_data_profile(
        &self,
        req: crate::model::GetProjectDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ProjectDataProfile>>;

    async fn list_file_store_data_profiles(
        &self,
        req: crate::model::ListFileStoreDataProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListFileStoreDataProfilesResponse>>;

    async fn get_file_store_data_profile(
        &self,
        req: crate::model::GetFileStoreDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::FileStoreDataProfile>>;

    async fn delete_file_store_data_profile(
        &self,
        req: crate::model::DeleteFileStoreDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn get_table_data_profile(
        &self,
        req: crate::model::GetTableDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TableDataProfile>>;

    async fn get_column_data_profile(
        &self,
        req: crate::model::GetColumnDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ColumnDataProfile>>;

    async fn delete_table_data_profile(
        &self,
        req: crate::model::DeleteTableDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn hybrid_inspect_dlp_job(
        &self,
        req: crate::model::HybridInspectDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::HybridInspectResponse>>;

    async fn finish_dlp_job(
        &self,
        req: crate::model::FinishDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn create_connection(
        &self,
        req: crate::model::CreateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Connection>>;

    async fn get_connection(
        &self,
        req: crate::model::GetConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Connection>>;

    async fn list_connections(
        &self,
        req: crate::model::ListConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListConnectionsResponse>>;

    async fn search_connections(
        &self,
        req: crate::model::SearchConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SearchConnectionsResponse>>;

    async fn delete_connection(
        &self,
        req: crate::model::DeleteConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn update_connection(
        &self,
        req: crate::model::UpdateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Connection>>;
}

/// All implementations of [super::DlpService] also implement [DlpService].
#[async_trait::async_trait]
impl<T: super::DlpService> DlpService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn inspect_content(
        &self,
        req: crate::model::InspectContentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::InspectContentResponse>> {
        T::inspect_content(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn redact_image(
        &self,
        req: crate::model::RedactImageRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::RedactImageResponse>> {
        T::redact_image(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn deidentify_content(
        &self,
        req: crate::model::DeidentifyContentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DeidentifyContentResponse>> {
        T::deidentify_content(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn reidentify_content(
        &self,
        req: crate::model::ReidentifyContentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReidentifyContentResponse>> {
        T::reidentify_content(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_info_types(
        &self,
        req: crate::model::ListInfoTypesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListInfoTypesResponse>> {
        T::list_info_types(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_inspect_template(
        &self,
        req: crate::model::CreateInspectTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::InspectTemplate>> {
        T::create_inspect_template(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_inspect_template(
        &self,
        req: crate::model::UpdateInspectTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::InspectTemplate>> {
        T::update_inspect_template(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_inspect_template(
        &self,
        req: crate::model::GetInspectTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::InspectTemplate>> {
        T::get_inspect_template(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_inspect_templates(
        &self,
        req: crate::model::ListInspectTemplatesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListInspectTemplatesResponse>> {
        T::list_inspect_templates(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_inspect_template(
        &self,
        req: crate::model::DeleteInspectTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_inspect_template(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_deidentify_template(
        &self,
        req: crate::model::CreateDeidentifyTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DeidentifyTemplate>> {
        T::create_deidentify_template(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_deidentify_template(
        &self,
        req: crate::model::UpdateDeidentifyTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DeidentifyTemplate>> {
        T::update_deidentify_template(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_deidentify_template(
        &self,
        req: crate::model::GetDeidentifyTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DeidentifyTemplate>> {
        T::get_deidentify_template(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_deidentify_templates(
        &self,
        req: crate::model::ListDeidentifyTemplatesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDeidentifyTemplatesResponse>> {
        T::list_deidentify_templates(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_deidentify_template(
        &self,
        req: crate::model::DeleteDeidentifyTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_deidentify_template(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_job_trigger(
        &self,
        req: crate::model::CreateJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::JobTrigger>> {
        T::create_job_trigger(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_job_trigger(
        &self,
        req: crate::model::UpdateJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::JobTrigger>> {
        T::update_job_trigger(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn hybrid_inspect_job_trigger(
        &self,
        req: crate::model::HybridInspectJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::HybridInspectResponse>> {
        T::hybrid_inspect_job_trigger(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_job_trigger(
        &self,
        req: crate::model::GetJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::JobTrigger>> {
        T::get_job_trigger(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_job_triggers(
        &self,
        req: crate::model::ListJobTriggersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListJobTriggersResponse>> {
        T::list_job_triggers(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_job_trigger(
        &self,
        req: crate::model::DeleteJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_job_trigger(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn activate_job_trigger(
        &self,
        req: crate::model::ActivateJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DlpJob>> {
        T::activate_job_trigger(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_discovery_config(
        &self,
        req: crate::model::CreateDiscoveryConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DiscoveryConfig>> {
        T::create_discovery_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_discovery_config(
        &self,
        req: crate::model::UpdateDiscoveryConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DiscoveryConfig>> {
        T::update_discovery_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_discovery_config(
        &self,
        req: crate::model::GetDiscoveryConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DiscoveryConfig>> {
        T::get_discovery_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_discovery_configs(
        &self,
        req: crate::model::ListDiscoveryConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDiscoveryConfigsResponse>> {
        T::list_discovery_configs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_discovery_config(
        &self,
        req: crate::model::DeleteDiscoveryConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_discovery_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_dlp_job(
        &self,
        req: crate::model::CreateDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DlpJob>> {
        T::create_dlp_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_dlp_jobs(
        &self,
        req: crate::model::ListDlpJobsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDlpJobsResponse>> {
        T::list_dlp_jobs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_dlp_job(
        &self,
        req: crate::model::GetDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DlpJob>> {
        T::get_dlp_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_dlp_job(
        &self,
        req: crate::model::DeleteDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_dlp_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_dlp_job(
        &self,
        req: crate::model::CancelDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::cancel_dlp_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_stored_info_type(
        &self,
        req: crate::model::CreateStoredInfoTypeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::StoredInfoType>> {
        T::create_stored_info_type(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_stored_info_type(
        &self,
        req: crate::model::UpdateStoredInfoTypeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::StoredInfoType>> {
        T::update_stored_info_type(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_stored_info_type(
        &self,
        req: crate::model::GetStoredInfoTypeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::StoredInfoType>> {
        T::get_stored_info_type(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_stored_info_types(
        &self,
        req: crate::model::ListStoredInfoTypesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListStoredInfoTypesResponse>> {
        T::list_stored_info_types(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_stored_info_type(
        &self,
        req: crate::model::DeleteStoredInfoTypeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_stored_info_type(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_project_data_profiles(
        &self,
        req: crate::model::ListProjectDataProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListProjectDataProfilesResponse>> {
        T::list_project_data_profiles(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_table_data_profiles(
        &self,
        req: crate::model::ListTableDataProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListTableDataProfilesResponse>> {
        T::list_table_data_profiles(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_column_data_profiles(
        &self,
        req: crate::model::ListColumnDataProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListColumnDataProfilesResponse>> {
        T::list_column_data_profiles(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_project_data_profile(
        &self,
        req: crate::model::GetProjectDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ProjectDataProfile>> {
        T::get_project_data_profile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_file_store_data_profiles(
        &self,
        req: crate::model::ListFileStoreDataProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListFileStoreDataProfilesResponse>>
    {
        T::list_file_store_data_profiles(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_file_store_data_profile(
        &self,
        req: crate::model::GetFileStoreDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::FileStoreDataProfile>> {
        T::get_file_store_data_profile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_file_store_data_profile(
        &self,
        req: crate::model::DeleteFileStoreDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_file_store_data_profile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_table_data_profile(
        &self,
        req: crate::model::GetTableDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TableDataProfile>> {
        T::get_table_data_profile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_column_data_profile(
        &self,
        req: crate::model::GetColumnDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ColumnDataProfile>> {
        T::get_column_data_profile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_table_data_profile(
        &self,
        req: crate::model::DeleteTableDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_table_data_profile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn hybrid_inspect_dlp_job(
        &self,
        req: crate::model::HybridInspectDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::HybridInspectResponse>> {
        T::hybrid_inspect_dlp_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn finish_dlp_job(
        &self,
        req: crate::model::FinishDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::finish_dlp_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_connection(
        &self,
        req: crate::model::CreateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Connection>> {
        T::create_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_connection(
        &self,
        req: crate::model::GetConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Connection>> {
        T::get_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_connections(
        &self,
        req: crate::model::ListConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListConnectionsResponse>> {
        T::list_connections(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn search_connections(
        &self,
        req: crate::model::SearchConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SearchConnectionsResponse>> {
        T::search_connections(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_connection(
        &self,
        req: crate::model::DeleteConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_connection(
        &self,
        req: crate::model::UpdateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Connection>> {
        T::update_connection(self, req, options).await
    }
}
