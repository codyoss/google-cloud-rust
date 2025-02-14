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

/// Implements [ProfilerService](crate::stubs::ProfilerService) using a [gax::http_client::ReqwestClient].
#[derive(Clone)]
pub struct ProfilerService {
    inner: gax::http_client::ReqwestClient,
}

impl std::fmt::Debug for ProfilerService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("ProfilerService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl ProfilerService {
    pub async fn new(config: gax::http_client::ClientConfig) -> Result<Self> {
        let inner = gax::http_client::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl crate::stubs::ProfilerService for ProfilerService {
    async fn create_profile(
        &self,
        req: crate::model::CreateProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Profile> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v2/{}/profiles", req.parent),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn create_offline_profile(
        &self,
        req: crate::model::CreateOfflineProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Profile> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v2/{}/profiles:createOffline", req.parent),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.profile), options)
            .await
    }

    async fn update_profile(
        &self,
        req: crate::model::UpdateProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Profile> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/v2/{}",
                    req.profile
                        .as_ref()
                        .ok_or_else(|| gax::path_parameter::missing("profile"))?
                        .name
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .update_mask
            .as_ref()
            .map(|p| serde_json::to_value(p).map_err(Error::serde))
            .transpose()?
            .into_iter()
            .fold(builder, |builder, v| {
                use gax::query_parameter::QueryParameter;
                v.add(builder, "updateMask")
            });
        self.inner
            .execute(builder, Some(req.profile), options)
            .await
    }
}

/// Implements [ExportService](crate::stubs::ExportService) using a [gax::http_client::ReqwestClient].
#[derive(Clone)]
pub struct ExportService {
    inner: gax::http_client::ReqwestClient,
}

impl std::fmt::Debug for ExportService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("ExportService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl ExportService {
    pub async fn new(config: gax::http_client::ClientConfig) -> Result<Self> {
        let inner = gax::http_client::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl crate::stubs::ExportService for ExportService {
    async fn list_profiles(
        &self,
        req: crate::model::ListProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListProfilesResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v2/{}/profiles", req.parent))
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }
}
