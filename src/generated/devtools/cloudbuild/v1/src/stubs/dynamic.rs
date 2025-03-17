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

/// A dyn-compatible, crate-private version of [super::CloudBuild].
#[async_trait::async_trait]
pub trait CloudBuild: std::fmt::Debug + Send + Sync {
    async fn create_build(
        &self,
        req: crate::model::CreateBuildRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_build(
        &self,
        req: crate::model::GetBuildRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Build>;

    async fn list_builds(
        &self,
        req: crate::model::ListBuildsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBuildsResponse>;

    async fn cancel_build(
        &self,
        req: crate::model::CancelBuildRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Build>;

    async fn retry_build(
        &self,
        req: crate::model::RetryBuildRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn approve_build(
        &self,
        req: crate::model::ApproveBuildRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_build_trigger(
        &self,
        req: crate::model::CreateBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BuildTrigger>;

    async fn get_build_trigger(
        &self,
        req: crate::model::GetBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BuildTrigger>;

    async fn list_build_triggers(
        &self,
        req: crate::model::ListBuildTriggersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBuildTriggersResponse>;

    async fn delete_build_trigger(
        &self,
        req: crate::model::DeleteBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn update_build_trigger(
        &self,
        req: crate::model::UpdateBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BuildTrigger>;

    async fn run_build_trigger(
        &self,
        req: crate::model::RunBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn receive_trigger_webhook(
        &self,
        req: crate::model::ReceiveTriggerWebhookRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ReceiveTriggerWebhookResponse>;

    async fn create_worker_pool(
        &self,
        req: crate::model::CreateWorkerPoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_worker_pool(
        &self,
        req: crate::model::GetWorkerPoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::WorkerPool>;

    async fn delete_worker_pool(
        &self,
        req: crate::model::DeleteWorkerPoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_worker_pool(
        &self,
        req: crate::model::UpdateWorkerPoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_worker_pools(
        &self,
        req: crate::model::ListWorkerPoolsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListWorkerPoolsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

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

/// All implementations of [super::CloudBuild] also implement [CloudBuild].
#[async_trait::async_trait]
impl<T: super::CloudBuild> CloudBuild for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_build(
        &self,
        req: crate::model::CreateBuildRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_build(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_build(
        &self,
        req: crate::model::GetBuildRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Build> {
        T::get_build(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_builds(
        &self,
        req: crate::model::ListBuildsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBuildsResponse> {
        T::list_builds(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_build(
        &self,
        req: crate::model::CancelBuildRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Build> {
        T::cancel_build(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn retry_build(
        &self,
        req: crate::model::RetryBuildRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::retry_build(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn approve_build(
        &self,
        req: crate::model::ApproveBuildRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::approve_build(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_build_trigger(
        &self,
        req: crate::model::CreateBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BuildTrigger> {
        T::create_build_trigger(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_build_trigger(
        &self,
        req: crate::model::GetBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BuildTrigger> {
        T::get_build_trigger(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_build_triggers(
        &self,
        req: crate::model::ListBuildTriggersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBuildTriggersResponse> {
        T::list_build_triggers(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_build_trigger(
        &self,
        req: crate::model::DeleteBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_build_trigger(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_build_trigger(
        &self,
        req: crate::model::UpdateBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BuildTrigger> {
        T::update_build_trigger(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn run_build_trigger(
        &self,
        req: crate::model::RunBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::run_build_trigger(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn receive_trigger_webhook(
        &self,
        req: crate::model::ReceiveTriggerWebhookRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ReceiveTriggerWebhookResponse> {
        T::receive_trigger_webhook(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_worker_pool(
        &self,
        req: crate::model::CreateWorkerPoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_worker_pool(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_worker_pool(
        &self,
        req: crate::model::GetWorkerPoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::WorkerPool> {
        T::get_worker_pool(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_worker_pool(
        &self,
        req: crate::model::DeleteWorkerPoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_worker_pool(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_worker_pool(
        &self,
        req: crate::model::UpdateWorkerPoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_worker_pool(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_worker_pools(
        &self,
        req: crate::model::ListWorkerPoolsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListWorkerPoolsResponse> {
        T::list_worker_pools(self, req, options).await
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
