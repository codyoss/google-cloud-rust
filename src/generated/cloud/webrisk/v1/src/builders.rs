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

pub mod web_risk_service {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::WebRiskService] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::WebRiskService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebRiskService>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a WebRiskService::compute_threat_list_diff call.
    #[derive(Clone, Debug)]
    pub struct ComputeThreatListDiff(RequestBuilder<crate::model::ComputeThreatListDiffRequest>);

    impl ComputeThreatListDiff {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebRiskService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ComputeThreatListDiffRequest>>(
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
        pub async fn send(self) -> Result<crate::model::ComputeThreatListDiffResponse> {
            (*self.0.stub)
                .compute_threat_list_diff(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [threat_type][crate::model::ComputeThreatListDiffRequest::threat_type].
        pub fn set_threat_type<T: Into<crate::model::ThreatType>>(mut self, v: T) -> Self {
            self.0.request.threat_type = v.into();
            self
        }

        /// Sets the value of [version_token][crate::model::ComputeThreatListDiffRequest::version_token].
        pub fn set_version_token<T: Into<::bytes::Bytes>>(mut self, v: T) -> Self {
            self.0.request.version_token = v.into();
            self
        }

        /// Sets the value of [constraints][crate::model::ComputeThreatListDiffRequest::constraints].
        pub fn set_constraints<
            T: Into<std::option::Option<crate::model::compute_threat_list_diff_request::Constraints>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.constraints = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ComputeThreatListDiff {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebRiskService::search_uris call.
    #[derive(Clone, Debug)]
    pub struct SearchUris(RequestBuilder<crate::model::SearchUrisRequest>);

    impl SearchUris {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebRiskService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::SearchUrisRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::SearchUrisResponse> {
            (*self.0.stub)
                .search_uris(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [uri][crate::model::SearchUrisRequest::uri].
        pub fn set_uri<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.uri = v.into();
            self
        }

        /// Sets the value of [threat_types][crate::model::SearchUrisRequest::threat_types].
        pub fn set_threat_types<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<crate::model::ThreatType>,
        {
            use std::iter::Iterator;
            self.0.request.threat_types = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for SearchUris {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebRiskService::search_hashes call.
    #[derive(Clone, Debug)]
    pub struct SearchHashes(RequestBuilder<crate::model::SearchHashesRequest>);

    impl SearchHashes {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebRiskService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::SearchHashesRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::SearchHashesResponse> {
            (*self.0.stub)
                .search_hashes(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [hash_prefix][crate::model::SearchHashesRequest::hash_prefix].
        pub fn set_hash_prefix<T: Into<::bytes::Bytes>>(mut self, v: T) -> Self {
            self.0.request.hash_prefix = v.into();
            self
        }

        /// Sets the value of [threat_types][crate::model::SearchHashesRequest::threat_types].
        pub fn set_threat_types<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<crate::model::ThreatType>,
        {
            use std::iter::Iterator;
            self.0.request.threat_types = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for SearchHashes {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebRiskService::create_submission call.
    #[derive(Clone, Debug)]
    pub struct CreateSubmission(RequestBuilder<crate::model::CreateSubmissionRequest>);

    impl CreateSubmission {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebRiskService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateSubmissionRequest>>(
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
        pub async fn send(self) -> Result<crate::model::Submission> {
            (*self.0.stub)
                .create_submission(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateSubmissionRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [submission][crate::model::CreateSubmissionRequest::submission].
        pub fn set_submission<T: Into<std::option::Option<crate::model::Submission>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.submission = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateSubmission {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebRiskService::submit_uri call.
    #[derive(Clone, Debug)]
    pub struct SubmitUri(RequestBuilder<crate::model::SubmitUriRequest>);

    impl SubmitUri {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebRiskService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::SubmitUriRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        ///
        /// # Long running operations
        ///
        /// This starts, but does not poll, a longrunning operation. More information
        /// on [submit_uri][super::super::client::WebRiskService::submit_uri].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .submit_uri(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `submit_uri`.
        pub fn poller(
            self,
        ) -> impl lro::Poller<crate::model::Submission, crate::model::SubmitUriMetadata> {
            type Operation =
                lro::Operation<crate::model::Submission, crate::model::SubmitUriMetadata>;
            let polling_policy = self.0.stub.get_polling_policy(&self.0.options);
            let polling_backoff_policy = self.0.stub.get_polling_backoff_policy(&self.0.options);

            let stub = self.0.stub.clone();
            let mut options = self.0.options.clone();
            options.set_retry_policy(gax::retry_policy::NeverRetry);
            let query = move |name| {
                let stub = stub.clone();
                let options = options.clone();
                async {
                    let op = GetOperation::new(stub)
                        .set_name(name)
                        .with_options(options)
                        .send()
                        .await?;
                    Ok(Operation::new(op))
                }
            };

            let start = move || async {
                let op = self.send().await?;
                Ok(Operation::new(op))
            };

            lro::new_poller(polling_policy, polling_backoff_policy, start, query)
        }

        /// Sets the value of [parent][crate::model::SubmitUriRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [submission][crate::model::SubmitUriRequest::submission].
        pub fn set_submission<T: Into<std::option::Option<crate::model::Submission>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.submission = v.into();
            self
        }

        /// Sets the value of [threat_info][crate::model::SubmitUriRequest::threat_info].
        pub fn set_threat_info<T: Into<std::option::Option<crate::model::ThreatInfo>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.threat_info = v.into();
            self
        }

        /// Sets the value of [threat_discovery][crate::model::SubmitUriRequest::threat_discovery].
        pub fn set_threat_discovery<T: Into<std::option::Option<crate::model::ThreatDiscovery>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.threat_discovery = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for SubmitUri {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebRiskService::list_operations call.
    #[derive(Clone, Debug)]
    pub struct ListOperations(RequestBuilder<longrunning::model::ListOperationsRequest>);

    impl ListOperations {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebRiskService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<longrunning::model::ListOperationsRequest>>(
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
        pub async fn send(self) -> Result<longrunning::model::ListOperationsResponse> {
            (*self.0.stub)
                .list_operations(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<longrunning::model::ListOperationsResponse, gax::error::Error>
        {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [name][longrunning::model::ListOperationsRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [filter][longrunning::model::ListOperationsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [page_size][longrunning::model::ListOperationsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][longrunning::model::ListOperationsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListOperations {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebRiskService::get_operation call.
    #[derive(Clone, Debug)]
    pub struct GetOperation(RequestBuilder<longrunning::model::GetOperationRequest>);

    impl GetOperation {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebRiskService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<longrunning::model::GetOperationRequest>>(
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
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .get_operation(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][longrunning::model::GetOperationRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetOperation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebRiskService::delete_operation call.
    #[derive(Clone, Debug)]
    pub struct DeleteOperation(RequestBuilder<longrunning::model::DeleteOperationRequest>);

    impl DeleteOperation {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebRiskService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<longrunning::model::DeleteOperationRequest>>(
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
                .delete_operation(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][longrunning::model::DeleteOperationRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteOperation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a WebRiskService::cancel_operation call.
    #[derive(Clone, Debug)]
    pub struct CancelOperation(RequestBuilder<longrunning::model::CancelOperationRequest>);

    impl CancelOperation {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::WebRiskService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<longrunning::model::CancelOperationRequest>>(
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
                .cancel_operation(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][longrunning::model::CancelOperationRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CancelOperation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
