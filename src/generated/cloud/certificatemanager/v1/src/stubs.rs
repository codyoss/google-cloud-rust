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

//! Traits to mock the clients in this library.
//!
//! Application developers may need to mock the clients in this library to test
//! how their application works with different (and sometimes hard to trigger)
//! client and service behavior. Such test can define mocks implementing the
//! trait(s) defined in this module, initialize the client with an instance of
//! this mock in their tests, and verify their application responds as expected.

#![allow(rustdoc::broken_intra_doc_links)]

use gax::error::Error;
use std::sync::Arc;

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::CertificateManager].
///
/// Application developers may need to implement this trait to mock
/// `client::CertificateManager`.  In other use-cases, application developers only
/// use `client::CertificateManager` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait CertificateManager: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::CertificateManager::list_certificates].
    fn list_certificates(
        &self,
        _req: crate::model::ListCertificatesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListCertificatesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListCertificatesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::CertificateManager::get_certificate].
    fn get_certificate(
        &self,
        _req: crate::model::GetCertificateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Certificate>> + Send {
        std::future::ready::<crate::Result<crate::model::Certificate>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::create_certificate].
    fn create_certificate(
        &self,
        _req: crate::model::CreateCertificateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::update_certificate].
    fn update_certificate(
        &self,
        _req: crate::model::UpdateCertificateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::delete_certificate].
    fn delete_certificate(
        &self,
        _req: crate::model::DeleteCertificateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::list_certificate_maps].
    fn list_certificate_maps(
        &self,
        _req: crate::model::ListCertificateMapsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListCertificateMapsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListCertificateMapsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::CertificateManager::get_certificate_map].
    fn get_certificate_map(
        &self,
        _req: crate::model::GetCertificateMapRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::CertificateMap>> + Send {
        std::future::ready::<crate::Result<crate::model::CertificateMap>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::create_certificate_map].
    fn create_certificate_map(
        &self,
        _req: crate::model::CreateCertificateMapRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::update_certificate_map].
    fn update_certificate_map(
        &self,
        _req: crate::model::UpdateCertificateMapRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::delete_certificate_map].
    fn delete_certificate_map(
        &self,
        _req: crate::model::DeleteCertificateMapRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::list_certificate_map_entries].
    fn list_certificate_map_entries(
        &self,
        _req: crate::model::ListCertificateMapEntriesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListCertificateMapEntriesResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListCertificateMapEntriesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::CertificateManager::get_certificate_map_entry].
    fn get_certificate_map_entry(
        &self,
        _req: crate::model::GetCertificateMapEntryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::CertificateMapEntry>> + Send
    {
        std::future::ready::<crate::Result<crate::model::CertificateMapEntry>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::create_certificate_map_entry].
    fn create_certificate_map_entry(
        &self,
        _req: crate::model::CreateCertificateMapEntryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::update_certificate_map_entry].
    fn update_certificate_map_entry(
        &self,
        _req: crate::model::UpdateCertificateMapEntryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::delete_certificate_map_entry].
    fn delete_certificate_map_entry(
        &self,
        _req: crate::model::DeleteCertificateMapEntryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::list_dns_authorizations].
    fn list_dns_authorizations(
        &self,
        _req: crate::model::ListDnsAuthorizationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListDnsAuthorizationsResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::ListDnsAuthorizationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::CertificateManager::get_dns_authorization].
    fn get_dns_authorization(
        &self,
        _req: crate::model::GetDnsAuthorizationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::DnsAuthorization>> + Send
    {
        std::future::ready::<crate::Result<crate::model::DnsAuthorization>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::create_dns_authorization].
    fn create_dns_authorization(
        &self,
        _req: crate::model::CreateDnsAuthorizationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::update_dns_authorization].
    fn update_dns_authorization(
        &self,
        _req: crate::model::UpdateDnsAuthorizationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::delete_dns_authorization].
    fn delete_dns_authorization(
        &self,
        _req: crate::model::DeleteDnsAuthorizationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::list_certificate_issuance_configs].
    fn list_certificate_issuance_configs(
        &self,
        _req: crate::model::ListCertificateIssuanceConfigsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListCertificateIssuanceConfigsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListCertificateIssuanceConfigsResponse>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::CertificateManager::get_certificate_issuance_config].
    fn get_certificate_issuance_config(
        &self,
        _req: crate::model::GetCertificateIssuanceConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::CertificateIssuanceConfig>> + Send
    {
        std::future::ready::<crate::Result<crate::model::CertificateIssuanceConfig>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::CertificateManager::create_certificate_issuance_config].
    fn create_certificate_issuance_config(
        &self,
        _req: crate::model::CreateCertificateIssuanceConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::delete_certificate_issuance_config].
    fn delete_certificate_issuance_config(
        &self,
        _req: crate::model::DeleteCertificateIssuanceConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::list_trust_configs].
    fn list_trust_configs(
        &self,
        _req: crate::model::ListTrustConfigsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListTrustConfigsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListTrustConfigsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::CertificateManager::get_trust_config].
    fn get_trust_config(
        &self,
        _req: crate::model::GetTrustConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TrustConfig>> + Send {
        std::future::ready::<crate::Result<crate::model::TrustConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::create_trust_config].
    fn create_trust_config(
        &self,
        _req: crate::model::CreateTrustConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::update_trust_config].
    fn update_trust_config(
        &self,
        _req: crate::model::UpdateTrustConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::delete_trust_config].
    fn delete_trust_config(
        &self,
        _req: crate::model::DeleteTrustConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::ListLocationsResponse>> + Send
    {
        std::future::ready::<crate::Result<location::model::ListLocationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::CertificateManager::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::CertificateManager::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::CertificateManager::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::CertificateManager::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Returns the polling policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        Arc::new(gax::polling_policy::Aip194Strict)
    }

    /// Returns the polling backoff policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_backoff_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        Arc::new(gax::exponential_backoff::ExponentialBackoff::default())
    }
}
