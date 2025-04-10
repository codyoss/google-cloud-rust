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

/// Implements [Locations](super::stub::Locations) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct Locations {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for Locations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Locations")
            .field("inner", &self.inner)
            .finish()
    }
}

impl Locations {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::Locations for Locations {
    async fn list_locations(
        &self,
        req: crate::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListLocationsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}"
                        , req.name
                )
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner.execute(
            builder,
            
            None::<gaxi::http::NoBody>,
            options,
        ).await
        .map(|r: gax::response::Response<crate::model::ListLocationsResponse>| r.into_body())
    }

    async fn get_location(
        &self,
        req: crate::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Location> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}"
                        , req.name
                )
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            
            None::<gaxi::http::NoBody>,
            options,
        ).await
        .map(|r: gax::response::Response<crate::model::Location>| r.into_body())
    }

}

