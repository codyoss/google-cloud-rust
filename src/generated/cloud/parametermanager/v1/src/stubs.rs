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

/// Defines the trait used to implement [super::client::ParameterManager].
///
/// Application developers may need to implement this trait to mock
/// `client::ParameterManager`.  In other use-cases, application developers only
/// use `client::ParameterManager` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait ParameterManager: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::ParameterManager::list_parameters].
    fn list_parameters(
        &self,
        _req: crate::model::ListParametersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListParametersResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListParametersResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ParameterManager::get_parameter].
    fn get_parameter(
        &self,
        _req: crate::model::GetParameterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Parameter>> + Send {
        std::future::ready::<crate::Result<crate::model::Parameter>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ParameterManager::create_parameter].
    fn create_parameter(
        &self,
        _req: crate::model::CreateParameterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Parameter>> + Send {
        std::future::ready::<crate::Result<crate::model::Parameter>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ParameterManager::update_parameter].
    fn update_parameter(
        &self,
        _req: crate::model::UpdateParameterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Parameter>> + Send {
        std::future::ready::<crate::Result<crate::model::Parameter>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ParameterManager::delete_parameter].
    fn delete_parameter(
        &self,
        _req: crate::model::DeleteParameterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::ParameterManager::list_parameter_versions].
    fn list_parameter_versions(
        &self,
        _req: crate::model::ListParameterVersionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListParameterVersionsResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::ListParameterVersionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ParameterManager::get_parameter_version].
    fn get_parameter_version(
        &self,
        _req: crate::model::GetParameterVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ParameterVersion>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ParameterVersion>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ParameterManager::render_parameter_version].
    fn render_parameter_version(
        &self,
        _req: crate::model::RenderParameterVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::RenderParameterVersionResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::RenderParameterVersionResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ParameterManager::create_parameter_version].
    fn create_parameter_version(
        &self,
        _req: crate::model::CreateParameterVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ParameterVersion>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ParameterVersion>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ParameterManager::update_parameter_version].
    fn update_parameter_version(
        &self,
        _req: crate::model::UpdateParameterVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ParameterVersion>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ParameterVersion>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ParameterManager::delete_parameter_version].
    fn delete_parameter_version(
        &self,
        _req: crate::model::DeleteParameterVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::ParameterManager::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::ListLocationsResponse>> + Send
    {
        std::future::ready::<crate::Result<location::model::ListLocationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ParameterManager::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }
}
