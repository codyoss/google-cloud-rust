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
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Web Security Scanner API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_websecurityscanner_v1::client::WebSecurityScanner;
/// let client = WebSecurityScanner::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Web Security Scanner Service identifies security vulnerabilities in web
/// applications hosted on Google Cloud. It crawls your application, and
/// attempts to exercise as many user inputs and event handlers as possible.
///
/// # Configuration
///
/// To configure `WebSecurityScanner` use the `with_*` methods in the type returned
/// by [builder()][WebSecurityScanner::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://websecurityscanner.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::web_security_scanner::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::web_security_scanner::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `WebSecurityScanner` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `WebSecurityScanner` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct WebSecurityScanner {
    inner: Arc<dyn super::stub::dynamic::WebSecurityScanner>,
}

impl WebSecurityScanner {
    /// Returns a builder for [WebSecurityScanner].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_websecurityscanner_v1::client::WebSecurityScanner;
    /// let client = WebSecurityScanner::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::web_security_scanner::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::web_security_scanner::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::WebSecurityScanner + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::WebSecurityScanner>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::WebSecurityScanner> {
        super::transport::WebSecurityScanner::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::WebSecurityScanner> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::WebSecurityScanner::new)
    }

    /// Creates a new ScanConfig.
    pub fn create_scan_config(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::web_security_scanner::CreateScanConfig {
        super::builder::web_security_scanner::CreateScanConfig::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes an existing ScanConfig and its child resources.
    pub fn delete_scan_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::web_security_scanner::DeleteScanConfig {
        super::builder::web_security_scanner::DeleteScanConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets a ScanConfig.
    pub fn get_scan_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::web_security_scanner::GetScanConfig {
        super::builder::web_security_scanner::GetScanConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists ScanConfigs under a given project.
    pub fn list_scan_configs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::web_security_scanner::ListScanConfigs {
        super::builder::web_security_scanner::ListScanConfigs::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a ScanConfig. This method support partial update of a ScanConfig.
    pub fn update_scan_config(
        &self,
        scan_config: impl Into<crate::model::ScanConfig>,
    ) -> super::builder::web_security_scanner::UpdateScanConfig {
        super::builder::web_security_scanner::UpdateScanConfig::new(self.inner.clone())
            .set_scan_config(scan_config.into())
    }

    /// Start a ScanRun according to the given ScanConfig.
    pub fn start_scan_run(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::web_security_scanner::StartScanRun {
        super::builder::web_security_scanner::StartScanRun::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets a ScanRun.
    pub fn get_scan_run(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::web_security_scanner::GetScanRun {
        super::builder::web_security_scanner::GetScanRun::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists ScanRuns under a given ScanConfig, in descending order of ScanRun
    /// stop time.
    pub fn list_scan_runs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::web_security_scanner::ListScanRuns {
        super::builder::web_security_scanner::ListScanRuns::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Stops a ScanRun. The stopped ScanRun is returned.
    pub fn stop_scan_run(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::web_security_scanner::StopScanRun {
        super::builder::web_security_scanner::StopScanRun::new(self.inner.clone())
            .set_name(name.into())
    }

    /// List CrawledUrls under a given ScanRun.
    pub fn list_crawled_urls(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::web_security_scanner::ListCrawledUrls {
        super::builder::web_security_scanner::ListCrawledUrls::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a Finding.
    pub fn get_finding(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::web_security_scanner::GetFinding {
        super::builder::web_security_scanner::GetFinding::new(self.inner.clone())
            .set_name(name.into())
    }

    /// List Findings under a given ScanRun.
    pub fn list_findings(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::web_security_scanner::ListFindings {
        super::builder::web_security_scanner::ListFindings::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// List all FindingTypeStats under a given ScanRun.
    pub fn list_finding_type_stats(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::web_security_scanner::ListFindingTypeStats {
        super::builder::web_security_scanner::ListFindingTypeStats::new(self.inner.clone())
            .set_parent(parent.into())
    }
}
