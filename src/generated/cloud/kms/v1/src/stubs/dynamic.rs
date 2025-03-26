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

/// A dyn-compatible, crate-private version of [super::Autokey].
#[async_trait::async_trait]
pub trait Autokey: std::fmt::Debug + Send + Sync {
    async fn create_key_handle(
        &self,
        req: crate::model::CreateKeyHandleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_key_handle(
        &self,
        req: crate::model::GetKeyHandleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::KeyHandle>;

    async fn list_key_handles(
        &self,
        req: crate::model::ListKeyHandlesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListKeyHandlesResponse>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location>;

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::Autokey] also implement [Autokey].
#[async_trait::async_trait]
impl<T: super::Autokey> Autokey for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_key_handle(
        &self,
        req: crate::model::CreateKeyHandleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_key_handle(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_key_handle(
        &self,
        req: crate::model::GetKeyHandleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::KeyHandle> {
        T::get_key_handle(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_key_handles(
        &self,
        req: crate::model::ListKeyHandlesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListKeyHandlesResponse> {
        T::list_key_handles(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location> {
        T::get_location(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse> {
        T::test_iam_permissions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
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

/// A dyn-compatible, crate-private version of [super::AutokeyAdmin].
#[async_trait::async_trait]
pub trait AutokeyAdmin: std::fmt::Debug + Send + Sync {
    async fn update_autokey_config(
        &self,
        req: crate::model::UpdateAutokeyConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AutokeyConfig>;

    async fn get_autokey_config(
        &self,
        req: crate::model::GetAutokeyConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AutokeyConfig>;

    async fn show_effective_autokey_config(
        &self,
        req: crate::model::ShowEffectiveAutokeyConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ShowEffectiveAutokeyConfigResponse>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location>;

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;
}

/// All implementations of [super::AutokeyAdmin] also implement [AutokeyAdmin].
#[async_trait::async_trait]
impl<T: super::AutokeyAdmin> AutokeyAdmin for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn update_autokey_config(
        &self,
        req: crate::model::UpdateAutokeyConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AutokeyConfig> {
        T::update_autokey_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_autokey_config(
        &self,
        req: crate::model::GetAutokeyConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AutokeyConfig> {
        T::get_autokey_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn show_effective_autokey_config(
        &self,
        req: crate::model::ShowEffectiveAutokeyConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ShowEffectiveAutokeyConfigResponse> {
        T::show_effective_autokey_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location> {
        T::get_location(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse> {
        T::test_iam_permissions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
    }
}

/// A dyn-compatible, crate-private version of [super::EkmService].
#[async_trait::async_trait]
pub trait EkmService: std::fmt::Debug + Send + Sync {
    async fn list_ekm_connections(
        &self,
        req: crate::model::ListEkmConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListEkmConnectionsResponse>;

    async fn get_ekm_connection(
        &self,
        req: crate::model::GetEkmConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EkmConnection>;

    async fn create_ekm_connection(
        &self,
        req: crate::model::CreateEkmConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EkmConnection>;

    async fn update_ekm_connection(
        &self,
        req: crate::model::UpdateEkmConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EkmConnection>;

    async fn get_ekm_config(
        &self,
        req: crate::model::GetEkmConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EkmConfig>;

    async fn update_ekm_config(
        &self,
        req: crate::model::UpdateEkmConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EkmConfig>;

    async fn verify_connectivity(
        &self,
        req: crate::model::VerifyConnectivityRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VerifyConnectivityResponse>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location>;

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;
}

/// All implementations of [super::EkmService] also implement [EkmService].
#[async_trait::async_trait]
impl<T: super::EkmService> EkmService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_ekm_connections(
        &self,
        req: crate::model::ListEkmConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListEkmConnectionsResponse> {
        T::list_ekm_connections(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_ekm_connection(
        &self,
        req: crate::model::GetEkmConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EkmConnection> {
        T::get_ekm_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_ekm_connection(
        &self,
        req: crate::model::CreateEkmConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EkmConnection> {
        T::create_ekm_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_ekm_connection(
        &self,
        req: crate::model::UpdateEkmConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EkmConnection> {
        T::update_ekm_connection(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_ekm_config(
        &self,
        req: crate::model::GetEkmConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EkmConfig> {
        T::get_ekm_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_ekm_config(
        &self,
        req: crate::model::UpdateEkmConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EkmConfig> {
        T::update_ekm_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn verify_connectivity(
        &self,
        req: crate::model::VerifyConnectivityRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VerifyConnectivityResponse> {
        T::verify_connectivity(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location> {
        T::get_location(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse> {
        T::test_iam_permissions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
    }
}

/// A dyn-compatible, crate-private version of [super::KeyManagementService].
#[async_trait::async_trait]
pub trait KeyManagementService: std::fmt::Debug + Send + Sync {
    async fn list_key_rings(
        &self,
        req: crate::model::ListKeyRingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListKeyRingsResponse>;

    async fn list_crypto_keys(
        &self,
        req: crate::model::ListCryptoKeysRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListCryptoKeysResponse>;

    async fn list_crypto_key_versions(
        &self,
        req: crate::model::ListCryptoKeyVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListCryptoKeyVersionsResponse>;

    async fn list_import_jobs(
        &self,
        req: crate::model::ListImportJobsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListImportJobsResponse>;

    async fn get_key_ring(
        &self,
        req: crate::model::GetKeyRingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::KeyRing>;

    async fn get_crypto_key(
        &self,
        req: crate::model::GetCryptoKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKey>;

    async fn get_crypto_key_version(
        &self,
        req: crate::model::GetCryptoKeyVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKeyVersion>;

    async fn get_public_key(
        &self,
        req: crate::model::GetPublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PublicKey>;

    async fn get_import_job(
        &self,
        req: crate::model::GetImportJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ImportJob>;

    async fn create_key_ring(
        &self,
        req: crate::model::CreateKeyRingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::KeyRing>;

    async fn create_crypto_key(
        &self,
        req: crate::model::CreateCryptoKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKey>;

    async fn create_crypto_key_version(
        &self,
        req: crate::model::CreateCryptoKeyVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKeyVersion>;

    async fn import_crypto_key_version(
        &self,
        req: crate::model::ImportCryptoKeyVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKeyVersion>;

    async fn create_import_job(
        &self,
        req: crate::model::CreateImportJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ImportJob>;

    async fn update_crypto_key(
        &self,
        req: crate::model::UpdateCryptoKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKey>;

    async fn update_crypto_key_version(
        &self,
        req: crate::model::UpdateCryptoKeyVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKeyVersion>;

    async fn update_crypto_key_primary_version(
        &self,
        req: crate::model::UpdateCryptoKeyPrimaryVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKey>;

    async fn destroy_crypto_key_version(
        &self,
        req: crate::model::DestroyCryptoKeyVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKeyVersion>;

    async fn restore_crypto_key_version(
        &self,
        req: crate::model::RestoreCryptoKeyVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKeyVersion>;

    async fn encrypt(
        &self,
        req: crate::model::EncryptRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EncryptResponse>;

    async fn decrypt(
        &self,
        req: crate::model::DecryptRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DecryptResponse>;

    async fn raw_encrypt(
        &self,
        req: crate::model::RawEncryptRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::RawEncryptResponse>;

    async fn raw_decrypt(
        &self,
        req: crate::model::RawDecryptRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::RawDecryptResponse>;

    async fn asymmetric_sign(
        &self,
        req: crate::model::AsymmetricSignRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AsymmetricSignResponse>;

    async fn asymmetric_decrypt(
        &self,
        req: crate::model::AsymmetricDecryptRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AsymmetricDecryptResponse>;

    async fn mac_sign(
        &self,
        req: crate::model::MacSignRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MacSignResponse>;

    async fn mac_verify(
        &self,
        req: crate::model::MacVerifyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MacVerifyResponse>;

    async fn generate_random_bytes(
        &self,
        req: crate::model::GenerateRandomBytesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GenerateRandomBytesResponse>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location>;

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;
}

/// All implementations of [super::KeyManagementService] also implement [KeyManagementService].
#[async_trait::async_trait]
impl<T: super::KeyManagementService> KeyManagementService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_key_rings(
        &self,
        req: crate::model::ListKeyRingsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListKeyRingsResponse> {
        T::list_key_rings(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_crypto_keys(
        &self,
        req: crate::model::ListCryptoKeysRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListCryptoKeysResponse> {
        T::list_crypto_keys(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_crypto_key_versions(
        &self,
        req: crate::model::ListCryptoKeyVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListCryptoKeyVersionsResponse> {
        T::list_crypto_key_versions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_import_jobs(
        &self,
        req: crate::model::ListImportJobsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListImportJobsResponse> {
        T::list_import_jobs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_key_ring(
        &self,
        req: crate::model::GetKeyRingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::KeyRing> {
        T::get_key_ring(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_crypto_key(
        &self,
        req: crate::model::GetCryptoKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKey> {
        T::get_crypto_key(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_crypto_key_version(
        &self,
        req: crate::model::GetCryptoKeyVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKeyVersion> {
        T::get_crypto_key_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_public_key(
        &self,
        req: crate::model::GetPublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PublicKey> {
        T::get_public_key(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_import_job(
        &self,
        req: crate::model::GetImportJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ImportJob> {
        T::get_import_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_key_ring(
        &self,
        req: crate::model::CreateKeyRingRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::KeyRing> {
        T::create_key_ring(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_crypto_key(
        &self,
        req: crate::model::CreateCryptoKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKey> {
        T::create_crypto_key(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_crypto_key_version(
        &self,
        req: crate::model::CreateCryptoKeyVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKeyVersion> {
        T::create_crypto_key_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn import_crypto_key_version(
        &self,
        req: crate::model::ImportCryptoKeyVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKeyVersion> {
        T::import_crypto_key_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_import_job(
        &self,
        req: crate::model::CreateImportJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ImportJob> {
        T::create_import_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_crypto_key(
        &self,
        req: crate::model::UpdateCryptoKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKey> {
        T::update_crypto_key(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_crypto_key_version(
        &self,
        req: crate::model::UpdateCryptoKeyVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKeyVersion> {
        T::update_crypto_key_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_crypto_key_primary_version(
        &self,
        req: crate::model::UpdateCryptoKeyPrimaryVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKey> {
        T::update_crypto_key_primary_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn destroy_crypto_key_version(
        &self,
        req: crate::model::DestroyCryptoKeyVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKeyVersion> {
        T::destroy_crypto_key_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn restore_crypto_key_version(
        &self,
        req: crate::model::RestoreCryptoKeyVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CryptoKeyVersion> {
        T::restore_crypto_key_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn encrypt(
        &self,
        req: crate::model::EncryptRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EncryptResponse> {
        T::encrypt(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn decrypt(
        &self,
        req: crate::model::DecryptRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DecryptResponse> {
        T::decrypt(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn raw_encrypt(
        &self,
        req: crate::model::RawEncryptRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::RawEncryptResponse> {
        T::raw_encrypt(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn raw_decrypt(
        &self,
        req: crate::model::RawDecryptRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::RawDecryptResponse> {
        T::raw_decrypt(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn asymmetric_sign(
        &self,
        req: crate::model::AsymmetricSignRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AsymmetricSignResponse> {
        T::asymmetric_sign(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn asymmetric_decrypt(
        &self,
        req: crate::model::AsymmetricDecryptRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AsymmetricDecryptResponse> {
        T::asymmetric_decrypt(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn mac_sign(
        &self,
        req: crate::model::MacSignRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MacSignResponse> {
        T::mac_sign(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn mac_verify(
        &self,
        req: crate::model::MacVerifyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MacVerifyResponse> {
        T::mac_verify(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn generate_random_bytes(
        &self,
        req: crate::model::GenerateRandomBytesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GenerateRandomBytesResponse> {
        T::generate_random_bytes(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location> {
        T::get_location(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse> {
        T::test_iam_permissions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
    }
}
