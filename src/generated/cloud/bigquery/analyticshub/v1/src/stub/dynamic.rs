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

/// A dyn-compatible, crate-private version of [super::AnalyticsHubService].
#[async_trait::async_trait]
pub trait AnalyticsHubService: std::fmt::Debug + Send + Sync {
    async fn list_data_exchanges(
        &self,
        req: crate::model::ListDataExchangesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDataExchangesResponse>>;

    async fn list_org_data_exchanges(
        &self,
        req: crate::model::ListOrgDataExchangesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListOrgDataExchangesResponse>>;

    async fn get_data_exchange(
        &self,
        req: crate::model::GetDataExchangeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataExchange>>;

    async fn create_data_exchange(
        &self,
        req: crate::model::CreateDataExchangeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataExchange>>;

    async fn update_data_exchange(
        &self,
        req: crate::model::UpdateDataExchangeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataExchange>>;

    async fn delete_data_exchange(
        &self,
        req: crate::model::DeleteDataExchangeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn list_listings(
        &self,
        req: crate::model::ListListingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListListingsResponse>>;

    async fn get_listing(
        &self,
        req: crate::model::GetListingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Listing>>;

    async fn create_listing(
        &self,
        req: crate::model::CreateListingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Listing>>;

    async fn update_listing(
        &self,
        req: crate::model::UpdateListingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Listing>>;

    async fn delete_listing(
        &self,
        req: crate::model::DeleteListingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn subscribe_listing(
        &self,
        req: crate::model::SubscribeListingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SubscribeListingResponse>>;

    async fn subscribe_data_exchange(
        &self,
        req: crate::model::SubscribeDataExchangeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn refresh_subscription(
        &self,
        req: crate::model::RefreshSubscriptionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn get_subscription(
        &self,
        req: crate::model::GetSubscriptionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Subscription>>;

    async fn list_subscriptions(
        &self,
        req: crate::model::ListSubscriptionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListSubscriptionsResponse>>;

    async fn list_shared_resource_subscriptions(
        &self,
        req: crate::model::ListSharedResourceSubscriptionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListSharedResourceSubscriptionsResponse>>;

    async fn revoke_subscription(
        &self,
        req: crate::model::RevokeSubscriptionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::RevokeSubscriptionResponse>>;

    async fn delete_subscription(
        &self,
        req: crate::model::DeleteSubscriptionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam_v1::model::Policy>>;

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam_v1::model::Policy>>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::AnalyticsHubService] also implement [AnalyticsHubService].
#[async_trait::async_trait]
impl<T: super::AnalyticsHubService> AnalyticsHubService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_data_exchanges(
        &self,
        req: crate::model::ListDataExchangesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDataExchangesResponse>> {
        T::list_data_exchanges(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_org_data_exchanges(
        &self,
        req: crate::model::ListOrgDataExchangesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListOrgDataExchangesResponse>> {
        T::list_org_data_exchanges(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_data_exchange(
        &self,
        req: crate::model::GetDataExchangeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataExchange>> {
        T::get_data_exchange(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_data_exchange(
        &self,
        req: crate::model::CreateDataExchangeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataExchange>> {
        T::create_data_exchange(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_data_exchange(
        &self,
        req: crate::model::UpdateDataExchangeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DataExchange>> {
        T::update_data_exchange(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_data_exchange(
        &self,
        req: crate::model::DeleteDataExchangeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_data_exchange(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_listings(
        &self,
        req: crate::model::ListListingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListListingsResponse>> {
        T::list_listings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_listing(
        &self,
        req: crate::model::GetListingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Listing>> {
        T::get_listing(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_listing(
        &self,
        req: crate::model::CreateListingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Listing>> {
        T::create_listing(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_listing(
        &self,
        req: crate::model::UpdateListingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Listing>> {
        T::update_listing(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_listing(
        &self,
        req: crate::model::DeleteListingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_listing(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn subscribe_listing(
        &self,
        req: crate::model::SubscribeListingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SubscribeListingResponse>> {
        T::subscribe_listing(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn subscribe_data_exchange(
        &self,
        req: crate::model::SubscribeDataExchangeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::subscribe_data_exchange(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn refresh_subscription(
        &self,
        req: crate::model::RefreshSubscriptionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::refresh_subscription(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_subscription(
        &self,
        req: crate::model::GetSubscriptionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Subscription>> {
        T::get_subscription(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_subscriptions(
        &self,
        req: crate::model::ListSubscriptionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListSubscriptionsResponse>> {
        T::list_subscriptions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_shared_resource_subscriptions(
        &self,
        req: crate::model::ListSharedResourceSubscriptionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListSharedResourceSubscriptionsResponse>>
    {
        T::list_shared_resource_subscriptions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn revoke_subscription(
        &self,
        req: crate::model::RevokeSubscriptionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::RevokeSubscriptionResponse>> {
        T::revoke_subscription(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_subscription(
        &self,
        req: crate::model::DeleteSubscriptionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_subscription(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam_v1::model::Policy>> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam_v1::model::Policy>> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>> {
        T::test_iam_permissions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
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
