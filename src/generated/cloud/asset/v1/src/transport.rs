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

/// Implements [AssetService](crate::stubs::AssetService) using a [gclient::ReqwestClient].
#[derive(Clone)]
pub struct AssetService {
    inner: gclient::ReqwestClient,
}

impl std::fmt::Debug for AssetService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("AssetService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl AssetService {
    pub async fn new(config: gclient::ClientConfig) -> Result<Self> {
        let inner = gclient::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl crate::stubs::AssetService for AssetService {
    async fn export_assets(
        &self,
        req: crate::model::ExportAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:exportAssets", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn list_assets(
        &self,
        req: crate::model::ListAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAssetsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}/assets", req.parent))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .read_time
            .as_ref()
            .map(|p| serde_json::to_value(p).map_err(Error::serde))
            .transpose()?
            .into_iter()
            .fold(builder, |builder, v| {
                use gclient::query_parameter::QueryParameter;
                v.add(builder, "readTime")
            });
        let builder = req
            .asset_types
            .iter()
            .fold(builder, |builder, p| builder.query(&[("assetTypes", p)]));
        let builder = builder.query(&[("contentType", &req.content_type.value())]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = req.relationship_types.iter().fold(builder, |builder, p| {
            builder.query(&[("relationshipTypes", p)])
        });
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn batch_get_assets_history(
        &self,
        req: crate::model::BatchGetAssetsHistoryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::BatchGetAssetsHistoryResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}:batchGetAssetsHistory", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .asset_names
            .iter()
            .fold(builder, |builder, p| builder.query(&[("assetNames", p)]));
        let builder = builder.query(&[("contentType", &req.content_type.value())]);
        let builder = req
            .read_time_window
            .as_ref()
            .map(|p| serde_json::to_value(p).map_err(Error::serde))
            .transpose()?
            .into_iter()
            .fold(builder, |builder, v| {
                use gclient::query_parameter::QueryParameter;
                v.add(builder, "readTimeWindow")
            });
        let builder = req.relationship_types.iter().fold(builder, |builder, p| {
            builder.query(&[("relationshipTypes", p)])
        });
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn create_feed(
        &self,
        req: crate::model::CreateFeedRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Feed> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}/feeds", req.parent))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn get_feed(
        &self,
        req: crate::model::GetFeedRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Feed> {
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

    async fn list_feeds(
        &self,
        req: crate::model::ListFeedsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListFeedsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}/feeds", req.parent))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn update_feed(
        &self,
        req: crate::model::UpdateFeedRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Feed> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/v1/{}",
                    req.feed
                        .as_ref()
                        .ok_or_else(|| gclient::path_parameter::missing("feed"))?
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

    async fn delete_feed(
        &self,
        req: crate::model::DeleteFeedRequest,
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

    async fn search_all_resources(
        &self,
        req: crate::model::SearchAllResourcesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SearchAllResourcesResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}:searchAllResources", req.scope),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("query", &req.query)]);
        let builder = req
            .asset_types
            .iter()
            .fold(builder, |builder, p| builder.query(&[("assetTypes", p)]));
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("orderBy", &req.order_by)]);
        let builder = req
            .read_mask
            .as_ref()
            .map(|p| serde_json::to_value(p).map_err(Error::serde))
            .transpose()?
            .into_iter()
            .fold(builder, |builder, v| {
                use gclient::query_parameter::QueryParameter;
                v.add(builder, "readMask")
            });
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn search_all_iam_policies(
        &self,
        req: crate::model::SearchAllIamPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SearchAllIamPoliciesResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}:searchAllIamPolicies", req.scope),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("query", &req.query)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = req
            .asset_types
            .iter()
            .fold(builder, |builder, p| builder.query(&[("assetTypes", p)]));
        let builder = builder.query(&[("orderBy", &req.order_by)]);
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn analyze_iam_policy(
        &self,
        req: crate::model::AnalyzeIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AnalyzeIamPolicyResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/{}:analyzeIamPolicy",
                    req.analysis_query
                        .as_ref()
                        .ok_or_else(|| gclient::path_parameter::missing("analysis_query"))?
                        .scope
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .analysis_query
            .as_ref()
            .map(|p| serde_json::to_value(p).map_err(Error::serde))
            .transpose()?
            .into_iter()
            .fold(builder, |builder, v| {
                use gclient::query_parameter::QueryParameter;
                v.add(builder, "analysisQuery")
            });
        let builder = builder.query(&[("savedAnalysisQuery", &req.saved_analysis_query)]);
        let builder = req
            .execution_timeout
            .as_ref()
            .map(|p| serde_json::to_value(p).map_err(Error::serde))
            .transpose()?
            .into_iter()
            .fold(builder, |builder, v| {
                use gclient::query_parameter::QueryParameter;
                v.add(builder, "executionTimeout")
            });
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn analyze_iam_policy_longrunning(
        &self,
        req: crate::model::AnalyzeIamPolicyLongrunningRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/{}:analyzeIamPolicyLongrunning",
                    req.analysis_query
                        .as_ref()
                        .ok_or_else(|| gclient::path_parameter::missing("analysis_query"))?
                        .scope
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn analyze_move(
        &self,
        req: crate::model::AnalyzeMoveRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AnalyzeMoveResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}:analyzeMove", req.resource),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("destinationParent", &req.destination_parent)]);
        let builder = builder.query(&[("view", &req.view.value())]);
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn query_assets(
        &self,
        req: crate::model::QueryAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::QueryAssetsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:queryAssets", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn create_saved_query(
        &self,
        req: crate::model::CreateSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SavedQuery> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/savedQueries", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("savedQueryId", &req.saved_query_id)]);
        self.inner
            .execute(builder, Some(req.saved_query), options)
            .await
    }

    async fn get_saved_query(
        &self,
        req: crate::model::GetSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SavedQuery> {
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

    async fn list_saved_queries(
        &self,
        req: crate::model::ListSavedQueriesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSavedQueriesResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/savedQueries", req.parent),
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
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn update_saved_query(
        &self,
        req: crate::model::UpdateSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SavedQuery> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/v1/{}",
                    req.saved_query
                        .as_ref()
                        .ok_or_else(|| gclient::path_parameter::missing("saved_query"))?
                        .name
                ),
            )
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
        self.inner
            .execute(builder, Some(req.saved_query), options)
            .await
    }

    async fn delete_saved_query(
        &self,
        req: crate::model::DeleteSavedQueryRequest,
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

    async fn batch_get_effective_iam_policies(
        &self,
        req: crate::model::BatchGetEffectiveIamPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::BatchGetEffectiveIamPoliciesResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/effectiveIamPolicies:batchGet", req.scope),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .names
            .iter()
            .fold(builder, |builder, p| builder.query(&[("names", p)]));
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn analyze_org_policies(
        &self,
        req: crate::model::AnalyzeOrgPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AnalyzeOrgPoliciesResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}:analyzeOrgPolicies", req.scope),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("constraint", &req.constraint)]);
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = req
            .page_size
            .iter()
            .fold(builder, |builder, p| builder.query(&[("pageSize", p)]));
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn analyze_org_policy_governed_containers(
        &self,
        req: crate::model::AnalyzeOrgPolicyGovernedContainersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AnalyzeOrgPolicyGovernedContainersResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}:analyzeOrgPolicyGovernedContainers", req.scope),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("constraint", &req.constraint)]);
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = req
            .page_size
            .iter()
            .fold(builder, |builder, p| builder.query(&[("pageSize", p)]));
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn analyze_org_policy_governed_assets(
        &self,
        req: crate::model::AnalyzeOrgPolicyGovernedAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AnalyzeOrgPolicyGovernedAssetsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}:analyzeOrgPolicyGovernedAssets", req.scope),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("constraint", &req.constraint)]);
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = req
            .page_size
            .iter()
            .fold(builder, |builder, p| builder.query(&[("pageSize", p)]));
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
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
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy> {
        self.inner.get_polling_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
