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
#[allow(unused_imports)]
use gax::error::Error;

/// Implements [TimeseriesInsightsController](super::stub::TimeseriesInsightsController) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct TimeseriesInsightsController {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for TimeseriesInsightsController {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("TimeseriesInsightsController")
            .field("inner", &self.inner)
            .finish()
    }
}

impl TimeseriesInsightsController {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::TimeseriesInsightsController for TimeseriesInsightsController {
    async fn list_data_sets(
        &self,
        req: crate::model::ListDataSetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDataSetsResponse>> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}/datasets", req.parent))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn create_data_set(
        &self,
        req: crate::model::CreateDataSetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::DataSet>> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/datasets", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.dataset), options)
            .await
    }

    async fn delete_data_set(
        &self,
        req: crate::model::DeleteDataSetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        let options = options.set_default_idempotency(reqwest::Method::DELETE.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::DELETE, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<wkt::Empty>| {
                let (parts, _) = r.into_parts();
                gax::response::Response::from_parts(parts, ())
            })
    }

    async fn append_events(
        &self,
        req: crate::model::AppendEventsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AppendEventsResponse>> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:appendEvents", req.dataset),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn query_data_set(
        &self,
        req: crate::model::QueryDataSetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::QueryDataSetResponse>> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:query", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn evaluate_slice(
        &self,
        req: crate::model::EvaluateSliceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::EvaluatedSlice>> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:evaluateSlice", req.dataset),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn evaluate_timeseries(
        &self,
        req: crate::model::EvaluateTimeseriesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::EvaluatedSlice>> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/datasets:evaluateTimeseries", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }
}
