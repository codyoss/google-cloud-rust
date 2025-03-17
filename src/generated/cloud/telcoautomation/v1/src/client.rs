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
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Telco Automation API.
///
/// # Service Description
///
/// TelcoAutomation Service manages the control plane cluster a.k.a.
/// Orchestration Cluster (GKE cluster with config controller) of TNA. It also
/// exposes blueprint APIs which manages the lifecycle of blueprints that control
/// the infrastructure setup (e.g GDCE clusters) and deployment of network
/// functions.
///
/// # Configuration
///
/// `TelcoAutomation` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `TelcoAutomation` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `TelcoAutomation` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct TelcoAutomation {
    inner: Arc<dyn super::stubs::dynamic::TelcoAutomation>,
}

impl TelcoAutomation {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stubs::TelcoAutomation + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stubs::dynamic::TelcoAutomation>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::TelcoAutomation> {
        super::transport::TelcoAutomation::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::TelcoAutomation> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::TelcoAutomation::new)
    }

    /// Lists OrchestrationClusters in a given project and location.
    pub fn list_orchestration_clusters(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::ListOrchestrationClusters {
        super::builders::telco_automation::ListOrchestrationClusters::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single OrchestrationCluster.
    pub fn get_orchestration_cluster(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::GetOrchestrationCluster {
        super::builders::telco_automation::GetOrchestrationCluster::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new OrchestrationCluster in a given project and location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_orchestration_cluster(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::CreateOrchestrationCluster {
        super::builders::telco_automation::CreateOrchestrationCluster::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a single OrchestrationCluster.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_orchestration_cluster(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::DeleteOrchestrationCluster {
        super::builders::telco_automation::DeleteOrchestrationCluster::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists EdgeSlms in a given project and location.
    pub fn list_edge_slms(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::ListEdgeSlms {
        super::builders::telco_automation::ListEdgeSlms::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single EdgeSlm.
    pub fn get_edge_slm(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::GetEdgeSlm {
        super::builders::telco_automation::GetEdgeSlm::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a new EdgeSlm in a given project and location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_edge_slm(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::CreateEdgeSlm {
        super::builders::telco_automation::CreateEdgeSlm::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a single EdgeSlm.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_edge_slm(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::DeleteEdgeSlm {
        super::builders::telco_automation::DeleteEdgeSlm::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a blueprint.
    pub fn create_blueprint(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::CreateBlueprint {
        super::builders::telco_automation::CreateBlueprint::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a blueprint.
    pub fn update_blueprint(
        &self,
        blueprint: impl Into<crate::model::Blueprint>,
    ) -> super::builders::telco_automation::UpdateBlueprint {
        super::builders::telco_automation::UpdateBlueprint::new(self.inner.clone())
            .set_blueprint(blueprint.into())
    }

    /// Returns the requested blueprint.
    pub fn get_blueprint(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::GetBlueprint {
        super::builders::telco_automation::GetBlueprint::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes a blueprint and all its revisions.
    pub fn delete_blueprint(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::DeleteBlueprint {
        super::builders::telco_automation::DeleteBlueprint::new(self.inner.clone())
            .set_name(name.into())
    }

    /// List all blueprints.
    pub fn list_blueprints(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::ListBlueprints {
        super::builders::telco_automation::ListBlueprints::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Approves a blueprint and commits a new revision.
    pub fn approve_blueprint(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::ApproveBlueprint {
        super::builders::telco_automation::ApproveBlueprint::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Proposes a blueprint for approval of changes.
    pub fn propose_blueprint(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::ProposeBlueprint {
        super::builders::telco_automation::ProposeBlueprint::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Rejects a blueprint revision proposal and flips it back to Draft state.
    pub fn reject_blueprint(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::RejectBlueprint {
        super::builders::telco_automation::RejectBlueprint::new(self.inner.clone())
            .set_name(name.into())
    }

    /// List blueprint revisions of a given blueprint.
    pub fn list_blueprint_revisions(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::ListBlueprintRevisions {
        super::builders::telco_automation::ListBlueprintRevisions::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Searches across blueprint revisions.
    pub fn search_blueprint_revisions(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::SearchBlueprintRevisions {
        super::builders::telco_automation::SearchBlueprintRevisions::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Searches across deployment revisions.
    pub fn search_deployment_revisions(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::SearchDeploymentRevisions {
        super::builders::telco_automation::SearchDeploymentRevisions::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Discards the changes in a blueprint and reverts the blueprint to the last
    /// approved blueprint revision. No changes take place if a blueprint does not
    /// have revisions.
    pub fn discard_blueprint_changes(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::DiscardBlueprintChanges {
        super::builders::telco_automation::DiscardBlueprintChanges::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists the blueprints in TNA's public catalog. Default page size = 20,
    /// Max Page Size = 100.
    pub fn list_public_blueprints(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::ListPublicBlueprints {
        super::builders::telco_automation::ListPublicBlueprints::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Returns the requested public blueprint.
    pub fn get_public_blueprint(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::GetPublicBlueprint {
        super::builders::telco_automation::GetPublicBlueprint::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a deployment.
    pub fn create_deployment(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::CreateDeployment {
        super::builders::telco_automation::CreateDeployment::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a deployment.
    pub fn update_deployment(
        &self,
        deployment: impl Into<crate::model::Deployment>,
    ) -> super::builders::telco_automation::UpdateDeployment {
        super::builders::telco_automation::UpdateDeployment::new(self.inner.clone())
            .set_deployment(deployment.into())
    }

    /// Returns the requested deployment.
    pub fn get_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::GetDeployment {
        super::builders::telco_automation::GetDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Removes the deployment by marking it as DELETING. Post which deployment and
    /// it's revisions gets deleted.
    pub fn remove_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::RemoveDeployment {
        super::builders::telco_automation::RemoveDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// List all deployments.
    pub fn list_deployments(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::ListDeployments {
        super::builders::telco_automation::ListDeployments::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// List deployment revisions of a given deployment.
    pub fn list_deployment_revisions(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::ListDeploymentRevisions {
        super::builders::telco_automation::ListDeploymentRevisions::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Discards the changes in a deployment and reverts the deployment to the last
    /// approved deployment revision. No changes take place if a deployment does
    /// not have revisions.
    pub fn discard_deployment_changes(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::DiscardDeploymentChanges {
        super::builders::telco_automation::DiscardDeploymentChanges::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Applies the deployment's YAML files to the parent orchestration cluster.
    pub fn apply_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::ApplyDeployment {
        super::builders::telco_automation::ApplyDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Returns the requested deployment status.
    pub fn compute_deployment_status(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::ComputeDeploymentStatus {
        super::builders::telco_automation::ComputeDeploymentStatus::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Rollback the active deployment to the given past approved deployment
    /// revision.
    pub fn rollback_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::RollbackDeployment {
        super::builders::telco_automation::RollbackDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Returns the requested hydrated deployment.
    pub fn get_hydrated_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::GetHydratedDeployment {
        super::builders::telco_automation::GetHydratedDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// List all hydrated deployments present under a deployment.
    pub fn list_hydrated_deployments(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::ListHydratedDeployments {
        super::builders::telco_automation::ListHydratedDeployments::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a hydrated deployment.
    pub fn update_hydrated_deployment(
        &self,
        hydrated_deployment: impl Into<crate::model::HydratedDeployment>,
    ) -> super::builders::telco_automation::UpdateHydratedDeployment {
        super::builders::telco_automation::UpdateHydratedDeployment::new(self.inner.clone())
            .set_hydrated_deployment(hydrated_deployment.into())
    }

    /// Applies a hydrated deployment to a workload cluster.
    pub fn apply_hydrated_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::ApplyHydratedDeployment {
        super::builders::telco_automation::ApplyHydratedDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::ListLocations {
        super::builders::telco_automation::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::GetLocation {
        super::builders::telco_automation::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::ListOperations {
        super::builders::telco_automation::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::GetOperation {
        super::builders::telco_automation::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::DeleteOperation {
        super::builders::telco_automation::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::telco_automation::CancelOperation {
        super::builders::telco_automation::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
