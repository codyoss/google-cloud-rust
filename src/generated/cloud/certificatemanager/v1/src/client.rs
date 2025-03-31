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
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Certificate Manager API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_certificatemanager_v1::client::CertificateManager;
/// let client = CertificateManager::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// API Overview
///
/// Certificates Manager API allows customers to see and manage all their TLS
/// certificates.
///
/// Certificates Manager API service provides methods to manage certificates,
/// group them into collections, and create serving configuration that can be
/// easily applied to other Cloud resources e.g. Target Proxies.
///
/// Data Model
///
/// The Certificates Manager service exposes the following resources:
///
/// * `Certificate` that describes a single TLS certificate.
/// * `CertificateMap` that describes a collection of certificates that can be
///   attached to a target resource.
/// * `CertificateMapEntry` that describes a single configuration entry that
///   consists of a SNI and a group of certificates. It's a subresource of
///   CertificateMap.
///
/// Certificate, CertificateMap and CertificateMapEntry IDs
/// have to fully match the regexp `[a-z0-9-]{1,63}`. In other words,
///
/// - only lower case letters, digits, and hyphen are allowed
/// - length of the resource ID has to be in [1,63] range.
///
/// Provides methods to manage Cloud Certificate Manager entities.
///
/// # Configuration
///
/// To configure `CertificateManager` use the `with_*` methods in the type returned
/// by [builder()][CertificateManager::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://certificatemanager.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::certificate_manager::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::certificate_manager::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `CertificateManager` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `CertificateManager` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct CertificateManager {
    inner: Arc<dyn super::stub::dynamic::CertificateManager>,
}

