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

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::EssentialContactsService].
///
/// Application developers may need to implement this trait to mock
/// `client::EssentialContactsService`.  In other use-cases, application developers only
/// use `client::EssentialContactsService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait EssentialContactsService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::EssentialContactsService::create_contact].
    fn create_contact(
        &self,
        _req: crate::model::CreateContactRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Contact>> + Send {
        std::future::ready::<crate::Result<crate::model::Contact>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EssentialContactsService::update_contact].
    fn update_contact(
        &self,
        _req: crate::model::UpdateContactRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Contact>> + Send {
        std::future::ready::<crate::Result<crate::model::Contact>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EssentialContactsService::list_contacts].
    fn list_contacts(
        &self,
        _req: crate::model::ListContactsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListContactsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListContactsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EssentialContactsService::get_contact].
    fn get_contact(
        &self,
        _req: crate::model::GetContactRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Contact>> + Send {
        std::future::ready::<crate::Result<crate::model::Contact>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EssentialContactsService::delete_contact].
    fn delete_contact(
        &self,
        _req: crate::model::DeleteContactRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::EssentialContactsService::compute_contacts].
    fn compute_contacts(
        &self,
        _req: crate::model::ComputeContactsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ComputeContactsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ComputeContactsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::EssentialContactsService::send_test_message].
    fn send_test_message(
        &self,
        _req: crate::model::SendTestMessageRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }
}
