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

pub mod api_keys {
    use crate::Result;
    use std::sync::Arc;

    /// A builder for [ApiKeys][super::super::client::ApiKeys].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_apikeys_v2::*;
    /// # use builder::api_keys::ClientBuilder;
    /// # use client::ApiKeys;
    /// let builder : ClientBuilder = ApiKeys::builder();
    /// let client = builder
    ///     .with_endpoint("https://apikeys.googleapis.com")
    ///     .build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::ApiKeys;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = ApiKeys;
            type Credentials = gaxi::options::Credentials;
            async fn build(self, config: gaxi::options::ClientConfig) -> gax::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [super::super::client::ApiKeys] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stub::dynamic::ApiKeys>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::ApiKeys>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [ApiKeys::create_key][super::super::client::ApiKeys::create_key] calls.
    #[derive(Clone, Debug)]
    pub struct CreateKey(RequestBuilder<crate::model::CreateKeyRequest>);

    impl CreateKey {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::ApiKeys>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateKeyRequest>>(mut self, v: V) -> Self {
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
        /// on [create_key][super::super::client::ApiKeys::create_key].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .create_key(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `create_key`.
        pub fn poller(self) -> impl lro::Poller<crate::model::Key, wkt::Empty> {
            type Operation = lro::Operation<crate::model::Key, wkt::Empty>;
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

        /// Sets the value of [parent][crate::model::CreateKeyRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [key][crate::model::CreateKeyRequest::key].
        pub fn set_key<T: Into<std::option::Option<crate::model::Key>>>(mut self, v: T) -> Self {
            self.0.request.key = v.into();
            self
        }

        /// Sets the value of [key_id][crate::model::CreateKeyRequest::key_id].
        pub fn set_key_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.key_id = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for CreateKey {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [ApiKeys::list_keys][super::super::client::ApiKeys::list_keys] calls.
    #[derive(Clone, Debug)]
    pub struct ListKeys(RequestBuilder<crate::model::ListKeysRequest>);

    impl ListKeys {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::ApiKeys>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListKeysRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListKeysResponse> {
            (*self.0.stub)
                .list_keys(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> impl gax::paginator::Paginator<crate::model::ListKeysResponse, gax::error::Error>
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

        /// Sets the value of [parent][crate::model::ListKeysRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListKeysRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListKeysRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [show_deleted][crate::model::ListKeysRequest::show_deleted].
        pub fn set_show_deleted<T: Into<bool>>(mut self, v: T) -> Self {
            self.0.request.show_deleted = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListKeys {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [ApiKeys::get_key][super::super::client::ApiKeys::get_key] calls.
    #[derive(Clone, Debug)]
    pub struct GetKey(RequestBuilder<crate::model::GetKeyRequest>);

    impl GetKey {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::ApiKeys>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetKeyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Key> {
            (*self.0.stub).get_key(self.0.request, self.0.options).await
        }

        /// Sets the value of [name][crate::model::GetKeyRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetKey {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [ApiKeys::get_key_string][super::super::client::ApiKeys::get_key_string] calls.
    #[derive(Clone, Debug)]
    pub struct GetKeyString(RequestBuilder<crate::model::GetKeyStringRequest>);

    impl GetKeyString {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::ApiKeys>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetKeyStringRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::GetKeyStringResponse> {
            (*self.0.stub)
                .get_key_string(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetKeyStringRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetKeyString {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [ApiKeys::update_key][super::super::client::ApiKeys::update_key] calls.
    #[derive(Clone, Debug)]
    pub struct UpdateKey(RequestBuilder<crate::model::UpdateKeyRequest>);

    impl UpdateKey {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::ApiKeys>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateKeyRequest>>(mut self, v: V) -> Self {
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
        /// on [update_key][super::super::client::ApiKeys::update_key].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .update_key(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `update_key`.
        pub fn poller(self) -> impl lro::Poller<crate::model::Key, wkt::Empty> {
            type Operation = lro::Operation<crate::model::Key, wkt::Empty>;
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

        /// Sets the value of [key][crate::model::UpdateKeyRequest::key].
        pub fn set_key<T: Into<std::option::Option<crate::model::Key>>>(mut self, v: T) -> Self {
            self.0.request.key = v.into();
            self
        }

        /// Sets the value of [update_mask][crate::model::UpdateKeyRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for UpdateKey {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [ApiKeys::delete_key][super::super::client::ApiKeys::delete_key] calls.
    #[derive(Clone, Debug)]
    pub struct DeleteKey(RequestBuilder<crate::model::DeleteKeyRequest>);

    impl DeleteKey {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::ApiKeys>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteKeyRequest>>(mut self, v: V) -> Self {
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
        /// on [delete_key][super::super::client::ApiKeys::delete_key].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .delete_key(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `delete_key`.
        pub fn poller(self) -> impl lro::Poller<crate::model::Key, wkt::Empty> {
            type Operation = lro::Operation<crate::model::Key, wkt::Empty>;
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

        /// Sets the value of [name][crate::model::DeleteKeyRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [etag][crate::model::DeleteKeyRequest::etag].
        pub fn set_etag<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.etag = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for DeleteKey {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [ApiKeys::undelete_key][super::super::client::ApiKeys::undelete_key] calls.
    #[derive(Clone, Debug)]
    pub struct UndeleteKey(RequestBuilder<crate::model::UndeleteKeyRequest>);

    impl UndeleteKey {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::ApiKeys>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UndeleteKeyRequest>>(mut self, v: V) -> Self {
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
        /// on [undelete_key][super::super::client::ApiKeys::undelete_key].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .undelete_key(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `undelete_key`.
        pub fn poller(self) -> impl lro::Poller<crate::model::Key, wkt::Empty> {
            type Operation = lro::Operation<crate::model::Key, wkt::Empty>;
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

        /// Sets the value of [name][crate::model::UndeleteKeyRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for UndeleteKey {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [ApiKeys::lookup_key][super::super::client::ApiKeys::lookup_key] calls.
    #[derive(Clone, Debug)]
    pub struct LookupKey(RequestBuilder<crate::model::LookupKeyRequest>);

    impl LookupKey {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::ApiKeys>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::LookupKeyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::LookupKeyResponse> {
            (*self.0.stub)
                .lookup_key(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [key_string][crate::model::LookupKeyRequest::key_string].
        pub fn set_key_string<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.key_string = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for LookupKey {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [ApiKeys::get_operation][super::super::client::ApiKeys::get_operation] calls.
    #[derive(Clone, Debug)]
    pub struct GetOperation(RequestBuilder<longrunning::model::GetOperationRequest>);

    impl GetOperation {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::ApiKeys>) -> Self {
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
}
