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

/// A dyn-compatible, crate-private version of [super::OsLoginService].
#[async_trait::async_trait]
pub trait OsLoginService: std::fmt::Debug + Send + Sync {
    async fn create_ssh_public_key(
        &self,
        req: crate::model::CreateSshPublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<oslogin_common::model::SshPublicKey>;

    async fn delete_posix_account(
        &self,
        req: crate::model::DeletePosixAccountRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn delete_ssh_public_key(
        &self,
        req: crate::model::DeleteSshPublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn get_login_profile(
        &self,
        req: crate::model::GetLoginProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LoginProfile>;

    async fn get_ssh_public_key(
        &self,
        req: crate::model::GetSshPublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<oslogin_common::model::SshPublicKey>;

    async fn import_ssh_public_key(
        &self,
        req: crate::model::ImportSshPublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ImportSshPublicKeyResponse>;

    async fn update_ssh_public_key(
        &self,
        req: crate::model::UpdateSshPublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<oslogin_common::model::SshPublicKey>;
}

/// All implementations of [super::OsLoginService] also implement [OsLoginService].
#[async_trait::async_trait]
impl<T: super::OsLoginService> OsLoginService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_ssh_public_key(
        &self,
        req: crate::model::CreateSshPublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<oslogin_common::model::SshPublicKey> {
        T::create_ssh_public_key(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_posix_account(
        &self,
        req: crate::model::DeletePosixAccountRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_posix_account(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_ssh_public_key(
        &self,
        req: crate::model::DeleteSshPublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_ssh_public_key(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_login_profile(
        &self,
        req: crate::model::GetLoginProfileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LoginProfile> {
        T::get_login_profile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_ssh_public_key(
        &self,
        req: crate::model::GetSshPublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<oslogin_common::model::SshPublicKey> {
        T::get_ssh_public_key(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn import_ssh_public_key(
        &self,
        req: crate::model::ImportSshPublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ImportSshPublicKeyResponse> {
        T::import_ssh_public_key(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_ssh_public_key(
        &self,
        req: crate::model::UpdateSshPublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<oslogin_common::model::SshPublicKey> {
        T::update_ssh_public_key(self, req, options).await
    }
}
