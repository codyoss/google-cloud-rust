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

/// Implements a [CloudShellService](super::stubs::CloudShellService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct CloudShellService<T>
where
    T: super::stubs::CloudShellService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> CloudShellService<T>
where
    T: super::stubs::CloudShellService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::CloudShellService for CloudShellService<T>
where
    T: super::stubs::CloudShellService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn get_environment(
        &self,
        req: crate::model::GetEnvironmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Environment> {
        self.inner.get_environment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn start_environment(
        &self,
        req: crate::model::StartEnvironmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.start_environment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn authorize_environment(
        &self,
        req: crate::model::AuthorizeEnvironmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.authorize_environment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn add_public_key(
        &self,
        req: crate::model::AddPublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.add_public_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn remove_public_key(
        &self,
        req: crate::model::RemovePublicKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.remove_public_key(req, options).await
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
