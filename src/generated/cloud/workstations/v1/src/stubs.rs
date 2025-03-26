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

/// Defines the trait used to implement [super::client::Workstations].
///
/// Application developers may need to implement this trait to mock
/// `client::Workstations`.  In other use-cases, application developers only
/// use `client::Workstations` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait Workstations: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::Workstations::get_workstation_cluster].
    fn get_workstation_cluster(
        &self,
        _req: crate::model::GetWorkstationClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::WorkstationCluster>> + Send
    {
        std::future::ready::<crate::Result<crate::model::WorkstationCluster>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Workstations::list_workstation_clusters].
    fn list_workstation_clusters(
        &self,
        _req: crate::model::ListWorkstationClustersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListWorkstationClustersResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListWorkstationClustersResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Workstations::create_workstation_cluster].
    fn create_workstation_cluster(
        &self,
        _req: crate::model::CreateWorkstationClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Workstations::update_workstation_cluster].
    fn update_workstation_cluster(
        &self,
        _req: crate::model::UpdateWorkstationClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Workstations::delete_workstation_cluster].
    fn delete_workstation_cluster(
        &self,
        _req: crate::model::DeleteWorkstationClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Workstations::get_workstation_config].
    fn get_workstation_config(
        &self,
        _req: crate::model::GetWorkstationConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::WorkstationConfig>> + Send
    {
        std::future::ready::<crate::Result<crate::model::WorkstationConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Workstations::list_workstation_configs].
    fn list_workstation_configs(
        &self,
        _req: crate::model::ListWorkstationConfigsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListWorkstationConfigsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListWorkstationConfigsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Workstations::list_usable_workstation_configs].
    fn list_usable_workstation_configs(
        &self,
        _req: crate::model::ListUsableWorkstationConfigsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListUsableWorkstationConfigsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListUsableWorkstationConfigsResponse>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Workstations::create_workstation_config].
    fn create_workstation_config(
        &self,
        _req: crate::model::CreateWorkstationConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Workstations::update_workstation_config].
    fn update_workstation_config(
        &self,
        _req: crate::model::UpdateWorkstationConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Workstations::delete_workstation_config].
    fn delete_workstation_config(
        &self,
        _req: crate::model::DeleteWorkstationConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Workstations::get_workstation].
    fn get_workstation(
        &self,
        _req: crate::model::GetWorkstationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Workstation>> + Send {
        std::future::ready::<crate::Result<crate::model::Workstation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Workstations::list_workstations].
    fn list_workstations(
        &self,
        _req: crate::model::ListWorkstationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListWorkstationsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListWorkstationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Workstations::list_usable_workstations].
    fn list_usable_workstations(
        &self,
        _req: crate::model::ListUsableWorkstationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListUsableWorkstationsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListUsableWorkstationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Workstations::create_workstation].
    fn create_workstation(
        &self,
        _req: crate::model::CreateWorkstationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Workstations::update_workstation].
    fn update_workstation(
        &self,
        _req: crate::model::UpdateWorkstationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Workstations::delete_workstation].
    fn delete_workstation(
        &self,
        _req: crate::model::DeleteWorkstationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Workstations::start_workstation].
    fn start_workstation(
        &self,
        _req: crate::model::StartWorkstationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Workstations::stop_workstation].
    fn stop_workstation(
        &self,
        _req: crate::model::StopWorkstationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Workstations::generate_access_token].
    fn generate_access_token(
        &self,
        _req: crate::model::GenerateAccessTokenRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::GenerateAccessTokenResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::GenerateAccessTokenResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Workstations::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Workstations::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Workstations::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::TestIamPermissionsResponse>> + Send
    {
        std::future::ready::<crate::Result<iam_v1::model::TestIamPermissionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Workstations::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
    + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Workstations::get_operation].
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

    /// Implements [super::client::Workstations::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Workstations::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Returns the polling error policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_error_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        Arc::new(gax::polling_error_policy::Aip194Strict)
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
