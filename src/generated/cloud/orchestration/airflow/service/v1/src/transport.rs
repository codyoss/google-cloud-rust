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

/// Implements [Environments](super::stub::Environments) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct Environments {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for Environments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Environments")
            .field("inner", &self.inner)
            .finish()
    }
}

impl Environments {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::Environments for Environments {
    async fn create_environment(
        &self,
        req: crate::model::CreateEnvironmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/environments", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.environment), options)
            .await
            .map(|r: gax::response::Response<longrunning::model::Operation>| r.into_body())
    }

    async fn get_environment(
        &self,
        req: crate::model::GetEnvironmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Environment> {
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
            .map(|r: gax::response::Response<crate::model::Environment>| r.into_body())
    }

    async fn list_environments(
        &self,
        req: crate::model::ListEnvironmentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListEnvironmentsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/environments", req.parent),
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
            .map(|r: gax::response::Response<crate::model::ListEnvironmentsResponse>| r.into_body())
    }

    async fn update_environment(
        &self,
        req: crate::model::UpdateEnvironmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
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
            .execute(builder, Some(req.environment), options)
            .await
            .map(|r: gax::response::Response<longrunning::model::Operation>| r.into_body())
    }

    async fn delete_environment(
        &self,
        req: crate::model::DeleteEnvironmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
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
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<longrunning::model::Operation>| r.into_body())
    }

    async fn execute_airflow_command(
        &self,
        req: crate::model::ExecuteAirflowCommandRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ExecuteAirflowCommandResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:executeAirflowCommand", req.environment),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await.map(
            |r: gax::response::Response<crate::model::ExecuteAirflowCommandResponse>| r.into_body(),
        )
    }

