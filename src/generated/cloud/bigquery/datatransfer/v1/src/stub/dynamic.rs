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

/// A dyn-compatible, crate-private version of [super::DataTransferService].
#[async_trait::async_trait]
pub trait DataTransferService: std::fmt::Debug + Send + Sync {
    async fn get_data_source(
        &self,
        req: crate::model::GetDataSourceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataSource>>;

    async fn list_data_sources(
        &self,
        req: crate::model::ListDataSourcesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDataSourcesResponse>>;

    async fn create_transfer_config(
        &self,
        req: crate::model::CreateTransferConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TransferConfig>>;

    async fn update_transfer_config(
        &self,
        req: crate::model::UpdateTransferConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TransferConfig>>;

    async fn delete_transfer_config(
        &self,
        req: crate::model::DeleteTransferConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn get_transfer_config(
        &self,
        req: crate::model::GetTransferConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TransferConfig>>;

    async fn list_transfer_configs(
        &self,
        req: crate::model::ListTransferConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListTransferConfigsResponse>>;

    async fn schedule_transfer_runs(
        &self,
        req: crate::model::ScheduleTransferRunsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ScheduleTransferRunsResponse>>;

    async fn start_manual_transfer_runs(
        &self,
        req: crate::model::StartManualTransferRunsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::StartManualTransferRunsResponse>>;

    async fn get_transfer_run(
        &self,
        req: crate::model::GetTransferRunRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TransferRun>>;

    async fn delete_transfer_run(
        &self,
        req: crate::model::DeleteTransferRunRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn list_transfer_runs(
        &self,
        req: crate::model::ListTransferRunsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListTransferRunsResponse>>;

    async fn list_transfer_logs(
        &self,
        req: crate::model::ListTransferLogsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListTransferLogsResponse>>;

    async fn check_valid_creds(
        &self,
        req: crate::model::CheckValidCredsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::CheckValidCredsResponse>>;

    async fn enroll_data_sources(
        &self,
        req: crate::model::EnrollDataSourcesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn unenroll_data_sources(
        &self,
        req: crate::model::UnenrollDataSourcesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::ListLocationsResponse>>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::Location>>;
}

/// All implementations of [super::DataTransferService] also implement [DataTransferService].
#[async_trait::async_trait]
impl<T: super::DataTransferService> DataTransferService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn get_data_source(
        &self,
        req: crate::model::GetDataSourceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataSource>> {
        T::get_data_source(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_data_sources(
        &self,
        req: crate::model::ListDataSourcesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDataSourcesResponse>> {
        T::list_data_sources(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_transfer_config(
        &self,
        req: crate::model::CreateTransferConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TransferConfig>> {
        T::create_transfer_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_transfer_config(
        &self,
        req: crate::model::UpdateTransferConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TransferConfig>> {
        T::update_transfer_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_transfer_config(
        &self,
        req: crate::model::DeleteTransferConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_transfer_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_transfer_config(
        &self,
        req: crate::model::GetTransferConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TransferConfig>> {
        T::get_transfer_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_transfer_configs(
        &self,
        req: crate::model::ListTransferConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListTransferConfigsResponse>> {
        T::list_transfer_configs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn schedule_transfer_runs(
        &self,
        req: crate::model::ScheduleTransferRunsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ScheduleTransferRunsResponse>> {
        T::schedule_transfer_runs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn start_manual_transfer_runs(
        &self,
        req: crate::model::StartManualTransferRunsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::StartManualTransferRunsResponse>> {
        T::start_manual_transfer_runs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_transfer_run(
        &self,
        req: crate::model::GetTransferRunRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TransferRun>> {
        T::get_transfer_run(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_transfer_run(
        &self,
        req: crate::model::DeleteTransferRunRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_transfer_run(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_transfer_runs(
        &self,
        req: crate::model::ListTransferRunsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListTransferRunsResponse>> {
        T::list_transfer_runs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_transfer_logs(
        &self,
        req: crate::model::ListTransferLogsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListTransferLogsResponse>> {
        T::list_transfer_logs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn check_valid_creds(
        &self,
        req: crate::model::CheckValidCredsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::CheckValidCredsResponse>> {
        T::check_valid_creds(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn enroll_data_sources(
        &self,
        req: crate::model::EnrollDataSourcesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::enroll_data_sources(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn unenroll_data_sources(
        &self,
        req: crate::model::UnenrollDataSourcesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::unenroll_data_sources(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::ListLocationsResponse>> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::Location>> {
        T::get_location(self, req, options).await
    }
}
