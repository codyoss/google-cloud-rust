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

/// A dyn-compatible, crate-private version of [super::ManagedIdentitiesService].
#[async_trait::async_trait]
pub trait ManagedIdentitiesService: std::fmt::Debug + Send + Sync {
    async fn create_microsoft_ad_domain(
        &self,
        req: crate::model::CreateMicrosoftAdDomainRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn reset_admin_password(
        &self,
        req: crate::model::ResetAdminPasswordRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ResetAdminPasswordResponse>;

    async fn list_domains(
        &self,
        req: crate::model::ListDomainsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDomainsResponse>;

    async fn get_domain(
        &self,
        req: crate::model::GetDomainRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Domain>;

    async fn update_domain(
        &self,
        req: crate::model::UpdateDomainRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_domain(
        &self,
        req: crate::model::DeleteDomainRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn attach_trust(
        &self,
        req: crate::model::AttachTrustRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn reconfigure_trust(
        &self,
        req: crate::model::ReconfigureTrustRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn detach_trust(
        &self,
        req: crate::model::DetachTrustRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn validate_trust(
        &self,
        req: crate::model::ValidateTrustRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::ManagedIdentitiesService] also implement [ManagedIdentitiesService].
#[async_trait::async_trait]
impl<T: super::ManagedIdentitiesService> ManagedIdentitiesService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_microsoft_ad_domain(
        &self,
        req: crate::model::CreateMicrosoftAdDomainRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_microsoft_ad_domain(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn reset_admin_password(
        &self,
        req: crate::model::ResetAdminPasswordRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ResetAdminPasswordResponse> {
        T::reset_admin_password(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_domains(
        &self,
        req: crate::model::ListDomainsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDomainsResponse> {
        T::list_domains(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_domain(
        &self,
        req: crate::model::GetDomainRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Domain> {
        T::get_domain(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_domain(
        &self,
        req: crate::model::UpdateDomainRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_domain(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_domain(
        &self,
        req: crate::model::DeleteDomainRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_domain(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn attach_trust(
        &self,
        req: crate::model::AttachTrustRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::attach_trust(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn reconfigure_trust(
        &self,
        req: crate::model::ReconfigureTrustRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::reconfigure_trust(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn detach_trust(
        &self,
        req: crate::model::DetachTrustRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::detach_trust(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn validate_trust(
        &self,
        req: crate::model::ValidateTrustRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::validate_trust(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::cancel_operation(self, req, options).await
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
