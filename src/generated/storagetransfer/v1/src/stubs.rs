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

/// Defines the trait used to implement [super::client::StorageTransferService].
///
/// Application developers may need to implement this trait to mock
/// `client::StorageTransferService`.  In other use-cases, application developers only
/// use `client::StorageTransferService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait StorageTransferService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::StorageTransferService::get_google_service_account].
    fn get_google_service_account(
        &self,
        _req: crate::model::GetGoogleServiceAccountRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::GoogleServiceAccount>> + Send
    {
        std::future::ready::<crate::Result<crate::model::GoogleServiceAccount>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::StorageTransferService::create_transfer_job].
    fn create_transfer_job(
        &self,
        _req: crate::model::CreateTransferJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TransferJob>> + Send {
        std::future::ready::<crate::Result<crate::model::TransferJob>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::StorageTransferService::update_transfer_job].
    fn update_transfer_job(
        &self,
        _req: crate::model::UpdateTransferJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TransferJob>> + Send {
        std::future::ready::<crate::Result<crate::model::TransferJob>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::StorageTransferService::get_transfer_job].
    fn get_transfer_job(
        &self,
        _req: crate::model::GetTransferJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TransferJob>> + Send {
        std::future::ready::<crate::Result<crate::model::TransferJob>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::StorageTransferService::list_transfer_jobs].
    fn list_transfer_jobs(
        &self,
        _req: crate::model::ListTransferJobsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListTransferJobsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListTransferJobsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::StorageTransferService::pause_transfer_operation].
    fn pause_transfer_operation(
        &self,
        _req: crate::model::PauseTransferOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::StorageTransferService::resume_transfer_operation].
    fn resume_transfer_operation(
        &self,
        _req: crate::model::ResumeTransferOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::StorageTransferService::run_transfer_job].
    fn run_transfer_job(
        &self,
        _req: crate::model::RunTransferJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::StorageTransferService::delete_transfer_job].
    fn delete_transfer_job(
        &self,
        _req: crate::model::DeleteTransferJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::StorageTransferService::create_agent_pool].
    fn create_agent_pool(
        &self,
        _req: crate::model::CreateAgentPoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AgentPool>> + Send {
        std::future::ready::<crate::Result<crate::model::AgentPool>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::StorageTransferService::update_agent_pool].
    fn update_agent_pool(
        &self,
        _req: crate::model::UpdateAgentPoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AgentPool>> + Send {
        std::future::ready::<crate::Result<crate::model::AgentPool>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::StorageTransferService::get_agent_pool].
    fn get_agent_pool(
        &self,
        _req: crate::model::GetAgentPoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AgentPool>> + Send {
        std::future::ready::<crate::Result<crate::model::AgentPool>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::StorageTransferService::list_agent_pools].
    fn list_agent_pools(
        &self,
        _req: crate::model::ListAgentPoolsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListAgentPoolsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListAgentPoolsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::StorageTransferService::delete_agent_pool].
    fn delete_agent_pool(
        &self,
        _req: crate::model::DeleteAgentPoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::StorageTransferService::list_operations].
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

    /// Implements [super::client::StorageTransferService::get_operation].
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

    /// Implements [super::client::StorageTransferService::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Returns the polling policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        Arc::new(gax::polling_policy::Aip194Strict)
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
