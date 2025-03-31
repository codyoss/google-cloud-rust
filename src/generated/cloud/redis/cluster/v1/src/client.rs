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
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Google Cloud Memorystore for Redis API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_redis_cluster_v1::client::CloudRedisCluster;
/// let client = CloudRedisCluster::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Configures and manages Cloud Memorystore for Redis clusters
///
/// Google Cloud Memorystore for Redis Cluster
///
/// The `redis.googleapis.com` service implements the Google Cloud Memorystore
/// for Redis API and defines the following resource model for managing Redis
/// clusters:
///
/// * The service works with a collection of cloud projects, named: `/projects/*`
/// * Each project has a collection of available locations, named: `/locations/*`
/// * Each location has a collection of Redis clusters, named: `/clusters/*`
/// * As such, Redis clusters are resources of the form:
///   `/projects/{project_id}/locations/{location_id}/clusters/{instance_id}`
///
/// Note that location_id must be a GCP `region`; for example:
///
/// * `projects/redpepper-1290/locations/us-central1/clusters/my-redis`
///
/// # Configuration
///
/// To configure `CloudRedisCluster` use the `with_*` methods in the type returned
/// by [builder()][CloudRedisCluster::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://redis.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::cloud_redis_cluster::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::cloud_redis_cluster::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `CloudRedisCluster` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `CloudRedisCluster` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct CloudRedisCluster {
    inner: Arc<dyn super::stub::dynamic::CloudRedisCluster>,
}

impl CloudRedisCluster {
    /// Returns a builder for [CloudRedisCluster].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_redis_cluster_v1::client::CloudRedisCluster;
    /// let client = CloudRedisCluster::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::cloud_redis_cluster::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::cloud_redis_cluster::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::CloudRedisCluster + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::CloudRedisCluster>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::CloudRedisCluster> {
        super::transport::CloudRedisCluster::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::CloudRedisCluster> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::CloudRedisCluster::new)
    }

    /// Lists all Redis clusters owned by a project in either the specified
    /// location (region) or all locations.
    ///
    /// The location should have the following format:
    ///
    /// * `projects/{project_id}/locations/{location_id}`
    ///
    /// If `location_id` is specified as `-` (wildcard), then all regions
    /// available to the project are queried, and the results are aggregated.
    pub fn list_clusters(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::ListClusters {
        super::builder::cloud_redis_cluster::ListClusters::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets the details of a specific Redis cluster.
    pub fn get_cluster(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::GetCluster {
        super::builder::cloud_redis_cluster::GetCluster::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates the metadata and configuration of a specific Redis cluster.
    ///
    /// Completed longrunning.Operation will contain the new cluster object
    /// in the response field. The returned operation is automatically deleted
    /// after a few hours, so there is no need to call DeleteOperation.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_cluster(
        &self,
        cluster: impl Into<crate::model::Cluster>,
    ) -> super::builder::cloud_redis_cluster::UpdateCluster {
        super::builder::cloud_redis_cluster::UpdateCluster::new(self.inner.clone())
            .set_cluster(cluster.into())
    }

    /// Deletes a specific Redis cluster. Cluster stops serving and data is
    /// deleted.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_cluster(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::DeleteCluster {
        super::builder::cloud_redis_cluster::DeleteCluster::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a Redis cluster based on the specified properties.
    /// The creation is executed asynchronously and callers may check the returned
    /// operation to track its progress. Once the operation is completed the Redis
    /// cluster will be fully functional. The completed longrunning.Operation will
    /// contain the new cluster object in the response field.
    ///
    /// The returned operation is automatically deleted after a few hours, so there
    /// is no need to call DeleteOperation.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_cluster(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::CreateCluster {
        super::builder::cloud_redis_cluster::CreateCluster::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets the details of certificate authority information for Redis cluster.
    pub fn get_cluster_certificate_authority(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::GetClusterCertificateAuthority {
        super::builder::cloud_redis_cluster::GetClusterCertificateAuthority::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Reschedules upcoming maintenance event.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn reschedule_cluster_maintenance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::RescheduleClusterMaintenance {
        super::builder::cloud_redis_cluster::RescheduleClusterMaintenance::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists all backup collections owned by a consumer project in either the
    /// specified location (region) or all locations.
    ///
    /// If `location_id` is specified as `-` (wildcard), then all regions
    /// available to the project are queried, and the results are aggregated.
    pub fn list_backup_collections(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::ListBackupCollections {
        super::builder::cloud_redis_cluster::ListBackupCollections::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Get a backup collection.
    pub fn get_backup_collection(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::GetBackupCollection {
        super::builder::cloud_redis_cluster::GetBackupCollection::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists all backups owned by a backup collection.
    pub fn list_backups(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::ListBackups {
        super::builder::cloud_redis_cluster::ListBackups::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets the details of a specific backup.
    pub fn get_backup(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::GetBackup {
        super::builder::cloud_redis_cluster::GetBackup::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes a specific backup.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_backup(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::DeleteBackup {
        super::builder::cloud_redis_cluster::DeleteBackup::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Exports a specific backup to a customer target Cloud Storage URI.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn export_backup(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::ExportBackup {
        super::builder::cloud_redis_cluster::ExportBackup::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Backup Redis Cluster.
    /// If this is the first time a backup is being created, a backup collection
    /// will be created at the backend, and this backup belongs to this collection.
    /// Both collection and backup will have a resource name. Backup will be
    /// executed for each shard. A replica (primary if nonHA) will be selected to
    /// perform the execution. Backup call will be rejected if there is an ongoing
    /// backup or update operation. Be aware that during preview, if the cluster's
    /// internal software version is too old, critical update will be performed
    /// before actual backup. Once the internal software version is updated to the
    /// minimum version required by the backup feature, subsequent backups will not
    /// require critical update. After preview, there will be no critical update
    /// needed for backup.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn backup_cluster(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::BackupCluster {
        super::builder::cloud_redis_cluster::BackupCluster::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::ListLocations {
        super::builder::cloud_redis_cluster::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::GetLocation {
        super::builder::cloud_redis_cluster::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::ListOperations {
        super::builder::cloud_redis_cluster::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::GetOperation {
        super::builder::cloud_redis_cluster::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::DeleteOperation {
        super::builder::cloud_redis_cluster::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_redis_cluster::CancelOperation {
        super::builder::cloud_redis_cluster::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
