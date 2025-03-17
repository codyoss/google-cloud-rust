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
/// # Service Description
///
/// Provides text analysis operations such as sentiment analysis and entity
/// recognition.
///
/// # Configuration
///
/// `LanguageService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `LanguageService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `LanguageService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct LanguageService {
    inner: Arc<dyn super::stubs::dynamic::LanguageService>,
}

impl LanguageService {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stubs::LanguageService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stubs::dynamic::LanguageService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::LanguageService> {
        super::transport::LanguageService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::LanguageService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::LanguageService::new)
    }

    /// Analyzes the sentiment of the provided text.
    pub fn analyze_sentiment(&self) -> super::builders::language_service::AnalyzeSentiment {
        super::builders::language_service::AnalyzeSentiment::new(self.inner.clone())
    }

    /// Finds named entities (currently proper names and common nouns) in the text
    /// along with entity types, probability, mentions for each entity, and
    /// other properties.
    pub fn analyze_entities(&self) -> super::builders::language_service::AnalyzeEntities {
        super::builders::language_service::AnalyzeEntities::new(self.inner.clone())
    }

    /// Classifies a document into categories.
    pub fn classify_text(&self) -> super::builders::language_service::ClassifyText {
        super::builders::language_service::ClassifyText::new(self.inner.clone())
    }

    /// Moderates a document for harmful and sensitive categories.
    pub fn moderate_text(&self) -> super::builders::language_service::ModerateText {
        super::builders::language_service::ModerateText::new(self.inner.clone())
    }

    /// A convenience method that provides all features in one call.
    pub fn annotate_text(&self) -> super::builders::language_service::AnnotateText {
        super::builders::language_service::AnnotateText::new(self.inner.clone())
    }
}
