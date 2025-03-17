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

/// A dyn-compatible, crate-private version of [super::TelcoAutomation].
#[async_trait::async_trait]
pub trait TelcoAutomation: std::fmt::Debug + Send + Sync {
    async fn list_orchestration_clusters(
        &self,
        req: crate::model::ListOrchestrationClustersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListOrchestrationClustersResponse>;

    async fn get_orchestration_cluster(
        &self,
        req: crate::model::GetOrchestrationClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::OrchestrationCluster>;

    async fn create_orchestration_cluster(
        &self,
        req: crate::model::CreateOrchestrationClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_orchestration_cluster(
        &self,
        req: crate::model::DeleteOrchestrationClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_edge_slms(
        &self,
        req: crate::model::ListEdgeSlmsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListEdgeSlmsResponse>;

    async fn get_edge_slm(
        &self,
        req: crate::model::GetEdgeSlmRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EdgeSlm>;

    async fn create_edge_slm(
        &self,
        req: crate::model::CreateEdgeSlmRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_edge_slm(
        &self,
        req: crate::model::DeleteEdgeSlmRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_blueprint(
        &self,
        req: crate::model::CreateBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Blueprint>;

    async fn update_blueprint(
        &self,
        req: crate::model::UpdateBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Blueprint>;

    async fn get_blueprint(
        &self,
        req: crate::model::GetBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Blueprint>;

    async fn delete_blueprint(
        &self,
        req: crate::model::DeleteBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn list_blueprints(
        &self,
        req: crate::model::ListBlueprintsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBlueprintsResponse>;

    async fn approve_blueprint(
        &self,
        req: crate::model::ApproveBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Blueprint>;

    async fn propose_blueprint(
        &self,
        req: crate::model::ProposeBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Blueprint>;

    async fn reject_blueprint(
        &self,
        req: crate::model::RejectBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Blueprint>;

    async fn list_blueprint_revisions(
        &self,
        req: crate::model::ListBlueprintRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBlueprintRevisionsResponse>;

    async fn search_blueprint_revisions(
        &self,
        req: crate::model::SearchBlueprintRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchBlueprintRevisionsResponse>;

    async fn search_deployment_revisions(
        &self,
        req: crate::model::SearchDeploymentRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchDeploymentRevisionsResponse>;

    async fn discard_blueprint_changes(
        &self,
        req: crate::model::DiscardBlueprintChangesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DiscardBlueprintChangesResponse>;

    async fn list_public_blueprints(
        &self,
        req: crate::model::ListPublicBlueprintsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPublicBlueprintsResponse>;

    async fn get_public_blueprint(
        &self,
        req: crate::model::GetPublicBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PublicBlueprint>;

    async fn create_deployment(
        &self,
        req: crate::model::CreateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Deployment>;

    async fn update_deployment(
        &self,
        req: crate::model::UpdateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Deployment>;

    async fn get_deployment(
        &self,
        req: crate::model::GetDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Deployment>;

    async fn remove_deployment(
        &self,
        req: crate::model::RemoveDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn list_deployments(
        &self,
        req: crate::model::ListDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDeploymentsResponse>;

    async fn list_deployment_revisions(
        &self,
        req: crate::model::ListDeploymentRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDeploymentRevisionsResponse>;

    async fn discard_deployment_changes(
        &self,
        req: crate::model::DiscardDeploymentChangesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DiscardDeploymentChangesResponse>;

    async fn apply_deployment(
        &self,
        req: crate::model::ApplyDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Deployment>;

    async fn compute_deployment_status(
        &self,
        req: crate::model::ComputeDeploymentStatusRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ComputeDeploymentStatusResponse>;

    async fn rollback_deployment(
        &self,
        req: crate::model::RollbackDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Deployment>;

    async fn get_hydrated_deployment(
        &self,
        req: crate::model::GetHydratedDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::HydratedDeployment>;

    async fn list_hydrated_deployments(
        &self,
        req: crate::model::ListHydratedDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListHydratedDeploymentsResponse>;

    async fn update_hydrated_deployment(
        &self,
        req: crate::model::UpdateHydratedDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::HydratedDeployment>;

    async fn apply_hydrated_deployment(
        &self,
        req: crate::model::ApplyHydratedDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::HydratedDeployment>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location>;

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

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::TelcoAutomation] also implement [TelcoAutomation].
#[async_trait::async_trait]
impl<T: super::TelcoAutomation> TelcoAutomation for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_orchestration_clusters(
        &self,
        req: crate::model::ListOrchestrationClustersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListOrchestrationClustersResponse> {
        T::list_orchestration_clusters(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_orchestration_cluster(
        &self,
        req: crate::model::GetOrchestrationClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::OrchestrationCluster> {
        T::get_orchestration_cluster(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_orchestration_cluster(
        &self,
        req: crate::model::CreateOrchestrationClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_orchestration_cluster(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_orchestration_cluster(
        &self,
        req: crate::model::DeleteOrchestrationClusterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_orchestration_cluster(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_edge_slms(
        &self,
        req: crate::model::ListEdgeSlmsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListEdgeSlmsResponse> {
        T::list_edge_slms(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_edge_slm(
        &self,
        req: crate::model::GetEdgeSlmRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EdgeSlm> {
        T::get_edge_slm(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_edge_slm(
        &self,
        req: crate::model::CreateEdgeSlmRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_edge_slm(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_edge_slm(
        &self,
        req: crate::model::DeleteEdgeSlmRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_edge_slm(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_blueprint(
        &self,
        req: crate::model::CreateBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Blueprint> {
        T::create_blueprint(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_blueprint(
        &self,
        req: crate::model::UpdateBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Blueprint> {
        T::update_blueprint(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_blueprint(
        &self,
        req: crate::model::GetBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Blueprint> {
        T::get_blueprint(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_blueprint(
        &self,
        req: crate::model::DeleteBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_blueprint(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_blueprints(
        &self,
        req: crate::model::ListBlueprintsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBlueprintsResponse> {
        T::list_blueprints(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn approve_blueprint(
        &self,
        req: crate::model::ApproveBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Blueprint> {
        T::approve_blueprint(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn propose_blueprint(
        &self,
        req: crate::model::ProposeBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Blueprint> {
        T::propose_blueprint(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn reject_blueprint(
        &self,
        req: crate::model::RejectBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Blueprint> {
        T::reject_blueprint(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_blueprint_revisions(
        &self,
        req: crate::model::ListBlueprintRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBlueprintRevisionsResponse> {
        T::list_blueprint_revisions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn search_blueprint_revisions(
        &self,
        req: crate::model::SearchBlueprintRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchBlueprintRevisionsResponse> {
        T::search_blueprint_revisions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn search_deployment_revisions(
        &self,
        req: crate::model::SearchDeploymentRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchDeploymentRevisionsResponse> {
        T::search_deployment_revisions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn discard_blueprint_changes(
        &self,
        req: crate::model::DiscardBlueprintChangesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DiscardBlueprintChangesResponse> {
        T::discard_blueprint_changes(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_public_blueprints(
        &self,
        req: crate::model::ListPublicBlueprintsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPublicBlueprintsResponse> {
        T::list_public_blueprints(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_public_blueprint(
        &self,
        req: crate::model::GetPublicBlueprintRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PublicBlueprint> {
        T::get_public_blueprint(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_deployment(
        &self,
        req: crate::model::CreateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Deployment> {
        T::create_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_deployment(
        &self,
        req: crate::model::UpdateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Deployment> {
        T::update_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_deployment(
        &self,
        req: crate::model::GetDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Deployment> {
        T::get_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn remove_deployment(
        &self,
        req: crate::model::RemoveDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::remove_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_deployments(
        &self,
        req: crate::model::ListDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDeploymentsResponse> {
        T::list_deployments(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_deployment_revisions(
        &self,
        req: crate::model::ListDeploymentRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDeploymentRevisionsResponse> {
        T::list_deployment_revisions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn discard_deployment_changes(
        &self,
        req: crate::model::DiscardDeploymentChangesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DiscardDeploymentChangesResponse> {
        T::discard_deployment_changes(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn apply_deployment(
        &self,
        req: crate::model::ApplyDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Deployment> {
        T::apply_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn compute_deployment_status(
        &self,
        req: crate::model::ComputeDeploymentStatusRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ComputeDeploymentStatusResponse> {
        T::compute_deployment_status(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn rollback_deployment(
        &self,
        req: crate::model::RollbackDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Deployment> {
        T::rollback_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_hydrated_deployment(
        &self,
        req: crate::model::GetHydratedDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::HydratedDeployment> {
        T::get_hydrated_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_hydrated_deployments(
        &self,
        req: crate::model::ListHydratedDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListHydratedDeploymentsResponse> {
        T::list_hydrated_deployments(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_hydrated_deployment(
        &self,
        req: crate::model::UpdateHydratedDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::HydratedDeployment> {
        T::update_hydrated_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn apply_hydrated_deployment(
        &self,
        req: crate::model::ApplyHydratedDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::HydratedDeployment> {
        T::apply_hydrated_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location> {
        T::get_location(self, req, options).await
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

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        T::get_polling_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}
