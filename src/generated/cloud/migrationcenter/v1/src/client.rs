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

/// Implements a client for the Migration Center API.
///
/// # Service Description
///
/// Service describing handlers for resources.
///
/// # Configuration
///
/// `MigrationCenter` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `MigrationCenter` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `MigrationCenter` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct MigrationCenter {
    inner: Arc<dyn crate::stubs::dynamic::MigrationCenter>,
}

impl MigrationCenter {
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
        T: crate::stubs::MigrationCenter + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::MigrationCenter>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::MigrationCenter> {
        crate::transport::MigrationCenter::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::MigrationCenter> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::MigrationCenter::new)
    }

    /// Lists all the assets in a given project and location.
    pub fn list_assets(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::ListAssets {
        crate::builders::migration_center::ListAssets::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets the details of an asset.
    pub fn get_asset(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::GetAsset {
        crate::builders::migration_center::GetAsset::new(self.inner.clone()).set_name(name.into())
    }

    /// Updates the parameters of an asset.
    pub fn update_asset(
        &self,
        asset: impl Into<crate::model::Asset>,
    ) -> crate::builders::migration_center::UpdateAsset {
        crate::builders::migration_center::UpdateAsset::new(self.inner.clone())
            .set_asset(asset.into())
    }

    /// Updates the parameters of a list of assets.
    pub fn batch_update_assets(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::BatchUpdateAssets {
        crate::builders::migration_center::BatchUpdateAssets::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes an asset.
    pub fn delete_asset(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::DeleteAsset {
        crate::builders::migration_center::DeleteAsset::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes list of Assets.
    pub fn batch_delete_assets(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::BatchDeleteAssets {
        crate::builders::migration_center::BatchDeleteAssets::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Reports a set of frames.
    pub fn report_asset_frames(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::ReportAssetFrames {
        crate::builders::migration_center::ReportAssetFrames::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Aggregates the requested fields based on provided function.
    pub fn aggregate_assets_values(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::AggregateAssetsValues {
        crate::builders::migration_center::AggregateAssetsValues::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates an import job.
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
    pub fn create_import_job(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::CreateImportJob {
        crate::builders::migration_center::CreateImportJob::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists all import jobs.
    pub fn list_import_jobs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::ListImportJobs {
        crate::builders::migration_center::ListImportJobs::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets the details of an import job.
    pub fn get_import_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::GetImportJob {
        crate::builders::migration_center::GetImportJob::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes an import job.
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
    pub fn delete_import_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::DeleteImportJob {
        crate::builders::migration_center::DeleteImportJob::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates an import job.
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
    pub fn update_import_job(
        &self,
        import_job: impl Into<crate::model::ImportJob>,
    ) -> crate::builders::migration_center::UpdateImportJob {
        crate::builders::migration_center::UpdateImportJob::new(self.inner.clone())
            .set_import_job(import_job.into())
    }

    /// Validates an import job.
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
    pub fn validate_import_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::ValidateImportJob {
        crate::builders::migration_center::ValidateImportJob::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Runs an import job.
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
    pub fn run_import_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::RunImportJob {
        crate::builders::migration_center::RunImportJob::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets an import data file.
    pub fn get_import_data_file(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::GetImportDataFile {
        crate::builders::migration_center::GetImportDataFile::new(self.inner.clone())
            .set_name(name.into())
    }

    /// List import data files.
    pub fn list_import_data_files(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::ListImportDataFiles {
        crate::builders::migration_center::ListImportDataFiles::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates an import data file.
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
    pub fn create_import_data_file(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::CreateImportDataFile {
        crate::builders::migration_center::CreateImportDataFile::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Delete an import data file.
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
    pub fn delete_import_data_file(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::DeleteImportDataFile {
        crate::builders::migration_center::DeleteImportDataFile::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists all groups in a given project and location.
    pub fn list_groups(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::ListGroups {
        crate::builders::migration_center::ListGroups::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets the details of a group.
    pub fn get_group(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::GetGroup {
        crate::builders::migration_center::GetGroup::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a new group in a given project and location.
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
    pub fn create_group(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::CreateGroup {
        crate::builders::migration_center::CreateGroup::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a group.
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
    pub fn update_group(
        &self,
        group: impl Into<crate::model::Group>,
    ) -> crate::builders::migration_center::UpdateGroup {
        crate::builders::migration_center::UpdateGroup::new(self.inner.clone())
            .set_group(group.into())
    }

    /// Deletes a group.
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
    pub fn delete_group(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::DeleteGroup {
        crate::builders::migration_center::DeleteGroup::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Adds assets to a group.
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
    pub fn add_assets_to_group(
        &self,
        group: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::AddAssetsToGroup {
        crate::builders::migration_center::AddAssetsToGroup::new(self.inner.clone())
            .set_group(group.into())
    }

    /// Removes assets from a group.
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
    pub fn remove_assets_from_group(
        &self,
        group: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::RemoveAssetsFromGroup {
        crate::builders::migration_center::RemoveAssetsFromGroup::new(self.inner.clone())
            .set_group(group.into())
    }

    /// Lists all error frames in a given source and location.
    pub fn list_error_frames(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::ListErrorFrames {
        crate::builders::migration_center::ListErrorFrames::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets the details of an error frame.
    pub fn get_error_frame(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::GetErrorFrame {
        crate::builders::migration_center::GetErrorFrame::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists all the sources in a given project and location.
    pub fn list_sources(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::ListSources {
        crate::builders::migration_center::ListSources::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets the details of a source.
    pub fn get_source(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::GetSource {
        crate::builders::migration_center::GetSource::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a new source in a given project and location.
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
    pub fn create_source(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::CreateSource {
        crate::builders::migration_center::CreateSource::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a source.
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
    pub fn update_source(
        &self,
        source: impl Into<crate::model::Source>,
    ) -> crate::builders::migration_center::UpdateSource {
        crate::builders::migration_center::UpdateSource::new(self.inner.clone())
            .set_source(source.into())
    }

    /// Deletes a source.
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
    pub fn delete_source(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::DeleteSource {
        crate::builders::migration_center::DeleteSource::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists all the preference sets in a given project and location.
    pub fn list_preference_sets(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::ListPreferenceSets {
        crate::builders::migration_center::ListPreferenceSets::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets the details of a preference set.
    pub fn get_preference_set(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::GetPreferenceSet {
        crate::builders::migration_center::GetPreferenceSet::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new preference set in a given project and location.
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
    pub fn create_preference_set(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::CreatePreferenceSet {
        crate::builders::migration_center::CreatePreferenceSet::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a preference set.
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
    pub fn update_preference_set(
        &self,
        preference_set: impl Into<crate::model::PreferenceSet>,
    ) -> crate::builders::migration_center::UpdatePreferenceSet {
        crate::builders::migration_center::UpdatePreferenceSet::new(self.inner.clone())
            .set_preference_set(preference_set.into())
    }

    /// Deletes a preference set.
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
    pub fn delete_preference_set(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::DeletePreferenceSet {
        crate::builders::migration_center::DeletePreferenceSet::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets the details of regional settings.
    pub fn get_settings(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::GetSettings {
        crate::builders::migration_center::GetSettings::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates the regional-level project settings.
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
    pub fn update_settings(
        &self,
        settings: impl Into<crate::model::Settings>,
    ) -> crate::builders::migration_center::UpdateSettings {
        crate::builders::migration_center::UpdateSettings::new(self.inner.clone())
            .set_settings(settings.into())
    }

    /// Creates a report configuration.
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
    pub fn create_report_config(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::CreateReportConfig {
        crate::builders::migration_center::CreateReportConfig::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single ReportConfig.
    pub fn get_report_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::GetReportConfig {
        crate::builders::migration_center::GetReportConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists ReportConfigs in a given project and location.
    pub fn list_report_configs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::ListReportConfigs {
        crate::builders::migration_center::ListReportConfigs::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a ReportConfig.
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
    pub fn delete_report_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::DeleteReportConfig {
        crate::builders::migration_center::DeleteReportConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a report.
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
    pub fn create_report(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::CreateReport {
        crate::builders::migration_center::CreateReport::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single Report.
    pub fn get_report(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::GetReport {
        crate::builders::migration_center::GetReport::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists Reports in a given ReportConfig.
    pub fn list_reports(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::ListReports {
        crate::builders::migration_center::ListReports::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a Report.
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
    pub fn delete_report(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::DeleteReport {
        crate::builders::migration_center::DeleteReport::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::ListLocations {
        crate::builders::migration_center::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::GetLocation {
        crate::builders::migration_center::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::ListOperations {
        crate::builders::migration_center::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::GetOperation {
        crate::builders::migration_center::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::DeleteOperation {
        crate::builders::migration_center::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_center::CancelOperation {
        crate::builders::migration_center::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
