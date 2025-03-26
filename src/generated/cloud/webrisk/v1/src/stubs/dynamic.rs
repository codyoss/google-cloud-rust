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

/// A dyn-compatible, crate-private version of [super::WebRiskService].
#[async_trait::async_trait]
pub trait WebRiskService: std::fmt::Debug + Send + Sync {
    async fn compute_threat_list_diff(
        &self,
        req: crate::model::ComputeThreatListDiffRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ComputeThreatListDiffResponse>;

    async fn search_uris(
        &self,
        req: crate::model::SearchUrisRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchUrisResponse>;

    async fn search_hashes(
        &self,
        req: crate::model::SearchHashesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchHashesResponse>;

    async fn create_submission(
        &self,
        req: crate::model::CreateSubmissionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Submission>;

    async fn submit_uri(
        &self,
        req: crate::model::SubmitUriRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

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

/// All implementations of [super::WebRiskService] also implement [WebRiskService].
#[async_trait::async_trait]
impl<T: super::WebRiskService> WebRiskService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn compute_threat_list_diff(
        &self,
        req: crate::model::ComputeThreatListDiffRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ComputeThreatListDiffResponse> {
        T::compute_threat_list_diff(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn search_uris(
        &self,
        req: crate::model::SearchUrisRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchUrisResponse> {
        T::search_uris(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn search_hashes(
        &self,
        req: crate::model::SearchHashesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchHashesResponse> {
        T::search_hashes(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_submission(
        &self,
        req: crate::model::CreateSubmissionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Submission> {
        T::create_submission(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn submit_uri(
        &self,
        req: crate::model::SubmitUriRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::submit_uri(self, req, options).await
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
