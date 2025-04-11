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

pub mod g_suite_add_ons {
    use crate::Result;
    use std::sync::Arc;

    /// A builder for [GSuiteAddOns][super::super::client::GSuiteAddOns].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_gsuiteaddons_v1::*;
    /// # use builder::g_suite_add_ons::ClientBuilder;
    /// # use client::GSuiteAddOns;
    /// let builder : ClientBuilder = GSuiteAddOns::builder();
    /// let client = builder
    ///     .with_endpoint("https://gsuiteaddons.googleapis.com")
    ///     .build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::GSuiteAddOns;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = GSuiteAddOns;
            type Credentials = gaxi::options::Credentials;
            async fn build(self, config: gaxi::options::ClientConfig) -> gax::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [super::super::client::GSuiteAddOns] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stub::dynamic::GSuiteAddOns>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::GSuiteAddOns>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [GSuiteAddOns::get_authorization][super::super::client::GSuiteAddOns::get_authorization] calls.
    #[derive(Clone, Debug)]
    pub struct GetAuthorization(RequestBuilder<crate::model::GetAuthorizationRequest>);

    impl GetAuthorization {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::GSuiteAddOns>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetAuthorizationRequest>>(
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
        pub async fn send(self) -> Result<crate::model::Authorization> {
            (*self.0.stub)
                .get_authorization(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::GetAuthorizationRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetAuthorization {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [GSuiteAddOns::create_deployment][super::super::client::GSuiteAddOns::create_deployment] calls.
    #[derive(Clone, Debug)]
    pub struct CreateDeployment(RequestBuilder<crate::model::CreateDeploymentRequest>);

    impl CreateDeployment {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::GSuiteAddOns>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateDeploymentRequest>>(
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
        pub async fn send(self) -> Result<crate::model::Deployment> {
            (*self.0.stub)
                .create_deployment(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [parent][crate::model::CreateDeploymentRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [deployment_id][crate::model::CreateDeploymentRequest::deployment_id].
        pub fn set_deployment_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.deployment_id = v.into();
            self
        }

        /// Sets the value of [deployment][crate::model::CreateDeploymentRequest::deployment].
        pub fn set_deployment<T: Into<std::option::Option<crate::model::Deployment>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.deployment = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for CreateDeployment {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [GSuiteAddOns::replace_deployment][super::super::client::GSuiteAddOns::replace_deployment] calls.
    #[derive(Clone, Debug)]
    pub struct ReplaceDeployment(RequestBuilder<crate::model::ReplaceDeploymentRequest>);

    impl ReplaceDeployment {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::GSuiteAddOns>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ReplaceDeploymentRequest>>(
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
        pub async fn send(self) -> Result<crate::model::Deployment> {
            (*self.0.stub)
                .replace_deployment(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [deployment][crate::model::ReplaceDeploymentRequest::deployment].
        pub fn set_deployment<T: Into<std::option::Option<crate::model::Deployment>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.deployment = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ReplaceDeployment {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [GSuiteAddOns::get_deployment][super::super::client::GSuiteAddOns::get_deployment] calls.
    #[derive(Clone, Debug)]
    pub struct GetDeployment(RequestBuilder<crate::model::GetDeploymentRequest>);

    impl GetDeployment {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::GSuiteAddOns>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetDeploymentRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Deployment> {
            (*self.0.stub)
                .get_deployment(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::GetDeploymentRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetDeployment {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [GSuiteAddOns::list_deployments][super::super::client::GSuiteAddOns::list_deployments] calls.
    #[derive(Clone, Debug)]
    pub struct ListDeployments(RequestBuilder<crate::model::ListDeploymentsRequest>);

    impl ListDeployments {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::GSuiteAddOns>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListDeploymentsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListDeploymentsResponse> {
            (*self.0.stub)
                .list_deployments(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> impl gax::paginator::Paginator<crate::model::ListDeploymentsResponse, gax::error::Error>
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

        /// Sets the value of [parent][crate::model::ListDeploymentsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListDeploymentsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListDeploymentsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListDeployments {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [GSuiteAddOns::delete_deployment][super::super::client::GSuiteAddOns::delete_deployment] calls.
    #[derive(Clone, Debug)]
    pub struct DeleteDeployment(RequestBuilder<crate::model::DeleteDeploymentRequest>);

    impl DeleteDeployment {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::GSuiteAddOns>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteDeploymentRequest>>(
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
                .delete_deployment(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::DeleteDeploymentRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [etag][crate::model::DeleteDeploymentRequest::etag].
        pub fn set_etag<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.etag = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for DeleteDeployment {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [GSuiteAddOns::install_deployment][super::super::client::GSuiteAddOns::install_deployment] calls.
    #[derive(Clone, Debug)]
    pub struct InstallDeployment(RequestBuilder<crate::model::InstallDeploymentRequest>);

    impl InstallDeployment {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::GSuiteAddOns>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::InstallDeploymentRequest>>(
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
                .install_deployment(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::InstallDeploymentRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for InstallDeployment {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [GSuiteAddOns::uninstall_deployment][super::super::client::GSuiteAddOns::uninstall_deployment] calls.
    #[derive(Clone, Debug)]
    pub struct UninstallDeployment(RequestBuilder<crate::model::UninstallDeploymentRequest>);

    impl UninstallDeployment {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::GSuiteAddOns>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UninstallDeploymentRequest>>(
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
                .uninstall_deployment(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::UninstallDeploymentRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for UninstallDeployment {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [GSuiteAddOns::get_install_status][super::super::client::GSuiteAddOns::get_install_status] calls.
    #[derive(Clone, Debug)]
    pub struct GetInstallStatus(RequestBuilder<crate::model::GetInstallStatusRequest>);

    impl GetInstallStatus {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::GSuiteAddOns>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetInstallStatusRequest>>(
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
        pub async fn send(self) -> Result<crate::model::InstallStatus> {
            (*self.0.stub)
                .get_install_status(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::GetInstallStatusRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetInstallStatus {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
