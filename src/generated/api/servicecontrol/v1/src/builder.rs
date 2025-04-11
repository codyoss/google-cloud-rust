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

pub mod quota_controller {
    use crate::Result;
    use std::sync::Arc;

    /// A builder for [QuotaController][super::super::client::QuotaController].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_api_servicecontrol_v1::*;
    /// # use builder::quota_controller::ClientBuilder;
    /// # use client::QuotaController;
    /// let builder : ClientBuilder = QuotaController::builder();
    /// let client = builder
    ///     .with_endpoint("https://servicecontrol.googleapis.com")
    ///     .build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::QuotaController;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = QuotaController;
            type Credentials = gaxi::options::Credentials;
            async fn build(self, config: gaxi::options::ClientConfig) -> gax::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [super::super::client::QuotaController] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stub::dynamic::QuotaController>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::QuotaController>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [QuotaController::allocate_quota][super::super::client::QuotaController::allocate_quota] calls.
    #[derive(Clone, Debug)]
    pub struct AllocateQuota(RequestBuilder<crate::model::AllocateQuotaRequest>);

    impl AllocateQuota {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::QuotaController>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::AllocateQuotaRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::AllocateQuotaResponse> {
            (*self.0.stub)
                .allocate_quota(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [service_name][crate::model::AllocateQuotaRequest::service_name].
        pub fn set_service_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.service_name = v.into();
            self
        }

        /// Sets the value of [allocate_operation][crate::model::AllocateQuotaRequest::allocate_operation].
        pub fn set_allocate_operation<
            T: Into<std::option::Option<crate::model::QuotaOperation>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.allocate_operation = v.into();
            self
        }

        /// Sets the value of [service_config_id][crate::model::AllocateQuotaRequest::service_config_id].
        pub fn set_service_config_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.service_config_id = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for AllocateQuota {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}

pub mod service_controller {
    use crate::Result;
    use std::sync::Arc;

    /// A builder for [ServiceController][super::super::client::ServiceController].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_api_servicecontrol_v1::*;
    /// # use builder::service_controller::ClientBuilder;
    /// # use client::ServiceController;
    /// let builder : ClientBuilder = ServiceController::builder();
    /// let client = builder
    ///     .with_endpoint("https://servicecontrol.googleapis.com")
    ///     .build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::ServiceController;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = ServiceController;
            type Credentials = gaxi::options::Credentials;
            async fn build(self, config: gaxi::options::ClientConfig) -> gax::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [super::super::client::ServiceController] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stub::dynamic::ServiceController>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::ServiceController>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [ServiceController::check][super::super::client::ServiceController::check] calls.
    #[derive(Clone, Debug)]
    pub struct Check(RequestBuilder<crate::model::CheckRequest>);

    impl Check {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::ServiceController>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CheckRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::CheckResponse> {
            (*self.0.stub)
                .check(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [service_name][crate::model::CheckRequest::service_name].
        pub fn set_service_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.service_name = v.into();
            self
        }

        /// Sets the value of [operation][crate::model::CheckRequest::operation].
        pub fn set_operation<T: Into<std::option::Option<crate::model::Operation>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.operation = v.into();
            self
        }

        /// Sets the value of [service_config_id][crate::model::CheckRequest::service_config_id].
        pub fn set_service_config_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.service_config_id = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for Check {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [ServiceController::report][super::super::client::ServiceController::report] calls.
    #[derive(Clone, Debug)]
    pub struct Report(RequestBuilder<crate::model::ReportRequest>);

    impl Report {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::ServiceController>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ReportRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ReportResponse> {
            (*self.0.stub)
                .report(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [service_name][crate::model::ReportRequest::service_name].
        pub fn set_service_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.service_name = v.into();
            self
        }

        /// Sets the value of [service_config_id][crate::model::ReportRequest::service_config_id].
        pub fn set_service_config_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.service_config_id = v.into();
            self
        }

        /// Sets the value of [operations][crate::model::ReportRequest::operations].
        pub fn set_operations<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<crate::model::Operation>,
        {
            use std::iter::Iterator;
            self.0.request.operations = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for Report {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
