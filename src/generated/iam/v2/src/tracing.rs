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

/// Implements a [Policies](super::stub::Policies) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Policies<T>
where
    T: super::stub::Policies + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Policies<T>
where
    T: super::stub::Policies + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::Policies for Policies<T>
where
    T: super::stub::Policies + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_policies(
        &self,
        req: crate::model::ListPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListPoliciesResponse>> {
        self.inner.list_policies(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_policy(
        &self,
        req: crate::model::GetPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Policy>> {
        self.inner.get_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_policy(
        &self,
        req: crate::model::CreatePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_policy(
        &self,
        req: crate::model::UpdatePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_policy(
        &self,
        req: crate::model::DeletePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
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
