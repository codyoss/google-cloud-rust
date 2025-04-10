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

/// Implements [SecretManagerService](super::stub::SecretManagerService) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct SecretManagerService {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for SecretManagerService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("SecretManagerService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl SecretManagerService {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::SecretManagerService for SecretManagerService {
    async fn list_secrets(
        &self,
        req: crate::model::ListSecretsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSecretsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/secrets"
                        , req.parent
                )
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("filter", &req.filter)]);
        self.inner.execute(
            builder,
            
            None::<gaxi::http::NoBody>,
            options,
        ).await
        .map(|r: gax::response::Response<crate::model::ListSecretsResponse>| r.into_body())
    }

    async fn create_secret(
        &self,
        req: crate::model::CreateSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/secrets"
                        , req.parent
                )
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("secretId", &req.secret_id)]);
        self.inner.execute(
            builder,
            Some(req.secret)
            ,
            options,
        ).await
        .map(|r: gax::response::Response<crate::model::Secret>| r.into_body())
    }

    async fn add_secret_version(
        &self,
        req: crate::model::AddSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:addVersion"
                        , req.parent
                )
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            Some(req)
            ,
            options,
        ).await
        .map(|r: gax::response::Response<crate::model::SecretVersion>| r.into_body())
    }

    async fn get_secret(
        &self,
        req: crate::model::GetSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
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
        .map(|r: gax::response::Response<crate::model::Secret>| r.into_body())
    }

    async fn update_secret(
        &self,
        req: crate::model::UpdateSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!("/v1/{}"
                        , req.secret.as_ref().ok_or_else(|| gaxi::path_parameter::missing("secret"))?.name
                )
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = req.update_mask.as_ref().map(|p| serde_json::to_value(p).map_err(Error::serde) ).transpose()?.into_iter().fold(builder, |builder, v| { use gaxi::query_parameter::QueryParameter; v.add(builder, "updateMask") });
        self.inner.execute(
            builder,
            Some(req.secret)
            ,
            options,
        ).await
        .map(|r: gax::response::Response<crate::model::Secret>| r.into_body())
    }

    async fn delete_secret(
        &self,
        req: crate::model::DeleteSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        let options = options.set_default_idempotency(reqwest::Method::DELETE.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::DELETE,
                format!("/v1/{}"
                        , req.name
                )
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("etag", &req.etag)]);
        self.inner.execute(
            builder,
            
            None::<gaxi::http::NoBody>,
            options,
        ).await
        .map(|_: gax::response::Response<wkt::Empty>| ())
    }

    async fn list_secret_versions(
        &self,
        req: crate::model::ListSecretVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSecretVersionsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/versions"
                        , req.parent
                )
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("filter", &req.filter)]);
        self.inner.execute(
            builder,
            
            None::<gaxi::http::NoBody>,
            options,
        ).await
        .map(|r: gax::response::Response<crate::model::ListSecretVersionsResponse>| r.into_body())
    }

    async fn get_secret_version(
        &self,
        req: crate::model::GetSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
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
        .map(|r: gax::response::Response<crate::model::SecretVersion>| r.into_body())
    }

    async fn access_secret_version(
        &self,
        req: crate::model::AccessSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AccessSecretVersionResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}:access"
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
        .map(|r: gax::response::Response<crate::model::AccessSecretVersionResponse>| r.into_body())
    }

    async fn disable_secret_version(
        &self,
        req: crate::model::DisableSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:disable"
                        , req.name
                )
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            Some(req)
            ,
            options,
        ).await
        .map(|r: gax::response::Response<crate::model::SecretVersion>| r.into_body())
    }

    async fn enable_secret_version(
        &self,
        req: crate::model::EnableSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:enable"
                        , req.name
                )
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            Some(req)
            ,
            options,
        ).await
        .map(|r: gax::response::Response<crate::model::SecretVersion>| r.into_body())
    }

    async fn destroy_secret_version(
        &self,
        req: crate::model::DestroySecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:destroy"
                        , req.name
                )
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            Some(req)
            ,
            options,
        ).await
        .map(|r: gax::response::Response<crate::model::SecretVersion>| r.into_body())
    }

    async fn set_iam_policy(
        &self,
        req: iam::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam::model::Policy> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:setIamPolicy"
                        , req.resource
                )
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            Some(req)
            ,
            options,
        ).await
        .map(|r: gax::response::Response<iam::model::Policy>| r.into_body())
    }

    async fn get_iam_policy(
        &self,
        req: iam::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam::model::Policy> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}:getIamPolicy"
                        , req.resource
                )
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = req.options.as_ref().map(|p| serde_json::to_value(p).map_err(Error::serde) ).transpose()?.into_iter().fold(builder, |builder, v| { use gaxi::query_parameter::QueryParameter; v.add(builder, "options") });
        self.inner.execute(
            builder,
            
            None::<gaxi::http::NoBody>,
            options,
        ).await
        .map(|r: gax::response::Response<iam::model::Policy>| r.into_body())
    }

    async fn test_iam_permissions(
        &self,
        req: iam::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam::model::TestIamPermissionsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:testIamPermissions"
                        , req.resource
                )
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            Some(req)
            ,
            options,
        ).await
        .map(|r: gax::response::Response<iam::model::TestIamPermissionsResponse>| r.into_body())
    }

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/locations"
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
        .map(|r: gax::response::Response<location::model::ListLocationsResponse>| r.into_body())
    }

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
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
        .map(|r: gax::response::Response<location::model::Location>| r.into_body())
    }

}

