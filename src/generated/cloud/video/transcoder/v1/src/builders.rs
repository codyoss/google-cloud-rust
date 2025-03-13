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

pub mod transcoder_service {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [crate::client::TranscoderService] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn crate::stubs::dynamic::TranscoderService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::TranscoderService>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a TranscoderService::create_job call.
    #[derive(Clone, Debug)]
    pub struct CreateJob(RequestBuilder<crate::model::CreateJobRequest>);

    impl CreateJob {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::TranscoderService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateJobRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Job> {
            (*self.0.stub)
                .create_job(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateJobRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [job][crate::model::CreateJobRequest::job].
        pub fn set_job<T: Into<std::option::Option<crate::model::Job>>>(mut self, v: T) -> Self {
            self.0.request.job = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateJob {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a TranscoderService::list_jobs call.
    #[derive(Clone, Debug)]
    pub struct ListJobs(RequestBuilder<crate::model::ListJobsRequest>);

    impl ListJobs {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::TranscoderService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListJobsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListJobsResponse> {
            (*self.0.stub)
                .list_jobs(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListJobsResponse, gax::error::Error> {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListJobsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListJobsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListJobsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListJobsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [order_by][crate::model::ListJobsRequest::order_by].
        pub fn set_order_by<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.order_by = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListJobs {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a TranscoderService::get_job call.
    #[derive(Clone, Debug)]
    pub struct GetJob(RequestBuilder<crate::model::GetJobRequest>);

    impl GetJob {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::TranscoderService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetJobRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Job> {
            (*self.0.stub).get_job(self.0.request, self.0.options).await
        }

        /// Sets the value of [name][crate::model::GetJobRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetJob {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a TranscoderService::delete_job call.
    #[derive(Clone, Debug)]
    pub struct DeleteJob(RequestBuilder<crate::model::DeleteJobRequest>);

    impl DeleteJob {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::TranscoderService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteJobRequest>>(mut self, v: V) -> Self {
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
                .delete_job(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteJobRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [allow_missing][crate::model::DeleteJobRequest::allow_missing].
        pub fn set_allow_missing<T: Into<bool>>(mut self, v: T) -> Self {
            self.0.request.allow_missing = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteJob {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a TranscoderService::create_job_template call.
    #[derive(Clone, Debug)]
    pub struct CreateJobTemplate(RequestBuilder<crate::model::CreateJobTemplateRequest>);

    impl CreateJobTemplate {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::TranscoderService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateJobTemplateRequest>>(
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
        pub async fn send(self) -> Result<crate::model::JobTemplate> {
            (*self.0.stub)
                .create_job_template(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateJobTemplateRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [job_template][crate::model::CreateJobTemplateRequest::job_template].
        pub fn set_job_template<T: Into<std::option::Option<crate::model::JobTemplate>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.job_template = v.into();
            self
        }

        /// Sets the value of [job_template_id][crate::model::CreateJobTemplateRequest::job_template_id].
        pub fn set_job_template_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.job_template_id = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateJobTemplate {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a TranscoderService::list_job_templates call.
    #[derive(Clone, Debug)]
    pub struct ListJobTemplates(RequestBuilder<crate::model::ListJobTemplatesRequest>);

    impl ListJobTemplates {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::TranscoderService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListJobTemplatesRequest>>(
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
        pub async fn send(self) -> Result<crate::model::ListJobTemplatesResponse> {
            (*self.0.stub)
                .list_job_templates(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListJobTemplatesResponse, gax::error::Error>
        {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListJobTemplatesRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListJobTemplatesRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListJobTemplatesRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListJobTemplatesRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [order_by][crate::model::ListJobTemplatesRequest::order_by].
        pub fn set_order_by<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.order_by = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListJobTemplates {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a TranscoderService::get_job_template call.
    #[derive(Clone, Debug)]
    pub struct GetJobTemplate(RequestBuilder<crate::model::GetJobTemplateRequest>);

    impl GetJobTemplate {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::TranscoderService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetJobTemplateRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::JobTemplate> {
            (*self.0.stub)
                .get_job_template(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetJobTemplateRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetJobTemplate {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a TranscoderService::delete_job_template call.
    #[derive(Clone, Debug)]
    pub struct DeleteJobTemplate(RequestBuilder<crate::model::DeleteJobTemplateRequest>);

    impl DeleteJobTemplate {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::TranscoderService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteJobTemplateRequest>>(
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
                .delete_job_template(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteJobTemplateRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [allow_missing][crate::model::DeleteJobTemplateRequest::allow_missing].
        pub fn set_allow_missing<T: Into<bool>>(mut self, v: T) -> Self {
            self.0.request.allow_missing = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteJobTemplate {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
