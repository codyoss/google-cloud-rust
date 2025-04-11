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

/// A dyn-compatible, crate-private version of [super::DocumentProcessorService].
#[async_trait::async_trait]
pub trait DocumentProcessorService: std::fmt::Debug + Send + Sync {
    async fn process_document(
        &self,
        req: crate::model::ProcessRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ProcessResponse>>;

    async fn batch_process_documents(
        &self,
        req: crate::model::BatchProcessRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn fetch_processor_types(
        &self,
        req: crate::model::FetchProcessorTypesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::FetchProcessorTypesResponse>>;

    async fn list_processor_types(
        &self,
        req: crate::model::ListProcessorTypesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListProcessorTypesResponse>>;

    async fn get_processor_type(
        &self,
        req: crate::model::GetProcessorTypeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ProcessorType>>;

    async fn list_processors(
        &self,
        req: crate::model::ListProcessorsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListProcessorsResponse>>;

    async fn get_processor(
        &self,
        req: crate::model::GetProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Processor>>;

    async fn train_processor_version(
        &self,
        req: crate::model::TrainProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn get_processor_version(
        &self,
        req: crate::model::GetProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ProcessorVersion>>;

    async fn list_processor_versions(
        &self,
        req: crate::model::ListProcessorVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListProcessorVersionsResponse>>;

    async fn delete_processor_version(
        &self,
        req: crate::model::DeleteProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn deploy_processor_version(
        &self,
        req: crate::model::DeployProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn undeploy_processor_version(
        &self,
        req: crate::model::UndeployProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn create_processor(
        &self,
        req: crate::model::CreateProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Processor>>;

    async fn delete_processor(
        &self,
        req: crate::model::DeleteProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn enable_processor(
        &self,
        req: crate::model::EnableProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn disable_processor(
        &self,
        req: crate::model::DisableProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn set_default_processor_version(
        &self,
        req: crate::model::SetDefaultProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn review_document(
        &self,
        req: crate::model::ReviewDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn evaluate_processor_version(
        &self,
        req: crate::model::EvaluateProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn get_evaluation(
        &self,
        req: crate::model::GetEvaluationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Evaluation>>;

    async fn list_evaluations(
        &self,
        req: crate::model::ListEvaluationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListEvaluationsResponse>>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::ListLocationsResponse>>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::Location>>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::DocumentProcessorService] also implement [DocumentProcessorService].
#[async_trait::async_trait]
impl<T: super::DocumentProcessorService> DocumentProcessorService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn process_document(
        &self,
        req: crate::model::ProcessRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ProcessResponse>> {
        T::process_document(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn batch_process_documents(
        &self,
        req: crate::model::BatchProcessRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::batch_process_documents(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn fetch_processor_types(
        &self,
        req: crate::model::FetchProcessorTypesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::FetchProcessorTypesResponse>> {
        T::fetch_processor_types(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_processor_types(
        &self,
        req: crate::model::ListProcessorTypesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListProcessorTypesResponse>> {
        T::list_processor_types(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_processor_type(
        &self,
        req: crate::model::GetProcessorTypeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ProcessorType>> {
        T::get_processor_type(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_processors(
        &self,
        req: crate::model::ListProcessorsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListProcessorsResponse>> {
        T::list_processors(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_processor(
        &self,
        req: crate::model::GetProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Processor>> {
        T::get_processor(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn train_processor_version(
        &self,
        req: crate::model::TrainProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::train_processor_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_processor_version(
        &self,
        req: crate::model::GetProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ProcessorVersion>> {
        T::get_processor_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_processor_versions(
        &self,
        req: crate::model::ListProcessorVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListProcessorVersionsResponse>> {
        T::list_processor_versions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_processor_version(
        &self,
        req: crate::model::DeleteProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_processor_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn deploy_processor_version(
        &self,
        req: crate::model::DeployProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::deploy_processor_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn undeploy_processor_version(
        &self,
        req: crate::model::UndeployProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::undeploy_processor_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_processor(
        &self,
        req: crate::model::CreateProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Processor>> {
        T::create_processor(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_processor(
        &self,
        req: crate::model::DeleteProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_processor(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn enable_processor(
        &self,
        req: crate::model::EnableProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::enable_processor(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn disable_processor(
        &self,
        req: crate::model::DisableProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::disable_processor(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_default_processor_version(
        &self,
        req: crate::model::SetDefaultProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::set_default_processor_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn review_document(
        &self,
        req: crate::model::ReviewDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::review_document(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn evaluate_processor_version(
        &self,
        req: crate::model::EvaluateProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::evaluate_processor_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_evaluation(
        &self,
        req: crate::model::GetEvaluationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Evaluation>> {
        T::get_evaluation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_evaluations(
        &self,
        req: crate::model::ListEvaluationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListEvaluationsResponse>> {
        T::list_evaluations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::ListLocationsResponse>> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::Location>> {
        T::get_location(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
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
