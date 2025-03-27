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

/// A dyn-compatible, crate-private version of [super::TranslationService].
#[async_trait::async_trait]
pub trait TranslationService: std::fmt::Debug + Send + Sync {
    async fn translate_text(
        &self,
        req: crate::model::TranslateTextRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TranslateTextResponse>;

    async fn romanize_text(
        &self,
        req: crate::model::RomanizeTextRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::RomanizeTextResponse>;

    async fn detect_language(
        &self,
        req: crate::model::DetectLanguageRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DetectLanguageResponse>;

    async fn get_supported_languages(
        &self,
        req: crate::model::GetSupportedLanguagesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SupportedLanguages>;

    async fn translate_document(
        &self,
        req: crate::model::TranslateDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TranslateDocumentResponse>;

    async fn batch_translate_text(
        &self,
        req: crate::model::BatchTranslateTextRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn batch_translate_document(
        &self,
        req: crate::model::BatchTranslateDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_glossary(
        &self,
        req: crate::model::CreateGlossaryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_glossary(
        &self,
        req: crate::model::UpdateGlossaryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_glossaries(
        &self,
        req: crate::model::ListGlossariesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListGlossariesResponse>;

    async fn get_glossary(
        &self,
        req: crate::model::GetGlossaryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Glossary>;

    async fn delete_glossary(
        &self,
        req: crate::model::DeleteGlossaryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_glossary_entry(
        &self,
        req: crate::model::GetGlossaryEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GlossaryEntry>;

    async fn list_glossary_entries(
        &self,
        req: crate::model::ListGlossaryEntriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListGlossaryEntriesResponse>;

    async fn create_glossary_entry(
        &self,
        req: crate::model::CreateGlossaryEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GlossaryEntry>;

    async fn update_glossary_entry(
        &self,
        req: crate::model::UpdateGlossaryEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GlossaryEntry>;

    async fn delete_glossary_entry(
        &self,
        req: crate::model::DeleteGlossaryEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    async fn create_dataset(
        &self,
        req: crate::model::CreateDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_dataset(
        &self,
        req: crate::model::GetDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Dataset>;

    async fn list_datasets(
        &self,
        req: crate::model::ListDatasetsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDatasetsResponse>;

    async fn delete_dataset(
        &self,
        req: crate::model::DeleteDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_adaptive_mt_dataset(
        &self,
        req: crate::model::CreateAdaptiveMtDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AdaptiveMtDataset>;

    async fn delete_adaptive_mt_dataset(
        &self,
        req: crate::model::DeleteAdaptiveMtDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    async fn get_adaptive_mt_dataset(
        &self,
        req: crate::model::GetAdaptiveMtDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AdaptiveMtDataset>;

    async fn list_adaptive_mt_datasets(
        &self,
        req: crate::model::ListAdaptiveMtDatasetsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListAdaptiveMtDatasetsResponse>;

    async fn adaptive_mt_translate(
        &self,
        req: crate::model::AdaptiveMtTranslateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AdaptiveMtTranslateResponse>;

    async fn get_adaptive_mt_file(
        &self,
        req: crate::model::GetAdaptiveMtFileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AdaptiveMtFile>;

    async fn delete_adaptive_mt_file(
        &self,
        req: crate::model::DeleteAdaptiveMtFileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    async fn import_adaptive_mt_file(
        &self,
        req: crate::model::ImportAdaptiveMtFileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ImportAdaptiveMtFileResponse>;

    async fn list_adaptive_mt_files(
        &self,
        req: crate::model::ListAdaptiveMtFilesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListAdaptiveMtFilesResponse>;

    async fn list_adaptive_mt_sentences(
        &self,
        req: crate::model::ListAdaptiveMtSentencesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListAdaptiveMtSentencesResponse>;

    async fn import_data(
        &self,
        req: crate::model::ImportDataRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn export_data(
        &self,
        req: crate::model::ExportDataRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_examples(
        &self,
        req: crate::model::ListExamplesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListExamplesResponse>;

    async fn create_model(
        &self,
        req: crate::model::CreateModelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_models(
        &self,
        req: crate::model::ListModelsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListModelsResponse>;

    async fn get_model(
        &self,
        req: crate::model::GetModelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Model>;

    async fn delete_model(
        &self,
        req: crate::model::DeleteModelRequest,
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
    ) -> crate::Result<()>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    async fn wait_operation(
        &self,
        req: longrunning::model::WaitOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::TranslationService] also implement [TranslationService].
#[async_trait::async_trait]
impl<T: super::TranslationService> TranslationService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn translate_text(
        &self,
        req: crate::model::TranslateTextRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TranslateTextResponse> {
        T::translate_text(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn romanize_text(
        &self,
        req: crate::model::RomanizeTextRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::RomanizeTextResponse> {
        T::romanize_text(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn detect_language(
        &self,
        req: crate::model::DetectLanguageRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DetectLanguageResponse> {
        T::detect_language(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_supported_languages(
        &self,
        req: crate::model::GetSupportedLanguagesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SupportedLanguages> {
        T::get_supported_languages(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn translate_document(
        &self,
        req: crate::model::TranslateDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TranslateDocumentResponse> {
        T::translate_document(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn batch_translate_text(
        &self,
        req: crate::model::BatchTranslateTextRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::batch_translate_text(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn batch_translate_document(
        &self,
        req: crate::model::BatchTranslateDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::batch_translate_document(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_glossary(
        &self,
        req: crate::model::CreateGlossaryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_glossary(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_glossary(
        &self,
        req: crate::model::UpdateGlossaryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_glossary(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_glossaries(
        &self,
        req: crate::model::ListGlossariesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListGlossariesResponse> {
        T::list_glossaries(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_glossary(
        &self,
        req: crate::model::GetGlossaryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Glossary> {
        T::get_glossary(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_glossary(
        &self,
        req: crate::model::DeleteGlossaryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_glossary(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_glossary_entry(
        &self,
        req: crate::model::GetGlossaryEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GlossaryEntry> {
        T::get_glossary_entry(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_glossary_entries(
        &self,
        req: crate::model::ListGlossaryEntriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListGlossaryEntriesResponse> {
        T::list_glossary_entries(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_glossary_entry(
        &self,
        req: crate::model::CreateGlossaryEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GlossaryEntry> {
        T::create_glossary_entry(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_glossary_entry(
        &self,
        req: crate::model::UpdateGlossaryEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GlossaryEntry> {
        T::update_glossary_entry(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_glossary_entry(
        &self,
        req: crate::model::DeleteGlossaryEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::delete_glossary_entry(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_dataset(
        &self,
        req: crate::model::CreateDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_dataset(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_dataset(
        &self,
        req: crate::model::GetDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Dataset> {
        T::get_dataset(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_datasets(
        &self,
        req: crate::model::ListDatasetsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDatasetsResponse> {
        T::list_datasets(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_dataset(
        &self,
        req: crate::model::DeleteDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_dataset(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_adaptive_mt_dataset(
        &self,
        req: crate::model::CreateAdaptiveMtDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AdaptiveMtDataset> {
        T::create_adaptive_mt_dataset(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_adaptive_mt_dataset(
        &self,
        req: crate::model::DeleteAdaptiveMtDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::delete_adaptive_mt_dataset(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_adaptive_mt_dataset(
        &self,
        req: crate::model::GetAdaptiveMtDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AdaptiveMtDataset> {
        T::get_adaptive_mt_dataset(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_adaptive_mt_datasets(
        &self,
        req: crate::model::ListAdaptiveMtDatasetsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListAdaptiveMtDatasetsResponse> {
        T::list_adaptive_mt_datasets(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn adaptive_mt_translate(
        &self,
        req: crate::model::AdaptiveMtTranslateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AdaptiveMtTranslateResponse> {
        T::adaptive_mt_translate(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_adaptive_mt_file(
        &self,
        req: crate::model::GetAdaptiveMtFileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AdaptiveMtFile> {
        T::get_adaptive_mt_file(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_adaptive_mt_file(
        &self,
        req: crate::model::DeleteAdaptiveMtFileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::delete_adaptive_mt_file(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn import_adaptive_mt_file(
        &self,
        req: crate::model::ImportAdaptiveMtFileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ImportAdaptiveMtFileResponse> {
        T::import_adaptive_mt_file(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_adaptive_mt_files(
        &self,
        req: crate::model::ListAdaptiveMtFilesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListAdaptiveMtFilesResponse> {
        T::list_adaptive_mt_files(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_adaptive_mt_sentences(
        &self,
        req: crate::model::ListAdaptiveMtSentencesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListAdaptiveMtSentencesResponse> {
        T::list_adaptive_mt_sentences(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn import_data(
        &self,
        req: crate::model::ImportDataRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::import_data(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn export_data(
        &self,
        req: crate::model::ExportDataRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::export_data(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_examples(
        &self,
        req: crate::model::ListExamplesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListExamplesResponse> {
        T::list_examples(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_model(
        &self,
        req: crate::model::CreateModelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_model(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_models(
        &self,
        req: crate::model::ListModelsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListModelsResponse> {
        T::list_models(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_model(
        &self,
        req: crate::model::GetModelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Model> {
        T::get_model(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_model(
        &self,
        req: crate::model::DeleteModelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_model(self, req, options).await
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
    ) -> crate::Result<()> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::cancel_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn wait_operation(
        &self,
        req: longrunning::model::WaitOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::wait_operation(self, req, options).await
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
