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

/// Implements [ClusterManager](super::stub::ClusterManager) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct ClusterManager {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for ClusterManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("ClusterManager")
            .field("inner", &self.inner)
            .finish()
    }
}

impl ClusterManager {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::ClusterManager for ClusterManager {
    async fn list_clusters(
        &self,
        req: crate::model::ListClustersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListClustersResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}/clusters", req.parent))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("projectId", &req.project_id)]);
        let builder = builder.query(&[("zone", &req.zone)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<crate::model::ListClustersResponse>| r.into_body())
    }

    async fn get_cluster(
        &self,
        req: crate::model::GetClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Cluster> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("projectId", &req.project_id)]);
        let builder = builder.query(&[("zone", &req.zone)]);
        let builder = builder.query(&[("clusterId", &req.cluster_id)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<crate::model::Cluster>| r.into_body())
    }

    async fn create_cluster(
        &self,
        req: crate::model::CreateClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/clusters", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn update_cluster(
        &self,
        req: crate::model::UpdateClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::PUT.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::PUT, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn update_node_pool(
        &self,
        req: crate::model::UpdateNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::PUT.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::PUT, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn set_node_pool_autoscaling(
        &self,
        req: crate::model::SetNodePoolAutoscalingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:setAutoscaling", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn set_logging_service(
        &self,
        req: crate::model::SetLoggingServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:setLogging", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn set_monitoring_service(
        &self,
        req: crate::model::SetMonitoringServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:setMonitoring", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn set_addons_config(
        &self,
        req: crate::model::SetAddonsConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:setAddons", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn set_locations(
        &self,
        req: crate::model::SetLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:setLocations", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn update_master(
        &self,
        req: crate::model::UpdateMasterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:updateMaster", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn set_master_auth(
        &self,
        req: crate::model::SetMasterAuthRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:setMasterAuth", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn delete_cluster(
        &self,
        req: crate::model::DeleteClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::DELETE.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::DELETE, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("projectId", &req.project_id)]);
        let builder = builder.query(&[("zone", &req.zone)]);
        let builder = builder.query(&[("clusterId", &req.cluster_id)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn list_operations(
        &self,
        req: crate::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListOperationsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/operations", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("projectId", &req.project_id)]);
        let builder = builder.query(&[("zone", &req.zone)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<crate::model::ListOperationsResponse>| r.into_body())
    }

    async fn get_operation(
        &self,
        req: crate::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("projectId", &req.project_id)]);
        let builder = builder.query(&[("zone", &req.zone)]);
        let builder = builder.query(&[("operationId", &req.operation_id)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn cancel_operation(
        &self,
        req: crate::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:cancel", req.name))
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

    async fn get_server_config(
        &self,
        req: crate::model::GetServerConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ServerConfig> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/serverConfig", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("projectId", &req.project_id)]);
        let builder = builder.query(&[("zone", &req.zone)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<crate::model::ServerConfig>| r.into_body())
    }

    async fn get_json_web_keys(
        &self,
        req: crate::model::GetJSONWebKeysRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GetJSONWebKeysResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}/jwks", req.parent))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<crate::model::GetJSONWebKeysResponse>| r.into_body())
    }

    async fn list_node_pools(
        &self,
        req: crate::model::ListNodePoolsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListNodePoolsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/nodePools", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("projectId", &req.project_id)]);
        let builder = builder.query(&[("zone", &req.zone)]);
        let builder = builder.query(&[("clusterId", &req.cluster_id)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<crate::model::ListNodePoolsResponse>| r.into_body())
    }

    async fn get_node_pool(
        &self,
        req: crate::model::GetNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::NodePool> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("projectId", &req.project_id)]);
        let builder = builder.query(&[("zone", &req.zone)]);
        let builder = builder.query(&[("clusterId", &req.cluster_id)]);
        let builder = builder.query(&[("nodePoolId", &req.node_pool_id)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<crate::model::NodePool>| r.into_body())
    }

    async fn create_node_pool(
        &self,
        req: crate::model::CreateNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/nodePools", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn delete_node_pool(
        &self,
        req: crate::model::DeleteNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::DELETE.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::DELETE, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("projectId", &req.project_id)]);
        let builder = builder.query(&[("zone", &req.zone)]);
        let builder = builder.query(&[("clusterId", &req.cluster_id)]);
        let builder = builder.query(&[("nodePoolId", &req.node_pool_id)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn complete_node_pool_upgrade(
        &self,
        req: crate::model::CompleteNodePoolUpgradeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:completeUpgrade", req.name),
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

    async fn rollback_node_pool_upgrade(
        &self,
        req: crate::model::RollbackNodePoolUpgradeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:rollback", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn set_node_pool_management(
        &self,
        req: crate::model::SetNodePoolManagementRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:setManagement", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn set_labels(
        &self,
        req: crate::model::SetLabelsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:setResourceLabels", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn set_legacy_abac(
        &self,
        req: crate::model::SetLegacyAbacRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:setLegacyAbac", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn start_ip_rotation(
        &self,
        req: crate::model::StartIPRotationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:startIpRotation", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn complete_ip_rotation(
        &self,
        req: crate::model::CompleteIPRotationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:completeIpRotation", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn set_node_pool_size(
        &self,
        req: crate::model::SetNodePoolSizeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:setSize", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn set_network_policy(
        &self,
        req: crate::model::SetNetworkPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:setNetworkPolicy", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn set_maintenance_policy(
        &self,
        req: crate::model::SetMaintenancePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:setMaintenancePolicy", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req), options)
            .await
            .map(|r: gax::response::Response<crate::model::Operation>| r.into_body())
    }

    async fn list_usable_subnetworks(
        &self,
        req: crate::model::ListUsableSubnetworksRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListUsableSubnetworksResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/aggregated/usableSubnetworks", req.parent),
            )
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
                |r: gax::response::Response<crate::model::ListUsableSubnetworksResponse>| {
                    r.into_body()
                },
            )
    }

    async fn check_autopilot_compatibility(
        &self,
        req: crate::model::CheckAutopilotCompatibilityRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::CheckAutopilotCompatibilityResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}:checkAutopilotCompatibility", req.name),
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
                |r: gax::response::Response<crate::model::CheckAutopilotCompatibilityResponse>| {
                    r.into_body()
                },
            )
    }
}
