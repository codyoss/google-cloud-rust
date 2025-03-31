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

/// Implements a client for the Sensitive Data Protection (DLP).
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_privacy_dlp_v2::client::DlpService;
/// let client = DlpService::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Sensitive Data Protection provides access to a powerful sensitive data
/// inspection, classification, and de-identification platform that works
/// on text, images, and Google Cloud storage repositories.
/// To learn more about concepts and find how-to guides see
/// <https://cloud.google.com/sensitive-data-protection/docs/>.
///
/// # Configuration
///
/// To configure `DlpService` use the `with_*` methods in the type returned
/// by [builder()][DlpService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://dlp.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::dlp_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::dlp_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `DlpService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `DlpService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct DlpService {
    inner: Arc<dyn super::stub::dynamic::DlpService>,
}

impl DlpService {
    /// Returns a builder for [DlpService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_privacy_dlp_v2::client::DlpService;
    /// let client = DlpService::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::dlp_service::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::dlp_service::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::DlpService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::DlpService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::DlpService> {
        super::transport::DlpService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::DlpService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::DlpService::new)
    }

    /// Finds potentially sensitive info in content.
    /// This method has limits on input size, processing time, and output size.
    ///
    /// When no InfoTypes or CustomInfoTypes are specified in this request, the
    /// system will automatically choose what detectors to run. By default this may
    /// be all types, but may change over time as detectors are updated.
    ///
    /// For how to guides, see
    /// <https://cloud.google.com/sensitive-data-protection/docs/inspecting-images>
    /// and
    /// <https://cloud.google.com/sensitive-data-protection/docs/inspecting-text>,
    pub fn inspect_content(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::InspectContent {
        super::builder::dlp_service::InspectContent::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Redacts potentially sensitive info from an image.
    /// This method has limits on input size, processing time, and output size.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/redacting-sensitive-data-images>
    /// to learn more.
    ///
    /// When no InfoTypes or CustomInfoTypes are specified in this request, the
    /// system will automatically choose what detectors to run. By default this may
    /// be all types, but may change over time as detectors are updated.
    pub fn redact_image(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::RedactImage {
        super::builder::dlp_service::RedactImage::new(self.inner.clone()).set_parent(parent.into())
    }

    /// De-identifies potentially sensitive info from a ContentItem.
    /// This method has limits on input size and output size.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/deidentify-sensitive-data>
    /// to learn more.
    ///
    /// When no InfoTypes or CustomInfoTypes are specified in this request, the
    /// system will automatically choose what detectors to run. By default this may
    /// be all types, but may change over time as detectors are updated.
    pub fn deidentify_content(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::DeidentifyContent {
        super::builder::dlp_service::DeidentifyContent::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Re-identifies content that has been de-identified.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/pseudonymization#re-identification_in_free_text_code_example>
    /// to learn more.
    pub fn reidentify_content(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::ReidentifyContent {
        super::builder::dlp_service::ReidentifyContent::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Returns a list of the sensitive information types that the DLP API
    /// supports. See
    /// <https://cloud.google.com/sensitive-data-protection/docs/infotypes-reference>
    /// to learn more.
    pub fn list_info_types(&self) -> super::builder::dlp_service::ListInfoTypes {
        super::builder::dlp_service::ListInfoTypes::new(self.inner.clone())
    }

    /// Creates an InspectTemplate for reusing frequently used configuration
    /// for inspecting content, images, and storage.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-templates>
    /// to learn more.
    pub fn create_inspect_template(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::CreateInspectTemplate {
        super::builder::dlp_service::CreateInspectTemplate::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the InspectTemplate.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-templates>
    /// to learn more.
    pub fn update_inspect_template(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::UpdateInspectTemplate {
        super::builder::dlp_service::UpdateInspectTemplate::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets an InspectTemplate.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-templates>
    /// to learn more.
    pub fn get_inspect_template(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::GetInspectTemplate {
        super::builder::dlp_service::GetInspectTemplate::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists InspectTemplates.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-templates>
    /// to learn more.
    pub fn list_inspect_templates(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::ListInspectTemplates {
        super::builder::dlp_service::ListInspectTemplates::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes an InspectTemplate.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-templates>
    /// to learn more.
    pub fn delete_inspect_template(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::DeleteInspectTemplate {
        super::builder::dlp_service::DeleteInspectTemplate::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a DeidentifyTemplate for reusing frequently used configuration
    /// for de-identifying content, images, and storage.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid>
    /// to learn more.
    pub fn create_deidentify_template(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::CreateDeidentifyTemplate {
        super::builder::dlp_service::CreateDeidentifyTemplate::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the DeidentifyTemplate.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid>
    /// to learn more.
    pub fn update_deidentify_template(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::UpdateDeidentifyTemplate {
        super::builder::dlp_service::UpdateDeidentifyTemplate::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets a DeidentifyTemplate.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid>
    /// to learn more.
    pub fn get_deidentify_template(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::GetDeidentifyTemplate {
        super::builder::dlp_service::GetDeidentifyTemplate::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists DeidentifyTemplates.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid>
    /// to learn more.
    pub fn list_deidentify_templates(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::ListDeidentifyTemplates {
        super::builder::dlp_service::ListDeidentifyTemplates::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a DeidentifyTemplate.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid>
    /// to learn more.
    pub fn delete_deidentify_template(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::DeleteDeidentifyTemplate {
        super::builder::dlp_service::DeleteDeidentifyTemplate::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a job trigger to run DLP actions such as scanning storage for
    /// sensitive information on a set schedule.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers>
    /// to learn more.
    pub fn create_job_trigger(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::CreateJobTrigger {
        super::builder::dlp_service::CreateJobTrigger::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a job trigger.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers>
    /// to learn more.
    pub fn update_job_trigger(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::UpdateJobTrigger {
        super::builder::dlp_service::UpdateJobTrigger::new(self.inner.clone()).set_name(name.into())
    }

    /// Inspect hybrid content and store findings to a trigger. The inspection
    /// will be processed asynchronously. To review the findings monitor the
    /// jobs within the trigger.
    pub fn hybrid_inspect_job_trigger(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::HybridInspectJobTrigger {
        super::builder::dlp_service::HybridInspectJobTrigger::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets a job trigger.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers>
    /// to learn more.
    pub fn get_job_trigger(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::GetJobTrigger {
        super::builder::dlp_service::GetJobTrigger::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists job triggers.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers>
    /// to learn more.
    pub fn list_job_triggers(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::ListJobTriggers {
        super::builder::dlp_service::ListJobTriggers::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a job trigger.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers>
    /// to learn more.
    pub fn delete_job_trigger(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::DeleteJobTrigger {
        super::builder::dlp_service::DeleteJobTrigger::new(self.inner.clone()).set_name(name.into())
    }

    /// Activate a job trigger. Causes the immediate execute of a trigger
    /// instead of waiting on the trigger event to occur.
    pub fn activate_job_trigger(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::ActivateJobTrigger {
        super::builder::dlp_service::ActivateJobTrigger::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a config for discovery to scan and profile storage.
    pub fn create_discovery_config(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::CreateDiscoveryConfig {
        super::builder::dlp_service::CreateDiscoveryConfig::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a discovery configuration.
    pub fn update_discovery_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::UpdateDiscoveryConfig {
        super::builder::dlp_service::UpdateDiscoveryConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets a discovery configuration.
    pub fn get_discovery_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::GetDiscoveryConfig {
        super::builder::dlp_service::GetDiscoveryConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists discovery configurations.
    pub fn list_discovery_configs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::ListDiscoveryConfigs {
        super::builder::dlp_service::ListDiscoveryConfigs::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a discovery configuration.
    pub fn delete_discovery_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::DeleteDiscoveryConfig {
        super::builder::dlp_service::DeleteDiscoveryConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new job to inspect storage or calculate risk metrics.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage>
    /// and
    /// <https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis>
    /// to learn more.
    ///
    /// When no InfoTypes or CustomInfoTypes are specified in inspect jobs, the
    /// system will automatically choose what detectors to run. By default this may
    /// be all types, but may change over time as detectors are updated.
    pub fn create_dlp_job(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::CreateDlpJob {
        super::builder::dlp_service::CreateDlpJob::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Lists DlpJobs that match the specified filter in the request.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage>
    /// and
    /// <https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis>
    /// to learn more.
    pub fn list_dlp_jobs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::ListDlpJobs {
        super::builder::dlp_service::ListDlpJobs::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets the latest state of a long-running DlpJob.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage>
    /// and
    /// <https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis>
    /// to learn more.
    pub fn get_dlp_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::GetDlpJob {
        super::builder::dlp_service::GetDlpJob::new(self.inner.clone()).set_name(name.into())
    }

    /// Deletes a long-running DlpJob. This method indicates that the client is
    /// no longer interested in the DlpJob result. The job will be canceled if
    /// possible.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage>
    /// and
    /// <https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis>
    /// to learn more.
    pub fn delete_dlp_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::DeleteDlpJob {
        super::builder::dlp_service::DeleteDlpJob::new(self.inner.clone()).set_name(name.into())
    }

    /// Starts asynchronous cancellation on a long-running DlpJob. The server
    /// makes a best effort to cancel the DlpJob, but success is not
    /// guaranteed.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage>
    /// and
    /// <https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis>
    /// to learn more.
    pub fn cancel_dlp_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::CancelDlpJob {
        super::builder::dlp_service::CancelDlpJob::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a pre-built stored infoType to be used for inspection.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes>
    /// to learn more.
    pub fn create_stored_info_type(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::CreateStoredInfoType {
        super::builder::dlp_service::CreateStoredInfoType::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the stored infoType by creating a new version. The existing version
    /// will continue to be used until the new version is ready.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes>
    /// to learn more.
    pub fn update_stored_info_type(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::UpdateStoredInfoType {
        super::builder::dlp_service::UpdateStoredInfoType::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets a stored infoType.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes>
    /// to learn more.
    pub fn get_stored_info_type(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::GetStoredInfoType {
        super::builder::dlp_service::GetStoredInfoType::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists stored infoTypes.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes>
    /// to learn more.
    pub fn list_stored_info_types(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::ListStoredInfoTypes {
        super::builder::dlp_service::ListStoredInfoTypes::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a stored infoType.
    /// See
    /// <https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes>
    /// to learn more.
    pub fn delete_stored_info_type(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::DeleteStoredInfoType {
        super::builder::dlp_service::DeleteStoredInfoType::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists project data profiles for an organization.
    pub fn list_project_data_profiles(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::ListProjectDataProfiles {
        super::builder::dlp_service::ListProjectDataProfiles::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists table data profiles for an organization.
    pub fn list_table_data_profiles(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::ListTableDataProfiles {
        super::builder::dlp_service::ListTableDataProfiles::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists column data profiles for an organization.
    pub fn list_column_data_profiles(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::ListColumnDataProfiles {
        super::builder::dlp_service::ListColumnDataProfiles::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a project data profile.
    pub fn get_project_data_profile(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::GetProjectDataProfile {
        super::builder::dlp_service::GetProjectDataProfile::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists file store data profiles for an organization.
    pub fn list_file_store_data_profiles(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::ListFileStoreDataProfiles {
        super::builder::dlp_service::ListFileStoreDataProfiles::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a file store data profile.
    pub fn get_file_store_data_profile(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::GetFileStoreDataProfile {
        super::builder::dlp_service::GetFileStoreDataProfile::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Delete a FileStoreDataProfile. Will not prevent the profile from being
    /// regenerated if the resource is still included in a discovery configuration.
    pub fn delete_file_store_data_profile(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::DeleteFileStoreDataProfile {
        super::builder::dlp_service::DeleteFileStoreDataProfile::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets a table data profile.
    pub fn get_table_data_profile(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::GetTableDataProfile {
        super::builder::dlp_service::GetTableDataProfile::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets a column data profile.
    pub fn get_column_data_profile(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::GetColumnDataProfile {
        super::builder::dlp_service::GetColumnDataProfile::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Delete a TableDataProfile. Will not prevent the profile from being
    /// regenerated if the table is still included in a discovery configuration.
    pub fn delete_table_data_profile(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::DeleteTableDataProfile {
        super::builder::dlp_service::DeleteTableDataProfile::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Inspect hybrid content and store findings to a job.
    /// To review the findings, inspect the job. Inspection will occur
    /// asynchronously.
    pub fn hybrid_inspect_dlp_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::HybridInspectDlpJob {
        super::builder::dlp_service::HybridInspectDlpJob::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Finish a running hybrid DlpJob. Triggers the finalization steps and running
    /// of any enabled actions that have not yet run.
    pub fn finish_dlp_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::FinishDlpJob {
        super::builder::dlp_service::FinishDlpJob::new(self.inner.clone()).set_name(name.into())
    }

    /// Create a Connection to an external data source.
    pub fn create_connection(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::CreateConnection {
        super::builder::dlp_service::CreateConnection::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Get a Connection by name.
    pub fn get_connection(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::GetConnection {
        super::builder::dlp_service::GetConnection::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists Connections in a parent. Use SearchConnections to see all connections
    /// within an organization.
    pub fn list_connections(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::ListConnections {
        super::builder::dlp_service::ListConnections::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Searches for Connections in a parent.
    pub fn search_connections(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::SearchConnections {
        super::builder::dlp_service::SearchConnections::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Delete a Connection.
    pub fn delete_connection(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::DeleteConnection {
        super::builder::dlp_service::DeleteConnection::new(self.inner.clone()).set_name(name.into())
    }

    /// Update a Connection.
    pub fn update_connection(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dlp_service::UpdateConnection {
        super::builder::dlp_service::UpdateConnection::new(self.inner.clone()).set_name(name.into())
    }
}
