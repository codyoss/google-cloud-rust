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

/// Implements [DataTransferService](super::stub::DataTransferService) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct DataTransferService {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for DataTransferService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("DataTransferService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl DataTransferService {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::DataTransferService for DataTransferService {
    async fn get_data_source(
        &self,
        req: crate::model::GetDataSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DataSource> {
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
            .map(|r: gax::response::Response<crate::model::DataSource>| r.into_body())
    }

    async fn list_data_sources(
        &self,
        req: crate::model::ListDataSourcesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDataSourcesResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/dataSources", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<crate::model::ListDataSourcesResponse>| r.into_body())
    }

    async fn create_transfer_config(
        &self,
        req: crate::model::CreateTransferConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TransferConfig> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/transferConfigs", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("authorizationCode", &req.authorization_code)]);
        let builder = builder.query(&[("versionInfo", &req.version_info)]);
        let builder = builder.query(&[("serviceAccountName", &req.service_account_name)]);
        self.inner
            .execute(builder, Some(req.transfer_config), options)
            .await
            .map(|r: gax::response::Response<crate::model::TransferConfig>| r.into_body())
    }

    async fn update_transfer_config(
        &self,
        req: crate::model::UpdateTransferConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TransferConfig> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/v1/{}",
                    req.transfer_config
                        .as_ref()
                        .ok_or_else(|| gaxi::path_parameter::missing("transfer_config"))?
                        .name
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("authorizationCode", &req.authorization_code)]);
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
        let builder = builder.query(&[("versionInfo", &req.version_info)]);
        let builder = builder.query(&[("serviceAccountName", &req.service_account_name)]);
        self.inner
            .execute(builder, Some(req.transfer_config), options)
            .await
            .map(|r: gax::response::Response<crate::model::TransferConfig>| r.into_body())
    }

    async fn delete_transfer_config(
        &self,
        req: crate::model::DeleteTransferConfigRequest,
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

    async fn get_transfer_config(
        &self,
        req: crate::model::GetTransferConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TransferConfig> {
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
            .map(|r: gax::response::Response<crate::model::TransferConfig>| r.into_body())
    }

    async fn list_transfer_configs(
        &self,
        req: crate::model::ListTransferConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTransferConfigsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/transferConfigs", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .data_source_ids
            .iter()
            .fold(builder, |builder, p| builder.query(&[("dataSourceIds", p)]));
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(
                |r: gax::response::Response<crate::model::ListTransferConfigsResponse>| {
                    r.into_body()
                },
            )
    }

    async fn schedule_transfer_runs(
        &self,
        req: crate::model::ScheduleTransferRunsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ScheduleTransferRunsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:scheduleRuns", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await.map(
            |r: gax::response::Response<crate::model::ScheduleTransferRunsResponse>| r.into_body(),
        )
    }

    async fn start_manual_transfer_runs(
        &self,
        req: crate::model::StartManualTransferRunsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::StartManualTransferRunsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:startManualRuns", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await.map(
            |r: gax::response::Response<crate::model::StartManualTransferRunsResponse>| {
                r.into_body()
            },
        )
    }

    async fn get_transfer_run(
        &self,
        req: crate::model::GetTransferRunRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TransferRun> {
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
            .map(|r: gax::response::Response<crate::model::TransferRun>| r.into_body())
    }

    async fn delete_transfer_run(
        &self,
        req: crate::model::DeleteTransferRunRequest,
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

    async fn list_transfer_runs(
        &self,
        req: crate::model::ListTransferRunsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTransferRunsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}/runs", req.parent))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req.states.iter().fold(builder, |builder, p| {
            builder.query(&[("states", p.value())])
        });
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("runAttempt", &req.run_attempt.value())]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<crate::model::ListTransferRunsResponse>| r.into_body())
    }

    async fn list_transfer_logs(
        &self,
        req: crate::model::ListTransferLogsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTransferLogsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/transferLogs", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = req.message_types.iter().fold(builder, |builder, p| {
            builder.query(&[("messageTypes", p.value())])
        });
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<crate::model::ListTransferLogsResponse>| r.into_body())
    }

    async fn check_valid_creds(
        &self,
        req: crate::model::CheckValidCredsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::CheckValidCredsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:checkValidCreds", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::CheckValidCredsResponse>| r.into_body())
    }

    async fn enroll_data_sources(
        &self,
        req: crate::model::EnrollDataSourcesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:enrollDataSources", req.name),
            )
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

    async fn unenroll_data_sources(
        &self,
        req: crate::model::UnenrollDataSourcesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:unenrollDataSources", req.name),
            )
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

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}/locations", req.name))
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
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<location::model::Location>| r.into_body())
    }
}
