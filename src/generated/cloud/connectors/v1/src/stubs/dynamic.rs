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

/// A dyn-compatible, crate-private version of [super::Connectors].
#[async_trait::async_trait]
pub trait Connectors: std::fmt::Debug + Send + Sync {
    async fn list_connections(
        &self,
        req: crate::model::ListConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConnectionsResponse>;

    async fn get_connection(
        &self,
        req: crate::model::GetConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Connection>;

    async fn create_connection(
        &self,
        req: crate::model::CreateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_connection(
        &self,
        req: crate::model::UpdateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_connection(
        &self,
        req: crate::model::DeleteConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_providers(
        &self,
        req: crate::model::ListProvidersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListProvidersResponse>;

    async fn get_provider(
        &self,
        req: crate::model::GetProviderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Provider>;

    async fn list_connectors(
        &self,
        req: crate::model::ListConnectorsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConnectorsResponse>;

    async fn get_connector(
        &self,
        req: crate::model::GetConnectorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Connector>;

    async fn list_connector_versions(
        &self,
        req: crate::model::ListConnectorVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConnectorVersionsResponse>;

    async fn get_connector_version(
        &self,
        req: crate::model::GetConnectorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ConnectorVersion>;

    async fn get_connection_schema_metadata(
        &self,
        req: crate::model::GetConnectionSchemaMetadataRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ConnectionSchemaMetadata>;

    async fn refresh_connection_schema_metadata(
        &self,
        req: crate::model::RefreshConnectionSchemaMetadataRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_runtime_entity_schemas(
        &self,
        req: crate::model::ListRuntimeEntitySchemasRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRuntimeEntitySchemasResponse>;

    async fn list_runtime_action_schemas(
        &self,
        req: crate::model::ListRuntimeActionSchemasRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRuntimeActionSchemasResponse>;

    async fn get_runtime_config(
        &self,
        req: crate::model::GetRuntimeConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::RuntimeConfig>;

    async fn get_global_settings(
        &self,
        req: crate::model::GetGlobalSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Settings>;

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

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;

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

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::Connectors] also implement [Connectors].
#[async_trait::async_trait]
impl<T: super::Connectors> Connectors for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_connections(
        &self,
        req: crate::model::ListConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConnectionsResponse> {
        T::list_connections(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_connection(
        &self,
        req: crate::model::GetConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Connection> {
        T::get_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_connection(
        &self,
        req: crate::model::CreateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_connection(
        &self,
        req: crate::model::UpdateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_connection(
        &self,
        req: crate::model::DeleteConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_providers(
        &self,
        req: crate::model::ListProvidersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListProvidersResponse> {
        T::list_providers(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_provider(
        &self,
        req: crate::model::GetProviderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Provider> {
        T::get_provider(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_connectors(
        &self,
        req: crate::model::ListConnectorsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConnectorsResponse> {
        T::list_connectors(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_connector(
        &self,
        req: crate::model::GetConnectorRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Connector> {
        T::get_connector(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_connector_versions(
        &self,
        req: crate::model::ListConnectorVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConnectorVersionsResponse> {
        T::list_connector_versions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_connector_version(
        &self,
        req: crate::model::GetConnectorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ConnectorVersion> {
        T::get_connector_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_connection_schema_metadata(
        &self,
        req: crate::model::GetConnectionSchemaMetadataRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ConnectionSchemaMetadata> {
        T::get_connection_schema_metadata(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn refresh_connection_schema_metadata(
        &self,
        req: crate::model::RefreshConnectionSchemaMetadataRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::refresh_connection_schema_metadata(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_runtime_entity_schemas(
        &self,
        req: crate::model::ListRuntimeEntitySchemasRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRuntimeEntitySchemasResponse> {
        T::list_runtime_entity_schemas(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_runtime_action_schemas(
        &self,
        req: crate::model::ListRuntimeActionSchemasRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRuntimeActionSchemasResponse> {
        T::list_runtime_action_schemas(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_runtime_config(
        &self,
        req: crate::model::GetRuntimeConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::RuntimeConfig> {
        T::get_runtime_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_global_settings(
        &self,
        req: crate::model::GetGlobalSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Settings> {
        T::get_global_settings(self, req, options).await
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
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse> {
        T::test_iam_permissions(self, req, options).await
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

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        T::get_polling_error_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}
