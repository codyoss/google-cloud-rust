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

//! Traits to mock the clients in this library.
//!
//! Application developers may need to mock the clients in this library to test
//! how their application works with different (and sometimes hard to trigger)
//! client and service behavior. Such test can define mocks implementing the
//! trait(s) defined in this module, initialize the client with an instance of
//! this mock in their tests, and verify their application responds as expected.

#![allow(rustdoc::broken_intra_doc_links)]

use gax::error::Error;
use std::sync::Arc;

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::BareMetalSolution].
///
/// Application developers may need to implement this trait to mock
/// `client::BareMetalSolution`.  In other use-cases, application developers only
/// use `client::BareMetalSolution` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait BareMetalSolution: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::BareMetalSolution::list_instances].
    fn list_instances(
        &self,
        _req: crate::model::ListInstancesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListInstancesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListInstancesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::get_instance].
    fn get_instance(
        &self,
        _req: crate::model::GetInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Instance>> + Send {
        std::future::ready::<crate::Result<crate::model::Instance>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::update_instance].
    fn update_instance(
        &self,
        _req: crate::model::UpdateInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::rename_instance].
    fn rename_instance(
        &self,
        _req: crate::model::RenameInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Instance>> + Send {
        std::future::ready::<crate::Result<crate::model::Instance>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::reset_instance].
    fn reset_instance(
        &self,
        _req: crate::model::ResetInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::start_instance].
    fn start_instance(
        &self,
        _req: crate::model::StartInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::stop_instance].
    fn stop_instance(
        &self,
        _req: crate::model::StopInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::enable_interactive_serial_console].
    fn enable_interactive_serial_console(
        &self,
        _req: crate::model::EnableInteractiveSerialConsoleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::disable_interactive_serial_console].
    fn disable_interactive_serial_console(
        &self,
        _req: crate::model::DisableInteractiveSerialConsoleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::detach_lun].
    fn detach_lun(
        &self,
        _req: crate::model::DetachLunRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::list_ssh_keys].
    fn list_ssh_keys(
        &self,
        _req: crate::model::ListSSHKeysRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListSSHKeysResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListSSHKeysResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::create_ssh_key].
    fn create_ssh_key(
        &self,
        _req: crate::model::CreateSSHKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SSHKey>> + Send {
        std::future::ready::<crate::Result<crate::model::SSHKey>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::delete_ssh_key].
    fn delete_ssh_key(
        &self,
        _req: crate::model::DeleteSSHKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::BareMetalSolution::list_volumes].
    fn list_volumes(
        &self,
        _req: crate::model::ListVolumesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListVolumesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListVolumesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::get_volume].
    fn get_volume(
        &self,
        _req: crate::model::GetVolumeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Volume>> + Send {
        std::future::ready::<crate::Result<crate::model::Volume>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::update_volume].
    fn update_volume(
        &self,
        _req: crate::model::UpdateVolumeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::rename_volume].
    fn rename_volume(
        &self,
        _req: crate::model::RenameVolumeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Volume>> + Send {
        std::future::ready::<crate::Result<crate::model::Volume>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::evict_volume].
    fn evict_volume(
        &self,
        _req: crate::model::EvictVolumeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::resize_volume].
    fn resize_volume(
        &self,
        _req: crate::model::ResizeVolumeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::list_networks].
    fn list_networks(
        &self,
        _req: crate::model::ListNetworksRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListNetworksResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListNetworksResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::list_network_usage].
    fn list_network_usage(
        &self,
        _req: crate::model::ListNetworkUsageRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListNetworkUsageResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListNetworkUsageResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::BareMetalSolution::get_network].
    fn get_network(
        &self,
        _req: crate::model::GetNetworkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Network>> + Send {
        std::future::ready::<crate::Result<crate::model::Network>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::update_network].
    fn update_network(
        &self,
        _req: crate::model::UpdateNetworkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::create_volume_snapshot].
    fn create_volume_snapshot(
        &self,
        _req: crate::model::CreateVolumeSnapshotRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::VolumeSnapshot>> + Send {
        std::future::ready::<crate::Result<crate::model::VolumeSnapshot>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::restore_volume_snapshot].
    fn restore_volume_snapshot(
        &self,
        _req: crate::model::RestoreVolumeSnapshotRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::delete_volume_snapshot].
    fn delete_volume_snapshot(
        &self,
        _req: crate::model::DeleteVolumeSnapshotRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::BareMetalSolution::get_volume_snapshot].
    fn get_volume_snapshot(
        &self,
        _req: crate::model::GetVolumeSnapshotRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::VolumeSnapshot>> + Send {
        std::future::ready::<crate::Result<crate::model::VolumeSnapshot>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::list_volume_snapshots].
    fn list_volume_snapshots(
        &self,
        _req: crate::model::ListVolumeSnapshotsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListVolumeSnapshotsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListVolumeSnapshotsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::BareMetalSolution::get_lun].
    fn get_lun(
        &self,
        _req: crate::model::GetLunRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Lun>> + Send {
        std::future::ready::<crate::Result<crate::model::Lun>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::BareMetalSolution::list_luns].
    fn list_luns(
        &self,
        _req: crate::model::ListLunsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListLunsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListLunsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::evict_lun].
    fn evict_lun(
        &self,
        _req: crate::model::EvictLunRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::get_nfs_share].
    fn get_nfs_share(
        &self,
        _req: crate::model::GetNfsShareRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::NfsShare>> + Send {
        std::future::ready::<crate::Result<crate::model::NfsShare>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::list_nfs_shares].
    fn list_nfs_shares(
        &self,
        _req: crate::model::ListNfsSharesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListNfsSharesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListNfsSharesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::update_nfs_share].
    fn update_nfs_share(
        &self,
        _req: crate::model::UpdateNfsShareRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::create_nfs_share].
    fn create_nfs_share(
        &self,
        _req: crate::model::CreateNfsShareRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::rename_nfs_share].
    fn rename_nfs_share(
        &self,
        _req: crate::model::RenameNfsShareRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::NfsShare>> + Send {
        std::future::ready::<crate::Result<crate::model::NfsShare>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::delete_nfs_share].
    fn delete_nfs_share(
        &self,
        _req: crate::model::DeleteNfsShareRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::list_provisioning_quotas].
    fn list_provisioning_quotas(
        &self,
        _req: crate::model::ListProvisioningQuotasRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListProvisioningQuotasResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::ListProvisioningQuotasResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::BareMetalSolution::submit_provisioning_config].
    fn submit_provisioning_config(
        &self,
        _req: crate::model::SubmitProvisioningConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::SubmitProvisioningConfigResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::SubmitProvisioningConfigResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::BareMetalSolution::get_provisioning_config].
    fn get_provisioning_config(
        &self,
        _req: crate::model::GetProvisioningConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ProvisioningConfig>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ProvisioningConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::create_provisioning_config].
    fn create_provisioning_config(
        &self,
        _req: crate::model::CreateProvisioningConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ProvisioningConfig>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ProvisioningConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::update_provisioning_config].
    fn update_provisioning_config(
        &self,
        _req: crate::model::UpdateProvisioningConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ProvisioningConfig>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ProvisioningConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::rename_network].
    fn rename_network(
        &self,
        _req: crate::model::RenameNetworkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Network>> + Send {
        std::future::ready::<crate::Result<crate::model::Network>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::list_os_images].
    fn list_os_images(
        &self,
        _req: crate::model::ListOSImagesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListOSImagesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListOSImagesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::ListLocationsResponse>> + Send
    {
        std::future::ready::<crate::Result<location::model::ListLocationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::BareMetalSolution::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::BareMetalSolution::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Returns the polling policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        Arc::new(gax::polling_policy::Aip194Strict)
    }

    /// Returns the polling backoff policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_backoff_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        Arc::new(gax::exponential_backoff::ExponentialBackoff::default())
    }
}
