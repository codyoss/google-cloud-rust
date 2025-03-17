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

/// Implements a [DlpService](super::stubs::DlpService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct DlpService<T>
where
    T: super::stubs::DlpService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> DlpService<T>
where
    T: super::stubs::DlpService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::DlpService for DlpService<T>
where
    T: super::stubs::DlpService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn inspect_content(
        &self,
        req: crate::model::InspectContentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::InspectContentResponse> {
        self.inner.inspect_content(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn redact_image(
        &self,
        req: crate::model::RedactImageRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::RedactImageResponse> {
        self.inner.redact_image(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn deidentify_content(
        &self,
        req: crate::model::DeidentifyContentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DeidentifyContentResponse> {
        self.inner.deidentify_content(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn reidentify_content(
        &self,
        req: crate::model::ReidentifyContentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ReidentifyContentResponse> {
        self.inner.reidentify_content(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_info_types(
        &self,
        req: crate::model::ListInfoTypesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListInfoTypesResponse> {
        self.inner.list_info_types(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_inspect_template(
        &self,
        req: crate::model::CreateInspectTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::InspectTemplate> {
        self.inner.create_inspect_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_inspect_template(
        &self,
        req: crate::model::UpdateInspectTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::InspectTemplate> {
        self.inner.update_inspect_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_inspect_template(
        &self,
        req: crate::model::GetInspectTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::InspectTemplate> {
        self.inner.get_inspect_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_inspect_templates(
        &self,
        req: crate::model::ListInspectTemplatesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListInspectTemplatesResponse> {
        self.inner.list_inspect_templates(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_inspect_template(
        &self,
        req: crate::model::DeleteInspectTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_inspect_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_deidentify_template(
        &self,
        req: crate::model::CreateDeidentifyTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DeidentifyTemplate> {
        self.inner.create_deidentify_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_deidentify_template(
        &self,
        req: crate::model::UpdateDeidentifyTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DeidentifyTemplate> {
        self.inner.update_deidentify_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_deidentify_template(
        &self,
        req: crate::model::GetDeidentifyTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DeidentifyTemplate> {
        self.inner.get_deidentify_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_deidentify_templates(
        &self,
        req: crate::model::ListDeidentifyTemplatesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDeidentifyTemplatesResponse> {
        self.inner.list_deidentify_templates(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_deidentify_template(
        &self,
        req: crate::model::DeleteDeidentifyTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_deidentify_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_job_trigger(
        &self,
        req: crate::model::CreateJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::JobTrigger> {
        self.inner.create_job_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_job_trigger(
        &self,
        req: crate::model::UpdateJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::JobTrigger> {
        self.inner.update_job_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn hybrid_inspect_job_trigger(
        &self,
        req: crate::model::HybridInspectJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::HybridInspectResponse> {
        self.inner.hybrid_inspect_job_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_job_trigger(
        &self,
        req: crate::model::GetJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::JobTrigger> {
        self.inner.get_job_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_job_triggers(
        &self,
        req: crate::model::ListJobTriggersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListJobTriggersResponse> {
        self.inner.list_job_triggers(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_job_trigger(
        &self,
        req: crate::model::DeleteJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_job_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn activate_job_trigger(
        &self,
        req: crate::model::ActivateJobTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DlpJob> {
        self.inner.activate_job_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_discovery_config(
        &self,
        req: crate::model::CreateDiscoveryConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DiscoveryConfig> {
        self.inner.create_discovery_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_discovery_config(
        &self,
        req: crate::model::UpdateDiscoveryConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DiscoveryConfig> {
        self.inner.update_discovery_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_discovery_config(
        &self,
        req: crate::model::GetDiscoveryConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DiscoveryConfig> {
        self.inner.get_discovery_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_discovery_configs(
        &self,
        req: crate::model::ListDiscoveryConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDiscoveryConfigsResponse> {
        self.inner.list_discovery_configs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_discovery_config(
        &self,
        req: crate::model::DeleteDiscoveryConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_discovery_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_dlp_job(
        &self,
        req: crate::model::CreateDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DlpJob> {
        self.inner.create_dlp_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_dlp_jobs(
        &self,
        req: crate::model::ListDlpJobsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDlpJobsResponse> {
        self.inner.list_dlp_jobs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_dlp_job(
        &self,
        req: crate::model::GetDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DlpJob> {
        self.inner.get_dlp_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_dlp_job(
        &self,
        req: crate::model::DeleteDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_dlp_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_dlp_job(
        &self,
        req: crate::model::CancelDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_dlp_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_stored_info_type(
        &self,
        req: crate::model::CreateStoredInfoTypeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::StoredInfoType> {
        self.inner.create_stored_info_type(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_stored_info_type(
        &self,
        req: crate::model::UpdateStoredInfoTypeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::StoredInfoType> {
        self.inner.update_stored_info_type(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_stored_info_type(
        &self,
        req: crate::model::GetStoredInfoTypeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::StoredInfoType> {
        self.inner.get_stored_info_type(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_stored_info_types(
        &self,
        req: crate::model::ListStoredInfoTypesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListStoredInfoTypesResponse> {
        self.inner.list_stored_info_types(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_stored_info_type(
        &self,
        req: crate::model::DeleteStoredInfoTypeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_stored_info_type(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_project_data_profiles(
        &self,
        req: crate::model::ListProjectDataProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListProjectDataProfilesResponse> {
        self.inner.list_project_data_profiles(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_table_data_profiles(
        &self,
        req: crate::model::ListTableDataProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTableDataProfilesResponse> {
        self.inner.list_table_data_profiles(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_column_data_profiles(
        &self,
        req: crate::model::ListColumnDataProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListColumnDataProfilesResponse> {
        self.inner.list_column_data_profiles(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_project_data_profile(
        &self,
        req: crate::model::GetProjectDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ProjectDataProfile> {
        self.inner.get_project_data_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_file_store_data_profiles(
        &self,
        req: crate::model::ListFileStoreDataProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListFileStoreDataProfilesResponse> {
        self.inner.list_file_store_data_profiles(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_file_store_data_profile(
        &self,
        req: crate::model::GetFileStoreDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::FileStoreDataProfile> {
        self.inner.get_file_store_data_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_file_store_data_profile(
        &self,
        req: crate::model::DeleteFileStoreDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner
            .delete_file_store_data_profile(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn get_table_data_profile(
        &self,
        req: crate::model::GetTableDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TableDataProfile> {
        self.inner.get_table_data_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_column_data_profile(
        &self,
        req: crate::model::GetColumnDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ColumnDataProfile> {
        self.inner.get_column_data_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_table_data_profile(
        &self,
        req: crate::model::DeleteTableDataProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_table_data_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn hybrid_inspect_dlp_job(
        &self,
        req: crate::model::HybridInspectDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::HybridInspectResponse> {
        self.inner.hybrid_inspect_dlp_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn finish_dlp_job(
        &self,
        req: crate::model::FinishDlpJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.finish_dlp_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_connection(
        &self,
        req: crate::model::CreateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Connection> {
        self.inner.create_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_connection(
        &self,
        req: crate::model::GetConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Connection> {
        self.inner.get_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_connections(
        &self,
        req: crate::model::ListConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListConnectionsResponse> {
        self.inner.list_connections(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn search_connections(
        &self,
        req: crate::model::SearchConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SearchConnectionsResponse> {
        self.inner.search_connections(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_connection(
        &self,
        req: crate::model::DeleteConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_connection(
        &self,
        req: crate::model::UpdateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Connection> {
        self.inner.update_connection(req, options).await
    }
}
