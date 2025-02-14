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

pub mod profiler_service {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [crate::client::ProfilerService] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn crate::stubs::dynamic::ProfilerService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ProfilerService>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a ProfilerService::create_profile call.
    #[derive(Clone, Debug)]
    pub struct CreateProfile(RequestBuilder<crate::model::CreateProfileRequest>);

    impl CreateProfile {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ProfilerService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateProfileRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Profile> {
            (*self.0.stub)
                .create_profile(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateProfileRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [deployment][crate::model::CreateProfileRequest::deployment].
        pub fn set_deployment<T: Into<std::option::Option<crate::model::Deployment>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.deployment = v.into();
            self
        }

        /// Sets the value of [profile_type][crate::model::CreateProfileRequest::profile_type].
        pub fn set_profile_type<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<crate::model::ProfileType>,
        {
            use std::iter::Iterator;
            self.0.request.profile_type = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateProfile {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ProfilerService::create_offline_profile call.
    #[derive(Clone, Debug)]
    pub struct CreateOfflineProfile(RequestBuilder<crate::model::CreateOfflineProfileRequest>);

    impl CreateOfflineProfile {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ProfilerService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateOfflineProfileRequest>>(
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
        pub async fn send(self) -> Result<crate::model::Profile> {
            (*self.0.stub)
                .create_offline_profile(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateOfflineProfileRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [profile][crate::model::CreateOfflineProfileRequest::profile].
        pub fn set_profile<T: Into<std::option::Option<crate::model::Profile>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.profile = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateOfflineProfile {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ProfilerService::update_profile call.
    #[derive(Clone, Debug)]
    pub struct UpdateProfile(RequestBuilder<crate::model::UpdateProfileRequest>);

    impl UpdateProfile {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ProfilerService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateProfileRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Profile> {
            (*self.0.stub)
                .update_profile(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [profile][crate::model::UpdateProfileRequest::profile].
        pub fn set_profile<T: Into<std::option::Option<crate::model::Profile>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.profile = v.into();
            self
        }

        /// Sets the value of [update_mask][crate::model::UpdateProfileRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for UpdateProfile {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}

pub mod export_service {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [crate::client::ExportService] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn crate::stubs::dynamic::ExportService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ExportService>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a ExportService::list_profiles call.
    #[derive(Clone, Debug)]
    pub struct ListProfiles(RequestBuilder<crate::model::ListProfilesRequest>);

    impl ListProfiles {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ExportService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListProfilesRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListProfilesResponse> {
            (*self.0.stub)
                .list_profiles(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        #[cfg(feature = "unstable-stream")]
        pub async fn stream(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListProfilesResponse, gax::error::Error>
        {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListProfilesRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListProfilesRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListProfilesRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListProfiles {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
