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

/// Implements [OsLoginService](super::stubs::OsLoginService) using a [gaxi::ReqwestClient].
#[derive(Clone)]
pub struct OsLoginService {
    inner: gaxi::ReqwestClient,
}

impl std::fmt::Debug for OsLoginService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("OsLoginService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl OsLoginService {
    pub async fn new(config: gax::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stubs::OsLoginService for OsLoginService {
    async fn create_ssh_public_key(
        &self,
        req: crate::model::CreateSshPublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<oslogin_common::model::SshPublicKey> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/sshPublicKeys", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.ssh_public_key), options)
            .await
    }

    async fn delete_posix_account(
        &self,
        req: crate::model::DeletePosixAccountRequest,
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
            .execute(builder, None::<gaxi::NoBody>, options)
            .await
    }

    async fn delete_ssh_public_key(
        &self,
        req: crate::model::DeleteSshPublicKeyRequest,
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
            .execute(builder, None::<gaxi::NoBody>, options)
            .await
    }

    async fn get_login_profile(
        &self,
        req: crate::model::GetLoginProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::LoginProfile> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/loginProfile", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("projectId", &req.project_id)]);
        let builder = builder.query(&[("systemId", &req.system_id)]);
        self.inner
            .execute(builder, None::<gaxi::NoBody>, options)
            .await
    }

    async fn get_ssh_public_key(
        &self,
        req: crate::model::GetSshPublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<oslogin_common::model::SshPublicKey> {
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
            .execute(builder, None::<gaxi::NoBody>, options)
            .await
    }

    async fn import_ssh_public_key(
        &self,
        req: crate::model::ImportSshPublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ImportSshPublicKeyResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:importSshPublicKey", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("projectId", &req.project_id)]);
        let builder = req
            .regions
            .iter()
            .fold(builder, |builder, p| builder.query(&[("regions", p)]));
        self.inner
            .execute(builder, Some(req.ssh_public_key), options)
            .await
    }

    async fn update_ssh_public_key(
        &self,
        req: crate::model::UpdateSshPublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<oslogin_common::model::SshPublicKey> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::PATCH, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
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
                use gaxi::query_parameter::QueryParameter;
                v.add(builder, "updateMask")
            });
        self.inner
            .execute(builder, Some(req.ssh_public_key), options)
            .await
    }
}
