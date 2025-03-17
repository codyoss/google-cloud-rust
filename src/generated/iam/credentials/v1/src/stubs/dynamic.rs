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

/// A dyn-compatible, crate-private version of [super::IAMCredentials].
#[async_trait::async_trait]
pub trait IAMCredentials: std::fmt::Debug + Send + Sync {
    async fn generate_access_token(
        &self,
        req: crate::model::GenerateAccessTokenRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GenerateAccessTokenResponse>;

    async fn generate_id_token(
        &self,
        req: crate::model::GenerateIdTokenRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GenerateIdTokenResponse>;

    async fn sign_blob(
        &self,
        req: crate::model::SignBlobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SignBlobResponse>;

    async fn sign_jwt(
        &self,
        req: crate::model::SignJwtRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SignJwtResponse>;
}

/// All implementations of [super::IAMCredentials] also implement [IAMCredentials].
#[async_trait::async_trait]
impl<T: super::IAMCredentials> IAMCredentials for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn generate_access_token(
        &self,
        req: crate::model::GenerateAccessTokenRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GenerateAccessTokenResponse> {
        T::generate_access_token(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn generate_id_token(
        &self,
        req: crate::model::GenerateIdTokenRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GenerateIdTokenResponse> {
        T::generate_id_token(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn sign_blob(
        &self,
        req: crate::model::SignBlobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SignBlobResponse> {
        T::sign_blob(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn sign_jwt(
        &self,
        req: crate::model::SignJwtRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SignJwtResponse> {
        T::sign_jwt(self, req, options).await
    }
}
