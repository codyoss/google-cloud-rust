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
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Cloud Profiler API.
///
/// # Service Description
///
/// Manage the collection of continuous profiling data provided by profiling
/// agents running in the cloud or by an offline provider of profiling data.
///
/// __The APIs listed in this service are intended for use within our profiler
/// agents only.__
///
/// # Configuration
///
/// `ProfilerService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `ProfilerService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `ProfilerService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct ProfilerService {
    inner: Arc<dyn crate::stubs::dynamic::ProfilerService>,
}

impl ProfilerService {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: crate::stubs::ProfilerService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::ProfilerService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::ProfilerService> {
        crate::transport::ProfilerService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::ProfilerService> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::ProfilerService::new)
    }

    /// CreateProfile creates a new profile resource in the online mode.
    ///
    /// _Direct use of this API is discouraged, please use a [supported
    /// profiler
    /// agent](https://cloud.google.com/profiler/docs/about-profiler#profiling_agent)
    /// instead for profile collection._
    ///
    /// The server ensures that the new profiles are created at a constant rate per
    /// deployment, so the creation request may hang for some time until the next
    /// profile session is available.
    ///
    /// The request may fail with ABORTED error if the creation is not available
    /// within ~1m, the response will indicate the duration of the backoff the
    /// client should take before attempting creating a profile again. The backoff
    /// duration is returned in google.rpc.RetryInfo extension on the response
    /// status. To a gRPC client, the extension will be return as a
    /// binary-serialized proto in the trailing metadata item named
    /// "google.rpc.retryinfo-bin".
    pub fn create_profile(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::profiler_service::CreateProfile {
        crate::builders::profiler_service::CreateProfile::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// CreateOfflineProfile creates a new profile resource in the offline
    /// mode. The client provides the profile to create along with the profile
    /// bytes, the server records it.
    ///
    /// _Direct use of this API is discouraged, please use a [supported
    /// profiler
    /// agent](https://cloud.google.com/profiler/docs/about-profiler#profiling_agent)
    /// instead for profile collection._
    pub fn create_offline_profile(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::profiler_service::CreateOfflineProfile {
        crate::builders::profiler_service::CreateOfflineProfile::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// UpdateProfile updates the profile bytes and labels on the profile resource
    /// created in the online mode. Updating the bytes for profiles created in the
    /// offline mode is currently not supported: the profile content must be
    /// provided at the time of the profile creation.
    ///
    /// _Direct use of this API is discouraged, please use a [supported
    /// profiler
    /// agent](https://cloud.google.com/profiler/docs/about-profiler#profiling_agent)
    /// instead for profile collection._
    pub fn update_profile(
        &self,
        profile: impl Into<crate::model::Profile>,
    ) -> crate::builders::profiler_service::UpdateProfile {
        crate::builders::profiler_service::UpdateProfile::new(self.inner.clone())
            .set_profile(profile.into())
    }
}

/// Implements a client for the Cloud Profiler API.
///
/// # Service Description
///
/// Service allows existing Cloud Profiler customers to export their profile data
/// out of Google Cloud.
///
/// # Configuration
///
/// `ExportService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `ExportService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `ExportService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct ExportService {
    inner: Arc<dyn crate::stubs::dynamic::ExportService>,
}

impl ExportService {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: crate::stubs::ExportService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::ExportService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::ExportService> {
        crate::transport::ExportService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::ExportService> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::ExportService::new)
    }

    /// Lists profiles which have been collected so far and for which the caller
    /// has permission to view.
    pub fn list_profiles(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::export_service::ListProfiles {
        crate::builders::export_service::ListProfiles::new(self.inner.clone())
            .set_parent(parent.into())
    }
}
