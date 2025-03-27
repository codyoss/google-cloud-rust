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

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::SqlBackupRunsService].
///
/// Application developers may need to implement this trait to mock
/// `client::SqlBackupRunsService`.  In other use-cases, application developers only
/// use `client::SqlBackupRunsService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait SqlBackupRunsService: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::SqlBackupRunsService::delete].
    fn delete(
        &self,
        _req: crate::model::SqlBackupRunsDeleteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlBackupRunsService::get].
    fn get(
        &self,
        _req: crate::model::SqlBackupRunsGetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::BackupRun>> + Send {
        std::future::ready::<crate::Result<crate::model::BackupRun>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlBackupRunsService::insert].
    fn insert(
        &self,
        _req: crate::model::SqlBackupRunsInsertRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlBackupRunsService::list].
    fn list(
        &self,
        _req: crate::model::SqlBackupRunsListRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::BackupRunsListResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::BackupRunsListResponse>>(Err(Error::other("unimplemented")))
    }
}

/// Defines the trait used to implement [super::client::SqlConnectService].
///
/// Application developers may need to implement this trait to mock
/// `client::SqlConnectService`.  In other use-cases, application developers only
/// use `client::SqlConnectService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait SqlConnectService: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::SqlConnectService::get_connect_settings].
    fn get_connect_settings(
        &self,
        _req: crate::model::GetConnectSettingsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ConnectSettings>> + Send {
        std::future::ready::<crate::Result<crate::model::ConnectSettings>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlConnectService::generate_ephemeral_cert].
    fn generate_ephemeral_cert(
        &self,
        _req: crate::model::GenerateEphemeralCertRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::GenerateEphemeralCertResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::GenerateEphemeralCertResponse>>(Err(Error::other("unimplemented")))
    }
}

/// Defines the trait used to implement [super::client::SqlDatabasesService].
///
/// Application developers may need to implement this trait to mock
/// `client::SqlDatabasesService`.  In other use-cases, application developers only
/// use `client::SqlDatabasesService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait SqlDatabasesService: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::SqlDatabasesService::delete].
    fn delete(
        &self,
        _req: crate::model::SqlDatabasesDeleteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlDatabasesService::get].
    fn get(
        &self,
        _req: crate::model::SqlDatabasesGetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Database>> + Send {
        std::future::ready::<crate::Result<crate::model::Database>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlDatabasesService::insert].
    fn insert(
        &self,
        _req: crate::model::SqlDatabasesInsertRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlDatabasesService::list].
    fn list(
        &self,
        _req: crate::model::SqlDatabasesListRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::DatabasesListResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::DatabasesListResponse>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlDatabasesService::patch].
    fn patch(
        &self,
        _req: crate::model::SqlDatabasesUpdateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlDatabasesService::update].
    fn update(
        &self,
        _req: crate::model::SqlDatabasesUpdateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }
}

/// Defines the trait used to implement [super::client::SqlFlagsService].
///
/// Application developers may need to implement this trait to mock
/// `client::SqlFlagsService`.  In other use-cases, application developers only
/// use `client::SqlFlagsService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait SqlFlagsService: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::SqlFlagsService::list].
    fn list(
        &self,
        _req: crate::model::SqlFlagsListRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::FlagsListResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::FlagsListResponse>>(Err(Error::other("unimplemented")))
    }
}

