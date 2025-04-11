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

/// A dyn-compatible, crate-private version of [super::Workflows].
#[async_trait::async_trait]
pub trait Workflows: std::fmt::Debug + Send + Sync {
    async fn list_workflows(
        &self,
        req: crate::model::ListWorkflowsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListWorkflowsResponse>>;

    async fn get_workflow(
        &self,
        req: crate::model::GetWorkflowRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Workflow>>;

    async fn create_workflow(
        &self,
        req: crate::model::CreateWorkflowRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_workflow(
        &self,
        req: crate::model::DeleteWorkflowRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn update_workflow(
        &self,
        req: crate::model::UpdateWorkflowRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn list_workflow_revisions(
        &self,
        req: crate::model::ListWorkflowRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListWorkflowRevisionsResponse>>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::ListLocationsResponse>>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::Location>>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::Workflows] also implement [Workflows].
#[async_trait::async_trait]
impl<T: super::Workflows> Workflows for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_workflows(
        &self,
        req: crate::model::ListWorkflowsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListWorkflowsResponse>> {
        T::list_workflows(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_workflow(
        &self,
        req: crate::model::GetWorkflowRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Workflow>> {
        T::get_workflow(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_workflow(
        &self,
        req: crate::model::CreateWorkflowRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_workflow(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_workflow(
        &self,
        req: crate::model::DeleteWorkflowRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_workflow(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_workflow(
        &self,
        req: crate::model::UpdateWorkflowRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::update_workflow(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_workflow_revisions(
        &self,
        req: crate::model::ListWorkflowRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListWorkflowRevisionsResponse>> {
        T::list_workflow_revisions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::ListLocationsResponse>> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::Location>> {
        T::get_location(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_operation(self, req, options).await
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
