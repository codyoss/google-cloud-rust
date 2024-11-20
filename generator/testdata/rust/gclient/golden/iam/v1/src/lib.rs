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

/// The messages and enums that are part of this client library.
pub mod model;

use gax::error::{Error, HttpError};
use google_cloud_auth::{Credential, CredentialConfig};
use std::sync::Arc;

/// A `Result` alias where the `Err` case is an [Error].
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct Client {
    inner: Arc<ClientRef>,
}

struct ClientRef {
    http_client: reqwest::Client,
    cred: Credential,
    endpoint: Option<String>,
}

#[derive(Default)]
pub struct ConfigBuilder {
    pub(crate) endpoint: Option<String>,
    pub(crate) client: Option<reqwest::Client>,
    pub(crate) cred: Option<Credential>,
}

impl ConfigBuilder {
    /// Returns a default [ConfigBuilder].
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets an endpoint that overrides the default endpoint for a service.
    pub fn set_endpoint<T: Into<String>>(mut self, v: T) -> Self {
        self.endpoint = Some(v.into());
        self
    }

    pub(crate) fn default_client() -> reqwest::Client {
        reqwest::Client::builder().build().unwrap()
    }

    pub(crate) async fn default_credential() -> Result<Credential> {
        let cc = CredentialConfig::builder()
            .scopes(vec![
                "https://www.googleapis.com/auth/cloud-platform".to_string()
            ])
            .build()
            .map_err(Error::authentication)?;
        Credential::find_default(cc)
            .await
            .map_err(Error::authentication)
    }
}

impl Client {
    pub async fn new() -> Result<Self> {
        Self::new_with_config(ConfigBuilder::new()).await
    }

    pub async fn new_with_config(conf: ConfigBuilder) -> Result<Self> {
        let inner = ClientRef {
            http_client: conf.client.unwrap_or(ConfigBuilder::default_client()),
            cred: conf
                .cred
                .unwrap_or(ConfigBuilder::default_credential().await?),
            endpoint: conf.endpoint,
        };
        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    /// API Overview
    ///
    /// Manages Identity and Access Management (IAM) policies.
    ///
    /// Any implementation of an API that offers access control features
    /// implements the google.iam.v1.IAMPolicy interface.
    ///
    /// ## Data model
    ///
    /// Access control is applied when a principal (user or service account), takes
    /// some action on a resource exposed by a service. Resources, identified by
    /// URI-like names, are the unit of access control specification. Service
    /// implementations can choose the granularity of access control and the
    /// supported permissions for their resources.
    /// For example one database service may allow access control to be
    /// specified only at the Table level, whereas another might allow access control
    /// to also be specified at the Column level.
    ///
    /// ## Policy Structure
    ///
    /// See google.iam.v1.Policy
    ///
    /// This is intentionally not a CRUD style API because access control policies
    /// are created and deleted implicitly with the resources to which they are
    /// attached.
    pub fn iam_policy(&self) -> Iampolicy {
        Iampolicy {
            client: self.clone(),
            base_path: self
                .inner
                .endpoint
                .clone()
                .unwrap_or("https://iam-meta-api.googleapis.com/".to_string()),
        }
    }
}

#[derive(serde::Serialize)]
#[allow(dead_code)]
struct NoBody {}

/// API Overview
///
/// Manages Identity and Access Management (IAM) policies.
///
/// Any implementation of an API that offers access control features
/// implements the google.iam.v1.IAMPolicy interface.
///
/// ## Data model
///
/// Access control is applied when a principal (user or service account), takes
/// some action on a resource exposed by a service. Resources, identified by
/// URI-like names, are the unit of access control specification. Service
/// implementations can choose the granularity of access control and the
/// supported permissions for their resources.
/// For example one database service may allow access control to be
/// specified only at the Table level, whereas another might allow access control
/// to also be specified at the Column level.
///
/// ## Policy Structure
///
/// See google.iam.v1.Policy
///
/// This is intentionally not a CRUD style API because access control policies
/// are created and deleted implicitly with the resources to which they are
/// attached.
pub struct Iampolicy {
    client: Client,
    base_path: String,
}

impl Iampolicy {
    /// Sets the access control policy on the specified resource. Replaces any
    /// existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    pub async fn set_iam_policy(
        &self,
        req: crate::model::SetIamPolicyRequest,
    ) -> Result<crate::model::Policy> {
        let client = self.client.inner.clone();
        let builder = client
            .http_client
            .post(format!(
                "{}/v1/{}:setIamPolicy",
                self.base_path, req.resource,
            ))
            .query(&[("alt", "json")]);
        self.execute(builder, Some(req)).await
    }

    /// Gets the access control policy for a resource.
    /// Returns an empty policy if the resource exists and does not have a policy
    /// set.
    pub async fn get_iam_policy(
        &self,
        req: crate::model::GetIamPolicyRequest,
    ) -> Result<crate::model::Policy> {
        let client = self.client.inner.clone();
        let builder = client
            .http_client
            .post(format!(
                "{}/v1/{}:getIamPolicy",
                self.base_path, req.resource,
            ))
            .query(&[("alt", "json")]);
        self.execute(builder, Some(req)).await
    }

    /// Returns permissions that a caller has on the specified resource.
    /// If the resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    pub async fn test_iam_permissions(
        &self,
        req: crate::model::TestIamPermissionsRequest,
    ) -> Result<crate::model::TestIamPermissionsResponse> {
        let client = self.client.inner.clone();
        let builder = client
            .http_client
            .post(format!(
                "{}/v1/{}:testIamPermissions",
                self.base_path, req.resource,
            ))
            .query(&[("alt", "json")]);
        self.execute(builder, Some(req)).await
    }

    async fn execute<I: serde::ser::Serialize, O: serde::de::DeserializeOwned>(
        &self,
        mut builder: reqwest::RequestBuilder,
        body: Option<I>,
    ) -> Result<O> {
        let client_ref = self.client.inner.clone();
        builder = builder.bearer_auth(
            &client_ref
                .cred
                .access_token()
                .await
                .map_err(Error::authentication)?
                .value,
        );
        if let Some(body) = body {
            builder = builder.json(&body);
        }
        let resp = builder.send().await.map_err(Error::io)?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let headers = gax::error::convert_headers(resp.headers());
            let body = resp.bytes().await.map_err(Error::io)?;
            return Err(HttpError::new(status, headers, Some(body)).into());
        }
        let response = resp.json::<O>().await.map_err(Error::serde)?;
        Ok(response)
    }
}
