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
use crate::Result;

/// Implements a [WebSecurityScanner](super::stub::WebSecurityScanner) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct WebSecurityScanner<T>
where
    T: super::stub::WebSecurityScanner + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> WebSecurityScanner<T>
where
    T: super::stub::WebSecurityScanner + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::WebSecurityScanner for WebSecurityScanner<T>
where
    T: super::stub::WebSecurityScanner + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_scan_config(
        &self,
        req: crate::model::CreateScanConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ScanConfig>> {
        self.inner.create_scan_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_scan_config(
        &self,
        req: crate::model::DeleteScanConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_scan_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_scan_config(
        &self,
        req: crate::model::GetScanConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ScanConfig>> {
        self.inner.get_scan_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_scan_configs(
        &self,
        req: crate::model::ListScanConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListScanConfigsResponse>> {
        self.inner.list_scan_configs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_scan_config(
        &self,
        req: crate::model::UpdateScanConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ScanConfig>> {
        self.inner.update_scan_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn start_scan_run(
        &self,
        req: crate::model::StartScanRunRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ScanRun>> {
        self.inner.start_scan_run(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_scan_run(
        &self,
        req: crate::model::GetScanRunRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ScanRun>> {
        self.inner.get_scan_run(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_scan_runs(
        &self,
        req: crate::model::ListScanRunsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListScanRunsResponse>> {
        self.inner.list_scan_runs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn stop_scan_run(
        &self,
        req: crate::model::StopScanRunRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ScanRun>> {
        self.inner.stop_scan_run(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_crawled_urls(
        &self,
        req: crate::model::ListCrawledUrlsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListCrawledUrlsResponse>> {
        self.inner.list_crawled_urls(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_finding(
        &self,
        req: crate::model::GetFindingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Finding>> {
        self.inner.get_finding(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_findings(
        &self,
        req: crate::model::ListFindingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListFindingsResponse>> {
        self.inner.list_findings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_finding_type_stats(
        &self,
        req: crate::model::ListFindingTypeStatsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListFindingTypeStatsResponse>> {
        self.inner.list_finding_type_stats(req, options).await
    }
}
