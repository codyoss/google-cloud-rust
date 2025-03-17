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

/// Implements a [Eventarc](super::stubs::Eventarc) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Eventarc<T>
where
    T: super::stubs::Eventarc + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Eventarc<T>
where
    T: super::stubs::Eventarc + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::Eventarc for Eventarc<T>
where
    T: super::stubs::Eventarc + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn get_trigger(
        &self,
        req: crate::model::GetTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Trigger> {
        self.inner.get_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_triggers(
        &self,
        req: crate::model::ListTriggersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTriggersResponse> {
        self.inner.list_triggers(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_trigger(
        &self,
        req: crate::model::CreateTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_trigger(
        &self,
        req: crate::model::UpdateTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_trigger(
        &self,
        req: crate::model::DeleteTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_channel(
        &self,
        req: crate::model::GetChannelRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Channel> {
        self.inner.get_channel(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_channels(
        &self,
        req: crate::model::ListChannelsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListChannelsResponse> {
        self.inner.list_channels(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_channel(
        &self,
        req: crate::model::CreateChannelRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_channel(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_channel(
        &self,
        req: crate::model::UpdateChannelRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_channel(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_channel(
        &self,
        req: crate::model::DeleteChannelRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_channel(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_provider(
        &self,
        req: crate::model::GetProviderRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Provider> {
        self.inner.get_provider(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_providers(
        &self,
        req: crate::model::ListProvidersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListProvidersResponse> {
        self.inner.list_providers(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_channel_connection(
        &self,
        req: crate::model::GetChannelConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ChannelConnection> {
        self.inner.get_channel_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_channel_connections(
        &self,
        req: crate::model::ListChannelConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListChannelConnectionsResponse> {
        self.inner.list_channel_connections(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_channel_connection(
        &self,
        req: crate::model::CreateChannelConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_channel_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_channel_connection(
        &self,
        req: crate::model::DeleteChannelConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_channel_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_google_channel_config(
        &self,
        req: crate::model::GetGoogleChannelConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GoogleChannelConfig> {
        self.inner.get_google_channel_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_google_channel_config(
        &self,
        req: crate::model::UpdateGoogleChannelConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GoogleChannelConfig> {
        self.inner.update_google_channel_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_message_bus(
        &self,
        req: crate::model::GetMessageBusRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::MessageBus> {
        self.inner.get_message_bus(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_message_buses(
        &self,
        req: crate::model::ListMessageBusesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListMessageBusesResponse> {
        self.inner.list_message_buses(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_message_bus_enrollments(
        &self,
        req: crate::model::ListMessageBusEnrollmentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListMessageBusEnrollmentsResponse> {
        self.inner.list_message_bus_enrollments(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_message_bus(
        &self,
        req: crate::model::CreateMessageBusRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_message_bus(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_message_bus(
        &self,
        req: crate::model::UpdateMessageBusRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_message_bus(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_message_bus(
        &self,
        req: crate::model::DeleteMessageBusRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_message_bus(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_enrollment(
        &self,
        req: crate::model::GetEnrollmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Enrollment> {
        self.inner.get_enrollment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_enrollments(
        &self,
        req: crate::model::ListEnrollmentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListEnrollmentsResponse> {
        self.inner.list_enrollments(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_enrollment(
        &self,
        req: crate::model::CreateEnrollmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_enrollment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_enrollment(
        &self,
        req: crate::model::UpdateEnrollmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_enrollment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_enrollment(
        &self,
        req: crate::model::DeleteEnrollmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_enrollment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_pipeline(
        &self,
        req: crate::model::GetPipelineRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Pipeline> {
        self.inner.get_pipeline(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_pipelines(
        &self,
        req: crate::model::ListPipelinesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListPipelinesResponse> {
        self.inner.list_pipelines(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_pipeline(
        &self,
        req: crate::model::CreatePipelineRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_pipeline(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_pipeline(
        &self,
        req: crate::model::UpdatePipelineRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_pipeline(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_pipeline(
        &self,
        req: crate::model::DeletePipelineRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_pipeline(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_google_api_source(
        &self,
        req: crate::model::GetGoogleApiSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GoogleApiSource> {
        self.inner.get_google_api_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_google_api_sources(
        &self,
        req: crate::model::ListGoogleApiSourcesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListGoogleApiSourcesResponse> {
        self.inner.list_google_api_sources(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_google_api_source(
        &self,
        req: crate::model::CreateGoogleApiSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_google_api_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_google_api_source(
        &self,
        req: crate::model::UpdateGoogleApiSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_google_api_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_google_api_source(
        &self,
        req: crate::model::DeleteGoogleApiSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_google_api_source(req, options).await
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

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy> {
        self.inner.get_polling_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
