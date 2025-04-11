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

/// Defines the trait used to implement [super::client::Policies].
///
/// Application developers may need to implement this trait to mock
/// `client::Policies`.  In other use-cases, application developers only
/// use `client::Policies` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait Policies: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::Policies::list_policies].
    fn list_policies(
        &self,
        _req: crate::model::ListPoliciesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListPoliciesResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListPoliciesResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Policies::get_policy].
    fn get_policy(
        &self,
        _req: crate::model::GetPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Policy>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Policy>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Policies::create_policy].
    fn create_policy(
        &self,
        _req: crate::model::CreatePolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Policies::update_policy].
    fn update_policy(
        &self,
        _req: crate::model::UpdatePolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Policies::delete_policy].
    fn delete_policy(
        &self,
        _req: crate::model::DeletePolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Policies::get_operation].
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
