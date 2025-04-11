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

/// Defines the trait used to implement [super::client::LoggingServiceV2].
///
/// Application developers may need to implement this trait to mock
/// `client::LoggingServiceV2`.  In other use-cases, application developers only
/// use `client::LoggingServiceV2` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait LoggingServiceV2: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::LoggingServiceV2::delete_log].
    fn delete_log(
        &self,
        _req: crate::model::DeleteLogRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::LoggingServiceV2::write_log_entries].
    fn write_log_entries(
        &self,
        _req: crate::model::WriteLogEntriesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::WriteLogEntriesResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::WriteLogEntriesResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::LoggingServiceV2::list_log_entries].
    fn list_log_entries(
        &self,
        _req: crate::model::ListLogEntriesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListLogEntriesResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListLogEntriesResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::LoggingServiceV2::list_monitored_resource_descriptors].
    fn list_monitored_resource_descriptors(
        &self,
        _req: crate::model::ListMonitoredResourceDescriptorsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<
            gax::response::Response<crate::model::ListMonitoredResourceDescriptorsResponse>,
        >,
    > + Send {
        std::future::ready::<
            crate::Result<
                gax::response::Response<crate::model::ListMonitoredResourceDescriptorsResponse>,
            >,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::LoggingServiceV2::list_logs].
    fn list_logs(
        &self,
        _req: crate::model::ListLogsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListLogsResponse>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::ListLogsResponse>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::LoggingServiceV2::list_operations].
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

    /// Implements [super::client::LoggingServiceV2::get_operation].
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

    /// Implements [super::client::LoggingServiceV2::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }
}

