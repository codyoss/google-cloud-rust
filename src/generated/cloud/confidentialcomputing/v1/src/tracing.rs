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

/// Implements a [ConfidentialComputing](super::stubs::ConfidentialComputing) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct ConfidentialComputing<T>
where
    T: super::stubs::ConfidentialComputing + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> ConfidentialComputing<T>
where
    T: super::stubs::ConfidentialComputing + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::ConfidentialComputing for ConfidentialComputing<T>
where
    T: super::stubs::ConfidentialComputing + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_challenge(
        &self,
        req: crate::model::CreateChallengeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Challenge> {
        self.inner.create_challenge(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn verify_attestation(
        &self,
        req: crate::model::VerifyAttestationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::VerifyAttestationResponse> {
        self.inner.verify_attestation(req, options).await
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
}
