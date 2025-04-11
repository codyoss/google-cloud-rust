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

/// Implements a [WebRiskService](super::stub::WebRiskService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct WebRiskService<T>
where
    T: super::stub::WebRiskService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> WebRiskService<T>
where
    T: super::stub::WebRiskService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::WebRiskService for WebRiskService<T>
where
    T: super::stub::WebRiskService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn compute_threat_list_diff(
        &self,
        req: crate::model::ComputeThreatListDiffRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ComputeThreatListDiffResponse>> {
        self.inner.compute_threat_list_diff(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn search_uris(
        &self,
        req: crate::model::SearchUrisRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SearchUrisResponse>> {
        self.inner.search_uris(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn search_hashes(
        &self,
        req: crate::model::SearchHashesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SearchHashesResponse>> {
        self.inner.search_hashes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_submission(
        &self,
        req: crate::model::CreateSubmissionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Submission>> {
        self.inner.create_submission(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn submit_uri(
        &self,
        req: crate::model::SubmitUriRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.submit_uri(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.cancel_operation(req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        self.inner.get_polling_error_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
