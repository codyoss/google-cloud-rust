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

/// Implements a [TranscoderService](super::stubs::TranscoderService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct TranscoderService<T>
where
    T: super::stubs::TranscoderService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> TranscoderService<T>
where
    T: super::stubs::TranscoderService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::TranscoderService for TranscoderService<T>
where
    T: super::stubs::TranscoderService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_job(
        &self,
        req: crate::model::CreateJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Job> {
        self.inner.create_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_jobs(
        &self,
        req: crate::model::ListJobsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListJobsResponse> {
        self.inner.list_jobs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_job(
        &self,
        req: crate::model::GetJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Job> {
        self.inner.get_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_job(
        &self,
        req: crate::model::DeleteJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_job_template(
        &self,
        req: crate::model::CreateJobTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::JobTemplate> {
        self.inner.create_job_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_job_templates(
        &self,
        req: crate::model::ListJobTemplatesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListJobTemplatesResponse> {
        self.inner.list_job_templates(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_job_template(
        &self,
        req: crate::model::GetJobTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::JobTemplate> {
        self.inner.get_job_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_job_template(
        &self,
        req: crate::model::DeleteJobTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_job_template(req, options).await
    }
}
