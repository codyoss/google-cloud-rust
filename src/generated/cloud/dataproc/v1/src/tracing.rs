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

/// Implements a [AutoscalingPolicyService](super::stub::AutoscalingPolicyService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct AutoscalingPolicyService<T>
where
    T: super::stub::AutoscalingPolicyService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> AutoscalingPolicyService<T>
where
    T: super::stub::AutoscalingPolicyService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::AutoscalingPolicyService for AutoscalingPolicyService<T>
where
    T: super::stub::AutoscalingPolicyService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_autoscaling_policy(
        &self,
        req: crate::model::CreateAutoscalingPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AutoscalingPolicy>> {
        self.inner.create_autoscaling_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_autoscaling_policy(
        &self,
        req: crate::model::UpdateAutoscalingPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AutoscalingPolicy>> {
        self.inner.update_autoscaling_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_autoscaling_policy(
        &self,
        req: crate::model::GetAutoscalingPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AutoscalingPolicy>> {
        self.inner.get_autoscaling_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_autoscaling_policies(
        &self,
        req: crate::model::ListAutoscalingPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListAutoscalingPoliciesResponse>> {
        self.inner.list_autoscaling_policies(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_autoscaling_policy(
        &self,
        req: crate::model::DeleteAutoscalingPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_autoscaling_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>> {
        self.inner.test_iam_permissions(req, options).await
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
}

/// Implements a [BatchController](super::stub::BatchController) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct BatchController<T>
where
    T: super::stub::BatchController + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> BatchController<T>
where
    T: super::stub::BatchController + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::BatchController for BatchController<T>
where
    T: super::stub::BatchController + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_batch(
        &self,
        req: crate::model::CreateBatchRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_batch(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_batch(
        &self,
        req: crate::model::GetBatchRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Batch>> {
        self.inner.get_batch(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_batches(
        &self,
        req: crate::model::ListBatchesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListBatchesResponse>> {
        self.inner.list_batches(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_batch(
        &self,
        req: crate::model::DeleteBatchRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_batch(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>> {
        self.inner.test_iam_permissions(req, options).await
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

/// Implements a [ClusterController](super::stub::ClusterController) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct ClusterController<T>
where
    T: super::stub::ClusterController + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> ClusterController<T>
where
    T: super::stub::ClusterController + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::ClusterController for ClusterController<T>
where
    T: super::stub::ClusterController + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_cluster(
        &self,
        req: crate::model::CreateClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_cluster(
        &self,
        req: crate::model::UpdateClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn stop_cluster(
        &self,
        req: crate::model::StopClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.stop_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn start_cluster(
        &self,
        req: crate::model::StartClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.start_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_cluster(
        &self,
        req: crate::model::DeleteClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_cluster(
        &self,
        req: crate::model::GetClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Cluster>> {
        self.inner.get_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_clusters(
        &self,
        req: crate::model::ListClustersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListClustersResponse>> {
        self.inner.list_clusters(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn diagnose_cluster(
        &self,
        req: crate::model::DiagnoseClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.diagnose_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>> {
        self.inner.test_iam_permissions(req, options).await
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

/// Implements a [JobController](super::stub::JobController) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct JobController<T>
where
    T: super::stub::JobController + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> JobController<T>
where
    T: super::stub::JobController + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::JobController for JobController<T>
where
    T: super::stub::JobController + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn submit_job(
        &self,
        req: crate::model::SubmitJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Job>> {
        self.inner.submit_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn submit_job_as_operation(
        &self,
        req: crate::model::SubmitJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.submit_job_as_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_job(
        &self,
        req: crate::model::GetJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Job>> {
        self.inner.get_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_jobs(
        &self,
        req: crate::model::ListJobsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListJobsResponse>> {
        self.inner.list_jobs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_job(
        &self,
        req: crate::model::UpdateJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Job>> {
        self.inner.update_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_job(
        &self,
        req: crate::model::CancelJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Job>> {
        self.inner.cancel_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_job(
        &self,
        req: crate::model::DeleteJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>> {
        self.inner.test_iam_permissions(req, options).await
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

/// Implements a [NodeGroupController](super::stub::NodeGroupController) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct NodeGroupController<T>
where
    T: super::stub::NodeGroupController + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> NodeGroupController<T>
where
    T: super::stub::NodeGroupController + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::NodeGroupController for NodeGroupController<T>
where
    T: super::stub::NodeGroupController + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_node_group(
        &self,
        req: crate::model::CreateNodeGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_node_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn resize_node_group(
        &self,
        req: crate::model::ResizeNodeGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.resize_node_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_node_group(
        &self,
        req: crate::model::GetNodeGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::NodeGroup>> {
        self.inner.get_node_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>> {
        self.inner.test_iam_permissions(req, options).await
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

/// Implements a [SessionTemplateController](super::stub::SessionTemplateController) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct SessionTemplateController<T>
where
    T: super::stub::SessionTemplateController + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> SessionTemplateController<T>
where
    T: super::stub::SessionTemplateController + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::SessionTemplateController for SessionTemplateController<T>
where
    T: super::stub::SessionTemplateController + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_session_template(
        &self,
        req: crate::model::CreateSessionTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SessionTemplate>> {
        self.inner.create_session_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_session_template(
        &self,
        req: crate::model::UpdateSessionTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SessionTemplate>> {
        self.inner.update_session_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_session_template(
        &self,
        req: crate::model::GetSessionTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SessionTemplate>> {
        self.inner.get_session_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_session_templates(
        &self,
        req: crate::model::ListSessionTemplatesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListSessionTemplatesResponse>> {
        self.inner.list_session_templates(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_session_template(
        &self,
        req: crate::model::DeleteSessionTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_session_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>> {
        self.inner.test_iam_permissions(req, options).await
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
}

/// Implements a [SessionController](super::stub::SessionController) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct SessionController<T>
where
    T: super::stub::SessionController + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> SessionController<T>
where
    T: super::stub::SessionController + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::SessionController for SessionController<T>
where
    T: super::stub::SessionController + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_session(
        &self,
        req: crate::model::CreateSessionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_session(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_session(
        &self,
        req: crate::model::GetSessionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Session>> {
        self.inner.get_session(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_sessions(
        &self,
        req: crate::model::ListSessionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListSessionsResponse>> {
        self.inner.list_sessions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn terminate_session(
        &self,
        req: crate::model::TerminateSessionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.terminate_session(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_session(
        &self,
        req: crate::model::DeleteSessionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_session(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>> {
        self.inner.test_iam_permissions(req, options).await
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

/// Implements a [WorkflowTemplateService](super::stub::WorkflowTemplateService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct WorkflowTemplateService<T>
where
    T: super::stub::WorkflowTemplateService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> WorkflowTemplateService<T>
where
    T: super::stub::WorkflowTemplateService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::WorkflowTemplateService for WorkflowTemplateService<T>
where
    T: super::stub::WorkflowTemplateService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_workflow_template(
        &self,
        req: crate::model::CreateWorkflowTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::WorkflowTemplate>> {
        self.inner.create_workflow_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_workflow_template(
        &self,
        req: crate::model::GetWorkflowTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::WorkflowTemplate>> {
        self.inner.get_workflow_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn instantiate_workflow_template(
        &self,
        req: crate::model::InstantiateWorkflowTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.instantiate_workflow_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn instantiate_inline_workflow_template(
        &self,
        req: crate::model::InstantiateInlineWorkflowTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner
            .instantiate_inline_workflow_template(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn update_workflow_template(
        &self,
        req: crate::model::UpdateWorkflowTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::WorkflowTemplate>> {
        self.inner.update_workflow_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_workflow_templates(
        &self,
        req: crate::model::ListWorkflowTemplatesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListWorkflowTemplatesResponse>> {
        self.inner.list_workflow_templates(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_workflow_template(
        &self,
        req: crate::model::DeleteWorkflowTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_workflow_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>> {
        self.inner.test_iam_permissions(req, options).await
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
