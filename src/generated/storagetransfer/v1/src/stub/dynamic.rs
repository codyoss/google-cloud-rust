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

/// A dyn-compatible, crate-private version of [super::StorageTransferService].
#[async_trait::async_trait]
pub trait StorageTransferService: std::fmt::Debug + Send + Sync {
    async fn get_google_service_account(
        &self,
        req: crate::model::GetGoogleServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::GoogleServiceAccount>>;

    async fn create_transfer_job(
        &self,
        req: crate::model::CreateTransferJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TransferJob>>;

    async fn update_transfer_job(
        &self,
        req: crate::model::UpdateTransferJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TransferJob>>;

    async fn get_transfer_job(
        &self,
        req: crate::model::GetTransferJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TransferJob>>;

    async fn list_transfer_jobs(
        &self,
        req: crate::model::ListTransferJobsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListTransferJobsResponse>>;

    async fn pause_transfer_operation(
        &self,
        req: crate::model::PauseTransferOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn resume_transfer_operation(
        &self,
        req: crate::model::ResumeTransferOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn run_transfer_job(
        &self,
        req: crate::model::RunTransferJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_transfer_job(
        &self,
        req: crate::model::DeleteTransferJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn create_agent_pool(
        &self,
        req: crate::model::CreateAgentPoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AgentPool>>;

    async fn update_agent_pool(
        &self,
        req: crate::model::UpdateAgentPoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AgentPool>>;

    async fn get_agent_pool(
        &self,
        req: crate::model::GetAgentPoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AgentPool>>;

    async fn list_agent_pools(
        &self,
        req: crate::model::ListAgentPoolsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListAgentPoolsResponse>>;

    async fn delete_agent_pool(
        &self,
        req: crate::model::DeleteAgentPoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

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

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
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

/// All implementations of [super::StorageTransferService] also implement [StorageTransferService].
#[async_trait::async_trait]
impl<T: super::StorageTransferService> StorageTransferService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn get_google_service_account(
        &self,
        req: crate::model::GetGoogleServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::GoogleServiceAccount>> {
        T::get_google_service_account(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_transfer_job(
        &self,
        req: crate::model::CreateTransferJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TransferJob>> {
        T::create_transfer_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_transfer_job(
        &self,
        req: crate::model::UpdateTransferJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TransferJob>> {
        T::update_transfer_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_transfer_job(
        &self,
        req: crate::model::GetTransferJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TransferJob>> {
        T::get_transfer_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_transfer_jobs(
        &self,
        req: crate::model::ListTransferJobsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListTransferJobsResponse>> {
        T::list_transfer_jobs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn pause_transfer_operation(
        &self,
        req: crate::model::PauseTransferOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::pause_transfer_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn resume_transfer_operation(
        &self,
        req: crate::model::ResumeTransferOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::resume_transfer_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn run_transfer_job(
        &self,
        req: crate::model::RunTransferJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::run_transfer_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_transfer_job(
        &self,
        req: crate::model::DeleteTransferJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_transfer_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_agent_pool(
        &self,
        req: crate::model::CreateAgentPoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AgentPool>> {
        T::create_agent_pool(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_agent_pool(
        &self,
        req: crate::model::UpdateAgentPoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AgentPool>> {
        T::update_agent_pool(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_agent_pool(
        &self,
        req: crate::model::GetAgentPoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AgentPool>> {
        T::get_agent_pool(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_agent_pools(
        &self,
        req: crate::model::ListAgentPoolsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListAgentPoolsResponse>> {
        T::list_agent_pools(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_agent_pool(
        &self,
        req: crate::model::DeleteAgentPoolRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_agent_pool(self, req, options).await
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
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
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
