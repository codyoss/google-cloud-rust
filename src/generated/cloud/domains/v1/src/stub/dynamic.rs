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

/// A dyn-compatible, crate-private version of [super::Domains].
#[async_trait::async_trait]
pub trait Domains: std::fmt::Debug + Send + Sync {
    async fn search_domains(
        &self,
        req: crate::model::SearchDomainsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SearchDomainsResponse>>;

    async fn retrieve_register_parameters(
        &self,
        req: crate::model::RetrieveRegisterParametersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::RetrieveRegisterParametersResponse>>;

    async fn register_domain(
        &self,
        req: crate::model::RegisterDomainRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn retrieve_transfer_parameters(
        &self,
        req: crate::model::RetrieveTransferParametersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::RetrieveTransferParametersResponse>>;

    async fn transfer_domain(
        &self,
        req: crate::model::TransferDomainRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn list_registrations(
        &self,
        req: crate::model::ListRegistrationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListRegistrationsResponse>>;

    async fn get_registration(
        &self,
        req: crate::model::GetRegistrationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Registration>>;

    async fn update_registration(
        &self,
        req: crate::model::UpdateRegistrationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn configure_management_settings(
        &self,
        req: crate::model::ConfigureManagementSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn configure_dns_settings(
        &self,
        req: crate::model::ConfigureDnsSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn configure_contact_settings(
        &self,
        req: crate::model::ConfigureContactSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn export_registration(
        &self,
        req: crate::model::ExportRegistrationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_registration(
        &self,
        req: crate::model::DeleteRegistrationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn retrieve_authorization_code(
        &self,
        req: crate::model::RetrieveAuthorizationCodeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AuthorizationCode>>;

    async fn reset_authorization_code(
        &self,
        req: crate::model::ResetAuthorizationCodeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AuthorizationCode>>;

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

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::Domains] also implement [Domains].
#[async_trait::async_trait]
impl<T: super::Domains> Domains for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn search_domains(
        &self,
        req: crate::model::SearchDomainsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SearchDomainsResponse>> {
        T::search_domains(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn retrieve_register_parameters(
        &self,
        req: crate::model::RetrieveRegisterParametersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::RetrieveRegisterParametersResponse>>
    {
        T::retrieve_register_parameters(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn register_domain(
        &self,
        req: crate::model::RegisterDomainRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::register_domain(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn retrieve_transfer_parameters(
        &self,
        req: crate::model::RetrieveTransferParametersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::RetrieveTransferParametersResponse>>
    {
        T::retrieve_transfer_parameters(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn transfer_domain(
        &self,
        req: crate::model::TransferDomainRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::transfer_domain(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_registrations(
        &self,
        req: crate::model::ListRegistrationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListRegistrationsResponse>> {
        T::list_registrations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_registration(
        &self,
        req: crate::model::GetRegistrationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Registration>> {
        T::get_registration(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_registration(
        &self,
        req: crate::model::UpdateRegistrationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::update_registration(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn configure_management_settings(
        &self,
        req: crate::model::ConfigureManagementSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::configure_management_settings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn configure_dns_settings(
        &self,
        req: crate::model::ConfigureDnsSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::configure_dns_settings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn configure_contact_settings(
        &self,
        req: crate::model::ConfigureContactSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::configure_contact_settings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn export_registration(
        &self,
        req: crate::model::ExportRegistrationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::export_registration(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_registration(
        &self,
        req: crate::model::DeleteRegistrationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_registration(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn retrieve_authorization_code(
        &self,
        req: crate::model::RetrieveAuthorizationCodeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AuthorizationCode>> {
        T::retrieve_authorization_code(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn reset_authorization_code(
        &self,
        req: crate::model::ResetAuthorizationCodeRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AuthorizationCode>> {
        T::reset_authorization_code(self, req, options).await
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
