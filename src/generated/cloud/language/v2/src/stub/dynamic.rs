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

/// A dyn-compatible, crate-private version of [super::LanguageService].
#[async_trait::async_trait]
pub trait LanguageService: std::fmt::Debug + Send + Sync {
    async fn analyze_sentiment(
        &self,
        req: crate::model::AnalyzeSentimentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnalyzeSentimentResponse>>;

    async fn analyze_entities(
        &self,
        req: crate::model::AnalyzeEntitiesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnalyzeEntitiesResponse>>;

    async fn classify_text(
        &self,
        req: crate::model::ClassifyTextRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ClassifyTextResponse>>;

    async fn moderate_text(
        &self,
        req: crate::model::ModerateTextRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ModerateTextResponse>>;

    async fn annotate_text(
        &self,
        req: crate::model::AnnotateTextRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnnotateTextResponse>>;
}

/// All implementations of [super::LanguageService] also implement [LanguageService].
#[async_trait::async_trait]
impl<T: super::LanguageService> LanguageService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn analyze_sentiment(
        &self,
        req: crate::model::AnalyzeSentimentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnalyzeSentimentResponse>> {
        T::analyze_sentiment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn analyze_entities(
        &self,
        req: crate::model::AnalyzeEntitiesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnalyzeEntitiesResponse>> {
        T::analyze_entities(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn classify_text(
        &self,
        req: crate::model::ClassifyTextRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ClassifyTextResponse>> {
        T::classify_text(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn moderate_text(
        &self,
        req: crate::model::ModerateTextRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ModerateTextResponse>> {
        T::moderate_text(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn annotate_text(
        &self,
        req: crate::model::AnnotateTextRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnnotateTextResponse>> {
        T::annotate_text(self, req, options).await
    }
}
