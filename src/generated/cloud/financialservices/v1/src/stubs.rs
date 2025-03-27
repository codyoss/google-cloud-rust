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

/// Defines the trait used to implement [super::client::Aml].
///
/// Application developers may need to implement this trait to mock
/// `client::Aml`.  In other use-cases, application developers only
/// use `client::Aml` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait Aml: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::Aml::list_instances].
    fn list_instances(
        &self,
        _req: crate::model::ListInstancesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListInstancesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListInstancesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::get_instance].
    fn get_instance(
        &self,
        _req: crate::model::GetInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Instance>> + Send {
        std::future::ready::<crate::Result<crate::model::Instance>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::create_instance].
    fn create_instance(
        &self,
        _req: crate::model::CreateInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::update_instance].
    fn update_instance(
        &self,
        _req: crate::model::UpdateInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::delete_instance].
    fn delete_instance(
        &self,
        _req: crate::model::DeleteInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::import_registered_parties].
    fn import_registered_parties(
        &self,
        _req: crate::model::ImportRegisteredPartiesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::export_registered_parties].
    fn export_registered_parties(
        &self,
        _req: crate::model::ExportRegisteredPartiesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::list_datasets].
    fn list_datasets(
        &self,
        _req: crate::model::ListDatasetsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListDatasetsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListDatasetsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::get_dataset].
    fn get_dataset(
        &self,
        _req: crate::model::GetDatasetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Dataset>> + Send {
        std::future::ready::<crate::Result<crate::model::Dataset>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::create_dataset].
    fn create_dataset(
        &self,
        _req: crate::model::CreateDatasetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::update_dataset].
    fn update_dataset(
        &self,
        _req: crate::model::UpdateDatasetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::delete_dataset].
    fn delete_dataset(
        &self,
        _req: crate::model::DeleteDatasetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::list_models].
    fn list_models(
        &self,
        _req: crate::model::ListModelsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListModelsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListModelsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::get_model].
    fn get_model(
        &self,
        _req: crate::model::GetModelRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Model>> + Send {
        std::future::ready::<crate::Result<crate::model::Model>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Aml::create_model].
    fn create_model(
        &self,
        _req: crate::model::CreateModelRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::update_model].
    fn update_model(
        &self,
        _req: crate::model::UpdateModelRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::export_model_metadata].
    fn export_model_metadata(
        &self,
        _req: crate::model::ExportModelMetadataRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::delete_model].
    fn delete_model(
        &self,
        _req: crate::model::DeleteModelRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::list_engine_configs].
    fn list_engine_configs(
        &self,
        _req: crate::model::ListEngineConfigsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListEngineConfigsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListEngineConfigsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Aml::get_engine_config].
    fn get_engine_config(
        &self,
        _req: crate::model::GetEngineConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::EngineConfig>> + Send {
        std::future::ready::<crate::Result<crate::model::EngineConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::create_engine_config].
    fn create_engine_config(
        &self,
        _req: crate::model::CreateEngineConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::update_engine_config].
    fn update_engine_config(
        &self,
        _req: crate::model::UpdateEngineConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::export_engine_config_metadata].
    fn export_engine_config_metadata(
        &self,
        _req: crate::model::ExportEngineConfigMetadataRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::delete_engine_config].
    fn delete_engine_config(
        &self,
        _req: crate::model::DeleteEngineConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::get_engine_version].
    fn get_engine_version(
        &self,
        _req: crate::model::GetEngineVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::EngineVersion>> + Send {
        std::future::ready::<crate::Result<crate::model::EngineVersion>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::list_engine_versions].
    fn list_engine_versions(
        &self,
        _req: crate::model::ListEngineVersionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListEngineVersionsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListEngineVersionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Aml::list_prediction_results].
    fn list_prediction_results(
        &self,
        _req: crate::model::ListPredictionResultsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListPredictionResultsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListPredictionResultsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Aml::get_prediction_result].
    fn get_prediction_result(
        &self,
        _req: crate::model::GetPredictionResultRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::PredictionResult>> + Send
    {
        std::future::ready::<crate::Result<crate::model::PredictionResult>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::create_prediction_result].
    fn create_prediction_result(
        &self,
        _req: crate::model::CreatePredictionResultRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::update_prediction_result].
    fn update_prediction_result(
        &self,
        _req: crate::model::UpdatePredictionResultRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::export_prediction_result_metadata].
    fn export_prediction_result_metadata(
        &self,
        _req: crate::model::ExportPredictionResultMetadataRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::delete_prediction_result].
    fn delete_prediction_result(
        &self,
        _req: crate::model::DeletePredictionResultRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::list_backtest_results].
    fn list_backtest_results(
        &self,
        _req: crate::model::ListBacktestResultsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListBacktestResultsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListBacktestResultsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Aml::get_backtest_result].
    fn get_backtest_result(
        &self,
        _req: crate::model::GetBacktestResultRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::BacktestResult>> + Send {
        std::future::ready::<crate::Result<crate::model::BacktestResult>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::create_backtest_result].
    fn create_backtest_result(
        &self,
        _req: crate::model::CreateBacktestResultRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::update_backtest_result].
    fn update_backtest_result(
        &self,
        _req: crate::model::UpdateBacktestResultRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::export_backtest_result_metadata].
    fn export_backtest_result_metadata(
        &self,
        _req: crate::model::ExportBacktestResultMetadataRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::delete_backtest_result].
    fn delete_backtest_result(
        &self,
        _req: crate::model::DeleteBacktestResultRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::list_locations].
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

    /// Implements [super::client::Aml::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Aml::list_operations].
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

    /// Implements [super::client::Aml::get_operation].
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

    /// Implements [super::client::Aml::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Aml::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
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
