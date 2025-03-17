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

pub mod parameter_manager {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::ParameterManager] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::ParameterManager>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::ParameterManager>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a ParameterManager::list_parameters call.
    #[derive(Clone, Debug)]
    pub struct ListParameters(RequestBuilder<crate::model::ListParametersRequest>);

    impl ListParameters {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::ParameterManager>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListParametersRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListParametersResponse> {
            (*self.0.stub)
                .list_parameters(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListParametersResponse, gax::error::Error>
        {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListParametersRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListParametersRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListParametersRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListParametersRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [order_by][crate::model::ListParametersRequest::order_by].
        pub fn set_order_by<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.order_by = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListParameters {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ParameterManager::get_parameter call.
    #[derive(Clone, Debug)]
    pub struct GetParameter(RequestBuilder<crate::model::GetParameterRequest>);

    impl GetParameter {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::ParameterManager>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetParameterRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Parameter> {
            (*self.0.stub)
                .get_parameter(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetParameterRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetParameter {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ParameterManager::create_parameter call.
    #[derive(Clone, Debug)]
    pub struct CreateParameter(RequestBuilder<crate::model::CreateParameterRequest>);

    impl CreateParameter {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::ParameterManager>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateParameterRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Parameter> {
            (*self.0.stub)
                .create_parameter(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateParameterRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [parameter_id][crate::model::CreateParameterRequest::parameter_id].
        pub fn set_parameter_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parameter_id = v.into();
            self
        }

        /// Sets the value of [parameter][crate::model::CreateParameterRequest::parameter].
        pub fn set_parameter<T: Into<std::option::Option<crate::model::Parameter>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.parameter = v.into();
            self
        }

        /// Sets the value of [request_id][crate::model::CreateParameterRequest::request_id].
        pub fn set_request_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.request_id = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateParameter {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ParameterManager::update_parameter call.
    #[derive(Clone, Debug)]
    pub struct UpdateParameter(RequestBuilder<crate::model::UpdateParameterRequest>);

    impl UpdateParameter {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::ParameterManager>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateParameterRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Parameter> {
            (*self.0.stub)
                .update_parameter(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [update_mask][crate::model::UpdateParameterRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }

        /// Sets the value of [parameter][crate::model::UpdateParameterRequest::parameter].
        pub fn set_parameter<T: Into<std::option::Option<crate::model::Parameter>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.parameter = v.into();
            self
        }

        /// Sets the value of [request_id][crate::model::UpdateParameterRequest::request_id].
        pub fn set_request_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.request_id = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for UpdateParameter {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ParameterManager::delete_parameter call.
    #[derive(Clone, Debug)]
    pub struct DeleteParameter(RequestBuilder<crate::model::DeleteParameterRequest>);

    impl DeleteParameter {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::ParameterManager>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteParameterRequest>>(mut self, v: V) -> Self {
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
                .delete_parameter(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteParameterRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [request_id][crate::model::DeleteParameterRequest::request_id].
        pub fn set_request_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.request_id = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteParameter {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ParameterManager::list_parameter_versions call.
    #[derive(Clone, Debug)]
    pub struct ListParameterVersions(RequestBuilder<crate::model::ListParameterVersionsRequest>);

    impl ListParameterVersions {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::ParameterManager>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListParameterVersionsRequest>>(
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
        pub async fn send(self) -> Result<crate::model::ListParameterVersionsResponse> {
            (*self.0.stub)
                .list_parameter_versions(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListParameterVersionsResponse, gax::error::Error>
        {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListParameterVersionsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListParameterVersionsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListParameterVersionsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListParameterVersionsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [order_by][crate::model::ListParameterVersionsRequest::order_by].
        pub fn set_order_by<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.order_by = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListParameterVersions {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ParameterManager::get_parameter_version call.
    #[derive(Clone, Debug)]
    pub struct GetParameterVersion(RequestBuilder<crate::model::GetParameterVersionRequest>);

    impl GetParameterVersion {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::ParameterManager>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetParameterVersionRequest>>(
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
        pub async fn send(self) -> Result<crate::model::ParameterVersion> {
            (*self.0.stub)
                .get_parameter_version(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetParameterVersionRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [view][crate::model::GetParameterVersionRequest::view].
        pub fn set_view<T: Into<crate::model::View>>(mut self, v: T) -> Self {
            self.0.request.view = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetParameterVersion {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ParameterManager::render_parameter_version call.
    #[derive(Clone, Debug)]
    pub struct RenderParameterVersion(RequestBuilder<crate::model::RenderParameterVersionRequest>);

    impl RenderParameterVersion {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::ParameterManager>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::RenderParameterVersionRequest>>(
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
        pub async fn send(self) -> Result<crate::model::RenderParameterVersionResponse> {
            (*self.0.stub)
                .render_parameter_version(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::RenderParameterVersionRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for RenderParameterVersion {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ParameterManager::create_parameter_version call.
    #[derive(Clone, Debug)]
    pub struct CreateParameterVersion(RequestBuilder<crate::model::CreateParameterVersionRequest>);

    impl CreateParameterVersion {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::ParameterManager>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateParameterVersionRequest>>(
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
        pub async fn send(self) -> Result<crate::model::ParameterVersion> {
            (*self.0.stub)
                .create_parameter_version(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateParameterVersionRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [parameter_version_id][crate::model::CreateParameterVersionRequest::parameter_version_id].
        pub fn set_parameter_version_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parameter_version_id = v.into();
            self
        }

        /// Sets the value of [parameter_version][crate::model::CreateParameterVersionRequest::parameter_version].
        pub fn set_parameter_version<
            T: Into<std::option::Option<crate::model::ParameterVersion>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.parameter_version = v.into();
            self
        }

        /// Sets the value of [request_id][crate::model::CreateParameterVersionRequest::request_id].
        pub fn set_request_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.request_id = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateParameterVersion {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ParameterManager::update_parameter_version call.
    #[derive(Clone, Debug)]
    pub struct UpdateParameterVersion(RequestBuilder<crate::model::UpdateParameterVersionRequest>);

    impl UpdateParameterVersion {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::ParameterManager>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateParameterVersionRequest>>(
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
        pub async fn send(self) -> Result<crate::model::ParameterVersion> {
            (*self.0.stub)
                .update_parameter_version(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [update_mask][crate::model::UpdateParameterVersionRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }

        /// Sets the value of [parameter_version][crate::model::UpdateParameterVersionRequest::parameter_version].
        pub fn set_parameter_version<
            T: Into<std::option::Option<crate::model::ParameterVersion>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.parameter_version = v.into();
            self
        }

        /// Sets the value of [request_id][crate::model::UpdateParameterVersionRequest::request_id].
        pub fn set_request_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.request_id = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for UpdateParameterVersion {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ParameterManager::delete_parameter_version call.
    #[derive(Clone, Debug)]
    pub struct DeleteParameterVersion(RequestBuilder<crate::model::DeleteParameterVersionRequest>);

    impl DeleteParameterVersion {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::ParameterManager>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteParameterVersionRequest>>(
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
                .delete_parameter_version(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteParameterVersionRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [request_id][crate::model::DeleteParameterVersionRequest::request_id].
        pub fn set_request_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.request_id = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteParameterVersion {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ParameterManager::list_locations call.
    #[derive(Clone, Debug)]
    pub struct ListLocations(RequestBuilder<location::model::ListLocationsRequest>);

    impl ListLocations {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::ParameterManager>) -> Self {
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
        ) -> gax::paginator::Paginator<location::model::ListLocationsResponse, gax::error::Error>
        {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
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

    impl gax::options::RequestBuilder for ListLocations {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ParameterManager::get_location call.
    #[derive(Clone, Debug)]
    pub struct GetLocation(RequestBuilder<location::model::GetLocationRequest>);

    impl GetLocation {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::ParameterManager>) -> Self {
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

    impl gax::options::RequestBuilder for GetLocation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
