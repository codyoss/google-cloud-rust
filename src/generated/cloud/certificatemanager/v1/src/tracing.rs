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

/// Implements a [CertificateManager](super::stubs::CertificateManager) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct CertificateManager<T>
where
    T: super::stubs::CertificateManager + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> CertificateManager<T>
where
    T: super::stubs::CertificateManager + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::CertificateManager for CertificateManager<T>
where
    T: super::stubs::CertificateManager + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_certificates(
        &self,
        req: crate::model::ListCertificatesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListCertificatesResponse> {
        self.inner.list_certificates(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_certificate(
        &self,
        req: crate::model::GetCertificateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Certificate> {
        self.inner.get_certificate(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_certificate(
        &self,
        req: crate::model::CreateCertificateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_certificate(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_certificate(
        &self,
        req: crate::model::UpdateCertificateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_certificate(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_certificate(
        &self,
        req: crate::model::DeleteCertificateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_certificate(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_certificate_maps(
        &self,
        req: crate::model::ListCertificateMapsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListCertificateMapsResponse> {
        self.inner.list_certificate_maps(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_certificate_map(
        &self,
        req: crate::model::GetCertificateMapRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::CertificateMap> {
        self.inner.get_certificate_map(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_certificate_map(
        &self,
        req: crate::model::CreateCertificateMapRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_certificate_map(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_certificate_map(
        &self,
        req: crate::model::UpdateCertificateMapRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_certificate_map(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_certificate_map(
        &self,
        req: crate::model::DeleteCertificateMapRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_certificate_map(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_certificate_map_entries(
        &self,
        req: crate::model::ListCertificateMapEntriesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListCertificateMapEntriesResponse> {
        self.inner.list_certificate_map_entries(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_certificate_map_entry(
        &self,
        req: crate::model::GetCertificateMapEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::CertificateMapEntry> {
        self.inner.get_certificate_map_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_certificate_map_entry(
        &self,
        req: crate::model::CreateCertificateMapEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_certificate_map_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_certificate_map_entry(
        &self,
        req: crate::model::UpdateCertificateMapEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_certificate_map_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_certificate_map_entry(
        &self,
        req: crate::model::DeleteCertificateMapEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_certificate_map_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_dns_authorizations(
        &self,
        req: crate::model::ListDnsAuthorizationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDnsAuthorizationsResponse> {
        self.inner.list_dns_authorizations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_dns_authorization(
        &self,
        req: crate::model::GetDnsAuthorizationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DnsAuthorization> {
        self.inner.get_dns_authorization(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_dns_authorization(
        &self,
        req: crate::model::CreateDnsAuthorizationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_dns_authorization(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_dns_authorization(
        &self,
        req: crate::model::UpdateDnsAuthorizationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_dns_authorization(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_dns_authorization(
        &self,
        req: crate::model::DeleteDnsAuthorizationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_dns_authorization(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_certificate_issuance_configs(
        &self,
        req: crate::model::ListCertificateIssuanceConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListCertificateIssuanceConfigsResponse> {
        self.inner
            .list_certificate_issuance_configs(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn get_certificate_issuance_config(
        &self,
        req: crate::model::GetCertificateIssuanceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::CertificateIssuanceConfig> {
        self.inner
            .get_certificate_issuance_config(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn create_certificate_issuance_config(
        &self,
        req: crate::model::CreateCertificateIssuanceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner
            .create_certificate_issuance_config(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn delete_certificate_issuance_config(
        &self,
        req: crate::model::DeleteCertificateIssuanceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner
            .delete_certificate_issuance_config(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn list_trust_configs(
        &self,
        req: crate::model::ListTrustConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTrustConfigsResponse> {
        self.inner.list_trust_configs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_trust_config(
        &self,
        req: crate::model::GetTrustConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TrustConfig> {
        self.inner.get_trust_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_trust_config(
        &self,
        req: crate::model::CreateTrustConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_trust_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_trust_config(
        &self,
        req: crate::model::UpdateTrustConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_trust_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_trust_config(
        &self,
        req: crate::model::DeleteTrustConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_trust_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
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
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
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
