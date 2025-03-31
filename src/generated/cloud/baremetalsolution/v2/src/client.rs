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

/// Implements a client for the Bare Metal Solution API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_baremetalsolution_v2::client::BareMetalSolution;
/// let client = BareMetalSolution::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Performs management operations on Bare Metal Solution servers.
///
/// The `baremetalsolution.googleapis.com` service provides management
/// capabilities for Bare Metal Solution servers. To access the API methods, you
/// must assign Bare Metal Solution IAM roles containing the desired permissions
/// to your staff in your Google Cloud project. You must also enable the Bare
/// Metal Solution API. Once enabled, the methods act
/// upon specific servers in your Bare Metal Solution environment.
///
/// # Configuration
///
/// To configure `BareMetalSolution` use the `with_*` methods in the type returned
/// by [builder()][BareMetalSolution::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://baremetalsolution.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::bare_metal_solution::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::bare_metal_solution::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `BareMetalSolution` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `BareMetalSolution` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct BareMetalSolution {
    inner: Arc<dyn super::stub::dynamic::BareMetalSolution>,
}

impl BareMetalSolution {
    /// Returns a builder for [BareMetalSolution].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_baremetalsolution_v2::client::BareMetalSolution;
    /// let client = BareMetalSolution::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::bare_metal_solution::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::bare_metal_solution::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::BareMetalSolution + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::BareMetalSolution>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::BareMetalSolution> {
        super::transport::BareMetalSolution::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::BareMetalSolution> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::BareMetalSolution::new)
    }

    /// List servers in a given project and location.
    pub fn list_instances(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::ListInstances {
        super::builder::bare_metal_solution::ListInstances::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Get details about a single server.
    pub fn get_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::GetInstance {
        super::builder::bare_metal_solution::GetInstance::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Update details of a single server.
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
    pub fn update_instance(
        &self,
        instance: impl Into<crate::model::Instance>,
    ) -> super::builder::bare_metal_solution::UpdateInstance {
        super::builder::bare_metal_solution::UpdateInstance::new(self.inner.clone())
            .set_instance(instance.into())
    }

    /// RenameInstance sets a new name for an instance.
    /// Use with caution, previous names become immediately invalidated.
    pub fn rename_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::RenameInstance {
        super::builder::bare_metal_solution::RenameInstance::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Perform an ungraceful, hard reset on a server. Equivalent to shutting the
    /// power off and then turning it back on.
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
    pub fn reset_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::ResetInstance {
        super::builder::bare_metal_solution::ResetInstance::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Starts a server that was shutdown.
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
    pub fn start_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::StartInstance {
        super::builder::bare_metal_solution::StartInstance::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Stop a running server.
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
    pub fn stop_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::StopInstance {
        super::builder::bare_metal_solution::StopInstance::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Enable the interactive serial console feature on an instance.
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
    pub fn enable_interactive_serial_console(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::EnableInteractiveSerialConsole {
        super::builder::bare_metal_solution::EnableInteractiveSerialConsole::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Disable the interactive serial console feature on an instance.
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
    pub fn disable_interactive_serial_console(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::DisableInteractiveSerialConsole {
        super::builder::bare_metal_solution::DisableInteractiveSerialConsole::new(
            self.inner.clone(),
        )
        .set_name(name.into())
    }

    /// Detach LUN from Instance.
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
    pub fn detach_lun(
        &self,
        instance: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::DetachLun {
        super::builder::bare_metal_solution::DetachLun::new(self.inner.clone())
            .set_instance(instance.into())
    }

    /// Lists the public SSH keys registered for the specified project.
    /// These SSH keys are used only for the interactive serial console feature.
    pub fn list_ssh_keys(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::ListSSHKeys {
        super::builder::bare_metal_solution::ListSSHKeys::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Register a public SSH key in the specified project for use with the
    /// interactive serial console feature.
    pub fn create_ssh_key(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::CreateSSHKey {
        super::builder::bare_metal_solution::CreateSSHKey::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a public SSH key registered in the specified project.
    pub fn delete_ssh_key(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::DeleteSSHKey {
        super::builder::bare_metal_solution::DeleteSSHKey::new(self.inner.clone())
            .set_name(name.into())
    }

    /// List storage volumes in a given project and location.
    pub fn list_volumes(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::ListVolumes {
        super::builder::bare_metal_solution::ListVolumes::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Get details of a single storage volume.
    pub fn get_volume(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::GetVolume {
        super::builder::bare_metal_solution::GetVolume::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Update details of a single storage volume.
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
    pub fn update_volume(
        &self,
        volume: impl Into<crate::model::Volume>,
    ) -> super::builder::bare_metal_solution::UpdateVolume {
        super::builder::bare_metal_solution::UpdateVolume::new(self.inner.clone())
            .set_volume(volume.into())
    }

    /// RenameVolume sets a new name for a volume.
    /// Use with caution, previous names become immediately invalidated.
    pub fn rename_volume(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::RenameVolume {
        super::builder::bare_metal_solution::RenameVolume::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Skips volume's cooloff and deletes it now.
    /// Volume must be in cooloff state.
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
    pub fn evict_volume(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::EvictVolume {
        super::builder::bare_metal_solution::EvictVolume::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Emergency Volume resize.
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
    pub fn resize_volume(
        &self,
        volume: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::ResizeVolume {
        super::builder::bare_metal_solution::ResizeVolume::new(self.inner.clone())
            .set_volume(volume.into())
    }

    /// List network in a given project and location.
    pub fn list_networks(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::ListNetworks {
        super::builder::bare_metal_solution::ListNetworks::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// List all Networks (and used IPs for each Network) in the vendor account
    /// associated with the specified project.
    pub fn list_network_usage(
        &self,
        location: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::ListNetworkUsage {
        super::builder::bare_metal_solution::ListNetworkUsage::new(self.inner.clone())
            .set_location(location.into())
    }

    /// Get details of a single network.
    pub fn get_network(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::GetNetwork {
        super::builder::bare_metal_solution::GetNetwork::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Update details of a single network.
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
    pub fn update_network(
        &self,
        network: impl Into<crate::model::Network>,
    ) -> super::builder::bare_metal_solution::UpdateNetwork {
        super::builder::bare_metal_solution::UpdateNetwork::new(self.inner.clone())
            .set_network(network.into())
    }

    /// Takes a snapshot of a boot volume.
    /// Returns INVALID_ARGUMENT if called for a non-boot volume.
    pub fn create_volume_snapshot(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::CreateVolumeSnapshot {
        super::builder::bare_metal_solution::CreateVolumeSnapshot::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Uses the specified snapshot to restore its parent volume.
    /// Returns INVALID_ARGUMENT if called for a non-boot volume.
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
    pub fn restore_volume_snapshot(
        &self,
        volume_snapshot: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::RestoreVolumeSnapshot {
        super::builder::bare_metal_solution::RestoreVolumeSnapshot::new(self.inner.clone())
            .set_volume_snapshot(volume_snapshot.into())
    }

    /// Deletes a volume snapshot.
    /// Returns INVALID_ARGUMENT if called for a non-boot volume.
    pub fn delete_volume_snapshot(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::DeleteVolumeSnapshot {
        super::builder::bare_metal_solution::DeleteVolumeSnapshot::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Returns the specified snapshot resource.
    /// Returns INVALID_ARGUMENT if called for a non-boot volume.
    pub fn get_volume_snapshot(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::GetVolumeSnapshot {
        super::builder::bare_metal_solution::GetVolumeSnapshot::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Retrieves the list of snapshots for the specified volume.
    /// Returns a response with an empty list of snapshots if called
    /// for a non-boot volume.
    pub fn list_volume_snapshots(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::ListVolumeSnapshots {
        super::builder::bare_metal_solution::ListVolumeSnapshots::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Get details of a single storage logical unit number(LUN).
    pub fn get_lun(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::GetLun {
        super::builder::bare_metal_solution::GetLun::new(self.inner.clone()).set_name(name.into())
    }

    /// List storage volume luns for given storage volume.
    pub fn list_luns(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::ListLuns {
        super::builder::bare_metal_solution::ListLuns::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Skips lun's cooloff and deletes it now.
    /// Lun must be in cooloff state.
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
    pub fn evict_lun(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::EvictLun {
        super::builder::bare_metal_solution::EvictLun::new(self.inner.clone()).set_name(name.into())
    }

    /// Get details of a single NFS share.
    pub fn get_nfs_share(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::GetNfsShare {
        super::builder::bare_metal_solution::GetNfsShare::new(self.inner.clone())
            .set_name(name.into())
    }

    /// List NFS shares.
    pub fn list_nfs_shares(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::ListNfsShares {
        super::builder::bare_metal_solution::ListNfsShares::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Update details of a single NFS share.
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
    pub fn update_nfs_share(
        &self,
        nfs_share: impl Into<crate::model::NfsShare>,
    ) -> super::builder::bare_metal_solution::UpdateNfsShare {
        super::builder::bare_metal_solution::UpdateNfsShare::new(self.inner.clone())
            .set_nfs_share(nfs_share.into())
    }

    /// Create an NFS share.
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
    pub fn create_nfs_share(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::CreateNfsShare {
        super::builder::bare_metal_solution::CreateNfsShare::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// RenameNfsShare sets a new name for an nfsshare.
    /// Use with caution, previous names become immediately invalidated.
    pub fn rename_nfs_share(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::RenameNfsShare {
        super::builder::bare_metal_solution::RenameNfsShare::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Delete an NFS share. The underlying volume is automatically deleted.
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
    pub fn delete_nfs_share(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::DeleteNfsShare {
        super::builder::bare_metal_solution::DeleteNfsShare::new(self.inner.clone())
            .set_name(name.into())
    }

    /// List the budget details to provision resources on a given project.
    pub fn list_provisioning_quotas(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::ListProvisioningQuotas {
        super::builder::bare_metal_solution::ListProvisioningQuotas::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Submit a provisiong configuration for a given project.
    pub fn submit_provisioning_config(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::SubmitProvisioningConfig {
        super::builder::bare_metal_solution::SubmitProvisioningConfig::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Get ProvisioningConfig by name.
    pub fn get_provisioning_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::GetProvisioningConfig {
        super::builder::bare_metal_solution::GetProvisioningConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Create new ProvisioningConfig.
    pub fn create_provisioning_config(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::CreateProvisioningConfig {
        super::builder::bare_metal_solution::CreateProvisioningConfig::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Update existing ProvisioningConfig.
    pub fn update_provisioning_config(
        &self,
        provisioning_config: impl Into<crate::model::ProvisioningConfig>,
    ) -> super::builder::bare_metal_solution::UpdateProvisioningConfig {
        super::builder::bare_metal_solution::UpdateProvisioningConfig::new(self.inner.clone())
            .set_provisioning_config(provisioning_config.into())
    }

    /// RenameNetwork sets a new name for a network.
    /// Use with caution, previous names become immediately invalidated.
    pub fn rename_network(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::RenameNetwork {
        super::builder::bare_metal_solution::RenameNetwork::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Retrieves the list of OS images which are currently approved.
    pub fn list_os_images(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::ListOSImages {
        super::builder::bare_metal_solution::ListOSImages::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::ListLocations {
        super::builder::bare_metal_solution::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::GetLocation {
        super::builder::bare_metal_solution::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::bare_metal_solution::GetOperation {
        super::builder::bare_metal_solution::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
