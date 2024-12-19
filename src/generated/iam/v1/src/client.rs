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
use std::sync::Arc;

/// An implementation of [crate::traits::IAMPolicy] to make requests with.
///
/// `IAMPolicy` has various configuration parameters, but the defaults
/// are set to work with most applications.
///
/// `IAMPolicy` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `IAMPolicy` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
///
/// API Overview
///
/// Manages Identity and Access Management (IAM) policies.
///
/// Any implementation of an API that offers access control features
/// implements the google.iam.v1.IAMPolicy interface.
///
/// ## Data model
///
/// Access control is applied when a principal (user or service account), takes
/// some action on a resource exposed by a service. Resources, identified by
/// URI-like names, are the unit of access control specification. Service
/// implementations can choose the granularity of access control and the
/// supported permissions for their resources.
/// For example one database service may allow access control to be
/// specified only at the Table level, whereas another might allow access control
/// to also be specified at the Column level.
///
/// ## Policy Structure
///
/// See google.iam.v1.Policy
///
/// This is intentionally not a CRUD style API because access control policies
/// are created and deleted implicitly with the resources to which they are
/// attached.
#[derive(Clone, Debug)]
pub struct IAMPolicy {
    inner: Arc<dyn crate::traits::dyntraits::IAMPolicy>,
}

impl IAMPolicy {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(crate::ConfigBuilder::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: crate::ConfigBuilder) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    async fn build_inner(
        conf: crate::ConfigBuilder,
    ) -> Result<Arc<dyn crate::traits::dyntraits::IAMPolicy>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: crate::ConfigBuilder) -> Result<impl crate::traits::IAMPolicy> {
        crate::transport::IAMPolicy::new(conf).await
    }

    async fn build_with_tracing(
        conf: crate::ConfigBuilder,
    ) -> Result<impl crate::traits::IAMPolicy> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::IAMPolicy::new)
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    pub fn set_iam_policy(&self) -> crate::builders::SetIamPolicy {
        crate::builders::SetIamPolicy::new(self.inner.clone())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(&self) -> crate::builders::GetIamPolicy {
        crate::builders::GetIamPolicy::new(self.inner.clone())
    }

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    pub fn test_iam_permissions(&self) -> crate::builders::TestIamPermissions {
        crate::builders::TestIamPermissions::new(self.inner.clone())
    }
}
