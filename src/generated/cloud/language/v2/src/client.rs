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
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Cloud Natural Language API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_language_v2::client::LanguageService;
/// let client = LanguageService::builder().build().await?;
/// // use `client` to make requests to the Cloud Natural Language API.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Provides text analysis operations such as sentiment analysis and entity
/// recognition.
///
/// # Configuration
///
/// To configure `LanguageService` use the `with_*` methods in the type returned
/// by [builder()][LanguageService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://language.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::language_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::language_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `LanguageService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `LanguageService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct LanguageService {
    inner: Arc<dyn super::stub::dynamic::LanguageService>,
}

impl LanguageService {
    /// Returns a builder for [LanguageService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_language_v2::client::LanguageService;
    /// let client = LanguageService::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::language_service::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::language_service::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::LanguageService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(
        conf: gaxi::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::LanguageService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::LanguageService> {
        super::transport::LanguageService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::LanguageService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::LanguageService::new)
    }

    /// Analyzes the sentiment of the provided text.
    pub fn analyze_sentiment(&self) -> super::builder::language_service::AnalyzeSentiment {
        super::builder::language_service::AnalyzeSentiment::new(self.inner.clone())
    }

    /// Finds named entities (currently proper names and common nouns) in the text
    /// along with entity types, probability, mentions for each entity, and
    /// other properties.
    pub fn analyze_entities(&self) -> super::builder::language_service::AnalyzeEntities {
        super::builder::language_service::AnalyzeEntities::new(self.inner.clone())
    }

    /// Classifies a document into categories.
    pub fn classify_text(&self) -> super::builder::language_service::ClassifyText {
        super::builder::language_service::ClassifyText::new(self.inner.clone())
    }

    /// Moderates a document for harmful and sensitive categories.
    pub fn moderate_text(&self) -> super::builder::language_service::ModerateText {
        super::builder::language_service::ModerateText::new(self.inner.clone())
    }

    /// A convenience method that provides all features in one call.
    pub fn annotate_text(&self) -> super::builder::language_service::AnnotateText {
        super::builder::language_service::AnnotateText::new(self.inner.clone())
    }
}
