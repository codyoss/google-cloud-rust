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

/// Implements a client for the Cloud Workstations API.
///
/// # Service Description
///
/// Service for interacting with Cloud Workstations.
///
/// # Configuration
///
/// `Workstations` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `Workstations` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Workstations` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Workstations {
    inner: Arc<dyn super::stubs::dynamic::Workstations>,
}

impl Workstations {
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
        T: super::stubs::Workstations + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stubs::dynamic::Workstations>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::Workstations> {
        super::transport::Workstations::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::Workstations> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::Workstations::new)
    }

    /// Returns the requested workstation cluster.
    pub fn get_workstation_cluster(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::workstations::GetWorkstationCluster {
        super::builders::workstations::GetWorkstationCluster::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Returns all workstation clusters in the specified location.
    pub fn list_workstation_clusters(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::workstations::ListWorkstationClusters {
        super::builders::workstations::ListWorkstationClusters::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates a new workstation cluster.
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
    pub fn create_workstation_cluster(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::workstations::CreateWorkstationCluster {
        super::builders::workstations::CreateWorkstationCluster::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates an existing workstation cluster.
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
    pub fn update_workstation_cluster(
        &self,
        workstation_cluster: impl Into<crate::model::WorkstationCluster>,
    ) -> super::builders::workstations::UpdateWorkstationCluster {
        super::builders::workstations::UpdateWorkstationCluster::new(self.inner.clone())
            .set_workstation_cluster(workstation_cluster.into())
    }

    /// Deletes the specified workstation cluster.
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
    pub fn delete_workstation_cluster(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::workstations::DeleteWorkstationCluster {
        super::builders::workstations::DeleteWorkstationCluster::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Returns the requested workstation configuration.
    pub fn get_workstation_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::workstations::GetWorkstationConfig {
        super::builders::workstations::GetWorkstationConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Returns all workstation configurations in the specified cluster.
    pub fn list_workstation_configs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::workstations::ListWorkstationConfigs {
        super::builders::workstations::ListWorkstationConfigs::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Returns all workstation configurations in the specified cluster on which
    /// the caller has the "workstations.workstation.create" permission.
    pub fn list_usable_workstation_configs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::workstations::ListUsableWorkstationConfigs {
        super::builders::workstations::ListUsableWorkstationConfigs::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates a new workstation configuration.
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
    pub fn create_workstation_config(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::workstations::CreateWorkstationConfig {
        super::builders::workstations::CreateWorkstationConfig::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates an existing workstation configuration.
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
    pub fn update_workstation_config(
        &self,
        workstation_config: impl Into<crate::model::WorkstationConfig>,
    ) -> super::builders::workstations::UpdateWorkstationConfig {
        super::builders::workstations::UpdateWorkstationConfig::new(self.inner.clone())
            .set_workstation_config(workstation_config.into())
    }

    /// Deletes the specified workstation configuration.
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
    pub fn delete_workstation_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::workstations::DeleteWorkstationConfig {
        super::builders::workstations::DeleteWorkstationConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Returns the requested workstation.
    pub fn get_workstation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::workstations::GetWorkstation {
        super::builders::workstations::GetWorkstation::new(self.inner.clone()).set_name(name.into())
    }

    /// Returns all Workstations using the specified workstation configuration.
    pub fn list_workstations(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::workstations::ListWorkstations {
        super::builders::workstations::ListWorkstations::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Returns all workstations using the specified workstation configuration
    /// on which the caller has the "workstations.workstations.use" permission.
    pub fn list_usable_workstations(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::workstations::ListUsableWorkstations {
        super::builders::workstations::ListUsableWorkstations::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates a new workstation.
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
    pub fn create_workstation(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::workstations::CreateWorkstation {
        super::builders::workstations::CreateWorkstation::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates an existing workstation.
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
    pub fn update_workstation(
        &self,
        workstation: impl Into<crate::model::Workstation>,
    ) -> super::builders::workstations::UpdateWorkstation {
        super::builders::workstations::UpdateWorkstation::new(self.inner.clone())
            .set_workstation(workstation.into())
    }

    /// Deletes the specified workstation.
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
    pub fn delete_workstation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::workstations::DeleteWorkstation {
        super::builders::workstations::DeleteWorkstation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Starts running a workstation so that users can connect to it.
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
    pub fn start_workstation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::workstations::StartWorkstation {
        super::builders::workstations::StartWorkstation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Stops running a workstation, reducing costs.
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
    pub fn stop_workstation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::workstations::StopWorkstation {
        super::builders::workstations::StopWorkstation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Returns a short-lived credential that can be used to send authenticated and
    /// authorized traffic to a workstation.
    pub fn generate_access_token(
        &self,
        workstation: impl Into<std::string::String>,
    ) -> super::builders::workstations::GenerateAccessToken {
        super::builders::workstations::GenerateAccessToken::new(self.inner.clone())
            .set_workstation(workstation.into())
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builders::workstations::SetIamPolicy {
        super::builders::workstations::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builders::workstations::GetIamPolicy {
        super::builders::workstations::GetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    pub fn test_iam_permissions(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builders::workstations::TestIamPermissions {
        super::builders::workstations::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::workstations::ListOperations {
        super::builders::workstations::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::workstations::GetOperation {
        super::builders::workstations::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::workstations::DeleteOperation {
        super::builders::workstations::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::workstations::CancelOperation {
        super::builders::workstations::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
