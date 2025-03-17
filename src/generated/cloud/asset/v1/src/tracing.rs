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

/// Implements a [AssetService](super::stubs::AssetService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct AssetService<T>
where
    T: super::stubs::AssetService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> AssetService<T>
where
    T: super::stubs::AssetService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::AssetService for AssetService<T>
where
    T: super::stubs::AssetService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn export_assets(
        &self,
        req: crate::model::ExportAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.export_assets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_assets(
        &self,
        req: crate::model::ListAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAssetsResponse> {
        self.inner.list_assets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn batch_get_assets_history(
        &self,
        req: crate::model::BatchGetAssetsHistoryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::BatchGetAssetsHistoryResponse> {
        self.inner.batch_get_assets_history(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_feed(
        &self,
        req: crate::model::CreateFeedRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Feed> {
        self.inner.create_feed(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_feed(
        &self,
        req: crate::model::GetFeedRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Feed> {
        self.inner.get_feed(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_feeds(
        &self,
        req: crate::model::ListFeedsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListFeedsResponse> {
        self.inner.list_feeds(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_feed(
        &self,
        req: crate::model::UpdateFeedRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Feed> {
        self.inner.update_feed(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_feed(
        &self,
        req: crate::model::DeleteFeedRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_feed(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn search_all_resources(
        &self,
        req: crate::model::SearchAllResourcesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SearchAllResourcesResponse> {
        self.inner.search_all_resources(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn search_all_iam_policies(
        &self,
        req: crate::model::SearchAllIamPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SearchAllIamPoliciesResponse> {
        self.inner.search_all_iam_policies(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn analyze_iam_policy(
        &self,
        req: crate::model::AnalyzeIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AnalyzeIamPolicyResponse> {
        self.inner.analyze_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn analyze_iam_policy_longrunning(
        &self,
        req: crate::model::AnalyzeIamPolicyLongrunningRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner
            .analyze_iam_policy_longrunning(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn analyze_move(
        &self,
        req: crate::model::AnalyzeMoveRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AnalyzeMoveResponse> {
        self.inner.analyze_move(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn query_assets(
        &self,
        req: crate::model::QueryAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::QueryAssetsResponse> {
        self.inner.query_assets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_saved_query(
        &self,
        req: crate::model::CreateSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SavedQuery> {
        self.inner.create_saved_query(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_saved_query(
        &self,
        req: crate::model::GetSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SavedQuery> {
        self.inner.get_saved_query(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_saved_queries(
        &self,
        req: crate::model::ListSavedQueriesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSavedQueriesResponse> {
        self.inner.list_saved_queries(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_saved_query(
        &self,
        req: crate::model::UpdateSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SavedQuery> {
        self.inner.update_saved_query(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_saved_query(
        &self,
        req: crate::model::DeleteSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_saved_query(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn batch_get_effective_iam_policies(
        &self,
        req: crate::model::BatchGetEffectiveIamPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::BatchGetEffectiveIamPoliciesResponse> {
        self.inner
            .batch_get_effective_iam_policies(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn analyze_org_policies(
        &self,
        req: crate::model::AnalyzeOrgPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AnalyzeOrgPoliciesResponse> {
        self.inner.analyze_org_policies(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn analyze_org_policy_governed_containers(
        &self,
        req: crate::model::AnalyzeOrgPolicyGovernedContainersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AnalyzeOrgPolicyGovernedContainersResponse> {
        self.inner
            .analyze_org_policy_governed_containers(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn analyze_org_policy_governed_assets(
        &self,
        req: crate::model::AnalyzeOrgPolicyGovernedAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AnalyzeOrgPolicyGovernedAssetsResponse> {
        self.inner
            .analyze_org_policy_governed_assets(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
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
