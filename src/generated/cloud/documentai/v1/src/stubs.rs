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

/// Defines the trait used to implement [super::client::DocumentProcessorService].
///
/// Application developers may need to implement this trait to mock
/// `client::DocumentProcessorService`.  In other use-cases, application developers only
/// use `client::DocumentProcessorService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait DocumentProcessorService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::DocumentProcessorService::process_document].
    fn process_document(
        &self,
        _req: crate::model::ProcessRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ProcessResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ProcessResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::batch_process_documents].
    fn batch_process_documents(
        &self,
        _req: crate::model::BatchProcessRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::fetch_processor_types].
    fn fetch_processor_types(
        &self,
        _req: crate::model::FetchProcessorTypesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::FetchProcessorTypesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::FetchProcessorTypesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::DocumentProcessorService::list_processor_types].
    fn list_processor_types(
        &self,
        _req: crate::model::ListProcessorTypesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListProcessorTypesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListProcessorTypesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::DocumentProcessorService::get_processor_type].
    fn get_processor_type(
        &self,
        _req: crate::model::GetProcessorTypeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ProcessorType>> + Send {
        std::future::ready::<crate::Result<crate::model::ProcessorType>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::list_processors].
    fn list_processors(
        &self,
        _req: crate::model::ListProcessorsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListProcessorsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListProcessorsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::DocumentProcessorService::get_processor].
    fn get_processor(
        &self,
        _req: crate::model::GetProcessorRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Processor>> + Send {
        std::future::ready::<crate::Result<crate::model::Processor>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::train_processor_version].
    fn train_processor_version(
        &self,
        _req: crate::model::TrainProcessorVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::get_processor_version].
    fn get_processor_version(
        &self,
        _req: crate::model::GetProcessorVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ProcessorVersion>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ProcessorVersion>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::list_processor_versions].
    fn list_processor_versions(
        &self,
        _req: crate::model::ListProcessorVersionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListProcessorVersionsResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::ListProcessorVersionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::DocumentProcessorService::delete_processor_version].
    fn delete_processor_version(
        &self,
        _req: crate::model::DeleteProcessorVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::deploy_processor_version].
    fn deploy_processor_version(
        &self,
        _req: crate::model::DeployProcessorVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::undeploy_processor_version].
    fn undeploy_processor_version(
        &self,
        _req: crate::model::UndeployProcessorVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::create_processor].
    fn create_processor(
        &self,
        _req: crate::model::CreateProcessorRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Processor>> + Send {
        std::future::ready::<crate::Result<crate::model::Processor>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::delete_processor].
    fn delete_processor(
        &self,
        _req: crate::model::DeleteProcessorRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::enable_processor].
    fn enable_processor(
        &self,
        _req: crate::model::EnableProcessorRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::disable_processor].
    fn disable_processor(
        &self,
        _req: crate::model::DisableProcessorRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::set_default_processor_version].
    fn set_default_processor_version(
        &self,
        _req: crate::model::SetDefaultProcessorVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::review_document].
    fn review_document(
        &self,
        _req: crate::model::ReviewDocumentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::evaluate_processor_version].
    fn evaluate_processor_version(
        &self,
        _req: crate::model::EvaluateProcessorVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::get_evaluation].
    fn get_evaluation(
        &self,
        _req: crate::model::GetEvaluationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Evaluation>> + Send {
        std::future::ready::<crate::Result<crate::model::Evaluation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::list_evaluations].
    fn list_evaluations(
        &self,
        _req: crate::model::ListEvaluationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListEvaluationsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListEvaluationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::DocumentProcessorService::list_locations].
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

    /// Implements [super::client::DocumentProcessorService::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DocumentProcessorService::list_operations].
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

    /// Implements [super::client::DocumentProcessorService::get_operation].
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

    /// Implements [super::client::DocumentProcessorService::cancel_operation].
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
