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

/// Implements a [Domains](crate::stubs::Domains) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Domains<T>
where
    T: crate::stubs::Domains + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Domains<T>
where
    T: crate::stubs::Domains + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::Domains for Domains<T>
where
    T: crate::stubs::Domains + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn search_domains(
        &self,
        req: crate::model::SearchDomainsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SearchDomainsResponse> {
        self.inner.search_domains(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn retrieve_register_parameters(
        &self,
        req: crate::model::RetrieveRegisterParametersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::RetrieveRegisterParametersResponse> {
        self.inner.retrieve_register_parameters(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn register_domain(
        &self,
        req: crate::model::RegisterDomainRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.register_domain(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn retrieve_transfer_parameters(
        &self,
        req: crate::model::RetrieveTransferParametersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::RetrieveTransferParametersResponse> {
        self.inner.retrieve_transfer_parameters(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn transfer_domain(
        &self,
        req: crate::model::TransferDomainRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.transfer_domain(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_registrations(
        &self,
        req: crate::model::ListRegistrationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListRegistrationsResponse> {
        self.inner.list_registrations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_registration(
        &self,
        req: crate::model::GetRegistrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Registration> {
        self.inner.get_registration(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_registration(
        &self,
        req: crate::model::UpdateRegistrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_registration(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn configure_management_settings(
        &self,
        req: crate::model::ConfigureManagementSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.configure_management_settings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn configure_dns_settings(
        &self,
        req: crate::model::ConfigureDnsSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.configure_dns_settings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn configure_contact_settings(
        &self,
        req: crate::model::ConfigureContactSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.configure_contact_settings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn export_registration(
        &self,
        req: crate::model::ExportRegistrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.export_registration(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_registration(
        &self,
        req: crate::model::DeleteRegistrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_registration(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn retrieve_authorization_code(
        &self,
        req: crate::model::RetrieveAuthorizationCodeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AuthorizationCode> {
        self.inner.retrieve_authorization_code(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn reset_authorization_code(
        &self,
        req: crate::model::ResetAuthorizationCodeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AuthorizationCode> {
        self.inner.reset_authorization_code(req, options).await
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
