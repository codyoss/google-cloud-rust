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

/// Implements [CaseAttachmentService](super::stub::CaseAttachmentService) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct CaseAttachmentService {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for CaseAttachmentService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("CaseAttachmentService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl CaseAttachmentService {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::CaseAttachmentService for CaseAttachmentService {
    async fn list_attachments(
        &self,
        req: crate::model::ListAttachmentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListAttachmentsResponse>> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v2/{}/attachments", req.parent),
            )
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
}

/// Implements [CaseService](super::stub::CaseService) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct CaseService {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for CaseService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("CaseService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl CaseService {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::CaseService for CaseService {
    async fn get_case(
        &self,
        req: crate::model::GetCaseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Case>> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v2/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn list_cases(
        &self,
        req: crate::model::ListCasesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListCasesResponse>> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v2/{}/cases", req.parent))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn search_cases(
        &self,
        req: crate::model::SearchCasesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SearchCasesResponse>> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v2/{}/cases:search", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("query", &req.query)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn create_case(
        &self,
        req: crate::model::CreateCaseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Case>> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v2/{}/cases", req.parent))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req.case), options).await
    }

    async fn update_case(
        &self,
        req: crate::model::UpdateCaseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Case>> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/v2/{}",
                    req.case
                        .as_ref()
                        .ok_or_else(|| gaxi::path_parameter::missing("case"))?
                        .name
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .update_mask
            .as_ref()
            .iter()
            .flat_map(|p| p.paths.iter())
            .fold(builder, |builder, v| builder.query(&[("updateMask", v)]));
        self.inner.execute(builder, Some(req.case), options).await
    }

    async fn escalate_case(
        &self,
        req: crate::model::EscalateCaseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Case>> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v2/{}:escalate", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn close_case(
        &self,
        req: crate::model::CloseCaseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Case>> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v2/{}:close", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn search_case_classifications(
        &self,
        req: crate::model::SearchCaseClassificationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SearchCaseClassificationsResponse>> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                "/v2/caseClassifications:search".to_string(),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("query", &req.query)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }
}

/// Implements [CommentService](super::stub::CommentService) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct CommentService {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for CommentService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("CommentService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl CommentService {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::CommentService for CommentService {
    async fn list_comments(
        &self,
        req: crate::model::ListCommentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListCommentsResponse>> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v2/{}/comments", req.parent))
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

    async fn create_comment(
        &self,
        req: crate::model::CreateCommentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Comment>> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v2/{}/comments", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.comment), options)
            .await
    }
}
