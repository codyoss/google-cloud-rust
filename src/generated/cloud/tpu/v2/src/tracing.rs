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

/// Implements a [Tpu](super::stubs::Tpu) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Tpu<T>
where
    T: super::stubs::Tpu + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Tpu<T>
where
    T: super::stubs::Tpu + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::Tpu for Tpu<T>
where
    T: super::stubs::Tpu + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_nodes(
        &self,
        req: crate::model::ListNodesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListNodesResponse> {
        self.inner.list_nodes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_node(
        &self,
        req: crate::model::GetNodeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Node> {
        self.inner.get_node(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_node(
        &self,
        req: crate::model::CreateNodeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_node(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_node(
        &self,
        req: crate::model::DeleteNodeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_node(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn stop_node(
        &self,
        req: crate::model::StopNodeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.stop_node(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn start_node(
        &self,
        req: crate::model::StartNodeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.start_node(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_node(
        &self,
        req: crate::model::UpdateNodeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_node(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_queued_resources(
        &self,
        req: crate::model::ListQueuedResourcesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListQueuedResourcesResponse> {
        self.inner.list_queued_resources(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_queued_resource(
        &self,
        req: crate::model::GetQueuedResourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::QueuedResource> {
        self.inner.get_queued_resource(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_queued_resource(
        &self,
        req: crate::model::CreateQueuedResourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_queued_resource(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_queued_resource(
        &self,
        req: crate::model::DeleteQueuedResourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_queued_resource(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn reset_queued_resource(
        &self,
        req: crate::model::ResetQueuedResourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.reset_queued_resource(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn generate_service_identity(
        &self,
        req: crate::model::GenerateServiceIdentityRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GenerateServiceIdentityResponse> {
        self.inner.generate_service_identity(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_accelerator_types(
        &self,
        req: crate::model::ListAcceleratorTypesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAcceleratorTypesResponse> {
        self.inner.list_accelerator_types(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_accelerator_type(
        &self,
        req: crate::model::GetAcceleratorTypeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AcceleratorType> {
        self.inner.get_accelerator_type(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_runtime_versions(
        &self,
        req: crate::model::ListRuntimeVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListRuntimeVersionsResponse> {
        self.inner.list_runtime_versions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_runtime_version(
        &self,
        req: crate::model::GetRuntimeVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::RuntimeVersion> {
        self.inner.get_runtime_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_guest_attributes(
        &self,
        req: crate::model::GetGuestAttributesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GetGuestAttributesResponse> {
        self.inner.get_guest_attributes(req, options).await
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
