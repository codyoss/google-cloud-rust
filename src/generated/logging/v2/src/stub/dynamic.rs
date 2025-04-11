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

use std::sync::Arc;

/// A dyn-compatible, crate-private version of [super::LoggingServiceV2].
#[async_trait::async_trait]
pub trait LoggingServiceV2: std::fmt::Debug + Send + Sync {
    async fn delete_log(
        &self,
        req: crate::model::DeleteLogRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn write_log_entries(
        &self,
        req: crate::model::WriteLogEntriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::WriteLogEntriesResponse>>;

    async fn list_log_entries(
        &self,
        req: crate::model::ListLogEntriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListLogEntriesResponse>>;

    async fn list_monitored_resource_descriptors(
        &self,
        req: crate::model::ListMonitoredResourceDescriptorsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<
        gax::response::Response<crate::model::ListMonitoredResourceDescriptorsResponse>,
    >;

    async fn list_logs(
        &self,
        req: crate::model::ListLogsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListLogsResponse>>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;
}

/// All implementations of [super::LoggingServiceV2] also implement [LoggingServiceV2].
#[async_trait::async_trait]
impl<T: super::LoggingServiceV2> LoggingServiceV2 for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn delete_log(
        &self,
        req: crate::model::DeleteLogRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_log(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn write_log_entries(
        &self,
        req: crate::model::WriteLogEntriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::WriteLogEntriesResponse>> {
        T::write_log_entries(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_log_entries(
        &self,
        req: crate::model::ListLogEntriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListLogEntriesResponse>> {
        T::list_log_entries(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_monitored_resource_descriptors(
        &self,
        req: crate::model::ListMonitoredResourceDescriptorsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<
        gax::response::Response<crate::model::ListMonitoredResourceDescriptorsResponse>,
    > {
        T::list_monitored_resource_descriptors(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_logs(
        &self,
        req: crate::model::ListLogsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListLogsResponse>> {
        T::list_logs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::cancel_operation(self, req, options).await
    }
}

/// A dyn-compatible, crate-private version of [super::ConfigServiceV2].
#[async_trait::async_trait]
pub trait ConfigServiceV2: std::fmt::Debug + Send + Sync {
    async fn list_buckets(
        &self,
        req: crate::model::ListBucketsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListBucketsResponse>>;

    async fn get_bucket(
        &self,
        req: crate::model::GetBucketRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogBucket>>;

    async fn create_bucket_async(
        &self,
        req: crate::model::CreateBucketRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn update_bucket_async(
        &self,
        req: crate::model::UpdateBucketRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn create_bucket(
        &self,
        req: crate::model::CreateBucketRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogBucket>>;

    async fn update_bucket(
        &self,
        req: crate::model::UpdateBucketRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogBucket>>;

    async fn delete_bucket(
        &self,
        req: crate::model::DeleteBucketRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn undelete_bucket(
        &self,
        req: crate::model::UndeleteBucketRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn list_views(
        &self,
        req: crate::model::ListViewsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListViewsResponse>>;

    async fn get_view(
        &self,
        req: crate::model::GetViewRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogView>>;

    async fn create_view(
        &self,
        req: crate::model::CreateViewRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogView>>;

    async fn update_view(
        &self,
        req: crate::model::UpdateViewRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogView>>;

    async fn delete_view(
        &self,
        req: crate::model::DeleteViewRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn list_sinks(
        &self,
        req: crate::model::ListSinksRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListSinksResponse>>;

    async fn get_sink(
        &self,
        req: crate::model::GetSinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogSink>>;

    async fn create_sink(
        &self,
        req: crate::model::CreateSinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogSink>>;

    async fn update_sink(
        &self,
        req: crate::model::UpdateSinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogSink>>;

    async fn delete_sink(
        &self,
        req: crate::model::DeleteSinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn create_link(
        &self,
        req: crate::model::CreateLinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_link(
        &self,
        req: crate::model::DeleteLinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn list_links(
        &self,
        req: crate::model::ListLinksRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListLinksResponse>>;

    async fn get_link(
        &self,
        req: crate::model::GetLinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Link>>;

    async fn list_exclusions(
        &self,
        req: crate::model::ListExclusionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListExclusionsResponse>>;

    async fn get_exclusion(
        &self,
        req: crate::model::GetExclusionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogExclusion>>;

    async fn create_exclusion(
        &self,
        req: crate::model::CreateExclusionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogExclusion>>;

    async fn update_exclusion(
        &self,
        req: crate::model::UpdateExclusionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogExclusion>>;

    async fn delete_exclusion(
        &self,
        req: crate::model::DeleteExclusionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn get_cmek_settings(
        &self,
        req: crate::model::GetCmekSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::CmekSettings>>;

    async fn update_cmek_settings(
        &self,
        req: crate::model::UpdateCmekSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::CmekSettings>>;

    async fn get_settings(
        &self,
        req: crate::model::GetSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Settings>>;

    async fn update_settings(
        &self,
        req: crate::model::UpdateSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Settings>>;

    async fn copy_log_entries(
        &self,
        req: crate::model::CopyLogEntriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::ConfigServiceV2] also implement [ConfigServiceV2].
#[async_trait::async_trait]
impl<T: super::ConfigServiceV2> ConfigServiceV2 for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_buckets(
        &self,
        req: crate::model::ListBucketsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListBucketsResponse>> {
        T::list_buckets(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_bucket(
        &self,
        req: crate::model::GetBucketRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogBucket>> {
        T::get_bucket(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_bucket_async(
        &self,
        req: crate::model::CreateBucketRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_bucket_async(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_bucket_async(
        &self,
        req: crate::model::UpdateBucketRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::update_bucket_async(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_bucket(
        &self,
        req: crate::model::CreateBucketRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogBucket>> {
        T::create_bucket(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_bucket(
        &self,
        req: crate::model::UpdateBucketRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogBucket>> {
        T::update_bucket(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_bucket(
        &self,
        req: crate::model::DeleteBucketRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_bucket(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn undelete_bucket(
        &self,
        req: crate::model::UndeleteBucketRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::undelete_bucket(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_views(
        &self,
        req: crate::model::ListViewsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListViewsResponse>> {
        T::list_views(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_view(
        &self,
        req: crate::model::GetViewRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogView>> {
        T::get_view(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_view(
        &self,
        req: crate::model::CreateViewRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogView>> {
        T::create_view(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_view(
        &self,
        req: crate::model::UpdateViewRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogView>> {
        T::update_view(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_view(
        &self,
        req: crate::model::DeleteViewRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_view(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_sinks(
        &self,
        req: crate::model::ListSinksRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListSinksResponse>> {
        T::list_sinks(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_sink(
        &self,
        req: crate::model::GetSinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogSink>> {
        T::get_sink(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_sink(
        &self,
        req: crate::model::CreateSinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogSink>> {
        T::create_sink(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_sink(
        &self,
        req: crate::model::UpdateSinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogSink>> {
        T::update_sink(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_sink(
        &self,
        req: crate::model::DeleteSinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_sink(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_link(
        &self,
        req: crate::model::CreateLinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_link(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_link(
        &self,
        req: crate::model::DeleteLinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_link(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_links(
        &self,
        req: crate::model::ListLinksRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListLinksResponse>> {
        T::list_links(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_link(
        &self,
        req: crate::model::GetLinkRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Link>> {
        T::get_link(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_exclusions(
        &self,
        req: crate::model::ListExclusionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListExclusionsResponse>> {
        T::list_exclusions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_exclusion(
        &self,
        req: crate::model::GetExclusionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogExclusion>> {
        T::get_exclusion(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_exclusion(
        &self,
        req: crate::model::CreateExclusionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogExclusion>> {
        T::create_exclusion(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_exclusion(
        &self,
        req: crate::model::UpdateExclusionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogExclusion>> {
        T::update_exclusion(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_exclusion(
        &self,
        req: crate::model::DeleteExclusionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_exclusion(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_cmek_settings(
        &self,
        req: crate::model::GetCmekSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::CmekSettings>> {
        T::get_cmek_settings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_cmek_settings(
        &self,
        req: crate::model::UpdateCmekSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::CmekSettings>> {
        T::update_cmek_settings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_settings(
        &self,
        req: crate::model::GetSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Settings>> {
        T::get_settings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_settings(
        &self,
        req: crate::model::UpdateSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Settings>> {
        T::update_settings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn copy_log_entries(
        &self,
        req: crate::model::CopyLogEntriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::copy_log_entries(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::cancel_operation(self, req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        T::get_polling_error_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}

/// A dyn-compatible, crate-private version of [super::MetricsServiceV2].
#[async_trait::async_trait]
pub trait MetricsServiceV2: std::fmt::Debug + Send + Sync {
    async fn list_log_metrics(
        &self,
        req: crate::model::ListLogMetricsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListLogMetricsResponse>>;

    async fn get_log_metric(
        &self,
        req: crate::model::GetLogMetricRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogMetric>>;

    async fn create_log_metric(
        &self,
        req: crate::model::CreateLogMetricRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogMetric>>;

    async fn update_log_metric(
        &self,
        req: crate::model::UpdateLogMetricRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogMetric>>;

    async fn delete_log_metric(
        &self,
        req: crate::model::DeleteLogMetricRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;
}

/// All implementations of [super::MetricsServiceV2] also implement [MetricsServiceV2].
#[async_trait::async_trait]
impl<T: super::MetricsServiceV2> MetricsServiceV2 for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_log_metrics(
        &self,
        req: crate::model::ListLogMetricsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListLogMetricsResponse>> {
        T::list_log_metrics(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_log_metric(
        &self,
        req: crate::model::GetLogMetricRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogMetric>> {
        T::get_log_metric(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_log_metric(
        &self,
        req: crate::model::CreateLogMetricRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogMetric>> {
        T::create_log_metric(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_log_metric(
        &self,
        req: crate::model::UpdateLogMetricRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::LogMetric>> {
        T::update_log_metric(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_log_metric(
        &self,
        req: crate::model::DeleteLogMetricRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_log_metric(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::cancel_operation(self, req, options).await
    }
}
