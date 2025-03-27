// Copyright 2024 Google LLC
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

/// Implements a [IAMPolicy](super::stubs::IAMPolicy) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct IAMPolicy<T>
where T: super::stubs::IAMPolicy + std::fmt::Debug + Send + Sync {
    inner: T,
}

impl<T> IAMPolicy<T>
where T: super::stubs::IAMPolicy + std::fmt::Debug + Send + Sync {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::IAMPolicy for IAMPolicy<T>
where T: super::stubs::IAMPolicy + std::fmt::Debug + Send + Sync {
    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: crate::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: crate::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: crate::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
    }

}

