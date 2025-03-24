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

pub mod web_security_scanner {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::WebSecurityScanner] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::WebSecurityScanner>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebSecurityScanner>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a WebSecurityScanner::create_scan_config call.
    #[derive(Clone, Debug)]
    pub struct CreateScanConfig(RequestBuilder<crate::model::CreateScanConfigRequest>);

    impl CreateScanConfig {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebSecurityScanner>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateScanConfigRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ScanConfig> {
            (*self.0.stub)
                .create_scan_config(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateScanConfigRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [scan_config][crate::model::CreateScanConfigRequest::scan_config].
        pub fn set_scan_config<T: Into<std::option::Option<crate::model::ScanConfig>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.scan_config = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateScanConfig {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebSecurityScanner::delete_scan_config call.
    #[derive(Clone, Debug)]
    pub struct DeleteScanConfig(RequestBuilder<crate::model::DeleteScanConfigRequest>);

    impl DeleteScanConfig {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebSecurityScanner>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteScanConfigRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<wkt::Empty> {
            (*self.0.stub)
                .delete_scan_config(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteScanConfigRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteScanConfig {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebSecurityScanner::get_scan_config call.
    #[derive(Clone, Debug)]
    pub struct GetScanConfig(RequestBuilder<crate::model::GetScanConfigRequest>);

    impl GetScanConfig {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebSecurityScanner>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetScanConfigRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ScanConfig> {
            (*self.0.stub)
                .get_scan_config(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetScanConfigRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetScanConfig {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebSecurityScanner::list_scan_configs call.
    #[derive(Clone, Debug)]
    pub struct ListScanConfigs(RequestBuilder<crate::model::ListScanConfigsRequest>);

    impl ListScanConfigs {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebSecurityScanner>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListScanConfigsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListScanConfigsResponse> {
            (*self.0.stub)
                .list_scan_configs(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListScanConfigsResponse, gax::error::Error>
        {
            use std::clone::Clone;
            let token = self.0.request.page_token.clone();
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListScanConfigsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListScanConfigsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListScanConfigsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListScanConfigs {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebSecurityScanner::update_scan_config call.
    #[derive(Clone, Debug)]
    pub struct UpdateScanConfig(RequestBuilder<crate::model::UpdateScanConfigRequest>);

    impl UpdateScanConfig {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebSecurityScanner>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateScanConfigRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ScanConfig> {
            (*self.0.stub)
                .update_scan_config(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [scan_config][crate::model::UpdateScanConfigRequest::scan_config].
        pub fn set_scan_config<T: Into<std::option::Option<crate::model::ScanConfig>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.scan_config = v.into();
            self
        }

        /// Sets the value of [update_mask][crate::model::UpdateScanConfigRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for UpdateScanConfig {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebSecurityScanner::start_scan_run call.
    #[derive(Clone, Debug)]
    pub struct StartScanRun(RequestBuilder<crate::model::StartScanRunRequest>);

    impl StartScanRun {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebSecurityScanner>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::StartScanRunRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ScanRun> {
            (*self.0.stub)
                .start_scan_run(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::StartScanRunRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for StartScanRun {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebSecurityScanner::get_scan_run call.
    #[derive(Clone, Debug)]
    pub struct GetScanRun(RequestBuilder<crate::model::GetScanRunRequest>);

    impl GetScanRun {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebSecurityScanner>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetScanRunRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ScanRun> {
            (*self.0.stub)
                .get_scan_run(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetScanRunRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetScanRun {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebSecurityScanner::list_scan_runs call.
    #[derive(Clone, Debug)]
    pub struct ListScanRuns(RequestBuilder<crate::model::ListScanRunsRequest>);

    impl ListScanRuns {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebSecurityScanner>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListScanRunsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListScanRunsResponse> {
            (*self.0.stub)
                .list_scan_runs(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListScanRunsResponse, gax::error::Error>
        {
            use std::clone::Clone;
            let token = self.0.request.page_token.clone();
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListScanRunsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListScanRunsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListScanRunsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListScanRuns {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebSecurityScanner::stop_scan_run call.
    #[derive(Clone, Debug)]
    pub struct StopScanRun(RequestBuilder<crate::model::StopScanRunRequest>);

    impl StopScanRun {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebSecurityScanner>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::StopScanRunRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ScanRun> {
            (*self.0.stub)
                .stop_scan_run(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::StopScanRunRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for StopScanRun {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebSecurityScanner::list_crawled_urls call.
    #[derive(Clone, Debug)]
    pub struct ListCrawledUrls(RequestBuilder<crate::model::ListCrawledUrlsRequest>);

    impl ListCrawledUrls {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebSecurityScanner>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListCrawledUrlsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListCrawledUrlsResponse> {
            (*self.0.stub)
                .list_crawled_urls(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListCrawledUrlsResponse, gax::error::Error>
        {
            use std::clone::Clone;
            let token = self.0.request.page_token.clone();
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListCrawledUrlsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListCrawledUrlsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListCrawledUrlsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListCrawledUrls {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebSecurityScanner::get_finding call.
    #[derive(Clone, Debug)]
    pub struct GetFinding(RequestBuilder<crate::model::GetFindingRequest>);

    impl GetFinding {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebSecurityScanner>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetFindingRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Finding> {
            (*self.0.stub)
                .get_finding(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetFindingRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetFinding {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebSecurityScanner::list_findings call.
    #[derive(Clone, Debug)]
    pub struct ListFindings(RequestBuilder<crate::model::ListFindingsRequest>);

    impl ListFindings {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebSecurityScanner>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListFindingsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListFindingsResponse> {
            (*self.0.stub)
                .list_findings(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListFindingsResponse, gax::error::Error>
        {
            use std::clone::Clone;
            let token = self.0.request.page_token.clone();
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListFindingsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListFindingsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListFindingsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListFindingsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListFindings {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebSecurityScanner::list_finding_type_stats call.
    #[derive(Clone, Debug)]
    pub struct ListFindingTypeStats(RequestBuilder<crate::model::ListFindingTypeStatsRequest>);

    impl ListFindingTypeStats {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebSecurityScanner>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListFindingTypeStatsRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListFindingTypeStatsResponse> {
            (*self.0.stub)
                .list_finding_type_stats(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::ListFindingTypeStatsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListFindingTypeStats {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
