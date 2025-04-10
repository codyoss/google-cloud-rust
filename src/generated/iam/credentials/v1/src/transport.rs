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

/// Implements [IAMCredentials](super::stub::IAMCredentials) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct IAMCredentials {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for IAMCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("IAMCredentials")
            .field("inner", &self.inner)
            .finish()
    }
}

impl IAMCredentials {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::IAMCredentials for IAMCredentials {
    async fn generate_access_token(
        &self,
        req: crate::model::GenerateAccessTokenRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GenerateAccessTokenResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:generateAccessToken", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await.map(
            |r: gax::response::Response<crate::model::GenerateAccessTokenResponse>| r.into_body(),
        )
    }

    async fn generate_id_token(
        &self,
        req: crate::model::GenerateIdTokenRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GenerateIdTokenResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:generateIdToken", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::GenerateIdTokenResponse>| r.into_body())
    }

    async fn sign_blob(
        &self,
        req: crate::model::SignBlobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SignBlobResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:signBlob", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::SignBlobResponse>| r.into_body())
    }

    async fn sign_jwt(
        &self,
        req: crate::model::SignJwtRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SignJwtResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:signJwt", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::SignJwtResponse>| r.into_body())
    }
}
