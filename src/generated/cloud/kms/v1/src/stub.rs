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

/// Defines the trait used to implement [super::client::Autokey].
///
/// Application developers may need to implement this trait to mock
/// `client::Autokey`.  In other use-cases, application developers only
/// use `client::Autokey` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait Autokey: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::Autokey::create_key_handle].
    fn create_key_handle(
        &self,
        _req: crate::model::CreateKeyHandleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Autokey::get_key_handle].
    fn get_key_handle(
        &self,
        _req: crate::model::GetKeyHandleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::KeyHandle>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::KeyHandle>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Autokey::list_key_handles].
    fn list_key_handles(
        &self,
        _req: crate::model::ListKeyHandlesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListKeyHandlesResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListKeyHandlesResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Autokey::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<location::model::ListLocationsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<location::model::ListLocationsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Autokey::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<location::model::Location>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<location::model::Location>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Autokey::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<iam_v1::model::Policy>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Autokey::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<iam_v1::model::Policy>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Autokey::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Autokey::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Returns the polling error policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_error_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        Arc::new(gax::polling_error_policy::Aip194Strict)
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

/// Defines the trait used to implement [super::client::AutokeyAdmin].
///
/// Application developers may need to implement this trait to mock
/// `client::AutokeyAdmin`.  In other use-cases, application developers only
/// use `client::AutokeyAdmin` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait AutokeyAdmin: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::AutokeyAdmin::update_autokey_config].
    fn update_autokey_config(
        &self,
        _req: crate::model::UpdateAutokeyConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::AutokeyConfig>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::AutokeyConfig>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::AutokeyAdmin::get_autokey_config].
    fn get_autokey_config(
        &self,
        _req: crate::model::GetAutokeyConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::AutokeyConfig>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::AutokeyConfig>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::AutokeyAdmin::show_effective_autokey_config].
    fn show_effective_autokey_config(
        &self,
        _req: crate::model::ShowEffectiveAutokeyConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<
            gax::response::Response<crate::model::ShowEffectiveAutokeyConfigResponse>,
        >,
    > + Send {
        std::future::ready::<
            crate::Result<
                gax::response::Response<crate::model::ShowEffectiveAutokeyConfigResponse>,
            >,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::AutokeyAdmin::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<location::model::ListLocationsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<location::model::ListLocationsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::AutokeyAdmin::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<location::model::Location>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<location::model::Location>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::AutokeyAdmin::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<iam_v1::model::Policy>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::AutokeyAdmin::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<iam_v1::model::Policy>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::AutokeyAdmin::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::AutokeyAdmin::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }
}

/// Defines the trait used to implement [super::client::EkmService].
///
/// Application developers may need to implement this trait to mock
/// `client::EkmService`.  In other use-cases, application developers only
/// use `client::EkmService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait EkmService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::EkmService::list_ekm_connections].
    fn list_ekm_connections(
        &self,
        _req: crate::model::ListEkmConnectionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListEkmConnectionsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListEkmConnectionsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::EkmService::get_ekm_connection].
    fn get_ekm_connection(
        &self,
        _req: crate::model::GetEkmConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::EkmConnection>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::EkmConnection>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::EkmService::create_ekm_connection].
    fn create_ekm_connection(
        &self,
        _req: crate::model::CreateEkmConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::EkmConnection>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::EkmConnection>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::EkmService::update_ekm_connection].
    fn update_ekm_connection(
        &self,
        _req: crate::model::UpdateEkmConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::EkmConnection>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::EkmConnection>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::EkmService::get_ekm_config].
    fn get_ekm_config(
        &self,
        _req: crate::model::GetEkmConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::EkmConfig>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::EkmConfig>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::EkmService::update_ekm_config].
    fn update_ekm_config(
        &self,
        _req: crate::model::UpdateEkmConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::EkmConfig>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::EkmConfig>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::EkmService::verify_connectivity].
    fn verify_connectivity(
        &self,
        _req: crate::model::VerifyConnectivityRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::VerifyConnectivityResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::VerifyConnectivityResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::EkmService::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<location::model::ListLocationsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<location::model::ListLocationsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::EkmService::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<location::model::Location>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<location::model::Location>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::EkmService::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<iam_v1::model::Policy>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::EkmService::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<iam_v1::model::Policy>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::EkmService::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::EkmService::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }
}

