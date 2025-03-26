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

/// Implements a [Builds](super::stubs::Builds) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Builds<T>
where
    T: super::stubs::Builds + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Builds<T>
where
    T: super::stubs::Builds + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::Builds for Builds<T>
where
    T: super::stubs::Builds + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn submit_build(
        &self,
        req: crate::model::SubmitBuildRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SubmitBuildResponse> {
        self.inner.submit_build(req, options).await
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

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn wait_operation(
        &self,
        req: longrunning::model::WaitOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.wait_operation(req, options).await
    }
}

/// Implements a [Executions](super::stubs::Executions) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Executions<T>
where
    T: super::stubs::Executions + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Executions<T>
where
    T: super::stubs::Executions + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::Executions for Executions<T>
where
    T: super::stubs::Executions + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn get_execution(
        &self,
        req: crate::model::GetExecutionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Execution> {
        self.inner.get_execution(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_executions(
        &self,
        req: crate::model::ListExecutionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListExecutionsResponse> {
        self.inner.list_executions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_execution(
        &self,
        req: crate::model::DeleteExecutionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_execution(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_execution(
        &self,
        req: crate::model::CancelExecutionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.cancel_execution(req, options).await
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

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn wait_operation(
        &self,
        req: longrunning::model::WaitOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.wait_operation(req, options).await
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

/// Implements a [Jobs](super::stubs::Jobs) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Jobs<T>
where
    T: super::stubs::Jobs + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Jobs<T>
where
    T: super::stubs::Jobs + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::Jobs for Jobs<T>
where
    T: super::stubs::Jobs + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_job(
        &self,
        req: crate::model::CreateJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_job(req, options).await
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
    async fn list_jobs(
        &self,
        req: crate::model::ListJobsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListJobsResponse> {
        self.inner.list_jobs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_job(
        &self,
        req: crate::model::UpdateJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_job(
        &self,
        req: crate::model::DeleteJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn run_job(
        &self,
        req: crate::model::RunJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.run_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
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

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn wait_operation(
        &self,
        req: longrunning::model::WaitOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.wait_operation(req, options).await
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

/// Implements a [Revisions](super::stubs::Revisions) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Revisions<T>
where
    T: super::stubs::Revisions + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Revisions<T>
where
    T: super::stubs::Revisions + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::Revisions for Revisions<T>
where
    T: super::stubs::Revisions + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn get_revision(
        &self,
        req: crate::model::GetRevisionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Revision> {
        self.inner.get_revision(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_revisions(
        &self,
        req: crate::model::ListRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListRevisionsResponse> {
        self.inner.list_revisions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_revision(
        &self,
        req: crate::model::DeleteRevisionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_revision(req, options).await
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

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn wait_operation(
        &self,
        req: longrunning::model::WaitOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.wait_operation(req, options).await
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

/// Implements a [Services](super::stubs::Services) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Services<T>
where
    T: super::stubs::Services + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Services<T>
where
    T: super::stubs::Services + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::Services for Services<T>
where
    T: super::stubs::Services + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_service(
        &self,
        req: crate::model::CreateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_service(
        &self,
        req: crate::model::GetServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Service> {
        self.inner.get_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_services(
        &self,
        req: crate::model::ListServicesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListServicesResponse> {
        self.inner.list_services(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_service(
        &self,
        req: crate::model::UpdateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_service(
        &self,
        req: crate::model::DeleteServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
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

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn wait_operation(
        &self,
        req: longrunning::model::WaitOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.wait_operation(req, options).await
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

/// Implements a [Tasks](super::stubs::Tasks) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Tasks<T>
where
    T: super::stubs::Tasks + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Tasks<T>
where
    T: super::stubs::Tasks + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::Tasks for Tasks<T>
where
    T: super::stubs::Tasks + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn get_task(
        &self,
        req: crate::model::GetTaskRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Task> {
        self.inner.get_task(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_tasks(
        &self,
        req: crate::model::ListTasksRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTasksResponse> {
        self.inner.list_tasks(req, options).await
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

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn wait_operation(
        &self,
        req: longrunning::model::WaitOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.wait_operation(req, options).await
    }
}
