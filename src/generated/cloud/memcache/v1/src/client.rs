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

/// Implements a client for the Cloud Memorystore for Memcached API.
///
/// # Service Description
///
/// Configures and manages Cloud Memorystore for Memcached instances.
///
/// The `memcache.googleapis.com` service implements the Google Cloud Memorystore
/// for Memcached API and defines the following resource model for managing
/// Memorystore Memcached (also called Memcached below) instances:
///
/// * The service works with a collection of cloud projects, named: `/projects/*`
/// * Each project has a collection of available locations, named: `/locations/*`
/// * Each location has a collection of Memcached instances, named:
///   `/instances/*`
/// * As such, Memcached instances are resources of the form:
///   `/projects/{project_id}/locations/{location_id}/instances/{instance_id}`
///
/// Note that location_id must be a GCP `region`; for example:
///
/// * `projects/my-memcached-project/locations/us-central1/instances/my-memcached`
///
/// # Configuration
///
/// `CloudMemcache` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `CloudMemcache` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `CloudMemcache` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct CloudMemcache {
    inner: Arc<dyn crate::stubs::dynamic::CloudMemcache>,
}

impl CloudMemcache {
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
        T: crate::stubs::CloudMemcache + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::CloudMemcache>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::CloudMemcache> {
        crate::transport::CloudMemcache::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::CloudMemcache> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::CloudMemcache::new)
    }

    /// Lists Instances in a given location.
    pub fn list_instances(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::cloud_memcache::ListInstances {
        crate::builders::cloud_memcache::ListInstances::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single Instance.
    pub fn get_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::cloud_memcache::GetInstance {
        crate::builders::cloud_memcache::GetInstance::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a new Instance in a given location.
    ///
    /// # Long running operations
    ///
    /// Calling [poller()] on the resulting builder returns an implementation of
    /// the [lro::Poller] trait. You need to call `Poller::poll` on this
    /// `Poller` at least once to start the LRO. You may periodically poll this
    /// object to find the status of the operation. The poller automatically
    /// extract the final response value and any intermediate metadata values.
    ///
    /// Calling [send()] on the resulting builder starts a LRO (long-Running
    /// Operation). LROs run in the background, and the application may poll
    /// them periodically to find out if they have succeeded, or failed. See
    /// below for instructions on how to manually use the resulting [Operation].
    /// We recommend `poller()` in favor of `send()`.
    ///
    /// ## Polling until completion
    ///
    /// Applications that do not care about intermediate results in a
    /// long-running operation may use the [until_done()] function:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_memcache_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::Instance, model::OperationMetadata>
    /// ) -> Result<model::Instance> {
    ///     poller.until_done().await
    /// }
    /// ```
    ///
    /// This will wait until the LRO completes (successfully or with an error).
    /// Applications can set the [PollingPolicy] and [PollingBackoffPolicy] to
    /// control for how long the function runs.
    ///
    /// ## Polling with detailed metadata updates
    ///
    /// Using the result of [poller()] follows a common pattern:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_memcache_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::Instance, model::OperationMetadata>
    /// ) -> Result<model::Instance> {
    ///     while let Some(p) = poller.poll().await {
    ///         match p {
    ///             lro::PollingResult::Completed(r) => { return r; },
    ///             lro::PollingResult::InProgress(m) => { println!("in progress {m:?}"); },
    ///             lro::PollingResult::PollingError(_) => { /* ignored */ },
    ///         }
    ///         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    ///     }
    ///     Err(gax::error::Error::other("LRO never completed"))
    /// }
    /// ```
    ///
    /// ## Manually polling long-running operations
    ///
    /// If you call [send()], you need to examine the contents of the resulting
    /// [Operation][longrunning::model::Operation] to determine the result of
    /// the operation.
    ///
    /// If the `done` field is `true`, the operation has completed. The `result`
    /// field contains the final response, this will be a [crate::model::Instance] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::OperationMetadata] value in the `metadata`
    /// field. This value would also be encoded as an [Any]. The metadata may
    /// include information about how much progress the LRO has made.
    ///
    /// To find out if the operation has completed, use the [get_operation]
    /// method and repeat the steps outlined above.
    ///
    /// Note that most errors on [get_operation] do not indicate that the
    /// long-running operation failed. Long-running operation failures return
    /// the error status in the [result] field.
    ///
    /// [send()]: crate::builders::cloud_memcache::CreateInstance::send
    /// [poller()]: crate::builders::cloud_memcache::CreateInstance::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn create_instance(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::cloud_memcache::CreateInstance {
        crate::builders::cloud_memcache::CreateInstance::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates an existing Instance in a given project and location.
    ///
    /// # Long running operations
    ///
    /// Calling [poller()] on the resulting builder returns an implementation of
    /// the [lro::Poller] trait. You need to call `Poller::poll` on this
    /// `Poller` at least once to start the LRO. You may periodically poll this
    /// object to find the status of the operation. The poller automatically
    /// extract the final response value and any intermediate metadata values.
    ///
    /// Calling [send()] on the resulting builder starts a LRO (long-Running
    /// Operation). LROs run in the background, and the application may poll
    /// them periodically to find out if they have succeeded, or failed. See
    /// below for instructions on how to manually use the resulting [Operation].
    /// We recommend `poller()` in favor of `send()`.
    ///
    /// ## Polling until completion
    ///
    /// Applications that do not care about intermediate results in a
    /// long-running operation may use the [until_done()] function:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_memcache_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::Instance, model::OperationMetadata>
    /// ) -> Result<model::Instance> {
    ///     poller.until_done().await
    /// }
    /// ```
    ///
    /// This will wait until the LRO completes (successfully or with an error).
    /// Applications can set the [PollingPolicy] and [PollingBackoffPolicy] to
    /// control for how long the function runs.
    ///
    /// ## Polling with detailed metadata updates
    ///
    /// Using the result of [poller()] follows a common pattern:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_memcache_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::Instance, model::OperationMetadata>
    /// ) -> Result<model::Instance> {
    ///     while let Some(p) = poller.poll().await {
    ///         match p {
    ///             lro::PollingResult::Completed(r) => { return r; },
    ///             lro::PollingResult::InProgress(m) => { println!("in progress {m:?}"); },
    ///             lro::PollingResult::PollingError(_) => { /* ignored */ },
    ///         }
    ///         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    ///     }
    ///     Err(gax::error::Error::other("LRO never completed"))
    /// }
    /// ```
    ///
    /// ## Manually polling long-running operations
    ///
    /// If you call [send()], you need to examine the contents of the resulting
    /// [Operation][longrunning::model::Operation] to determine the result of
    /// the operation.
    ///
    /// If the `done` field is `true`, the operation has completed. The `result`
    /// field contains the final response, this will be a [crate::model::Instance] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::OperationMetadata] value in the `metadata`
    /// field. This value would also be encoded as an [Any]. The metadata may
    /// include information about how much progress the LRO has made.
    ///
    /// To find out if the operation has completed, use the [get_operation]
    /// method and repeat the steps outlined above.
    ///
    /// Note that most errors on [get_operation] do not indicate that the
    /// long-running operation failed. Long-running operation failures return
    /// the error status in the [result] field.
    ///
    /// [send()]: crate::builders::cloud_memcache::UpdateInstance::send
    /// [poller()]: crate::builders::cloud_memcache::UpdateInstance::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn update_instance(
        &self,
        instance: impl Into<crate::model::Instance>,
    ) -> crate::builders::cloud_memcache::UpdateInstance {
        crate::builders::cloud_memcache::UpdateInstance::new(self.inner.clone())
            .set_instance(instance.into())
    }

    /// Updates the defined Memcached parameters for an existing instance.
    /// This method only stages the parameters, it must be followed by
    /// `ApplyParameters` to apply the parameters to nodes of the Memcached
    /// instance.
    ///
    /// # Long running operations
    ///
    /// Calling [poller()] on the resulting builder returns an implementation of
    /// the [lro::Poller] trait. You need to call `Poller::poll` on this
    /// `Poller` at least once to start the LRO. You may periodically poll this
    /// object to find the status of the operation. The poller automatically
    /// extract the final response value and any intermediate metadata values.
    ///
    /// Calling [send()] on the resulting builder starts a LRO (long-Running
    /// Operation). LROs run in the background, and the application may poll
    /// them periodically to find out if they have succeeded, or failed. See
    /// below for instructions on how to manually use the resulting [Operation].
    /// We recommend `poller()` in favor of `send()`.
    ///
    /// ## Polling until completion
    ///
    /// Applications that do not care about intermediate results in a
    /// long-running operation may use the [until_done()] function:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_memcache_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::Instance, model::OperationMetadata>
    /// ) -> Result<model::Instance> {
    ///     poller.until_done().await
    /// }
    /// ```
    ///
    /// This will wait until the LRO completes (successfully or with an error).
    /// Applications can set the [PollingPolicy] and [PollingBackoffPolicy] to
    /// control for how long the function runs.
    ///
    /// ## Polling with detailed metadata updates
    ///
    /// Using the result of [poller()] follows a common pattern:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_memcache_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::Instance, model::OperationMetadata>
    /// ) -> Result<model::Instance> {
    ///     while let Some(p) = poller.poll().await {
    ///         match p {
    ///             lro::PollingResult::Completed(r) => { return r; },
    ///             lro::PollingResult::InProgress(m) => { println!("in progress {m:?}"); },
    ///             lro::PollingResult::PollingError(_) => { /* ignored */ },
    ///         }
    ///         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    ///     }
    ///     Err(gax::error::Error::other("LRO never completed"))
    /// }
    /// ```
    ///
    /// ## Manually polling long-running operations
    ///
    /// If you call [send()], you need to examine the contents of the resulting
    /// [Operation][longrunning::model::Operation] to determine the result of
    /// the operation.
    ///
    /// If the `done` field is `true`, the operation has completed. The `result`
    /// field contains the final response, this will be a [crate::model::Instance] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::OperationMetadata] value in the `metadata`
    /// field. This value would also be encoded as an [Any]. The metadata may
    /// include information about how much progress the LRO has made.
    ///
    /// To find out if the operation has completed, use the [get_operation]
    /// method and repeat the steps outlined above.
    ///
    /// Note that most errors on [get_operation] do not indicate that the
    /// long-running operation failed. Long-running operation failures return
    /// the error status in the [result] field.
    ///
    /// [send()]: crate::builders::cloud_memcache::UpdateParameters::send
    /// [poller()]: crate::builders::cloud_memcache::UpdateParameters::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn update_parameters(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::cloud_memcache::UpdateParameters {
        crate::builders::cloud_memcache::UpdateParameters::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes a single Instance.
    ///
    /// # Long running operations
    ///
    /// Calling [poller()] on the resulting builder returns an implementation of
    /// the [lro::Poller] trait. You need to call `Poller::poll` on this
    /// `Poller` at least once to start the LRO. You may periodically poll this
    /// object to find the status of the operation. The poller automatically
    /// extract the final response value and any intermediate metadata values.
    ///
    /// Calling [send()] on the resulting builder starts a LRO (long-Running
    /// Operation). LROs run in the background, and the application may poll
    /// them periodically to find out if they have succeeded, or failed. See
    /// below for instructions on how to manually use the resulting [Operation].
    /// We recommend `poller()` in favor of `send()`.
    ///
    /// ## Polling until completion
    ///
    /// Applications that do not care about intermediate results in a
    /// long-running operation may use the [until_done()] function:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_memcache_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<wkt::Empty, model::OperationMetadata>
    /// ) -> Result<wkt::Empty> {
    ///     poller.until_done().await
    /// }
    /// ```
    ///
    /// This will wait until the LRO completes (successfully or with an error).
    /// Applications can set the [PollingPolicy] and [PollingBackoffPolicy] to
    /// control for how long the function runs.
    ///
    /// ## Polling with detailed metadata updates
    ///
    /// Using the result of [poller()] follows a common pattern:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_memcache_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<wkt::Empty, model::OperationMetadata>
    /// ) -> Result<wkt::Empty> {
    ///     while let Some(p) = poller.poll().await {
    ///         match p {
    ///             lro::PollingResult::Completed(r) => { return r; },
    ///             lro::PollingResult::InProgress(m) => { println!("in progress {m:?}"); },
    ///             lro::PollingResult::PollingError(_) => { /* ignored */ },
    ///         }
    ///         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    ///     }
    ///     Err(gax::error::Error::other("LRO never completed"))
    /// }
    /// ```
    ///
    /// ## Manually polling long-running operations
    ///
    /// If you call [send()], you need to examine the contents of the resulting
    /// [Operation][longrunning::model::Operation] to determine the result of
    /// the operation.
    ///
    /// If the `done` field is `true`, the operation has completed. The `result`
    /// field contains the final response, this will be a [wkt::Empty] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::OperationMetadata] value in the `metadata`
    /// field. This value would also be encoded as an [Any]. The metadata may
    /// include information about how much progress the LRO has made.
    ///
    /// To find out if the operation has completed, use the [get_operation]
    /// method and repeat the steps outlined above.
    ///
    /// Note that most errors on [get_operation] do not indicate that the
    /// long-running operation failed. Long-running operation failures return
    /// the error status in the [result] field.
    ///
    /// [send()]: crate::builders::cloud_memcache::DeleteInstance::send
    /// [poller()]: crate::builders::cloud_memcache::DeleteInstance::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn delete_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::cloud_memcache::DeleteInstance {
        crate::builders::cloud_memcache::DeleteInstance::new(self.inner.clone())
            .set_name(name.into())
    }

    /// `ApplyParameters` restarts the set of specified nodes in order to update
    /// them to the current set of parameters for the Memcached Instance.
    ///
    /// # Long running operations
    ///
    /// Calling [poller()] on the resulting builder returns an implementation of
    /// the [lro::Poller] trait. You need to call `Poller::poll` on this
    /// `Poller` at least once to start the LRO. You may periodically poll this
    /// object to find the status of the operation. The poller automatically
    /// extract the final response value and any intermediate metadata values.
    ///
    /// Calling [send()] on the resulting builder starts a LRO (long-Running
    /// Operation). LROs run in the background, and the application may poll
    /// them periodically to find out if they have succeeded, or failed. See
    /// below for instructions on how to manually use the resulting [Operation].
    /// We recommend `poller()` in favor of `send()`.
    ///
    /// ## Polling until completion
    ///
    /// Applications that do not care about intermediate results in a
    /// long-running operation may use the [until_done()] function:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_memcache_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::Instance, model::OperationMetadata>
    /// ) -> Result<model::Instance> {
    ///     poller.until_done().await
    /// }
    /// ```
    ///
    /// This will wait until the LRO completes (successfully or with an error).
    /// Applications can set the [PollingPolicy] and [PollingBackoffPolicy] to
    /// control for how long the function runs.
    ///
    /// ## Polling with detailed metadata updates
    ///
    /// Using the result of [poller()] follows a common pattern:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_memcache_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::Instance, model::OperationMetadata>
    /// ) -> Result<model::Instance> {
    ///     while let Some(p) = poller.poll().await {
    ///         match p {
    ///             lro::PollingResult::Completed(r) => { return r; },
    ///             lro::PollingResult::InProgress(m) => { println!("in progress {m:?}"); },
    ///             lro::PollingResult::PollingError(_) => { /* ignored */ },
    ///         }
    ///         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    ///     }
    ///     Err(gax::error::Error::other("LRO never completed"))
    /// }
    /// ```
    ///
    /// ## Manually polling long-running operations
    ///
    /// If you call [send()], you need to examine the contents of the resulting
    /// [Operation][longrunning::model::Operation] to determine the result of
    /// the operation.
    ///
    /// If the `done` field is `true`, the operation has completed. The `result`
    /// field contains the final response, this will be a [crate::model::Instance] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::OperationMetadata] value in the `metadata`
    /// field. This value would also be encoded as an [Any]. The metadata may
    /// include information about how much progress the LRO has made.
    ///
    /// To find out if the operation has completed, use the [get_operation]
    /// method and repeat the steps outlined above.
    ///
    /// Note that most errors on [get_operation] do not indicate that the
    /// long-running operation failed. Long-running operation failures return
    /// the error status in the [result] field.
    ///
    /// [send()]: crate::builders::cloud_memcache::ApplyParameters::send
    /// [poller()]: crate::builders::cloud_memcache::ApplyParameters::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn apply_parameters(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::cloud_memcache::ApplyParameters {
        crate::builders::cloud_memcache::ApplyParameters::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Reschedules upcoming maintenance event.
    ///
    /// # Long running operations
    ///
    /// Calling [poller()] on the resulting builder returns an implementation of
    /// the [lro::Poller] trait. You need to call `Poller::poll` on this
    /// `Poller` at least once to start the LRO. You may periodically poll this
    /// object to find the status of the operation. The poller automatically
    /// extract the final response value and any intermediate metadata values.
    ///
    /// Calling [send()] on the resulting builder starts a LRO (long-Running
    /// Operation). LROs run in the background, and the application may poll
    /// them periodically to find out if they have succeeded, or failed. See
    /// below for instructions on how to manually use the resulting [Operation].
    /// We recommend `poller()` in favor of `send()`.
    ///
    /// ## Polling until completion
    ///
    /// Applications that do not care about intermediate results in a
    /// long-running operation may use the [until_done()] function:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_memcache_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::Instance, model::OperationMetadata>
    /// ) -> Result<model::Instance> {
    ///     poller.until_done().await
    /// }
    /// ```
    ///
    /// This will wait until the LRO completes (successfully or with an error).
    /// Applications can set the [PollingPolicy] and [PollingBackoffPolicy] to
    /// control for how long the function runs.
    ///
    /// ## Polling with detailed metadata updates
    ///
    /// Using the result of [poller()] follows a common pattern:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_memcache_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::Instance, model::OperationMetadata>
    /// ) -> Result<model::Instance> {
    ///     while let Some(p) = poller.poll().await {
    ///         match p {
    ///             lro::PollingResult::Completed(r) => { return r; },
    ///             lro::PollingResult::InProgress(m) => { println!("in progress {m:?}"); },
    ///             lro::PollingResult::PollingError(_) => { /* ignored */ },
    ///         }
    ///         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    ///     }
    ///     Err(gax::error::Error::other("LRO never completed"))
    /// }
    /// ```
    ///
    /// ## Manually polling long-running operations
    ///
    /// If you call [send()], you need to examine the contents of the resulting
    /// [Operation][longrunning::model::Operation] to determine the result of
    /// the operation.
    ///
    /// If the `done` field is `true`, the operation has completed. The `result`
    /// field contains the final response, this will be a [crate::model::Instance] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::OperationMetadata] value in the `metadata`
    /// field. This value would also be encoded as an [Any]. The metadata may
    /// include information about how much progress the LRO has made.
    ///
    /// To find out if the operation has completed, use the [get_operation]
    /// method and repeat the steps outlined above.
    ///
    /// Note that most errors on [get_operation] do not indicate that the
    /// long-running operation failed. Long-running operation failures return
    /// the error status in the [result] field.
    ///
    /// [send()]: crate::builders::cloud_memcache::RescheduleMaintenance::send
    /// [poller()]: crate::builders::cloud_memcache::RescheduleMaintenance::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn reschedule_maintenance(
        &self,
        instance: impl Into<std::string::String>,
    ) -> crate::builders::cloud_memcache::RescheduleMaintenance {
        crate::builders::cloud_memcache::RescheduleMaintenance::new(self.inner.clone())
            .set_instance(instance.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::cloud_memcache::ListLocations {
        crate::builders::cloud_memcache::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::cloud_memcache::GetLocation {
        crate::builders::cloud_memcache::GetLocation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::cloud_memcache::ListOperations {
        crate::builders::cloud_memcache::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::cloud_memcache::GetOperation {
        crate::builders::cloud_memcache::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::cloud_memcache::DeleteOperation {
        crate::builders::cloud_memcache::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::cloud_memcache::CancelOperation {
        crate::builders::cloud_memcache::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
