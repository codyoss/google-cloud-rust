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

/// A dyn-compatible, crate-private version of [super::Speech].
#[async_trait::async_trait]
pub trait Speech: std::fmt::Debug + Send + Sync {
    async fn create_recognizer(
        &self,
        req: crate::model::CreateRecognizerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_recognizers(
        &self,
        req: crate::model::ListRecognizersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRecognizersResponse>;

    async fn get_recognizer(
        &self,
        req: crate::model::GetRecognizerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Recognizer>;

    async fn update_recognizer(
        &self,
        req: crate::model::UpdateRecognizerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_recognizer(
        &self,
        req: crate::model::DeleteRecognizerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn undelete_recognizer(
        &self,
        req: crate::model::UndeleteRecognizerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn recognize(
        &self,
        req: crate::model::RecognizeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::RecognizeResponse>;

    async fn batch_recognize(
        &self,
        req: crate::model::BatchRecognizeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_config(
        &self,
        req: crate::model::GetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Config>;

    async fn update_config(
        &self,
        req: crate::model::UpdateConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Config>;

    async fn create_custom_class(
        &self,
        req: crate::model::CreateCustomClassRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_custom_classes(
        &self,
        req: crate::model::ListCustomClassesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListCustomClassesResponse>;

    async fn get_custom_class(
        &self,
        req: crate::model::GetCustomClassRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CustomClass>;

    async fn update_custom_class(
        &self,
        req: crate::model::UpdateCustomClassRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_custom_class(
        &self,
        req: crate::model::DeleteCustomClassRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn undelete_custom_class(
        &self,
        req: crate::model::UndeleteCustomClassRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_phrase_set(
        &self,
        req: crate::model::CreatePhraseSetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_phrase_sets(
        &self,
        req: crate::model::ListPhraseSetsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPhraseSetsResponse>;

    async fn get_phrase_set(
        &self,
        req: crate::model::GetPhraseSetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PhraseSet>;

    async fn update_phrase_set(
        &self,
        req: crate::model::UpdatePhraseSetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_phrase_set(
        &self,
        req: crate::model::DeletePhraseSetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn undelete_phrase_set(
        &self,
        req: crate::model::UndeletePhraseSetRequest,
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

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::Speech] also implement [Speech].
#[async_trait::async_trait]
impl<T: super::Speech> Speech for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_recognizer(
        &self,
        req: crate::model::CreateRecognizerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_recognizer(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_recognizers(
        &self,
        req: crate::model::ListRecognizersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRecognizersResponse> {
        T::list_recognizers(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_recognizer(
        &self,
        req: crate::model::GetRecognizerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Recognizer> {
        T::get_recognizer(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_recognizer(
        &self,
        req: crate::model::UpdateRecognizerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_recognizer(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_recognizer(
        &self,
        req: crate::model::DeleteRecognizerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_recognizer(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn undelete_recognizer(
        &self,
        req: crate::model::UndeleteRecognizerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::undelete_recognizer(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn recognize(
        &self,
        req: crate::model::RecognizeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::RecognizeResponse> {
        T::recognize(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn batch_recognize(
        &self,
        req: crate::model::BatchRecognizeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::batch_recognize(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_config(
        &self,
        req: crate::model::GetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Config> {
        T::get_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_config(
        &self,
        req: crate::model::UpdateConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Config> {
        T::update_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_custom_class(
        &self,
        req: crate::model::CreateCustomClassRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_custom_class(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_custom_classes(
        &self,
        req: crate::model::ListCustomClassesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListCustomClassesResponse> {
        T::list_custom_classes(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_custom_class(
        &self,
        req: crate::model::GetCustomClassRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CustomClass> {
        T::get_custom_class(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_custom_class(
        &self,
        req: crate::model::UpdateCustomClassRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_custom_class(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_custom_class(
        &self,
        req: crate::model::DeleteCustomClassRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_custom_class(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn undelete_custom_class(
        &self,
        req: crate::model::UndeleteCustomClassRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::undelete_custom_class(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_phrase_set(
        &self,
        req: crate::model::CreatePhraseSetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_phrase_set(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_phrase_sets(
        &self,
        req: crate::model::ListPhraseSetsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPhraseSetsResponse> {
        T::list_phrase_sets(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_phrase_set(
        &self,
        req: crate::model::GetPhraseSetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PhraseSet> {
        T::get_phrase_set(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_phrase_set(
        &self,
        req: crate::model::UpdatePhraseSetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_phrase_set(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_phrase_set(
        &self,
        req: crate::model::DeletePhraseSetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_phrase_set(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn undelete_phrase_set(
        &self,
        req: crate::model::UndeletePhraseSetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::undelete_phrase_set(self, req, options).await
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