/// Defines the trait used to implement [super::client::SqlInstancesService].
///
/// Application developers may need to implement this trait to mock
/// `client::SqlInstancesService`.  In other use-cases, application developers only
/// use `client::SqlInstancesService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait SqlInstancesService: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::SqlInstancesService::add_server_ca].
    fn add_server_ca(
        &self,
        _req: crate::model::SqlInstancesAddServerCaRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::clone].
    fn clone(
        &self,
        _req: crate::model::SqlInstancesCloneRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::delete].
    fn delete(
        &self,
        _req: crate::model::SqlInstancesDeleteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::demote_master].
    fn demote_master(
        &self,
        _req: crate::model::SqlInstancesDemoteMasterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::demote].
    fn demote(
        &self,
        _req: crate::model::SqlInstancesDemoteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::export].
    fn export(
        &self,
        _req: crate::model::SqlInstancesExportRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::failover].
    fn failover(
        &self,
        _req: crate::model::SqlInstancesFailoverRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::reencrypt].
    fn reencrypt(
        &self,
        _req: crate::model::SqlInstancesReencryptRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::get].
    fn get(
        &self,
        _req: crate::model::SqlInstancesGetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::DatabaseInstance>> + Send {
        std::future::ready::<crate::Result<crate::model::DatabaseInstance>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::import].
    fn import(
        &self,
        _req: crate::model::SqlInstancesImportRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::insert].
    fn insert(
        &self,
        _req: crate::model::SqlInstancesInsertRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::list].
    fn list(
        &self,
        _req: crate::model::SqlInstancesListRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::InstancesListResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::InstancesListResponse>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::list_server_cas].
    fn list_server_cas(
        &self,
        _req: crate::model::SqlInstancesListServerCasRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::InstancesListServerCasResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::InstancesListServerCasResponse>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::patch].
    fn patch(
        &self,
        _req: crate::model::SqlInstancesPatchRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::promote_replica].
    fn promote_replica(
        &self,
        _req: crate::model::SqlInstancesPromoteReplicaRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::switchover].
    fn switchover(
        &self,
        _req: crate::model::SqlInstancesSwitchoverRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::reset_ssl_config].
    fn reset_ssl_config(
        &self,
        _req: crate::model::SqlInstancesResetSslConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::restart].
    fn restart(
        &self,
        _req: crate::model::SqlInstancesRestartRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::restore_backup].
    fn restore_backup(
        &self,
        _req: crate::model::SqlInstancesRestoreBackupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::rotate_server_ca].
    fn rotate_server_ca(
        &self,
        _req: crate::model::SqlInstancesRotateServerCaRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::start_replica].
    fn start_replica(
        &self,
        _req: crate::model::SqlInstancesStartReplicaRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::stop_replica].
    fn stop_replica(
        &self,
        _req: crate::model::SqlInstancesStopReplicaRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::truncate_log].
    fn truncate_log(
        &self,
        _req: crate::model::SqlInstancesTruncateLogRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::update].
    fn update(
        &self,
        _req: crate::model::SqlInstancesUpdateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::create_ephemeral].
    fn create_ephemeral(
        &self,
        _req: crate::model::SqlInstancesCreateEphemeralCertRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SslCert>> + Send {
        std::future::ready::<crate::Result<crate::model::SslCert>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::reschedule_maintenance].
    fn reschedule_maintenance(
        &self,
        _req: crate::model::SqlInstancesRescheduleMaintenanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::verify_external_sync_settings].
    fn verify_external_sync_settings(
        &self,
        _req: crate::model::SqlInstancesVerifyExternalSyncSettingsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SqlInstancesVerifyExternalSyncSettingsResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::SqlInstancesVerifyExternalSyncSettingsResponse>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::start_external_sync].
    fn start_external_sync(
        &self,
        _req: crate::model::SqlInstancesStartExternalSyncRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::perform_disk_shrink].
    fn perform_disk_shrink(
        &self,
        _req: crate::model::SqlInstancesPerformDiskShrinkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::get_disk_shrink_config].
    fn get_disk_shrink_config(
        &self,
        _req: crate::model::SqlInstancesGetDiskShrinkConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SqlInstancesGetDiskShrinkConfigResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::SqlInstancesGetDiskShrinkConfigResponse>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::reset_replica_size].
    fn reset_replica_size(
        &self,
        _req: crate::model::SqlInstancesResetReplicaSizeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::get_latest_recovery_time].
    fn get_latest_recovery_time(
        &self,
        _req: crate::model::SqlInstancesGetLatestRecoveryTimeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SqlInstancesGetLatestRecoveryTimeResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::SqlInstancesGetLatestRecoveryTimeResponse>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::acquire_ssrs_lease].
    fn acquire_ssrs_lease(
        &self,
        _req: crate::model::SqlInstancesAcquireSsrsLeaseRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SqlInstancesAcquireSsrsLeaseResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::SqlInstancesAcquireSsrsLeaseResponse>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlInstancesService::release_ssrs_lease].
    fn release_ssrs_lease(
        &self,
        _req: crate::model::SqlInstancesReleaseSsrsLeaseRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SqlInstancesReleaseSsrsLeaseResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::SqlInstancesReleaseSsrsLeaseResponse>>(Err(Error::other("unimplemented")))
    }
}

/// Defines the trait used to implement [super::client::SqlOperationsService].
///
/// Application developers may need to implement this trait to mock
/// `client::SqlOperationsService`.  In other use-cases, application developers only
/// use `client::SqlOperationsService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait SqlOperationsService: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::SqlOperationsService::get].
    fn get(
        &self,
        _req: crate::model::SqlOperationsGetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlOperationsService::list].
    fn list(
        &self,
        _req: crate::model::SqlOperationsListRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::OperationsListResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::OperationsListResponse>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlOperationsService::cancel].
    fn cancel(
        &self,
        _req: crate::model::SqlOperationsCancelRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }
}

/// Defines the trait used to implement [super::client::SqlSslCertsService].
///
/// Application developers may need to implement this trait to mock
/// `client::SqlSslCertsService`.  In other use-cases, application developers only
/// use `client::SqlSslCertsService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait SqlSslCertsService: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::SqlSslCertsService::delete].
    fn delete(
        &self,
        _req: crate::model::SqlSslCertsDeleteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlSslCertsService::get].
    fn get(
        &self,
        _req: crate::model::SqlSslCertsGetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SslCert>> + Send {
        std::future::ready::<crate::Result<crate::model::SslCert>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlSslCertsService::insert].
    fn insert(
        &self,
        _req: crate::model::SqlSslCertsInsertRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SslCertsInsertResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::SslCertsInsertResponse>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlSslCertsService::list].
    fn list(
        &self,
        _req: crate::model::SqlSslCertsListRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SslCertsListResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::SslCertsListResponse>>(Err(Error::other("unimplemented")))
    }
}

/// Defines the trait used to implement [super::client::SqlTiersService].
///
/// Application developers may need to implement this trait to mock
/// `client::SqlTiersService`.  In other use-cases, application developers only
/// use `client::SqlTiersService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait SqlTiersService: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::SqlTiersService::list].
    fn list(
        &self,
        _req: crate::model::SqlTiersListRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TiersListResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::TiersListResponse>>(Err(Error::other("unimplemented")))
    }
}

/// Defines the trait used to implement [super::client::SqlUsersService].
///
/// Application developers may need to implement this trait to mock
/// `client::SqlUsersService`.  In other use-cases, application developers only
/// use `client::SqlUsersService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait SqlUsersService: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::SqlUsersService::delete].
    fn delete(
        &self,
        _req: crate::model::SqlUsersDeleteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlUsersService::get].
    fn get(
        &self,
        _req: crate::model::SqlUsersGetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::User>> + Send {
        std::future::ready::<crate::Result<crate::model::User>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlUsersService::insert].
    fn insert(
        &self,
        _req: crate::model::SqlUsersInsertRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlUsersService::list].
    fn list(
        &self,
        _req: crate::model::SqlUsersListRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::UsersListResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::UsersListResponse>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::SqlUsersService::update].
    fn update(
        &self,
        _req: crate::model::SqlUsersUpdateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other("unimplemented")))
    }
}

