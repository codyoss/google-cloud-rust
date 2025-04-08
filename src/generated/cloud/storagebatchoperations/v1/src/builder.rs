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

pub mod storage_batch_operations {
    use crate::Result;
    use std::sync::Arc;

    /// A builder for [StorageBatchOperations][super::super::client::StorageBatchOperations].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_storagebatchoperations_v1::*;
    /// # use builder::storage_batch_operations::ClientBuilder;
    /// # use client::StorageBatchOperations;
    /// let builder : ClientBuilder = StorageBatchOperations::builder();
    /// let client = builder
    ///     .with_endpoint("https://storagebatchoperations.googleapis.com")
    ///     .build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::StorageBatchOperations;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = StorageBatchOperations;
            type Credentials = gaxi::options::Credentials;
            async fn build(self, config: gaxi::options::ClientConfig) -> gax::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [super::super::client::StorageBatchOperations] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stub::dynamic::StorageBatchOperations>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stub::dynamic::StorageBatchOperations>,
        ) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [StorageBatchOperations::list_jobs][super::super::client::StorageBatchOperations::list_jobs] calls.
    #[derive(Clone, Debug)]
    pub struct ListJobs(RequestBuilder<crate::model::ListJobsRequest>);

    impl ListJobs {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stub::dynamic::StorageBatchOperations>,
        ) -> Self {
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
        ) -> impl gax::paginator::Paginator<crate::model::ListJobsResponse, gax::error::Error>
        {
            use std::clone::Clone;
            let token = self.0.request.page_token.clone();
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::internal::new_paginator(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListJobsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListJobsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
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

        /// Sets the value of [order_by][crate::model::ListJobsRequest::order_by].
        pub fn set_order_by<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.order_by = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListJobs {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [StorageBatchOperations::get_job][super::super::client::StorageBatchOperations::get_job] calls.
    #[derive(Clone, Debug)]
    pub struct GetJob(RequestBuilder<crate::model::GetJobRequest>);

    impl GetJob {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stub::dynamic::StorageBatchOperations>,
        ) -> Self {
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

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetJob {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [StorageBatchOperations::create_job][super::super::client::StorageBatchOperations::create_job] calls.
    #[derive(Clone, Debug)]
    pub struct CreateJob(RequestBuilder<crate::model::CreateJobRequest>);

    impl CreateJob {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stub::dynamic::StorageBatchOperations>,
        ) -> Self {
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
        ///
        /// # Long running operations
        ///
        /// This starts, but does not poll, a longrunning operation. More information
        /// on [create_job][super::super::client::StorageBatchOperations::create_job].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .create_job(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `create_job`.
        pub fn poller(
            self,
        ) -> impl lro::Poller<crate::model::Job, crate::model::OperationMetadata> {
            type Operation = lro::Operation<crate::model::Job, crate::model::OperationMetadata>;
            let polling_error_policy = self.0.stub.get_polling_error_policy(&self.0.options);
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

            lro::new_poller(polling_error_policy, polling_backoff_policy, start, query)
        }

        /// Sets the value of [parent][crate::model::CreateJobRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [job_id][crate::model::CreateJobRequest::job_id].
        pub fn set_job_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.job_id = v.into();
            self
        }

        /// Sets the value of [job][crate::model::CreateJobRequest::job].
        pub fn set_job<T: Into<std::option::Option<crate::model::Job>>>(mut self, v: T) -> Self {
            self.0.request.job = v.into();
            self
        }

        /// Sets the value of [request_id][crate::model::CreateJobRequest::request_id].
        pub fn set_request_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.request_id = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for CreateJob {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [StorageBatchOperations::delete_job][super::super::client::StorageBatchOperations::delete_job] calls.
    #[derive(Clone, Debug)]
    pub struct DeleteJob(RequestBuilder<crate::model::DeleteJobRequest>);

    impl DeleteJob {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stub::dynamic::StorageBatchOperations>,
        ) -> Self {
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
        pub async fn send(self) -> Result<()> {
            (*self.0.stub)
                .delete_job(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteJobRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [request_id][crate::model::DeleteJobRequest::request_id].
        pub fn set_request_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.request_id = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for DeleteJob {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [StorageBatchOperations::cancel_job][super::super::client::StorageBatchOperations::cancel_job] calls.
    #[derive(Clone, Debug)]
    pub struct CancelJob(RequestBuilder<crate::model::CancelJobRequest>);

    impl CancelJob {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stub::dynamic::StorageBatchOperations>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CancelJobRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::CancelJobResponse> {
            (*self.0.stub)
                .cancel_job(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::CancelJobRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [request_id][crate::model::CancelJobRequest::request_id].
        pub fn set_request_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.request_id = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for CancelJob {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [StorageBatchOperations::list_locations][super::super::client::StorageBatchOperations::list_locations] calls.
    #[derive(Clone, Debug)]
    pub struct ListLocations(RequestBuilder<location::model::ListLocationsRequest>);

    impl ListLocations {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stub::dynamic::StorageBatchOperations>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<location::model::ListLocationsRequest>>(
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
        pub async fn send(self) -> Result<location::model::ListLocationsResponse> {
            (*self.0.stub)
                .list_locations(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> impl gax::paginator::Paginator<location::model::ListLocationsResponse, gax::error::Error>
        {
            use std::clone::Clone;
            let token = self.0.request.page_token.clone();
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::internal::new_paginator(token, execute)
        }

        /// Sets the value of [name][location::model::ListLocationsRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [filter][location::model::ListLocationsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [page_size][location::model::ListLocationsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][location::model::ListLocationsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListLocations {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [StorageBatchOperations::get_location][super::super::client::StorageBatchOperations::get_location] calls.
    #[derive(Clone, Debug)]
    pub struct GetLocation(RequestBuilder<location::model::GetLocationRequest>);

    impl GetLocation {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stub::dynamic::StorageBatchOperations>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<location::model::GetLocationRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<location::model::Location> {
            (*self.0.stub)
                .get_location(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][location::model::GetLocationRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetLocation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [StorageBatchOperations::list_operations][super::super::client::StorageBatchOperations::list_operations] calls.
    #[derive(Clone, Debug)]
    pub struct ListOperations(RequestBuilder<longrunning::model::ListOperationsRequest>);

    impl ListOperations {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stub::dynamic::StorageBatchOperations>,
        ) -> Self {
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
        ) -> impl gax::paginator::Paginator<longrunning::model::ListOperationsResponse, gax::error::Error>
        {
            use std::clone::Clone;
            let token = self.0.request.page_token.clone();
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::internal::new_paginator(token, execute)
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

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListOperations {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [StorageBatchOperations::get_operation][super::super::client::StorageBatchOperations::get_operation] calls.
    #[derive(Clone, Debug)]
    pub struct GetOperation(RequestBuilder<longrunning::model::GetOperationRequest>);

    impl GetOperation {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stub::dynamic::StorageBatchOperations>,
        ) -> Self {
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

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetOperation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [StorageBatchOperations::delete_operation][super::super::client::StorageBatchOperations::delete_operation] calls.
    #[derive(Clone, Debug)]
    pub struct DeleteOperation(RequestBuilder<longrunning::model::DeleteOperationRequest>);

    impl DeleteOperation {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stub::dynamic::StorageBatchOperations>,
        ) -> Self {
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
        pub async fn send(self) -> Result<()> {
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

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for DeleteOperation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [StorageBatchOperations::cancel_operation][super::super::client::StorageBatchOperations::cancel_operation] calls.
    #[derive(Clone, Debug)]
    pub struct CancelOperation(RequestBuilder<longrunning::model::CancelOperationRequest>);

    impl CancelOperation {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stub::dynamic::StorageBatchOperations>,
        ) -> Self {
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
        pub async fn send(self) -> Result<()> {
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

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for CancelOperation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
