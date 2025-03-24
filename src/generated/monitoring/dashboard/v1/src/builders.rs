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

pub mod dashboards_service {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::DashboardsService] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::DashboardsService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::DashboardsService>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a DashboardsService::create_dashboard call.
    #[derive(Clone, Debug)]
    pub struct CreateDashboard(RequestBuilder<crate::model::CreateDashboardRequest>);

    impl CreateDashboard {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::DashboardsService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateDashboardRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Dashboard> {
            (*self.0.stub)
                .create_dashboard(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateDashboardRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [dashboard][crate::model::CreateDashboardRequest::dashboard].
        pub fn set_dashboard<T: Into<std::option::Option<crate::model::Dashboard>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.dashboard = v.into();
            self
        }

        /// Sets the value of [validate_only][crate::model::CreateDashboardRequest::validate_only].
        pub fn set_validate_only<T: Into<bool>>(mut self, v: T) -> Self {
            self.0.request.validate_only = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateDashboard {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a DashboardsService::list_dashboards call.
    #[derive(Clone, Debug)]
    pub struct ListDashboards(RequestBuilder<crate::model::ListDashboardsRequest>);

    impl ListDashboards {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::DashboardsService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListDashboardsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListDashboardsResponse> {
            (*self.0.stub)
                .list_dashboards(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListDashboardsResponse, gax::error::Error>
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

        /// Sets the value of [parent][crate::model::ListDashboardsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListDashboardsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListDashboardsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListDashboards {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a DashboardsService::get_dashboard call.
    #[derive(Clone, Debug)]
    pub struct GetDashboard(RequestBuilder<crate::model::GetDashboardRequest>);

    impl GetDashboard {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::DashboardsService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetDashboardRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Dashboard> {
            (*self.0.stub)
                .get_dashboard(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetDashboardRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetDashboard {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a DashboardsService::delete_dashboard call.
    #[derive(Clone, Debug)]
    pub struct DeleteDashboard(RequestBuilder<crate::model::DeleteDashboardRequest>);

    impl DeleteDashboard {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::DashboardsService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteDashboardRequest>>(mut self, v: V) -> Self {
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
                .delete_dashboard(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteDashboardRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteDashboard {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a DashboardsService::update_dashboard call.
    #[derive(Clone, Debug)]
    pub struct UpdateDashboard(RequestBuilder<crate::model::UpdateDashboardRequest>);

    impl UpdateDashboard {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::DashboardsService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateDashboardRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Dashboard> {
            (*self.0.stub)
                .update_dashboard(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [dashboard][crate::model::UpdateDashboardRequest::dashboard].
        pub fn set_dashboard<T: Into<std::option::Option<crate::model::Dashboard>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.dashboard = v.into();
            self
        }

        /// Sets the value of [validate_only][crate::model::UpdateDashboardRequest::validate_only].
        pub fn set_validate_only<T: Into<bool>>(mut self, v: T) -> Self {
            self.0.request.validate_only = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for UpdateDashboard {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