/// Defines the trait used to implement [super::client::KeyManagementService].
///
/// Application developers may need to implement this trait to mock
/// `client::KeyManagementService`.  In other use-cases, application developers only
/// use `client::KeyManagementService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait KeyManagementService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::KeyManagementService::list_key_rings].
    fn list_key_rings(
        &self,
        _req: crate::model::ListKeyRingsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListKeyRingsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListKeyRingsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::KeyManagementService::list_crypto_keys].
    fn list_crypto_keys(
        &self,
        _req: crate::model::ListCryptoKeysRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListCryptoKeysResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListCryptoKeysResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::KeyManagementService::list_crypto_key_versions].
    fn list_crypto_key_versions(
        &self,
        _req: crate::model::ListCryptoKeyVersionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<
            gax::response::Response<crate::model::ListCryptoKeyVersionsResponse>,
        >,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListCryptoKeyVersionsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::KeyManagementService::list_import_jobs].
    fn list_import_jobs(
        &self,
        _req: crate::model::ListImportJobsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListImportJobsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListImportJobsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::KeyManagementService::get_key_ring].
    fn get_key_ring(
        &self,
        _req: crate::model::GetKeyRingRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::KeyRing>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::KeyRing>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::KeyManagementService::get_crypto_key].
    fn get_crypto_key(
        &self,
        _req: crate::model::GetCryptoKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::CryptoKey>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::CryptoKey>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::KeyManagementService::get_crypto_key_version].
    fn get_crypto_key_version(
        &self,
        _req: crate::model::GetCryptoKeyVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::CryptoKeyVersion>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::CryptoKeyVersion>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::KeyManagementService::get_public_key].
    fn get_public_key(
        &self,
        _req: crate::model::GetPublicKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::PublicKey>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::PublicKey>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::KeyManagementService::get_import_job].
    fn get_import_job(
        &self,
        _req: crate::model::GetImportJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ImportJob>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::ImportJob>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::KeyManagementService::create_key_ring].
    fn create_key_ring(
        &self,
        _req: crate::model::CreateKeyRingRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::KeyRing>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::KeyRing>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::KeyManagementService::create_crypto_key].
    fn create_crypto_key(
        &self,
        _req: crate::model::CreateCryptoKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::CryptoKey>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::CryptoKey>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::KeyManagementService::create_crypto_key_version].
    fn create_crypto_key_version(
        &self,
        _req: crate::model::CreateCryptoKeyVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::CryptoKeyVersion>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::CryptoKeyVersion>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::KeyManagementService::import_crypto_key_version].
    fn import_crypto_key_version(
        &self,
        _req: crate::model::ImportCryptoKeyVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::CryptoKeyVersion>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::CryptoKeyVersion>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::KeyManagementService::create_import_job].
    fn create_import_job(
        &self,
        _req: crate::model::CreateImportJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ImportJob>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::ImportJob>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::KeyManagementService::update_crypto_key].
    fn update_crypto_key(
        &self,
        _req: crate::model::UpdateCryptoKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::CryptoKey>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::CryptoKey>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::KeyManagementService::update_crypto_key_version].
    fn update_crypto_key_version(
        &self,
        _req: crate::model::UpdateCryptoKeyVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::CryptoKeyVersion>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::CryptoKeyVersion>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::KeyManagementService::update_crypto_key_primary_version].
    fn update_crypto_key_primary_version(
        &self,
        _req: crate::model::UpdateCryptoKeyPrimaryVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::CryptoKey>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::CryptoKey>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::KeyManagementService::destroy_crypto_key_version].
    fn destroy_crypto_key_version(
        &self,
        _req: crate::model::DestroyCryptoKeyVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::CryptoKeyVersion>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::CryptoKeyVersion>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::KeyManagementService::restore_crypto_key_version].
    fn restore_crypto_key_version(
        &self,
        _req: crate::model::RestoreCryptoKeyVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::CryptoKeyVersion>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::CryptoKeyVersion>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::KeyManagementService::encrypt].
    fn encrypt(
        &self,
        _req: crate::model::EncryptRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::EncryptResponse>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::EncryptResponse>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::KeyManagementService::decrypt].
    fn decrypt(
        &self,
        _req: crate::model::DecryptRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::DecryptResponse>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::DecryptResponse>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::KeyManagementService::raw_encrypt].
    fn raw_encrypt(
        &self,
        _req: crate::model::RawEncryptRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::RawEncryptResponse>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::RawEncryptResponse>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::KeyManagementService::raw_decrypt].
    fn raw_decrypt(
        &self,
        _req: crate::model::RawDecryptRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::RawDecryptResponse>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::RawDecryptResponse>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::KeyManagementService::asymmetric_sign].
    fn asymmetric_sign(
        &self,
        _req: crate::model::AsymmetricSignRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::AsymmetricSignResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::AsymmetricSignResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::KeyManagementService::asymmetric_decrypt].
    fn asymmetric_decrypt(
        &self,
        _req: crate::model::AsymmetricDecryptRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::AsymmetricDecryptResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::AsymmetricDecryptResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::KeyManagementService::mac_sign].
    fn mac_sign(
        &self,
        _req: crate::model::MacSignRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::MacSignResponse>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::MacSignResponse>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::KeyManagementService::mac_verify].
    fn mac_verify(
        &self,
        _req: crate::model::MacVerifyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::MacVerifyResponse>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::MacVerifyResponse>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::KeyManagementService::generate_random_bytes].
    fn generate_random_bytes(
        &self,
        _req: crate::model::GenerateRandomBytesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::GenerateRandomBytesResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::GenerateRandomBytesResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::KeyManagementService::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<location::model::ListLocationsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<location::model::ListLocationsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::KeyManagementService::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<location::model::Location>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<location::model::Location>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::KeyManagementService::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<iam_v1::model::Policy>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::KeyManagementService::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<iam_v1::model::Policy>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::KeyManagementService::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::KeyManagementService::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }
}
