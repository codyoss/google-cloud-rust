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

/// Implements a [CloudDeploy](super::stubs::CloudDeploy) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct CloudDeploy<T>
where
    T: super::stubs::CloudDeploy + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> CloudDeploy<T>
where
    T: super::stubs::CloudDeploy + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::CloudDeploy for CloudDeploy<T>
where
    T: super::stubs::CloudDeploy + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_delivery_pipelines(
        &self,
        req: crate::model::ListDeliveryPipelinesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDeliveryPipelinesResponse> {
        self.inner.list_delivery_pipelines(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_delivery_pipeline(
        &self,
        req: crate::model::GetDeliveryPipelineRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DeliveryPipeline> {
        self.inner.get_delivery_pipeline(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_delivery_pipeline(
        &self,
        req: crate::model::CreateDeliveryPipelineRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_delivery_pipeline(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_delivery_pipeline(
        &self,
        req: crate::model::UpdateDeliveryPipelineRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_delivery_pipeline(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_delivery_pipeline(
        &self,
        req: crate::model::DeleteDeliveryPipelineRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_delivery_pipeline(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_targets(
        &self,
        req: crate::model::ListTargetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTargetsResponse> {
        self.inner.list_targets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn rollback_target(
        &self,
        req: crate::model::RollbackTargetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::RollbackTargetResponse> {
        self.inner.rollback_target(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_target(
        &self,
        req: crate::model::GetTargetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Target> {
        self.inner.get_target(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_target(
        &self,
        req: crate::model::CreateTargetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_target(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_target(
        &self,
        req: crate::model::UpdateTargetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_target(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_target(
        &self,
        req: crate::model::DeleteTargetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_target(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_custom_target_types(
        &self,
        req: crate::model::ListCustomTargetTypesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListCustomTargetTypesResponse> {
        self.inner.list_custom_target_types(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_custom_target_type(
        &self,
        req: crate::model::GetCustomTargetTypeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::CustomTargetType> {
        self.inner.get_custom_target_type(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_custom_target_type(
        &self,
        req: crate::model::CreateCustomTargetTypeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_custom_target_type(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_custom_target_type(
        &self,
        req: crate::model::UpdateCustomTargetTypeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_custom_target_type(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_custom_target_type(
        &self,
        req: crate::model::DeleteCustomTargetTypeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_custom_target_type(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_releases(
        &self,
        req: crate::model::ListReleasesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListReleasesResponse> {
        self.inner.list_releases(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_release(
        &self,
        req: crate::model::GetReleaseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Release> {
        self.inner.get_release(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_release(
        &self,
        req: crate::model::CreateReleaseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_release(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn abandon_release(
        &self,
        req: crate::model::AbandonReleaseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AbandonReleaseResponse> {
        self.inner.abandon_release(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_deploy_policy(
        &self,
        req: crate::model::CreateDeployPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_deploy_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_deploy_policy(
        &self,
        req: crate::model::UpdateDeployPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_deploy_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_deploy_policy(
        &self,
        req: crate::model::DeleteDeployPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_deploy_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_deploy_policies(
        &self,
        req: crate::model::ListDeployPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDeployPoliciesResponse> {
        self.inner.list_deploy_policies(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_deploy_policy(
        &self,
        req: crate::model::GetDeployPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DeployPolicy> {
        self.inner.get_deploy_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn approve_rollout(
        &self,
        req: crate::model::ApproveRolloutRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ApproveRolloutResponse> {
        self.inner.approve_rollout(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn advance_rollout(
        &self,
        req: crate::model::AdvanceRolloutRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AdvanceRolloutResponse> {
        self.inner.advance_rollout(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_rollout(
        &self,
        req: crate::model::CancelRolloutRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::CancelRolloutResponse> {
        self.inner.cancel_rollout(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_rollouts(
        &self,
        req: crate::model::ListRolloutsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListRolloutsResponse> {
        self.inner.list_rollouts(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_rollout(
        &self,
        req: crate::model::GetRolloutRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Rollout> {
        self.inner.get_rollout(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_rollout(
        &self,
        req: crate::model::CreateRolloutRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_rollout(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn ignore_job(
        &self,
        req: crate::model::IgnoreJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::IgnoreJobResponse> {
        self.inner.ignore_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn retry_job(
        &self,
        req: crate::model::RetryJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::RetryJobResponse> {
        self.inner.retry_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_job_runs(
        &self,
        req: crate::model::ListJobRunsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListJobRunsResponse> {
        self.inner.list_job_runs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_job_run(
        &self,
        req: crate::model::GetJobRunRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::JobRun> {
        self.inner.get_job_run(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn terminate_job_run(
        &self,
        req: crate::model::TerminateJobRunRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TerminateJobRunResponse> {
        self.inner.terminate_job_run(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_config(
        &self,
        req: crate::model::GetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Config> {
        self.inner.get_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_automation(
        &self,
        req: crate::model::CreateAutomationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_automation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_automation(
        &self,
        req: crate::model::UpdateAutomationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_automation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_automation(
        &self,
        req: crate::model::DeleteAutomationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_automation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_automation(
        &self,
        req: crate::model::GetAutomationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Automation> {
        self.inner.get_automation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_automations(
        &self,
        req: crate::model::ListAutomationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAutomationsResponse> {
        self.inner.list_automations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_automation_run(
        &self,
        req: crate::model::GetAutomationRunRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AutomationRun> {
        self.inner.get_automation_run(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_automation_runs(
        &self,
        req: crate::model::ListAutomationRunsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAutomationRunsResponse> {
        self.inner.list_automation_runs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_automation_run(
        &self,
        req: crate::model::CancelAutomationRunRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::CancelAutomationRunResponse> {
        self.inner.cancel_automation_run(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
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
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
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
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
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
