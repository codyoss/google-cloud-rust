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

/// Implements [Iam](crate::stubs::Iam) using a [gclient::ReqwestClient].
#[derive(Clone)]
pub struct Iam {
    inner: gclient::ReqwestClient,
}

impl std::fmt::Debug for Iam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Iam").field("inner", &self.inner).finish()
    }
}

impl Iam {
    pub async fn new(config: gclient::ClientConfig) -> Result<Self> {
        let inner = gclient::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl crate::stubs::Iam for Iam {
    async fn list_service_accounts(
        &self,
        req: crate::model::ListServiceAccountsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListServiceAccountsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/serviceAccounts", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn get_service_account(
        &self,
        req: crate::model::GetServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ServiceAccount> {
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

    async fn create_service_account(
        &self,
        req: crate::model::CreateServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ServiceAccount> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/serviceAccounts", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn update_service_account(
        &self,
        req: crate::model::ServiceAccount,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ServiceAccount> {
        let options = options.set_default_idempotency(reqwest::Method::PUT.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::PUT, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn patch_service_account(
        &self,
        req: crate::model::PatchServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ServiceAccount> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/v1/{}",
                    req.service_account
                        .as_ref()
                        .ok_or_else(|| gclient::path_parameter::missing("service_account"))?
                        .name
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn delete_service_account(
        &self,
        req: crate::model::DeleteServiceAccountRequest,
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

    async fn undelete_service_account(
        &self,
        req: crate::model::UndeleteServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::UndeleteServiceAccountResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:undelete", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn enable_service_account(
        &self,
        req: crate::model::EnableServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:enable", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn disable_service_account(
        &self,
        req: crate::model::DisableServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:disable", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn list_service_account_keys(
        &self,
        req: crate::model::ListServiceAccountKeysRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListServiceAccountKeysResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}/keys", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req.key_types.iter().fold(builder, |builder, p| {
            builder.query(&[("keyTypes", p.value())])
        });
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn get_service_account_key(
        &self,
        req: crate::model::GetServiceAccountKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ServiceAccountKey> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("publicKeyType", &req.public_key_type.value())]);
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn create_service_account_key(
        &self,
        req: crate::model::CreateServiceAccountKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ServiceAccountKey> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}/keys", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn upload_service_account_key(
        &self,
        req: crate::model::UploadServiceAccountKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ServiceAccountKey> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/keys:upload", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn delete_service_account_key(
        &self,
        req: crate::model::DeleteServiceAccountKeyRequest,
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

    async fn disable_service_account_key(
        &self,
        req: crate::model::DisableServiceAccountKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:disable", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn enable_service_account_key(
        &self,
        req: crate::model::EnableServiceAccountKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:enable", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
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
        self.inner.execute(builder, Some(req), options).await
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
        self.inner.execute(builder, Some(req), options).await
    }

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:getIamPolicy", req.resource),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .options
            .as_ref()
            .map(|p| serde_json::to_value(p).map_err(Error::serde))
            .transpose()?
            .into_iter()
            .fold(builder, |builder, v| {
                use gclient::query_parameter::QueryParameter;
                v.add(builder, "options")
            });
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:setIamPolicy", req.resource),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:testIamPermissions", req.resource),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn query_grantable_roles(
        &self,
        req: crate::model::QueryGrantableRolesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::QueryGrantableRolesResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                "/v1/roles:queryGrantableRoles".to_string(),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn list_roles(
        &self,
        req: crate::model::ListRolesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListRolesResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, "/v1/roles".to_string())
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("parent", &req.parent)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("view", &req.view.value())]);
        let builder = builder.query(&[("showDeleted", &req.show_deleted)]);
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn get_role(
        &self,
        req: crate::model::GetRoleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Role> {
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

    async fn create_role(
        &self,
        req: crate::model::CreateRoleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Role> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}/roles", req.parent))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn update_role(
        &self,
        req: crate::model::UpdateRoleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Role> {
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
                use gclient::query_parameter::QueryParameter;
                v.add(builder, "updateMask")
            });
        self.inner.execute(builder, Some(req.role), options).await
    }

    async fn delete_role(
        &self,
        req: crate::model::DeleteRoleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Role> {
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
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn undelete_role(
        &self,
        req: crate::model::UndeleteRoleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Role> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:undelete", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn query_testable_permissions(
        &self,
        req: crate::model::QueryTestablePermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::QueryTestablePermissionsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                "/v1/permissions:queryTestablePermissions".to_string(),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn query_auditable_services(
        &self,
        req: crate::model::QueryAuditableServicesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::QueryAuditableServicesResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                "/v1/iamPolicies:queryAuditableServices".to_string(),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn lint_policy(
        &self,
        req: crate::model::LintPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::LintPolicyResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                "/v1/iamPolicies:lintPolicy".to_string(),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }
}
