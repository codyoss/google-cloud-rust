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

/// Defines the trait used to implement [super::client::FunctionService].
///
/// Application developers may need to implement this trait to mock
/// `client::FunctionService`.  In other use-cases, application developers only
/// use `client::FunctionService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait FunctionService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::FunctionService::get_function].
    fn get_function(
        &self,
        _req: crate::model::GetFunctionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Function>> + Send {
        std::future::ready::<crate::Result<crate::model::Function>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::FunctionService::list_functions].
    fn list_functions(
        &self,
        _req: crate::model::ListFunctionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListFunctionsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListFunctionsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::FunctionService::create_function].
    fn create_function(
        &self,
        _req: crate::model::CreateFunctionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::FunctionService::update_function].
    fn update_function(
        &self,
        _req: crate::model::UpdateFunctionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::FunctionService::delete_function].
    fn delete_function(
        &self,
        _req: crate::model::DeleteFunctionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::FunctionService::generate_upload_url].
    fn generate_upload_url(
        &self,
        _req: crate::model::GenerateUploadUrlRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::GenerateUploadUrlResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::GenerateUploadUrlResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::FunctionService::generate_download_url].
    fn generate_download_url(
        &self,
        _req: crate::model::GenerateDownloadUrlRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::GenerateDownloadUrlResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::GenerateDownloadUrlResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::FunctionService::list_runtimes].
    fn list_runtimes(
        &self,
        _req: crate::model::ListRuntimesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListRuntimesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListRuntimesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::FunctionService::list_locations].
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

    /// Implements [super::client::FunctionService::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::FunctionService::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::FunctionService::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::TestIamPermissionsResponse>> + Send
    {
        std::future::ready::<crate::Result<iam_v1::model::TestIamPermissionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::FunctionService::list_operations].
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

    /// Implements [super::client::FunctionService::get_operation].
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