    async fn stop_airflow_command(
        &self,
        req: crate::model::StopAirflowCommandRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::StopAirflowCommandResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:stopAirflowCommand", req.environment),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await.map(
            |r: gax::response::Response<crate::model::StopAirflowCommandResponse>| r.into_body(),
        )
    }

    async fn poll_airflow_command(
        &self,
        req: crate::model::PollAirflowCommandRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::PollAirflowCommandResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:pollAirflowCommand", req.environment),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await.map(
            |r: gax::response::Response<crate::model::PollAirflowCommandResponse>| r.into_body(),
        )
    }

    async fn list_workloads(
        &self,
        req: crate::model::ListWorkloadsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListWorkloadsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/workloads", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("filter", &req.filter)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<crate::model::ListWorkloadsResponse>| r.into_body())
    }

    async fn check_upgrade(
        &self,
        req: crate::model::CheckUpgradeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:checkUpgrade", req.environment),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<longrunning::model::Operation>| r.into_body())
    }

    async fn create_user_workloads_secret(
        &self,
        req: crate::model::CreateUserWorkloadsSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::UserWorkloadsSecret> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/userWorkloadsSecrets", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.user_workloads_secret), options)
            .await
            .map(|r: gax::response::Response<crate::model::UserWorkloadsSecret>| r.into_body())
    }

    async fn get_user_workloads_secret(
        &self,
        req: crate::model::GetUserWorkloadsSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::UserWorkloadsSecret> {
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
            .map(|r: gax::response::Response<crate::model::UserWorkloadsSecret>| r.into_body())
    }

    async fn list_user_workloads_secrets(
        &self,
        req: crate::model::ListUserWorkloadsSecretsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListUserWorkloadsSecretsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/userWorkloadsSecrets", req.parent),
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
            .map(
                |r: gax::response::Response<crate::model::ListUserWorkloadsSecretsResponse>| {
                    r.into_body()
                },
            )
    }

    async fn update_user_workloads_secret(
        &self,
        req: crate::model::UpdateUserWorkloadsSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::UserWorkloadsSecret> {
        let options = options.set_default_idempotency(reqwest::Method::PUT.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PUT,
                format!(
                    "/v1/{}",
                    req.user_workloads_secret
                        .as_ref()
                        .ok_or_else(|| gaxi::path_parameter::missing("user_workloads_secret"))?
                        .name
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.user_workloads_secret), options)
            .await
            .map(|r: gax::response::Response<crate::model::UserWorkloadsSecret>| r.into_body())
    }

    async fn delete_user_workloads_secret(
        &self,
        req: crate::model::DeleteUserWorkloadsSecretRequest,
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
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|_: gax::response::Response<wkt::Empty>| ())
    }

    async fn create_user_workloads_config_map(
        &self,
        req: crate::model::CreateUserWorkloadsConfigMapRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::UserWorkloadsConfigMap> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/userWorkloadsConfigMaps", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.user_workloads_config_map), options)
            .await
            .map(|r: gax::response::Response<crate::model::UserWorkloadsConfigMap>| r.into_body())
    }

    async fn get_user_workloads_config_map(
        &self,
        req: crate::model::GetUserWorkloadsConfigMapRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::UserWorkloadsConfigMap> {
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
            .map(|r: gax::response::Response<crate::model::UserWorkloadsConfigMap>| r.into_body())
    }

    async fn list_user_workloads_config_maps(
        &self,
        req: crate::model::ListUserWorkloadsConfigMapsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListUserWorkloadsConfigMapsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/userWorkloadsConfigMaps", req.parent),
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
            .map(
                |r: gax::response::Response<crate::model::ListUserWorkloadsConfigMapsResponse>| {
                    r.into_body()
                },
            )
    }

    async fn update_user_workloads_config_map(
        &self,
        req: crate::model::UpdateUserWorkloadsConfigMapRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::UserWorkloadsConfigMap> {
        let options = options.set_default_idempotency(reqwest::Method::PUT.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PUT,
                format!(
                    "/v1/{}",
                    req.user_workloads_config_map
                        .as_ref()
                        .ok_or_else(|| gaxi::path_parameter::missing("user_workloads_config_map"))?
                        .name
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.user_workloads_config_map), options)
            .await
            .map(|r: gax::response::Response<crate::model::UserWorkloadsConfigMap>| r.into_body())
    }

    async fn delete_user_workloads_config_map(
        &self,
        req: crate::model::DeleteUserWorkloadsConfigMapRequest,
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
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|_: gax::response::Response<wkt::Empty>| ())
    }

    async fn save_snapshot(
        &self,
        req: crate::model::SaveSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:saveSnapshot", req.environment),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<longrunning::model::Operation>| r.into_body())
    }

    async fn load_snapshot(
        &self,
        req: crate::model::LoadSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:loadSnapshot", req.environment),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<longrunning::model::Operation>| r.into_body())
    }

    async fn database_failover(
        &self,
        req: crate::model::DatabaseFailoverRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:databaseFailover", req.environment),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<longrunning::model::Operation>| r.into_body())
    }

    async fn fetch_database_properties(
        &self,
        req: crate::model::FetchDatabasePropertiesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::FetchDatabasePropertiesResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}:fetchDatabaseProperties", req.environment),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(
                |r: gax::response::Response<crate::model::FetchDatabasePropertiesResponse>| {
                    r.into_body()
                },
            )
    }

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}/operations", req.name))
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
            .map(
                |r: gax::response::Response<longrunning::model::ListOperationsResponse>| {
                    r.into_body()
                },
            )
    }

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
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
            .map(|r: gax::response::Response<longrunning::model::Operation>| r.into_body())
    }

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
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
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|_: gax::response::Response<wkt::Empty>| ())
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        self.inner.get_polling_error_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}

/// Implements [ImageVersions](super::stub::ImageVersions) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct ImageVersions {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for ImageVersions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("ImageVersions")
            .field("inner", &self.inner)
            .finish()
    }
}

impl ImageVersions {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::ImageVersions for ImageVersions {
    async fn list_image_versions(
        &self,
        req: crate::model::ListImageVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListImageVersionsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/imageVersions", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("includePastReleases", &req.include_past_releases)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(
                |r: gax::response::Response<crate::model::ListImageVersionsResponse>| r.into_body(),
            )
    }

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}/operations", req.name))
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
            .map(
                |r: gax::response::Response<longrunning::model::ListOperationsResponse>| {
                    r.into_body()
                },
            )
    }

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
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
            .map(|r: gax::response::Response<longrunning::model::Operation>| r.into_body())
    }

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
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
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|_: gax::response::Response<wkt::Empty>| ())
    }
}
