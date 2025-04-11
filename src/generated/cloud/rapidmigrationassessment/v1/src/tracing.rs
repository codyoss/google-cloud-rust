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

/// Implements a [RapidMigrationAssessment](super::stub::RapidMigrationAssessment) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct RapidMigrationAssessment<T>
where
    T: super::stub::RapidMigrationAssessment + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> RapidMigrationAssessment<T>
where
    T: super::stub::RapidMigrationAssessment + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::RapidMigrationAssessment for RapidMigrationAssessment<T>
where
    T: super::stub::RapidMigrationAssessment + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_collector(
        &self,
        req: crate::model::CreateCollectorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_collector(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_annotation(
        &self,
        req: crate::model::CreateAnnotationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_annotation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_annotation(
        &self,
        req: crate::model::GetAnnotationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Annotation>> {
        self.inner.get_annotation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_collectors(
        &self,
        req: crate::model::ListCollectorsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListCollectorsResponse>> {
        self.inner.list_collectors(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_collector(
        &self,
        req: crate::model::GetCollectorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Collector>> {
        self.inner.get_collector(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_collector(
        &self,
        req: crate::model::UpdateCollectorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_collector(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_collector(
        &self,
        req: crate::model::DeleteCollectorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_collector(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn resume_collector(
        &self,
        req: crate::model::ResumeCollectorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.resume_collector(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn register_collector(
        &self,
        req: crate::model::RegisterCollectorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.register_collector(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn pause_collector(
        &self,
        req: crate::model::PauseCollectorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.pause_collector(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::ListLocationsResponse>> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::Location>> {
        self.inner.get_location(req, options).await
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
