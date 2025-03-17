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

/// Implements a client for the Connectors API.
///
/// # Service Description
///
/// Connectors is the interface for managing Connectors.
///
/// # Configuration
///
/// `Connectors` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `Connectors` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Connectors` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Connectors {
    inner: Arc<dyn super::stubs::dynamic::Connectors>,
}

impl Connectors {
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
        T: super::stubs::Connectors + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stubs::dynamic::Connectors>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::Connectors> {
        super::transport::Connectors::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::Connectors> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::Connectors::new)
    }

    /// Lists Connections in a given project and location.
    pub fn list_connections(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::connectors::ListConnections {
        super::builders::connectors::ListConnections::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single Connection.
    pub fn get_connection(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::connectors::GetConnection {
        super::builders::connectors::GetConnection::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a new Connection in a given project and location.
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
    pub fn create_connection(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::connectors::CreateConnection {
        super::builders::connectors::CreateConnection::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a single Connection.
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
    pub fn update_connection(
        &self,
        connection: impl Into<crate::model::Connection>,
    ) -> super::builders::connectors::UpdateConnection {
        super::builders::connectors::UpdateConnection::new(self.inner.clone())
            .set_connection(connection.into())
    }

    /// Deletes a single Connection.
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
    pub fn delete_connection(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::connectors::DeleteConnection {
        super::builders::connectors::DeleteConnection::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists Providers in a given project and location.
    pub fn list_providers(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::connectors::ListProviders {
        super::builders::connectors::ListProviders::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a provider.
    pub fn get_provider(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::connectors::GetProvider {
        super::builders::connectors::GetProvider::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists Connectors in a given project and location.
    pub fn list_connectors(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::connectors::ListConnectors {
        super::builders::connectors::ListConnectors::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single Connector.
    pub fn get_connector(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::connectors::GetConnector {
        super::builders::connectors::GetConnector::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists Connector Versions in a given project and location.
    pub fn list_connector_versions(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::connectors::ListConnectorVersions {
        super::builders::connectors::ListConnectorVersions::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single connector version.
    pub fn get_connector_version(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::connectors::GetConnectorVersion {
        super::builders::connectors::GetConnectorVersion::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets schema metadata of a connection.
    /// SchemaMetadata is a singleton resource for each connection.
    pub fn get_connection_schema_metadata(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::connectors::GetConnectionSchemaMetadata {
        super::builders::connectors::GetConnectionSchemaMetadata::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Refresh runtime schema of a connection.
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
    pub fn refresh_connection_schema_metadata(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::connectors::RefreshConnectionSchemaMetadata {
        super::builders::connectors::RefreshConnectionSchemaMetadata::new(self.inner.clone())
            .set_name(name.into())
    }

    /// List schema of a runtime entities filtered by entity name.
    pub fn list_runtime_entity_schemas(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::connectors::ListRuntimeEntitySchemas {
        super::builders::connectors::ListRuntimeEntitySchemas::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// List schema of a runtime actions filtered by action name.
    pub fn list_runtime_action_schemas(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::connectors::ListRuntimeActionSchemas {
        super::builders::connectors::ListRuntimeActionSchemas::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets the runtimeConfig of a location.
    /// RuntimeConfig is a singleton resource for each location.
    pub fn get_runtime_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::connectors::GetRuntimeConfig {
        super::builders::connectors::GetRuntimeConfig::new(self.inner.clone()).set_name(name.into())
    }

    /// GetGlobalSettings gets settings of a project.
    /// GlobalSettings is a singleton resource.
    pub fn get_global_settings(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::connectors::GetGlobalSettings {
        super::builders::connectors::GetGlobalSettings::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::connectors::ListLocations {
        super::builders::connectors::ListLocations::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::connectors::GetLocation {
        super::builders::connectors::GetLocation::new(self.inner.clone()).set_name(name.into())
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builders::connectors::SetIamPolicy {
        super::builders::connectors::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builders::connectors::GetIamPolicy {
        super::builders::connectors::GetIamPolicy::new(self.inner.clone())
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
    ) -> super::builders::connectors::TestIamPermissions {
        super::builders::connectors::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Lists operations that match the specified filter in the request. If
    /// the server doesn't support this method, it returns `UNIMPLEMENTED`.
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::connectors::ListOperations {
        super::builders::connectors::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::connectors::GetOperation {
        super::builders::connectors::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::connectors::DeleteOperation {
        super::builders::connectors::DeleteOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::connectors::CancelOperation {
        super::builders::connectors::CancelOperation::new(self.inner.clone()).set_name(name.into())
    }
}
