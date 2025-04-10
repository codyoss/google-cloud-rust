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

/// Implements [GSuiteAddOns](super::stub::GSuiteAddOns) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct GSuiteAddOns {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for GSuiteAddOns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("GSuiteAddOns")
            .field("inner", &self.inner)
            .finish()
    }
}

impl GSuiteAddOns {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::GSuiteAddOns for GSuiteAddOns {
    async fn get_authorization(
        &self,
        req: crate::model::GetAuthorizationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Authorization> {
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
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<crate::model::Authorization>| r.into_body())
    }

    async fn create_deployment(
        &self,
        req: crate::model::CreateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Deployment> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/deployments", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("deploymentId", &req.deployment_id)]);
        self.inner
            .execute(builder, Some(req.deployment), options)
            .await
            .map(|r: gax::response::Response<crate::model::Deployment>| r.into_body())
    }

    async fn replace_deployment(
        &self,
        req: crate::model::ReplaceDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Deployment> {
        let options = options.set_default_idempotency(reqwest::Method::PUT.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PUT,
                format!(
                    "/v1/{}",
                    req.deployment
                        .as_ref()
                        .ok_or_else(|| gaxi::path_parameter::missing("deployment"))?
                        .name
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.deployment), options)
            .await
            .map(|r: gax::response::Response<crate::model::Deployment>| r.into_body())
    }

    async fn get_deployment(
        &self,
        req: crate::model::GetDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Deployment> {
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
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<crate::model::Deployment>| r.into_body())
    }

    async fn list_deployments(
        &self,
        req: crate::model::ListDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDeploymentsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/deployments", req.parent),
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
            .map(|r: gax::response::Response<crate::model::ListDeploymentsResponse>| r.into_body())
    }

    async fn delete_deployment(
        &self,
        req: crate::model::DeleteDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        let options = options.set_default_idempotency(reqwest::Method::DELETE.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::DELETE, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("etag", &req.etag)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|_: gax::response::Response<wkt::Empty>| ())
    }

    async fn install_deployment(
        &self,
        req: crate::model::InstallDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:install", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|_: gax::response::Response<wkt::Empty>| ())
    }

    async fn uninstall_deployment(
        &self,
        req: crate::model::UninstallDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:uninstall", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|_: gax::response::Response<wkt::Empty>| ())
    }

    async fn get_install_status(
        &self,
        req: crate::model::GetInstallStatusRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::InstallStatus> {
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
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<crate::model::InstallStatus>| r.into_body())
    }
}
