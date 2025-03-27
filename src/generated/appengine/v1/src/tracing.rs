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

/// Implements a [Applications](super::stubs::Applications) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Applications<T>
where
    T: super::stubs::Applications + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Applications<T>
where
    T: super::stubs::Applications + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::Applications for Applications<T>
where
    T: super::stubs::Applications + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn get_application(
        &self,
        req: crate::model::GetApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Application> {
        self.inner.get_application(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_application(
        &self,
        req: crate::model::CreateApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_application(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_application(
        &self,
        req: crate::model::UpdateApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_application(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn repair_application(
        &self,
        req: crate::model::RepairApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.repair_application(req, options).await
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

/// Implements a [Services](super::stubs::Services) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Services<T>
where
    T: super::stubs::Services + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Services<T>
where
    T: super::stubs::Services + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::Services for Services<T>
where
    T: super::stubs::Services + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_services(
        &self,
        req: crate::model::ListServicesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListServicesResponse> {
        self.inner.list_services(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_service(
        &self,
        req: crate::model::GetServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Service> {
        self.inner.get_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_service(
        &self,
        req: crate::model::UpdateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_service(
        &self,
        req: crate::model::DeleteServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_service(req, options).await
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

/// Implements a [Versions](super::stubs::Versions) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Versions<T>
where
    T: super::stubs::Versions + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Versions<T>
where
    T: super::stubs::Versions + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::Versions for Versions<T>
where
    T: super::stubs::Versions + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_versions(
        &self,
        req: crate::model::ListVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListVersionsResponse> {
        self.inner.list_versions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_version(
        &self,
        req: crate::model::GetVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Version> {
        self.inner.get_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_version(
        &self,
        req: crate::model::CreateVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_version(
        &self,
        req: crate::model::UpdateVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_version(
        &self,
        req: crate::model::DeleteVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_version(req, options).await
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

/// Implements a [Instances](super::stubs::Instances) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Instances<T>
where
    T: super::stubs::Instances + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Instances<T>
where
    T: super::stubs::Instances + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::Instances for Instances<T>
where
    T: super::stubs::Instances + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_instances(
        &self,
        req: crate::model::ListInstancesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListInstancesResponse> {
        self.inner.list_instances(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_instance(
        &self,
        req: crate::model::GetInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Instance> {
        self.inner.get_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_instance(
        &self,
        req: crate::model::DeleteInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn debug_instance(
        &self,
        req: crate::model::DebugInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.debug_instance(req, options).await
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

/// Implements a [Firewall](super::stubs::Firewall) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Firewall<T>
where
    T: super::stubs::Firewall + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Firewall<T>
where
    T: super::stubs::Firewall + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::Firewall for Firewall<T>
where
    T: super::stubs::Firewall + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_ingress_rules(
        &self,
        req: crate::model::ListIngressRulesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListIngressRulesResponse> {
        self.inner.list_ingress_rules(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn batch_update_ingress_rules(
        &self,
        req: crate::model::BatchUpdateIngressRulesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::BatchUpdateIngressRulesResponse> {
        self.inner.batch_update_ingress_rules(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_ingress_rule(
        &self,
        req: crate::model::CreateIngressRuleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::FirewallRule> {
        self.inner.create_ingress_rule(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_ingress_rule(
        &self,
        req: crate::model::GetIngressRuleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::FirewallRule> {
        self.inner.get_ingress_rule(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_ingress_rule(
        &self,
        req: crate::model::UpdateIngressRuleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::FirewallRule> {
        self.inner.update_ingress_rule(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_ingress_rule(
        &self,
        req: crate::model::DeleteIngressRuleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_ingress_rule(req, options).await
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
}

/// Implements a [AuthorizedDomains](super::stubs::AuthorizedDomains) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct AuthorizedDomains<T>
where
    T: super::stubs::AuthorizedDomains + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> AuthorizedDomains<T>
where
    T: super::stubs::AuthorizedDomains + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::AuthorizedDomains for AuthorizedDomains<T>
where
    T: super::stubs::AuthorizedDomains + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_authorized_domains(
        &self,
        req: crate::model::ListAuthorizedDomainsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAuthorizedDomainsResponse> {
        self.inner.list_authorized_domains(req, options).await
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
}

/// Implements a [AuthorizedCertificates](super::stubs::AuthorizedCertificates) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct AuthorizedCertificates<T>
where
    T: super::stubs::AuthorizedCertificates + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> AuthorizedCertificates<T>
where
    T: super::stubs::AuthorizedCertificates + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::AuthorizedCertificates for AuthorizedCertificates<T>
where
    T: super::stubs::AuthorizedCertificates + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_authorized_certificates(
        &self,
        req: crate::model::ListAuthorizedCertificatesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAuthorizedCertificatesResponse> {
        self.inner.list_authorized_certificates(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_authorized_certificate(
        &self,
        req: crate::model::GetAuthorizedCertificateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AuthorizedCertificate> {
        self.inner.get_authorized_certificate(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_authorized_certificate(
        &self,
        req: crate::model::CreateAuthorizedCertificateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AuthorizedCertificate> {
        self.inner.create_authorized_certificate(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_authorized_certificate(
        &self,
        req: crate::model::UpdateAuthorizedCertificateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AuthorizedCertificate> {
        self.inner.update_authorized_certificate(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_authorized_certificate(
        &self,
        req: crate::model::DeleteAuthorizedCertificateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_authorized_certificate(req, options).await
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
}

/// Implements a [DomainMappings](super::stubs::DomainMappings) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct DomainMappings<T>
where
    T: super::stubs::DomainMappings + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> DomainMappings<T>
where
    T: super::stubs::DomainMappings + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::DomainMappings for DomainMappings<T>
where
    T: super::stubs::DomainMappings + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_domain_mappings(
        &self,
        req: crate::model::ListDomainMappingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDomainMappingsResponse> {
        self.inner.list_domain_mappings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_domain_mapping(
        &self,
        req: crate::model::GetDomainMappingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DomainMapping> {
        self.inner.get_domain_mapping(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_domain_mapping(
        &self,
        req: crate::model::CreateDomainMappingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_domain_mapping(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_domain_mapping(
        &self,
        req: crate::model::UpdateDomainMappingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_domain_mapping(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_domain_mapping(
        &self,
        req: crate::model::DeleteDomainMappingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_domain_mapping(req, options).await
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
