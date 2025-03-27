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

//! Traits to mock the clients in this library.
//!
//! Application developers may need to mock the clients in this library to test
//! how their application works with different (and sometimes hard to trigger)
//! client and service behavior. Such test can define mocks implementing the
//! trait(s) defined in this module, initialize the client with an instance of
//! this mock in their tests, and verify their application responds as expected.

#![allow(rustdoc::broken_intra_doc_links)]

use gax::error::Error;
use std::sync::Arc;

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::Connectors].
///
/// Application developers may need to implement this trait to mock
/// `client::Connectors`.  In other use-cases, application developers only
/// use `client::Connectors` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait Connectors: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::Connectors::list_connections].
    fn list_connections(
        &self,
        _req: crate::model::ListConnectionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListConnectionsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListConnectionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Connectors::get_connection].
    fn get_connection(
        &self,
        _req: crate::model::GetConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Connection>> + Send {
        std::future::ready::<crate::Result<crate::model::Connection>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Connectors::create_connection].
    fn create_connection(
        &self,
        _req: crate::model::CreateConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Connectors::update_connection].
    fn update_connection(
        &self,
        _req: crate::model::UpdateConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Connectors::delete_connection].
    fn delete_connection(
        &self,
        _req: crate::model::DeleteConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Connectors::list_providers].
    fn list_providers(
        &self,
        _req: crate::model::ListProvidersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListProvidersResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListProvidersResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Connectors::get_provider].
    fn get_provider(
        &self,
        _req: crate::model::GetProviderRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Provider>> + Send {
        std::future::ready::<crate::Result<crate::model::Provider>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Connectors::list_connectors].
    fn list_connectors(
        &self,
        _req: crate::model::ListConnectorsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListConnectorsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListConnectorsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Connectors::get_connector].
    fn get_connector(
        &self,
        _req: crate::model::GetConnectorRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Connector>> + Send {
        std::future::ready::<crate::Result<crate::model::Connector>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Connectors::list_connector_versions].
    fn list_connector_versions(
        &self,
        _req: crate::model::ListConnectorVersionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListConnectorVersionsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListConnectorVersionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Connectors::get_connector_version].
    fn get_connector_version(
        &self,
        _req: crate::model::GetConnectorVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ConnectorVersion>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ConnectorVersion>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Connectors::get_connection_schema_metadata].
    fn get_connection_schema_metadata(
        &self,
        _req: crate::model::GetConnectionSchemaMetadataRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ConnectionSchemaMetadata>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ConnectionSchemaMetadata>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Connectors::refresh_connection_schema_metadata].
    fn refresh_connection_schema_metadata(
        &self,
        _req: crate::model::RefreshConnectionSchemaMetadataRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Connectors::list_runtime_entity_schemas].
    fn list_runtime_entity_schemas(
        &self,
        _req: crate::model::ListRuntimeEntitySchemasRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListRuntimeEntitySchemasResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListRuntimeEntitySchemasResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Connectors::list_runtime_action_schemas].
    fn list_runtime_action_schemas(
        &self,
        _req: crate::model::ListRuntimeActionSchemasRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListRuntimeActionSchemasResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListRuntimeActionSchemasResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Connectors::get_runtime_config].
    fn get_runtime_config(
        &self,
        _req: crate::model::GetRuntimeConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::RuntimeConfig>> + Send {
        std::future::ready::<crate::Result<crate::model::RuntimeConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Connectors::get_global_settings].
    fn get_global_settings(
        &self,
        _req: crate::model::GetGlobalSettingsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Settings>> + Send {
        std::future::ready::<crate::Result<crate::model::Settings>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Connectors::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::ListLocationsResponse>> + Send
    {
        std::future::ready::<crate::Result<location::model::ListLocationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Connectors::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Connectors::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Connectors::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Connectors::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::TestIamPermissionsResponse>> + Send
    {
        std::future::ready::<crate::Result<iam_v1::model::TestIamPermissionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Connectors::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
    + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Connectors::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Connectors::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Connectors::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }

    /// Returns the polling error policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_error_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        Arc::new(gax::polling_error_policy::Aip194Strict)
    }

    /// Returns the polling backoff policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_backoff_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        Arc::new(gax::exponential_backoff::ExponentialBackoff::default())
    }
}
