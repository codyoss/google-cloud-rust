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

/// A dyn-compatible, crate-private version of [super::DataAccessControlService].
#[async_trait::async_trait]
pub trait DataAccessControlService: std::fmt::Debug + Send + Sync {
    async fn create_data_access_label(
        &self,
        req: crate::model::CreateDataAccessLabelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataAccessLabel>>;

    async fn get_data_access_label(
        &self,
        req: crate::model::GetDataAccessLabelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataAccessLabel>>;

    async fn list_data_access_labels(
        &self,
        req: crate::model::ListDataAccessLabelsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDataAccessLabelsResponse>>;

    async fn update_data_access_label(
        &self,
        req: crate::model::UpdateDataAccessLabelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataAccessLabel>>;

    async fn delete_data_access_label(
        &self,
        req: crate::model::DeleteDataAccessLabelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn create_data_access_scope(
        &self,
        req: crate::model::CreateDataAccessScopeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataAccessScope>>;

    async fn get_data_access_scope(
        &self,
        req: crate::model::GetDataAccessScopeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataAccessScope>>;

    async fn list_data_access_scopes(
        &self,
        req: crate::model::ListDataAccessScopesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDataAccessScopesResponse>>;

    async fn update_data_access_scope(
        &self,
        req: crate::model::UpdateDataAccessScopeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataAccessScope>>;

    async fn delete_data_access_scope(
        &self,
        req: crate::model::DeleteDataAccessScopeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;
}

/// All implementations of [super::DataAccessControlService] also implement [DataAccessControlService].
#[async_trait::async_trait]
impl<T: super::DataAccessControlService> DataAccessControlService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_data_access_label(
        &self,
        req: crate::model::CreateDataAccessLabelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataAccessLabel>> {
        T::create_data_access_label(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_data_access_label(
        &self,
        req: crate::model::GetDataAccessLabelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataAccessLabel>> {
        T::get_data_access_label(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_data_access_labels(
        &self,
        req: crate::model::ListDataAccessLabelsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDataAccessLabelsResponse>> {
        T::list_data_access_labels(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_data_access_label(
        &self,
        req: crate::model::UpdateDataAccessLabelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataAccessLabel>> {
        T::update_data_access_label(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_data_access_label(
        &self,
        req: crate::model::DeleteDataAccessLabelRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_data_access_label(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_data_access_scope(
        &self,
        req: crate::model::CreateDataAccessScopeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataAccessScope>> {
        T::create_data_access_scope(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_data_access_scope(
        &self,
        req: crate::model::GetDataAccessScopeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataAccessScope>> {
        T::get_data_access_scope(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_data_access_scopes(
        &self,
        req: crate::model::ListDataAccessScopesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDataAccessScopesResponse>> {
        T::list_data_access_scopes(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_data_access_scope(
        &self,
        req: crate::model::UpdateDataAccessScopeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataAccessScope>> {
        T::update_data_access_scope(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_data_access_scope(
        &self,
        req: crate::model::DeleteDataAccessScopeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_data_access_scope(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::cancel_operation(self, req, options).await
    }
}

/// A dyn-compatible, crate-private version of [super::EntityService].
#[async_trait::async_trait]
pub trait EntityService: std::fmt::Debug + Send + Sync {
    async fn get_watchlist(
        &self,
        req: crate::model::GetWatchlistRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Watchlist>>;

    async fn list_watchlists(
        &self,
        req: crate::model::ListWatchlistsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListWatchlistsResponse>>;

    async fn create_watchlist(
        &self,
        req: crate::model::CreateWatchlistRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Watchlist>>;

    async fn update_watchlist(
        &self,
        req: crate::model::UpdateWatchlistRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Watchlist>>;

    async fn delete_watchlist(
        &self,
        req: crate::model::DeleteWatchlistRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;
}

/// All implementations of [super::EntityService] also implement [EntityService].
#[async_trait::async_trait]
impl<T: super::EntityService> EntityService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn get_watchlist(
        &self,
        req: crate::model::GetWatchlistRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Watchlist>> {
        T::get_watchlist(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_watchlists(
        &self,
        req: crate::model::ListWatchlistsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListWatchlistsResponse>> {
        T::list_watchlists(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_watchlist(
        &self,
        req: crate::model::CreateWatchlistRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Watchlist>> {
        T::create_watchlist(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_watchlist(
        &self,
        req: crate::model::UpdateWatchlistRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Watchlist>> {
        T::update_watchlist(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_watchlist(
        &self,
        req: crate::model::DeleteWatchlistRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_watchlist(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::cancel_operation(self, req, options).await
    }
}

/// A dyn-compatible, crate-private version of [super::InstanceService].
#[async_trait::async_trait]
pub trait InstanceService: std::fmt::Debug + Send + Sync {
    async fn get_instance(
        &self,
        req: crate::model::GetInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Instance>>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;
}

/// All implementations of [super::InstanceService] also implement [InstanceService].
#[async_trait::async_trait]
impl<T: super::InstanceService> InstanceService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn get_instance(
        &self,
        req: crate::model::GetInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Instance>> {
        T::get_instance(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::cancel_operation(self, req, options).await
    }
}

/// A dyn-compatible, crate-private version of [super::ReferenceListService].
#[async_trait::async_trait]
pub trait ReferenceListService: std::fmt::Debug + Send + Sync {
    async fn get_reference_list(
        &self,
        req: crate::model::GetReferenceListRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReferenceList>>;

    async fn list_reference_lists(
        &self,
        req: crate::model::ListReferenceListsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListReferenceListsResponse>>;

    async fn create_reference_list(
        &self,
        req: crate::model::CreateReferenceListRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReferenceList>>;

    async fn update_reference_list(
        &self,
        req: crate::model::UpdateReferenceListRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReferenceList>>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;
}

/// All implementations of [super::ReferenceListService] also implement [ReferenceListService].
#[async_trait::async_trait]
impl<T: super::ReferenceListService> ReferenceListService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn get_reference_list(
        &self,
        req: crate::model::GetReferenceListRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReferenceList>> {
        T::get_reference_list(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_reference_lists(
        &self,
        req: crate::model::ListReferenceListsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListReferenceListsResponse>> {
        T::list_reference_lists(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_reference_list(
        &self,
        req: crate::model::CreateReferenceListRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReferenceList>> {
        T::create_reference_list(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_reference_list(
        &self,
        req: crate::model::UpdateReferenceListRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReferenceList>> {
        T::update_reference_list(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::cancel_operation(self, req, options).await
    }
}

/// A dyn-compatible, crate-private version of [super::RuleService].
#[async_trait::async_trait]
pub trait RuleService: std::fmt::Debug + Send + Sync {
    async fn create_rule(
        &self,
        req: crate::model::CreateRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Rule>>;

    async fn get_rule(
        &self,
        req: crate::model::GetRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Rule>>;

    async fn list_rules(
        &self,
        req: crate::model::ListRulesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListRulesResponse>>;

    async fn update_rule(
        &self,
        req: crate::model::UpdateRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Rule>>;

    async fn delete_rule(
        &self,
        req: crate::model::DeleteRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn list_rule_revisions(
        &self,
        req: crate::model::ListRuleRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListRuleRevisionsResponse>>;

    async fn create_retrohunt(
        &self,
        req: crate::model::CreateRetrohuntRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn get_retrohunt(
        &self,
        req: crate::model::GetRetrohuntRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Retrohunt>>;

    async fn list_retrohunts(
        &self,
        req: crate::model::ListRetrohuntsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListRetrohuntsResponse>>;

    async fn get_rule_deployment(
        &self,
        req: crate::model::GetRuleDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::RuleDeployment>>;

    async fn list_rule_deployments(
        &self,
        req: crate::model::ListRuleDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListRuleDeploymentsResponse>>;

    async fn update_rule_deployment(
        &self,
        req: crate::model::UpdateRuleDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::RuleDeployment>>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::RuleService] also implement [RuleService].
#[async_trait::async_trait]
impl<T: super::RuleService> RuleService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_rule(
        &self,
        req: crate::model::CreateRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Rule>> {
        T::create_rule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_rule(
        &self,
        req: crate::model::GetRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Rule>> {
        T::get_rule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_rules(
        &self,
        req: crate::model::ListRulesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListRulesResponse>> {
        T::list_rules(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_rule(
        &self,
        req: crate::model::UpdateRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Rule>> {
        T::update_rule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_rule(
        &self,
        req: crate::model::DeleteRuleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_rule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_rule_revisions(
        &self,
        req: crate::model::ListRuleRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListRuleRevisionsResponse>> {
        T::list_rule_revisions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_retrohunt(
        &self,
        req: crate::model::CreateRetrohuntRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_retrohunt(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_retrohunt(
        &self,
        req: crate::model::GetRetrohuntRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Retrohunt>> {
        T::get_retrohunt(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_retrohunts(
        &self,
        req: crate::model::ListRetrohuntsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListRetrohuntsResponse>> {
        T::list_retrohunts(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_rule_deployment(
        &self,
        req: crate::model::GetRuleDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::RuleDeployment>> {
        T::get_rule_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_rule_deployments(
        &self,
        req: crate::model::ListRuleDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListRuleDeploymentsResponse>> {
        T::list_rule_deployments(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_rule_deployment(
        &self,
        req: crate::model::UpdateRuleDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::RuleDeployment>> {
        T::update_rule_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::cancel_operation(self, req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        T::get_polling_error_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}