/// Defines the trait used to implement [super::client::ConfigServiceV2].
///
/// Application developers may need to implement this trait to mock
/// `client::ConfigServiceV2`.  In other use-cases, application developers only
/// use `client::ConfigServiceV2` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait ConfigServiceV2: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::ConfigServiceV2::list_buckets].
    fn list_buckets(
        &self,
        _req: crate::model::ListBucketsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListBucketsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListBucketsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::ConfigServiceV2::get_bucket].
    fn get_bucket(
        &self,
        _req: crate::model::GetBucketRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::LogBucket>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::LogBucket>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ConfigServiceV2::create_bucket_async].
    fn create_bucket_async(
        &self,
        _req: crate::model::CreateBucketRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::ConfigServiceV2::update_bucket_async].
    fn update_bucket_async(
        &self,
        _req: crate::model::UpdateBucketRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::ConfigServiceV2::create_bucket].
    fn create_bucket(
        &self,
        _req: crate::model::CreateBucketRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::LogBucket>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::LogBucket>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ConfigServiceV2::update_bucket].
    fn update_bucket(
        &self,
        _req: crate::model::UpdateBucketRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::LogBucket>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::LogBucket>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ConfigServiceV2::delete_bucket].
    fn delete_bucket(
        &self,
        _req: crate::model::DeleteBucketRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ConfigServiceV2::undelete_bucket].
    fn undelete_bucket(
        &self,
        _req: crate::model::UndeleteBucketRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ConfigServiceV2::list_views].
    fn list_views(
        &self,
        _req: crate::model::ListViewsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListViewsResponse>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::ListViewsResponse>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::ConfigServiceV2::get_view].
    fn get_view(
        &self,
        _req: crate::model::GetViewRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::LogView>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::LogView>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ConfigServiceV2::create_view].
    fn create_view(
        &self,
        _req: crate::model::CreateViewRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::LogView>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::LogView>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ConfigServiceV2::update_view].
    fn update_view(
        &self,
        _req: crate::model::UpdateViewRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::LogView>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::LogView>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ConfigServiceV2::delete_view].
    fn delete_view(
        &self,
        _req: crate::model::DeleteViewRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ConfigServiceV2::list_sinks].
    fn list_sinks(
        &self,
        _req: crate::model::ListSinksRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListSinksResponse>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::ListSinksResponse>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::ConfigServiceV2::get_sink].
    fn get_sink(
        &self,
        _req: crate::model::GetSinkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::LogSink>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::LogSink>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ConfigServiceV2::create_sink].
    fn create_sink(
        &self,
        _req: crate::model::CreateSinkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::LogSink>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::LogSink>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ConfigServiceV2::update_sink].
    fn update_sink(
        &self,
        _req: crate::model::UpdateSinkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::LogSink>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::LogSink>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ConfigServiceV2::delete_sink].
    fn delete_sink(
        &self,
        _req: crate::model::DeleteSinkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ConfigServiceV2::create_link].
    fn create_link(
        &self,
        _req: crate::model::CreateLinkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::ConfigServiceV2::delete_link].
    fn delete_link(
        &self,
        _req: crate::model::DeleteLinkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::ConfigServiceV2::list_links].
    fn list_links(
        &self,
        _req: crate::model::ListLinksRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListLinksResponse>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::ListLinksResponse>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::ConfigServiceV2::get_link].
    fn get_link(
        &self,
        _req: crate::model::GetLinkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Link>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Link>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ConfigServiceV2::list_exclusions].
    fn list_exclusions(
        &self,
        _req: crate::model::ListExclusionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListExclusionsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListExclusionsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::ConfigServiceV2::get_exclusion].
    fn get_exclusion(
        &self,
        _req: crate::model::GetExclusionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::LogExclusion>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::LogExclusion>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::ConfigServiceV2::create_exclusion].
    fn create_exclusion(
        &self,
        _req: crate::model::CreateExclusionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::LogExclusion>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::LogExclusion>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::ConfigServiceV2::update_exclusion].
    fn update_exclusion(
        &self,
        _req: crate::model::UpdateExclusionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::LogExclusion>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::LogExclusion>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::ConfigServiceV2::delete_exclusion].
    fn delete_exclusion(
        &self,
        _req: crate::model::DeleteExclusionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ConfigServiceV2::get_cmek_settings].
    fn get_cmek_settings(
        &self,
        _req: crate::model::GetCmekSettingsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::CmekSettings>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::CmekSettings>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::ConfigServiceV2::update_cmek_settings].
    fn update_cmek_settings(
        &self,
        _req: crate::model::UpdateCmekSettingsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::CmekSettings>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::CmekSettings>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::ConfigServiceV2::get_settings].
    fn get_settings(
        &self,
        _req: crate::model::GetSettingsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Settings>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Settings>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ConfigServiceV2::update_settings].
    fn update_settings(
        &self,
        _req: crate::model::UpdateSettingsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Settings>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Settings>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ConfigServiceV2::copy_log_entries].
    fn copy_log_entries(
        &self,
        _req: crate::model::CopyLogEntriesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::ConfigServiceV2::list_operations].
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

    /// Implements [super::client::ConfigServiceV2::get_operation].
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

    /// Implements [super::client::ConfigServiceV2::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
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

/// Defines the trait used to implement [super::client::MetricsServiceV2].
///
/// Application developers may need to implement this trait to mock
/// `client::MetricsServiceV2`.  In other use-cases, application developers only
/// use `client::MetricsServiceV2` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait MetricsServiceV2: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::MetricsServiceV2::list_log_metrics].
    fn list_log_metrics(
        &self,
        _req: crate::model::ListLogMetricsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListLogMetricsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListLogMetricsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::MetricsServiceV2::get_log_metric].
    fn get_log_metric(
        &self,
        _req: crate::model::GetLogMetricRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::LogMetric>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::LogMetric>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::MetricsServiceV2::create_log_metric].
    fn create_log_metric(
        &self,
        _req: crate::model::CreateLogMetricRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::LogMetric>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::LogMetric>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::MetricsServiceV2::update_log_metric].
    fn update_log_metric(
        &self,
        _req: crate::model::UpdateLogMetricRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::LogMetric>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::LogMetric>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::MetricsServiceV2::delete_log_metric].
    fn delete_log_metric(
        &self,
        _req: crate::model::DeleteLogMetricRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::MetricsServiceV2::list_operations].
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

    /// Implements [super::client::MetricsServiceV2::get_operation].
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

    /// Implements [super::client::MetricsServiceV2::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }
}
