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

/// Implements [WebRiskService](crate::stubs::WebRiskService) using a [gclient::ReqwestClient].
#[derive(Clone)]
pub struct WebRiskService {
    inner: gclient::ReqwestClient,
}

impl std::fmt::Debug for WebRiskService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("WebRiskService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl WebRiskService {
    pub async fn new(config: gclient::ClientConfig) -> Result<Self> {
        let inner = gclient::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl crate::stubs::WebRiskService for WebRiskService {
    async fn compute_threat_list_diff(
        &self,
        req: crate::model::ComputeThreatListDiffRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ComputeThreatListDiffResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                "/v1/threatLists:computeDiff".to_string(),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("threatType", &req.threat_type.value())]);
        let builder = builder.query(&[("versionToken", &req.version_token)]);
        let builder = req
            .constraints
            .as_ref()
            .map(|p| serde_json::to_value(p).map_err(Error::serde))
            .transpose()?
            .into_iter()
            .fold(builder, |builder, v| {
                use gclient::query_parameter::QueryParameter;
                v.add(builder, "constraints")
            });
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn search_uris(
        &self,
        req: crate::model::SearchUrisRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SearchUrisResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, "/v1/uris:search".to_string())
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("uri", &req.uri)]);
        let builder = req.threat_types.iter().fold(builder, |builder, p| {
            builder.query(&[("threatTypes", p.value())])
        });
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn search_hashes(
        &self,
        req: crate::model::SearchHashesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SearchHashesResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, "/v1/hashes:search".to_string())
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("hashPrefix", &req.hash_prefix)]);
        let builder = req.threat_types.iter().fold(builder, |builder, p| {
            builder.query(&[("threatTypes", p.value())])
        });
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn create_submission(
        &self,
        req: crate::model::CreateSubmissionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Submission> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/submissions", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.submission), options)
            .await
    }

    async fn submit_uri(
        &self,
        req: crate::model::SubmitUriRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/uris:submit", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}/operations", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
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
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:cancel", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy> {
        self.inner.get_polling_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
