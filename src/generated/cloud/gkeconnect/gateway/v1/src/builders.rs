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

pub mod gateway_control {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::GatewayControl] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::GatewayControl>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::GatewayControl>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a GatewayControl::generate_credentials call.
    #[derive(Clone, Debug)]
    pub struct GenerateCredentials(RequestBuilder<crate::model::GenerateCredentialsRequest>);

    impl GenerateCredentials {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::GatewayControl>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GenerateCredentialsRequest>>(
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
        pub async fn send(self) -> Result<crate::model::GenerateCredentialsResponse> {
            (*self.0.stub)
                .generate_credentials(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GenerateCredentialsRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [force_use_agent][crate::model::GenerateCredentialsRequest::force_use_agent].
        pub fn set_force_use_agent<T: Into<bool>>(mut self, v: T) -> Self {
            self.0.request.force_use_agent = v.into();
            self
        }

        /// Sets the value of [version][crate::model::GenerateCredentialsRequest::version].
        pub fn set_version<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.version = v.into();
            self
        }

        /// Sets the value of [kubernetes_namespace][crate::model::GenerateCredentialsRequest::kubernetes_namespace].
        pub fn set_kubernetes_namespace<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.kubernetes_namespace = v.into();
            self
        }

        /// Sets the value of [operating_system][crate::model::GenerateCredentialsRequest::operating_system].
        pub fn set_operating_system<
            T: Into<crate::model::generate_credentials_request::OperatingSystem>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.operating_system = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GenerateCredentials {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
