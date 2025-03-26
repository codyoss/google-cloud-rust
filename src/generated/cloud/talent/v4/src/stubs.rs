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

/// Defines the trait used to implement [super::client::CompanyService].
///
/// Application developers may need to implement this trait to mock
/// `client::CompanyService`.  In other use-cases, application developers only
/// use `client::CompanyService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait CompanyService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::CompanyService::create_company].
    fn create_company(
        &self,
        _req: crate::model::CreateCompanyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Company>> + Send {
        std::future::ready::<crate::Result<crate::model::Company>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CompanyService::get_company].
    fn get_company(
        &self,
        _req: crate::model::GetCompanyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Company>> + Send {
        std::future::ready::<crate::Result<crate::model::Company>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CompanyService::update_company].
    fn update_company(
        &self,
        _req: crate::model::UpdateCompanyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Company>> + Send {
        std::future::ready::<crate::Result<crate::model::Company>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CompanyService::delete_company].
    fn delete_company(
        &self,
        _req: crate::model::DeleteCompanyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::CompanyService::list_companies].
    fn list_companies(
        &self,
        _req: crate::model::ListCompaniesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListCompaniesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListCompaniesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CompanyService::get_operation].
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
}

/// Defines the trait used to implement [super::client::Completion].
///
/// Application developers may need to implement this trait to mock
/// `client::Completion`.  In other use-cases, application developers only
/// use `client::Completion` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait Completion: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::Completion::complete_query].
    fn complete_query(
        &self,
        _req: crate::model::CompleteQueryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::CompleteQueryResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::CompleteQueryResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Completion::get_operation].
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
}

/// Defines the trait used to implement [super::client::EventService].
///
/// Application developers may need to implement this trait to mock
/// `client::EventService`.  In other use-cases, application developers only
/// use `client::EventService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait EventService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::EventService::create_client_event].
    fn create_client_event(
        &self,
        _req: crate::model::CreateClientEventRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ClientEvent>> + Send {
        std::future::ready::<crate::Result<crate::model::ClientEvent>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EventService::get_operation].
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
}

/// Defines the trait used to implement [super::client::JobService].
///
/// Application developers may need to implement this trait to mock
/// `client::JobService`.  In other use-cases, application developers only
/// use `client::JobService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait JobService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::JobService::create_job].
    fn create_job(
        &self,
        _req: crate::model::CreateJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Job>> + Send {
        std::future::ready::<crate::Result<crate::model::Job>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::JobService::batch_create_jobs].
    fn batch_create_jobs(
        &self,
        _req: crate::model::BatchCreateJobsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::JobService::get_job].
    fn get_job(
        &self,
        _req: crate::model::GetJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Job>> + Send {
        std::future::ready::<crate::Result<crate::model::Job>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::JobService::update_job].
    fn update_job(
        &self,
        _req: crate::model::UpdateJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Job>> + Send {
        std::future::ready::<crate::Result<crate::model::Job>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::JobService::batch_update_jobs].
    fn batch_update_jobs(
        &self,
        _req: crate::model::BatchUpdateJobsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::JobService::delete_job].
    fn delete_job(
        &self,
        _req: crate::model::DeleteJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::JobService::batch_delete_jobs].
    fn batch_delete_jobs(
        &self,
        _req: crate::model::BatchDeleteJobsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::JobService::list_jobs].
    fn list_jobs(
        &self,
        _req: crate::model::ListJobsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListJobsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListJobsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::JobService::search_jobs].
    fn search_jobs(
        &self,
        _req: crate::model::SearchJobsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SearchJobsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::SearchJobsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::JobService::search_jobs_for_alert].
    fn search_jobs_for_alert(
        &self,
        _req: crate::model::SearchJobsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SearchJobsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::SearchJobsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::JobService::get_operation].
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

/// Defines the trait used to implement [super::client::TenantService].
///
/// Application developers may need to implement this trait to mock
/// `client::TenantService`.  In other use-cases, application developers only
/// use `client::TenantService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait TenantService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::TenantService::create_tenant].
    fn create_tenant(
        &self,
        _req: crate::model::CreateTenantRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Tenant>> + Send {
        std::future::ready::<crate::Result<crate::model::Tenant>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::TenantService::get_tenant].
    fn get_tenant(
        &self,
        _req: crate::model::GetTenantRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Tenant>> + Send {
        std::future::ready::<crate::Result<crate::model::Tenant>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::TenantService::update_tenant].
    fn update_tenant(
        &self,
        _req: crate::model::UpdateTenantRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Tenant>> + Send {
        std::future::ready::<crate::Result<crate::model::Tenant>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::TenantService::delete_tenant].
    fn delete_tenant(
        &self,
        _req: crate::model::DeleteTenantRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::TenantService::list_tenants].
    fn list_tenants(
        &self,
        _req: crate::model::ListTenantsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListTenantsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListTenantsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::TenantService::get_operation].
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
}
