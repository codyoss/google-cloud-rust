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

/// A dyn-compatible, crate-private version of [super::Datastream].
#[async_trait::async_trait]
pub trait Datastream: std::fmt::Debug + Send + Sync {
    async fn list_connection_profiles(
        &self,
        req: crate::model::ListConnectionProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConnectionProfilesResponse>;

    async fn get_connection_profile(
        &self,
        req: crate::model::GetConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ConnectionProfile>;

    async fn create_connection_profile(
        &self,
        req: crate::model::CreateConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_connection_profile(
        &self,
        req: crate::model::UpdateConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_connection_profile(
        &self,
        req: crate::model::DeleteConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn discover_connection_profile(
        &self,
        req: crate::model::DiscoverConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DiscoverConnectionProfileResponse>;

    async fn list_streams(
        &self,
        req: crate::model::ListStreamsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListStreamsResponse>;

    async fn get_stream(
        &self,
        req: crate::model::GetStreamRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Stream>;

    async fn create_stream(
        &self,
        req: crate::model::CreateStreamRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_stream(
        &self,
        req: crate::model::UpdateStreamRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_stream(
        &self,
        req: crate::model::DeleteStreamRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn run_stream(
        &self,
        req: crate::model::RunStreamRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_stream_object(
        &self,
        req: crate::model::GetStreamObjectRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::StreamObject>;

    async fn lookup_stream_object(
        &self,
        req: crate::model::LookupStreamObjectRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::StreamObject>;

    async fn list_stream_objects(
        &self,
        req: crate::model::ListStreamObjectsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListStreamObjectsResponse>;

    async fn start_backfill_job(
        &self,
        req: crate::model::StartBackfillJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::StartBackfillJobResponse>;

    async fn stop_backfill_job(
        &self,
        req: crate::model::StopBackfillJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::StopBackfillJobResponse>;

    async fn fetch_static_ips(
        &self,
        req: crate::model::FetchStaticIpsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FetchStaticIpsResponse>;

    async fn create_private_connection(
        &self,
        req: crate::model::CreatePrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_private_connection(
        &self,
        req: crate::model::GetPrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PrivateConnection>;

    async fn list_private_connections(
        &self,
        req: crate::model::ListPrivateConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPrivateConnectionsResponse>;

    async fn delete_private_connection(
        &self,
        req: crate::model::DeletePrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_route(
        &self,
        req: crate::model::CreateRouteRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_route(
        &self,
        req: crate::model::GetRouteRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Route>;

    async fn list_routes(
        &self,
        req: crate::model::ListRoutesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRoutesResponse>;

    async fn delete_route(
        &self,
        req: crate::model::DeleteRouteRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

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

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::Datastream] also implement [Datastream].
#[async_trait::async_trait]
impl<T: super::Datastream> Datastream for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_connection_profiles(
        &self,
        req: crate::model::ListConnectionProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListConnectionProfilesResponse> {
        T::list_connection_profiles(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_connection_profile(
        &self,
        req: crate::model::GetConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ConnectionProfile> {
        T::get_connection_profile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_connection_profile(
        &self,
        req: crate::model::CreateConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_connection_profile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_connection_profile(
        &self,
        req: crate::model::UpdateConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_connection_profile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_connection_profile(
        &self,
        req: crate::model::DeleteConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_connection_profile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn discover_connection_profile(
        &self,
        req: crate::model::DiscoverConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DiscoverConnectionProfileResponse> {
        T::discover_connection_profile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_streams(
        &self,
        req: crate::model::ListStreamsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListStreamsResponse> {
        T::list_streams(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_stream(
        &self,
        req: crate::model::GetStreamRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Stream> {
        T::get_stream(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_stream(
        &self,
        req: crate::model::CreateStreamRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_stream(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_stream(
        &self,
        req: crate::model::UpdateStreamRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_stream(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_stream(
        &self,
        req: crate::model::DeleteStreamRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_stream(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn run_stream(
        &self,
        req: crate::model::RunStreamRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::run_stream(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_stream_object(
        &self,
        req: crate::model::GetStreamObjectRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::StreamObject> {
        T::get_stream_object(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn lookup_stream_object(
        &self,
        req: crate::model::LookupStreamObjectRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::StreamObject> {
        T::lookup_stream_object(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_stream_objects(
        &self,
        req: crate::model::ListStreamObjectsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListStreamObjectsResponse> {
        T::list_stream_objects(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn start_backfill_job(
        &self,
        req: crate::model::StartBackfillJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::StartBackfillJobResponse> {
        T::start_backfill_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn stop_backfill_job(
        &self,
        req: crate::model::StopBackfillJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::StopBackfillJobResponse> {
        T::stop_backfill_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn fetch_static_ips(
        &self,
        req: crate::model::FetchStaticIpsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FetchStaticIpsResponse> {
        T::fetch_static_ips(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_private_connection(
        &self,
        req: crate::model::CreatePrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_private_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_private_connection(
        &self,
        req: crate::model::GetPrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PrivateConnection> {
        T::get_private_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_private_connections(
        &self,
        req: crate::model::ListPrivateConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPrivateConnectionsResponse> {
        T::list_private_connections(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_private_connection(
        &self,
        req: crate::model::DeletePrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_private_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_route(
        &self,
        req: crate::model::CreateRouteRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_route(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_route(
        &self,
        req: crate::model::GetRouteRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Route> {
        T::get_route(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_routes(
        &self,
        req: crate::model::ListRoutesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRoutesResponse> {
        T::list_routes(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_route(
        &self,
        req: crate::model::DeleteRouteRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_route(self, req, options).await
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