impl CertificateManager {
    /// Returns a builder for [CertificateManager].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_certificatemanager_v1::client::CertificateManager;
    /// let client = CertificateManager::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::certificate_manager::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::certificate_manager::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::CertificateManager + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::CertificateManager>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::CertificateManager> {
        super::transport::CertificateManager::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::CertificateManager> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::CertificateManager::new)
    }

    /// Lists Certificates in a given project and location.
    pub fn list_certificates(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::ListCertificates {
        super::builder::certificate_manager::ListCertificates::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single Certificate.
    pub fn get_certificate(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::GetCertificate {
        super::builder::certificate_manager::GetCertificate::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new Certificate in a given project and location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_certificate(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::CreateCertificate {
        super::builder::certificate_manager::CreateCertificate::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a Certificate.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_certificate(
        &self,
        certificate: impl Into<crate::model::Certificate>,
    ) -> super::builder::certificate_manager::UpdateCertificate {
        super::builder::certificate_manager::UpdateCertificate::new(self.inner.clone())
            .set_certificate(certificate.into())
    }

    /// Deletes a single Certificate.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_certificate(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::DeleteCertificate {
        super::builder::certificate_manager::DeleteCertificate::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists CertificateMaps in a given project and location.
    pub fn list_certificate_maps(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::ListCertificateMaps {
        super::builder::certificate_manager::ListCertificateMaps::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single CertificateMap.
    pub fn get_certificate_map(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::GetCertificateMap {
        super::builder::certificate_manager::GetCertificateMap::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new CertificateMap in a given project and location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_certificate_map(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::CreateCertificateMap {
        super::builder::certificate_manager::CreateCertificateMap::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a CertificateMap.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_certificate_map(
        &self,
        certificate_map: impl Into<crate::model::CertificateMap>,
    ) -> super::builder::certificate_manager::UpdateCertificateMap {
        super::builder::certificate_manager::UpdateCertificateMap::new(self.inner.clone())
            .set_certificate_map(certificate_map.into())
    }

    /// Deletes a single CertificateMap. A Certificate Map can't be deleted
    /// if it contains Certificate Map Entries. Remove all the entries from
    /// the map before calling this method.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_certificate_map(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::DeleteCertificateMap {
        super::builder::certificate_manager::DeleteCertificateMap::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists CertificateMapEntries in a given project and location.
    pub fn list_certificate_map_entries(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::ListCertificateMapEntries {
        super::builder::certificate_manager::ListCertificateMapEntries::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single CertificateMapEntry.
    pub fn get_certificate_map_entry(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::GetCertificateMapEntry {
        super::builder::certificate_manager::GetCertificateMapEntry::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new CertificateMapEntry in a given project and location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_certificate_map_entry(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::CreateCertificateMapEntry {
        super::builder::certificate_manager::CreateCertificateMapEntry::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a CertificateMapEntry.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_certificate_map_entry(
        &self,
        certificate_map_entry: impl Into<crate::model::CertificateMapEntry>,
    ) -> super::builder::certificate_manager::UpdateCertificateMapEntry {
        super::builder::certificate_manager::UpdateCertificateMapEntry::new(self.inner.clone())
            .set_certificate_map_entry(certificate_map_entry.into())
    }

    /// Deletes a single CertificateMapEntry.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_certificate_map_entry(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::DeleteCertificateMapEntry {
        super::builder::certificate_manager::DeleteCertificateMapEntry::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists DnsAuthorizations in a given project and location.
    pub fn list_dns_authorizations(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::ListDnsAuthorizations {
        super::builder::certificate_manager::ListDnsAuthorizations::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single DnsAuthorization.
    pub fn get_dns_authorization(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::GetDnsAuthorization {
        super::builder::certificate_manager::GetDnsAuthorization::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new DnsAuthorization in a given project and location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_dns_authorization(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::CreateDnsAuthorization {
        super::builder::certificate_manager::CreateDnsAuthorization::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a DnsAuthorization.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_dns_authorization(
        &self,
        dns_authorization: impl Into<crate::model::DnsAuthorization>,
    ) -> super::builder::certificate_manager::UpdateDnsAuthorization {
        super::builder::certificate_manager::UpdateDnsAuthorization::new(self.inner.clone())
            .set_dns_authorization(dns_authorization.into())
    }

    /// Deletes a single DnsAuthorization.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_dns_authorization(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::DeleteDnsAuthorization {
        super::builder::certificate_manager::DeleteDnsAuthorization::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists CertificateIssuanceConfigs in a given project and location.
    pub fn list_certificate_issuance_configs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::ListCertificateIssuanceConfigs {
        super::builder::certificate_manager::ListCertificateIssuanceConfigs::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single CertificateIssuanceConfig.
    pub fn get_certificate_issuance_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::GetCertificateIssuanceConfig {
        super::builder::certificate_manager::GetCertificateIssuanceConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new CertificateIssuanceConfig in a given project and location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_certificate_issuance_config(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::CreateCertificateIssuanceConfig {
        super::builder::certificate_manager::CreateCertificateIssuanceConfig::new(
            self.inner.clone(),
        )
        .set_parent(parent.into())
    }

    /// Deletes a single CertificateIssuanceConfig.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_certificate_issuance_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::DeleteCertificateIssuanceConfig {
        super::builder::certificate_manager::DeleteCertificateIssuanceConfig::new(
            self.inner.clone(),
        )
        .set_name(name.into())
    }

    /// Lists TrustConfigs in a given project and location.
    pub fn list_trust_configs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::ListTrustConfigs {
        super::builder::certificate_manager::ListTrustConfigs::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single TrustConfig.
    pub fn get_trust_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::GetTrustConfig {
        super::builder::certificate_manager::GetTrustConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new TrustConfig in a given project and location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_trust_config(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::CreateTrustConfig {
        super::builder::certificate_manager::CreateTrustConfig::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a TrustConfig.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_trust_config(
        &self,
        trust_config: impl Into<crate::model::TrustConfig>,
    ) -> super::builder::certificate_manager::UpdateTrustConfig {
        super::builder::certificate_manager::UpdateTrustConfig::new(self.inner.clone())
            .set_trust_config(trust_config.into())
    }

    /// Deletes a single TrustConfig.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_trust_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::DeleteTrustConfig {
        super::builder::certificate_manager::DeleteTrustConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::ListLocations {
        super::builder::certificate_manager::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::GetLocation {
        super::builder::certificate_manager::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::ListOperations {
        super::builder::certificate_manager::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::GetOperation {
        super::builder::certificate_manager::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::DeleteOperation {
        super::builder::certificate_manager::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_manager::CancelOperation {
        super::builder::certificate_manager::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
