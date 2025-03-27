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

/// Implements a [GkeHub](super::stubs::GkeHub) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct GkeHub<T>
where
    T: super::stubs::GkeHub + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> GkeHub<T>
where
    T: super::stubs::GkeHub + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::GkeHub for GkeHub<T>
where
    T: super::stubs::GkeHub + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_memberships(
        &self,
        req: crate::model::ListMembershipsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListMembershipsResponse> {
        self.inner.list_memberships(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_features(
        &self,
        req: crate::model::ListFeaturesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListFeaturesResponse> {
        self.inner.list_features(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_membership(
        &self,
        req: crate::model::GetMembershipRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Membership> {
        self.inner.get_membership(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_feature(
        &self,
        req: crate::model::GetFeatureRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Feature> {
        self.inner.get_feature(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_membership(
        &self,
        req: crate::model::CreateMembershipRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_membership(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_feature(
        &self,
        req: crate::model::CreateFeatureRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_feature(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_membership(
        &self,
        req: crate::model::DeleteMembershipRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_membership(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_feature(
        &self,
        req: crate::model::DeleteFeatureRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_feature(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_membership(
        &self,
        req: crate::model::UpdateMembershipRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_membership(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_feature(
        &self,
        req: crate::model::UpdateFeatureRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_feature(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn generate_connect_manifest(
        &self,
        req: crate::model::GenerateConnectManifestRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GenerateConnectManifestResponse> {
        self.inner.generate_connect_manifest(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.cancel_operation(req, options).await
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
