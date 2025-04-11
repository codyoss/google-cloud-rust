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

/// Defines the trait used to implement [super::client::PrivilegedAccessManager].
///
/// Application developers may need to implement this trait to mock
/// `client::PrivilegedAccessManager`.  In other use-cases, application developers only
/// use `client::PrivilegedAccessManager` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait PrivilegedAccessManager: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::PrivilegedAccessManager::check_onboarding_status].
    fn check_onboarding_status(
        &self,
        _req: crate::model::CheckOnboardingStatusRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<
            gax::response::Response<crate::model::CheckOnboardingStatusResponse>,
        >,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::CheckOnboardingStatusResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::PrivilegedAccessManager::list_entitlements].
    fn list_entitlements(
        &self,
        _req: crate::model::ListEntitlementsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListEntitlementsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListEntitlementsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::PrivilegedAccessManager::search_entitlements].
    fn search_entitlements(
        &self,
        _req: crate::model::SearchEntitlementsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::SearchEntitlementsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::SearchEntitlementsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::PrivilegedAccessManager::get_entitlement].
    fn get_entitlement(
        &self,
        _req: crate::model::GetEntitlementRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Entitlement>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Entitlement>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::PrivilegedAccessManager::create_entitlement].
    fn create_entitlement(
        &self,
        _req: crate::model::CreateEntitlementRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::PrivilegedAccessManager::delete_entitlement].
    fn delete_entitlement(
        &self,
        _req: crate::model::DeleteEntitlementRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::PrivilegedAccessManager::update_entitlement].
    fn update_entitlement(
        &self,
        _req: crate::model::UpdateEntitlementRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::PrivilegedAccessManager::list_grants].
    fn list_grants(
        &self,
        _req: crate::model::ListGrantsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListGrantsResponse>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::ListGrantsResponse>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::PrivilegedAccessManager::search_grants].
    fn search_grants(
        &self,
        _req: crate::model::SearchGrantsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::SearchGrantsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::SearchGrantsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::PrivilegedAccessManager::get_grant].
    fn get_grant(
        &self,
        _req: crate::model::GetGrantRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Grant>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Grant>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::PrivilegedAccessManager::create_grant].
    fn create_grant(
        &self,
        _req: crate::model::CreateGrantRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Grant>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Grant>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::PrivilegedAccessManager::approve_grant].
    fn approve_grant(
        &self,
        _req: crate::model::ApproveGrantRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Grant>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Grant>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::PrivilegedAccessManager::deny_grant].
    fn deny_grant(
        &self,
        _req: crate::model::DenyGrantRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Grant>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Grant>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::PrivilegedAccessManager::revoke_grant].
    fn revoke_grant(
        &self,
        _req: crate::model::RevokeGrantRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::PrivilegedAccessManager::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<location::model::ListLocationsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<location::model::ListLocationsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::PrivilegedAccessManager::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<location::model::Location>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<location::model::Location>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::PrivilegedAccessManager::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::PrivilegedAccessManager::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::PrivilegedAccessManager::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
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
