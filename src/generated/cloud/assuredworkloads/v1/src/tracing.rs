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

/// Implements a [AssuredWorkloadsService](super::stubs::AssuredWorkloadsService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct AssuredWorkloadsService<T>
where
    T: super::stubs::AssuredWorkloadsService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> AssuredWorkloadsService<T>
where
    T: super::stubs::AssuredWorkloadsService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::AssuredWorkloadsService for AssuredWorkloadsService<T>
where
    T: super::stubs::AssuredWorkloadsService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_workload(
        &self,
        req: crate::model::CreateWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_workload(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_workload(
        &self,
        req: crate::model::UpdateWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Workload> {
        self.inner.update_workload(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn restrict_allowed_resources(
        &self,
        req: crate::model::RestrictAllowedResourcesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::RestrictAllowedResourcesResponse> {
        self.inner.restrict_allowed_resources(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_workload(
        &self,
        req: crate::model::DeleteWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_workload(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_workload(
        &self,
        req: crate::model::GetWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Workload> {
        self.inner.get_workload(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_workloads(
        &self,
        req: crate::model::ListWorkloadsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListWorkloadsResponse> {
        self.inner.list_workloads(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
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
