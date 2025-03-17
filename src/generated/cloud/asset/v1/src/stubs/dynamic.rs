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

use std::sync::Arc;

/// A dyn-compatible, crate-private version of [super::AssetService].
#[async_trait::async_trait]
pub trait AssetService: std::fmt::Debug + Send + Sync {
    async fn export_assets(
        &self,
        req: crate::model::ExportAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_assets(
        &self,
        req: crate::model::ListAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListAssetsResponse>;

    async fn batch_get_assets_history(
        &self,
        req: crate::model::BatchGetAssetsHistoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BatchGetAssetsHistoryResponse>;

    async fn create_feed(
        &self,
        req: crate::model::CreateFeedRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Feed>;

    async fn get_feed(
        &self,
        req: crate::model::GetFeedRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Feed>;

    async fn list_feeds(
        &self,
        req: crate::model::ListFeedsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListFeedsResponse>;

    async fn update_feed(
        &self,
        req: crate::model::UpdateFeedRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Feed>;

    async fn delete_feed(
        &self,
        req: crate::model::DeleteFeedRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn search_all_resources(
        &self,
        req: crate::model::SearchAllResourcesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchAllResourcesResponse>;

    async fn search_all_iam_policies(
        &self,
        req: crate::model::SearchAllIamPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchAllIamPoliciesResponse>;

    async fn analyze_iam_policy(
        &self,
        req: crate::model::AnalyzeIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AnalyzeIamPolicyResponse>;

    async fn analyze_iam_policy_longrunning(
        &self,
        req: crate::model::AnalyzeIamPolicyLongrunningRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn analyze_move(
        &self,
        req: crate::model::AnalyzeMoveRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AnalyzeMoveResponse>;

    async fn query_assets(
        &self,
        req: crate::model::QueryAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::QueryAssetsResponse>;

    async fn create_saved_query(
        &self,
        req: crate::model::CreateSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SavedQuery>;

    async fn get_saved_query(
        &self,
        req: crate::model::GetSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SavedQuery>;

    async fn list_saved_queries(
        &self,
        req: crate::model::ListSavedQueriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListSavedQueriesResponse>;

    async fn update_saved_query(
        &self,
        req: crate::model::UpdateSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SavedQuery>;

    async fn delete_saved_query(
        &self,
        req: crate::model::DeleteSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn batch_get_effective_iam_policies(
        &self,
        req: crate::model::BatchGetEffectiveIamPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BatchGetEffectiveIamPoliciesResponse>;

    async fn analyze_org_policies(
        &self,
        req: crate::model::AnalyzeOrgPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AnalyzeOrgPoliciesResponse>;

    async fn analyze_org_policy_governed_containers(
        &self,
        req: crate::model::AnalyzeOrgPolicyGovernedContainersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AnalyzeOrgPolicyGovernedContainersResponse>;

    async fn analyze_org_policy_governed_assets(
        &self,
        req: crate::model::AnalyzeOrgPolicyGovernedAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AnalyzeOrgPolicyGovernedAssetsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::AssetService] also implement [AssetService].
#[async_trait::async_trait]
impl<T: super::AssetService> AssetService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn export_assets(
        &self,
        req: crate::model::ExportAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::export_assets(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_assets(
        &self,
        req: crate::model::ListAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListAssetsResponse> {
        T::list_assets(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn batch_get_assets_history(
        &self,
        req: crate::model::BatchGetAssetsHistoryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BatchGetAssetsHistoryResponse> {
        T::batch_get_assets_history(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_feed(
        &self,
        req: crate::model::CreateFeedRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Feed> {
        T::create_feed(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_feed(
        &self,
        req: crate::model::GetFeedRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Feed> {
        T::get_feed(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_feeds(
        &self,
        req: crate::model::ListFeedsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListFeedsResponse> {
        T::list_feeds(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_feed(
        &self,
        req: crate::model::UpdateFeedRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Feed> {
        T::update_feed(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_feed(
        &self,
        req: crate::model::DeleteFeedRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_feed(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn search_all_resources(
        &self,
        req: crate::model::SearchAllResourcesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchAllResourcesResponse> {
        T::search_all_resources(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn search_all_iam_policies(
        &self,
        req: crate::model::SearchAllIamPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchAllIamPoliciesResponse> {
        T::search_all_iam_policies(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn analyze_iam_policy(
        &self,
        req: crate::model::AnalyzeIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AnalyzeIamPolicyResponse> {
        T::analyze_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn analyze_iam_policy_longrunning(
        &self,
        req: crate::model::AnalyzeIamPolicyLongrunningRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::analyze_iam_policy_longrunning(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn analyze_move(
        &self,
        req: crate::model::AnalyzeMoveRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AnalyzeMoveResponse> {
        T::analyze_move(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn query_assets(
        &self,
        req: crate::model::QueryAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::QueryAssetsResponse> {
        T::query_assets(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_saved_query(
        &self,
        req: crate::model::CreateSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SavedQuery> {
        T::create_saved_query(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_saved_query(
        &self,
        req: crate::model::GetSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SavedQuery> {
        T::get_saved_query(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_saved_queries(
        &self,
        req: crate::model::ListSavedQueriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListSavedQueriesResponse> {
        T::list_saved_queries(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_saved_query(
        &self,
        req: crate::model::UpdateSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SavedQuery> {
        T::update_saved_query(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_saved_query(
        &self,
        req: crate::model::DeleteSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_saved_query(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn batch_get_effective_iam_policies(
        &self,
        req: crate::model::BatchGetEffectiveIamPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BatchGetEffectiveIamPoliciesResponse> {
        T::batch_get_effective_iam_policies(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn analyze_org_policies(
        &self,
        req: crate::model::AnalyzeOrgPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AnalyzeOrgPoliciesResponse> {
        T::analyze_org_policies(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn analyze_org_policy_governed_containers(
        &self,
        req: crate::model::AnalyzeOrgPolicyGovernedContainersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AnalyzeOrgPolicyGovernedContainersResponse> {
        T::analyze_org_policy_governed_containers(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn analyze_org_policy_governed_assets(
        &self,
        req: crate::model::AnalyzeOrgPolicyGovernedAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AnalyzeOrgPolicyGovernedAssetsResponse> {
        T::analyze_org_policy_governed_assets(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        T::get_polling_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}
