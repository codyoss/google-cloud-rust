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

pub mod assured_workloads_service {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::AssuredWorkloadsService] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::AssuredWorkloadsService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::AssuredWorkloadsService>,
        ) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a AssuredWorkloadsService::create_workload call.
    #[derive(Clone, Debug)]
    pub struct CreateWorkload(RequestBuilder<crate::model::CreateWorkloadRequest>);

    impl CreateWorkload {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::AssuredWorkloadsService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateWorkloadRequest>>(mut self, v: V) -> Self {
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
        /// on [create_workload][super::super::client::AssuredWorkloadsService::create_workload].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .create_workload(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `create_workload`.
        pub fn poller(
            self,
        ) -> impl lro::Poller<crate::model::Workload, crate::model::CreateWorkloadOperationMetadata>
        {
            type Operation = lro::Operation<
                crate::model::Workload,
                crate::model::CreateWorkloadOperationMetadata,
            >;
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

        /// Sets the value of [parent][crate::model::CreateWorkloadRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [workload][crate::model::CreateWorkloadRequest::workload].
        pub fn set_workload<T: Into<std::option::Option<crate::model::Workload>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.workload = v.into();
            self
        }

        /// Sets the value of [external_id][crate::model::CreateWorkloadRequest::external_id].
        pub fn set_external_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.external_id = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateWorkload {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a AssuredWorkloadsService::update_workload call.
    #[derive(Clone, Debug)]
    pub struct UpdateWorkload(RequestBuilder<crate::model::UpdateWorkloadRequest>);

    impl UpdateWorkload {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::AssuredWorkloadsService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateWorkloadRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Workload> {
            (*self.0.stub)
                .update_workload(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [workload][crate::model::UpdateWorkloadRequest::workload].
        pub fn set_workload<T: Into<std::option::Option<crate::model::Workload>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.workload = v.into();
            self
        }

        /// Sets the value of [update_mask][crate::model::UpdateWorkloadRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for UpdateWorkload {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a AssuredWorkloadsService::restrict_allowed_resources call.
    #[derive(Clone, Debug)]
    pub struct RestrictAllowedResources(
        RequestBuilder<crate::model::RestrictAllowedResourcesRequest>,
    );

    impl RestrictAllowedResources {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::AssuredWorkloadsService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::RestrictAllowedResourcesRequest>>(
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
        pub async fn send(self) -> Result<crate::model::RestrictAllowedResourcesResponse> {
            (*self.0.stub)
                .restrict_allowed_resources(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::RestrictAllowedResourcesRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [restriction_type][crate::model::RestrictAllowedResourcesRequest::restriction_type].
        pub fn set_restriction_type<
            T: Into<crate::model::restrict_allowed_resources_request::RestrictionType>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.restriction_type = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for RestrictAllowedResources {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a AssuredWorkloadsService::delete_workload call.
    #[derive(Clone, Debug)]
    pub struct DeleteWorkload(RequestBuilder<crate::model::DeleteWorkloadRequest>);

    impl DeleteWorkload {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::AssuredWorkloadsService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteWorkloadRequest>>(mut self, v: V) -> Self {
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
                .delete_workload(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteWorkloadRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [etag][crate::model::DeleteWorkloadRequest::etag].
        pub fn set_etag<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.etag = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteWorkload {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a AssuredWorkloadsService::get_workload call.
    #[derive(Clone, Debug)]
    pub struct GetWorkload(RequestBuilder<crate::model::GetWorkloadRequest>);

    impl GetWorkload {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::AssuredWorkloadsService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetWorkloadRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Workload> {
            (*self.0.stub)
                .get_workload(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetWorkloadRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetWorkload {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a AssuredWorkloadsService::list_workloads call.
    #[derive(Clone, Debug)]
    pub struct ListWorkloads(RequestBuilder<crate::model::ListWorkloadsRequest>);

    impl ListWorkloads {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::AssuredWorkloadsService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListWorkloadsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListWorkloadsResponse> {
            (*self.0.stub)
                .list_workloads(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListWorkloadsResponse, gax::error::Error>
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

        /// Sets the value of [parent][crate::model::ListWorkloadsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListWorkloadsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListWorkloadsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListWorkloadsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListWorkloads {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a AssuredWorkloadsService::list_operations call.
    #[derive(Clone, Debug)]
    pub struct ListOperations(RequestBuilder<longrunning::model::ListOperationsRequest>);

    impl ListOperations {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::AssuredWorkloadsService>,
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
        ) -> gax::paginator::Paginator<longrunning::model::ListOperationsResponse, gax::error::Error>
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

    /// The request builder for a AssuredWorkloadsService::get_operation call.
    #[derive(Clone, Debug)]
    pub struct GetOperation(RequestBuilder<longrunning::model::GetOperationRequest>);

    impl GetOperation {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::AssuredWorkloadsService>,
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

    impl gax::options::RequestBuilder for GetOperation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
