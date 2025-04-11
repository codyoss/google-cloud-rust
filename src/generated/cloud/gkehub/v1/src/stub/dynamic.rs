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

/// A dyn-compatible, crate-private version of [super::GkeHub].
#[async_trait::async_trait]
pub trait GkeHub: std::fmt::Debug + Send + Sync {
    async fn list_memberships(
        &self,
        req: crate::model::ListMembershipsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListMembershipsResponse>>;

    async fn list_features(
        &self,
        req: crate::model::ListFeaturesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListFeaturesResponse>>;

    async fn get_membership(
        &self,
        req: crate::model::GetMembershipRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Membership>>;

    async fn get_feature(
        &self,
        req: crate::model::GetFeatureRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Feature>>;

    async fn create_membership(
        &self,
        req: crate::model::CreateMembershipRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn create_feature(
        &self,
        req: crate::model::CreateFeatureRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_membership(
        &self,
        req: crate::model::DeleteMembershipRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_feature(
        &self,
        req: crate::model::DeleteFeatureRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn update_membership(
        &self,
        req: crate::model::UpdateMembershipRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn update_feature(
        &self,
        req: crate::model::UpdateFeatureRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn generate_connect_manifest(
        &self,
        req: crate::model::GenerateConnectManifestRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::GenerateConnectManifestResponse>>;

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

/// All implementations of [super::GkeHub] also implement [GkeHub].
#[async_trait::async_trait]
impl<T: super::GkeHub> GkeHub for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_memberships(
        &self,
        req: crate::model::ListMembershipsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListMembershipsResponse>> {
        T::list_memberships(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_features(
        &self,
        req: crate::model::ListFeaturesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListFeaturesResponse>> {
        T::list_features(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_membership(
        &self,
        req: crate::model::GetMembershipRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Membership>> {
        T::get_membership(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_feature(
        &self,
        req: crate::model::GetFeatureRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Feature>> {
        T::get_feature(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_membership(
        &self,
        req: crate::model::CreateMembershipRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_membership(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_feature(
        &self,
        req: crate::model::CreateFeatureRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_feature(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_membership(
        &self,
        req: crate::model::DeleteMembershipRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_membership(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_feature(
        &self,
        req: crate::model::DeleteFeatureRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_feature(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_membership(
        &self,
        req: crate::model::UpdateMembershipRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::update_membership(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_feature(
        &self,
        req: crate::model::UpdateFeatureRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::update_feature(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn generate_connect_manifest(
        &self,
        req: crate::model::GenerateConnectManifestRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::GenerateConnectManifestResponse>> {
        T::generate_connect_manifest(self, req, options).await
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
