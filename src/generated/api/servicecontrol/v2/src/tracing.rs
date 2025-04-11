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

/// Implements a [ServiceController](super::stub::ServiceController) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct ServiceController<T>
where
    T: super::stub::ServiceController + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> ServiceController<T>
where
    T: super::stub::ServiceController + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::ServiceController for ServiceController<T>
where
    T: super::stub::ServiceController + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn check(
        &self,
        req: crate::model::CheckRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::CheckResponse>> {
        self.inner.check(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn report(
        &self,
        req: crate::model::ReportRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ReportResponse>> {
        self.inner.report(req, options).await
    }
}
