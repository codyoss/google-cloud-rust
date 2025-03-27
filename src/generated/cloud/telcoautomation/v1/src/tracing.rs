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

/// Implements a [TelcoAutomation](super::stubs::TelcoAutomation) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct TelcoAutomation<T>
where
    T: super::stubs::TelcoAutomation + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> TelcoAutomation<T>
where
    T: super::stubs::TelcoAutomation + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::TelcoAutomation for TelcoAutomation<T>
where
    T: super::stubs::TelcoAutomation + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_orchestration_clusters(
        &self,
        req: crate::model::ListOrchestrationClustersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListOrchestrationClustersResponse> {
        self.inner.list_orchestration_clusters(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_orchestration_cluster(
        &self,
        req: crate::model::GetOrchestrationClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::OrchestrationCluster> {
        self.inner.get_orchestration_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_orchestration_cluster(
        &self,
        req: crate::model::CreateOrchestrationClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_orchestration_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_orchestration_cluster(
        &self,
        req: crate::model::DeleteOrchestrationClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_orchestration_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_edge_slms(
        &self,
        req: crate::model::ListEdgeSlmsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListEdgeSlmsResponse> {
        self.inner.list_edge_slms(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_edge_slm(
        &self,
        req: crate::model::GetEdgeSlmRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::EdgeSlm> {
        self.inner.get_edge_slm(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_edge_slm(
        &self,
        req: crate::model::CreateEdgeSlmRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_edge_slm(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_edge_slm(
        &self,
        req: crate::model::DeleteEdgeSlmRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_edge_slm(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_blueprint(
        &self,
        req: crate::model::CreateBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Blueprint> {
        self.inner.create_blueprint(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_blueprint(
        &self,
        req: crate::model::UpdateBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Blueprint> {
        self.inner.update_blueprint(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_blueprint(
        &self,
        req: crate::model::GetBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Blueprint> {
        self.inner.get_blueprint(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_blueprint(
        &self,
        req: crate::model::DeleteBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_blueprint(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_blueprints(
        &self,
        req: crate::model::ListBlueprintsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListBlueprintsResponse> {
        self.inner.list_blueprints(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn approve_blueprint(
        &self,
        req: crate::model::ApproveBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Blueprint> {
        self.inner.approve_blueprint(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn propose_blueprint(
        &self,
        req: crate::model::ProposeBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Blueprint> {
        self.inner.propose_blueprint(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn reject_blueprint(
        &self,
        req: crate::model::RejectBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Blueprint> {
        self.inner.reject_blueprint(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_blueprint_revisions(
        &self,
        req: crate::model::ListBlueprintRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListBlueprintRevisionsResponse> {
        self.inner.list_blueprint_revisions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn search_blueprint_revisions(
        &self,
        req: crate::model::SearchBlueprintRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SearchBlueprintRevisionsResponse> {
        self.inner.search_blueprint_revisions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn search_deployment_revisions(
        &self,
        req: crate::model::SearchDeploymentRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SearchDeploymentRevisionsResponse> {
        self.inner.search_deployment_revisions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn discard_blueprint_changes(
        &self,
        req: crate::model::DiscardBlueprintChangesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DiscardBlueprintChangesResponse> {
        self.inner.discard_blueprint_changes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_public_blueprints(
        &self,
        req: crate::model::ListPublicBlueprintsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListPublicBlueprintsResponse> {
        self.inner.list_public_blueprints(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_public_blueprint(
        &self,
        req: crate::model::GetPublicBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::PublicBlueprint> {
        self.inner.get_public_blueprint(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_deployment(
        &self,
        req: crate::model::CreateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Deployment> {
        self.inner.create_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_deployment(
        &self,
        req: crate::model::UpdateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Deployment> {
        self.inner.update_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_deployment(
        &self,
        req: crate::model::GetDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Deployment> {
        self.inner.get_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn remove_deployment(
        &self,
        req: crate::model::RemoveDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.remove_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_deployments(
        &self,
        req: crate::model::ListDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDeploymentsResponse> {
        self.inner.list_deployments(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_deployment_revisions(
        &self,
        req: crate::model::ListDeploymentRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDeploymentRevisionsResponse> {
        self.inner.list_deployment_revisions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn discard_deployment_changes(
        &self,
        req: crate::model::DiscardDeploymentChangesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DiscardDeploymentChangesResponse> {
        self.inner.discard_deployment_changes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn apply_deployment(
        &self,
        req: crate::model::ApplyDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Deployment> {
        self.inner.apply_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn compute_deployment_status(
        &self,
        req: crate::model::ComputeDeploymentStatusRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ComputeDeploymentStatusResponse> {
        self.inner.compute_deployment_status(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn rollback_deployment(
        &self,
        req: crate::model::RollbackDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Deployment> {
        self.inner.rollback_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_hydrated_deployment(
        &self,
        req: crate::model::GetHydratedDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::HydratedDeployment> {
        self.inner.get_hydrated_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_hydrated_deployments(
        &self,
        req: crate::model::ListHydratedDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListHydratedDeploymentsResponse> {
        self.inner.list_hydrated_deployments(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_hydrated_deployment(
        &self,
        req: crate::model::UpdateHydratedDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::HydratedDeployment> {
        self.inner.update_hydrated_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn apply_hydrated_deployment(
        &self,
        req: crate::model::ApplyHydratedDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::HydratedDeployment> {
        self.inner.apply_hydrated_deployment(req, options).await
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
    ) -> Result<()> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
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
