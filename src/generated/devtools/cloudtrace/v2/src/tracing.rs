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
use crate::Result;

/// Implements a [TraceService](super::stub::TraceService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct TraceService<T>
where
    T: super::stub::TraceService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> TraceService<T>
where
    T: super::stub::TraceService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::TraceService for TraceService<T>
where
    T: super::stub::TraceService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn batch_write_spans(
        &self,
        req: crate::model::BatchWriteSpansRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.batch_write_spans(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_span(
        &self,
        req: crate::model::Span,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Span>> {
        self.inner.create_span(req, options).await
    }
}
