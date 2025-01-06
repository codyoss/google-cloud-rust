// Copyright 2024 Google LLC
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

/// Implements [Locations](crate::traits::Locations) using a [gax::http_client::ReqwestClient].
#[derive(Clone)]
pub struct Locations {
    inner: gax::http_client::ReqwestClient,
}

impl std::fmt::Debug for Locations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Locations")
            .field("inner", &self.inner)
            .finish()
    }
}

impl Locations {
    pub async fn new(config: gax::http_client::ClientConfig) -> Result<Self> {
        let inner = gax::http_client::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl crate::traits::Locations for Locations {
    async fn list_locations(
        &self,
        req: crate::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListLocationsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self.inner.builder(
            reqwest::Method::GET, format!("/v1/{}"
               , req.name
            ))
            .query(&[("alt", "json")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = gax::query_parameter::add(builder, "filter", &req.filter).map_err(Error::other)?;
        let builder = gax::query_parameter::add(builder, "pageSize", &req.page_size).map_err(Error::other)?;
        let builder = gax::query_parameter::add(builder, "pageToken", &req.page_token).map_err(Error::other)?;
        self.inner.execute(
            builder,
            None::<gax::http_client::NoBody>,
            options,
        ).await
    }

    async fn get_location(
        &self,
        req: crate::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Location> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self.inner.builder(
            reqwest::Method::GET, format!("/v1/{}"
               , req.name
            ))
            .query(&[("alt", "json")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            None::<gax::http_client::NoBody>,
            options,
        ).await
    }

}

