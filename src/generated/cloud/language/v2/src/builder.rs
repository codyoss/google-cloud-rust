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

pub mod language_service {
    use crate::Result;
    use std::sync::Arc;

    /// A builder for [LanguageService][super::super::client::LanguageService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_language_v2::*;
    /// # use builder::language_service::ClientBuilder;
    /// # use client::LanguageService;
    /// let builder : ClientBuilder = LanguageService::builder();
    /// let client = builder
    ///     .with_endpoint("https://language.googleapis.com")
    ///     .build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::LanguageService;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = LanguageService;
            type Credentials = gaxi::options::Credentials;
            async fn build(self, config: gaxi::options::ClientConfig) -> gax::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [super::super::client::LanguageService] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stub::dynamic::LanguageService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::LanguageService>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [LanguageService::analyze_sentiment][super::super::client::LanguageService::analyze_sentiment] calls.
    #[derive(Clone, Debug)]
    pub struct AnalyzeSentiment(RequestBuilder<crate::model::AnalyzeSentimentRequest>);

    impl AnalyzeSentiment {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::LanguageService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::AnalyzeSentimentRequest>>(
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
        pub async fn send(self) -> Result<crate::model::AnalyzeSentimentResponse> {
            (*self.0.stub)
                .analyze_sentiment(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [document][crate::model::AnalyzeSentimentRequest::document].
        pub fn set_document<T: Into<std::option::Option<crate::model::Document>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.document = v.into();
            self
        }

        /// Sets the value of [encoding_type][crate::model::AnalyzeSentimentRequest::encoding_type].
        pub fn set_encoding_type<T: Into<crate::model::EncodingType>>(mut self, v: T) -> Self {
            self.0.request.encoding_type = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for AnalyzeSentiment {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [LanguageService::analyze_entities][super::super::client::LanguageService::analyze_entities] calls.
    #[derive(Clone, Debug)]
    pub struct AnalyzeEntities(RequestBuilder<crate::model::AnalyzeEntitiesRequest>);

    impl AnalyzeEntities {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::LanguageService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::AnalyzeEntitiesRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::AnalyzeEntitiesResponse> {
            (*self.0.stub)
                .analyze_entities(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [document][crate::model::AnalyzeEntitiesRequest::document].
        pub fn set_document<T: Into<std::option::Option<crate::model::Document>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.document = v.into();
            self
        }

        /// Sets the value of [encoding_type][crate::model::AnalyzeEntitiesRequest::encoding_type].
        pub fn set_encoding_type<T: Into<crate::model::EncodingType>>(mut self, v: T) -> Self {
            self.0.request.encoding_type = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for AnalyzeEntities {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [LanguageService::classify_text][super::super::client::LanguageService::classify_text] calls.
    #[derive(Clone, Debug)]
    pub struct ClassifyText(RequestBuilder<crate::model::ClassifyTextRequest>);

    impl ClassifyText {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::LanguageService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ClassifyTextRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ClassifyTextResponse> {
            (*self.0.stub)
                .classify_text(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [document][crate::model::ClassifyTextRequest::document].
        pub fn set_document<T: Into<std::option::Option<crate::model::Document>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.document = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ClassifyText {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [LanguageService::moderate_text][super::super::client::LanguageService::moderate_text] calls.
    #[derive(Clone, Debug)]
    pub struct ModerateText(RequestBuilder<crate::model::ModerateTextRequest>);

    impl ModerateText {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::LanguageService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ModerateTextRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ModerateTextResponse> {
            (*self.0.stub)
                .moderate_text(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [document][crate::model::ModerateTextRequest::document].
        pub fn set_document<T: Into<std::option::Option<crate::model::Document>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.document = v.into();
            self
        }

        /// Sets the value of [model_version][crate::model::ModerateTextRequest::model_version].
        pub fn set_model_version<T: Into<crate::model::moderate_text_request::ModelVersion>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.model_version = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ModerateText {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [LanguageService::annotate_text][super::super::client::LanguageService::annotate_text] calls.
    #[derive(Clone, Debug)]
    pub struct AnnotateText(RequestBuilder<crate::model::AnnotateTextRequest>);

    impl AnnotateText {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::LanguageService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::AnnotateTextRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::AnnotateTextResponse> {
            (*self.0.stub)
                .annotate_text(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [document][crate::model::AnnotateTextRequest::document].
        pub fn set_document<T: Into<std::option::Option<crate::model::Document>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.document = v.into();
            self
        }

        /// Sets the value of [features][crate::model::AnnotateTextRequest::features].
        pub fn set_features<
            T: Into<std::option::Option<crate::model::annotate_text_request::Features>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.features = v.into();
            self
        }

        /// Sets the value of [encoding_type][crate::model::AnnotateTextRequest::encoding_type].
        pub fn set_encoding_type<T: Into<crate::model::EncodingType>>(mut self, v: T) -> Self {
            self.0.request.encoding_type = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for AnnotateText {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
