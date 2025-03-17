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

/// Defines the trait used to implement [super::client::AppHub].
///
/// Application developers may need to implement this trait to mock
/// `client::AppHub`.  In other use-cases, application developers only
/// use `client::AppHub` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait AppHub: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::AppHub::lookup_service_project_attachment].
    fn lookup_service_project_attachment(
        &self,
        _req: crate::model::LookupServiceProjectAttachmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::LookupServiceProjectAttachmentResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::LookupServiceProjectAttachmentResponse>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::AppHub::list_service_project_attachments].
    fn list_service_project_attachments(
        &self,
        _req: crate::model::ListServiceProjectAttachmentsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListServiceProjectAttachmentsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListServiceProjectAttachmentsResponse>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::AppHub::create_service_project_attachment].
    fn create_service_project_attachment(
        &self,
        _req: crate::model::CreateServiceProjectAttachmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::get_service_project_attachment].
    fn get_service_project_attachment(
        &self,
        _req: crate::model::GetServiceProjectAttachmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ServiceProjectAttachment>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ServiceProjectAttachment>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::AppHub::delete_service_project_attachment].
    fn delete_service_project_attachment(
        &self,
        _req: crate::model::DeleteServiceProjectAttachmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::detach_service_project_attachment].
    fn detach_service_project_attachment(
        &self,
        _req: crate::model::DetachServiceProjectAttachmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::DetachServiceProjectAttachmentResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::DetachServiceProjectAttachmentResponse>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::AppHub::list_discovered_services].
    fn list_discovered_services(
        &self,
        _req: crate::model::ListDiscoveredServicesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListDiscoveredServicesResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::ListDiscoveredServicesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::AppHub::get_discovered_service].
    fn get_discovered_service(
        &self,
        _req: crate::model::GetDiscoveredServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::DiscoveredService>> + Send
    {
        std::future::ready::<crate::Result<crate::model::DiscoveredService>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::lookup_discovered_service].
    fn lookup_discovered_service(
        &self,
        _req: crate::model::LookupDiscoveredServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::LookupDiscoveredServiceResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::LookupDiscoveredServiceResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::AppHub::list_services].
    fn list_services(
        &self,
        _req: crate::model::ListServicesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListServicesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListServicesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::create_service].
    fn create_service(
        &self,
        _req: crate::model::CreateServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::get_service].
    fn get_service(
        &self,
        _req: crate::model::GetServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Service>> + Send {
        std::future::ready::<crate::Result<crate::model::Service>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::update_service].
    fn update_service(
        &self,
        _req: crate::model::UpdateServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::delete_service].
    fn delete_service(
        &self,
        _req: crate::model::DeleteServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::list_discovered_workloads].
    fn list_discovered_workloads(
        &self,
        _req: crate::model::ListDiscoveredWorkloadsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListDiscoveredWorkloadsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListDiscoveredWorkloadsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::AppHub::get_discovered_workload].
    fn get_discovered_workload(
        &self,
        _req: crate::model::GetDiscoveredWorkloadRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::DiscoveredWorkload>> + Send
    {
        std::future::ready::<crate::Result<crate::model::DiscoveredWorkload>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::lookup_discovered_workload].
    fn lookup_discovered_workload(
        &self,
        _req: crate::model::LookupDiscoveredWorkloadRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::LookupDiscoveredWorkloadResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::LookupDiscoveredWorkloadResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::AppHub::list_workloads].
    fn list_workloads(
        &self,
        _req: crate::model::ListWorkloadsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListWorkloadsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListWorkloadsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::create_workload].
    fn create_workload(
        &self,
        _req: crate::model::CreateWorkloadRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::get_workload].
    fn get_workload(
        &self,
        _req: crate::model::GetWorkloadRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Workload>> + Send {
        std::future::ready::<crate::Result<crate::model::Workload>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::update_workload].
    fn update_workload(
        &self,
        _req: crate::model::UpdateWorkloadRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::delete_workload].
    fn delete_workload(
        &self,
        _req: crate::model::DeleteWorkloadRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::list_applications].
    fn list_applications(
        &self,
        _req: crate::model::ListApplicationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListApplicationsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListApplicationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::AppHub::create_application].
    fn create_application(
        &self,
        _req: crate::model::CreateApplicationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::get_application].
    fn get_application(
        &self,
        _req: crate::model::GetApplicationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Application>> + Send {
        std::future::ready::<crate::Result<crate::model::Application>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::update_application].
    fn update_application(
        &self,
        _req: crate::model::UpdateApplicationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::delete_application].
    fn delete_application(
        &self,
        _req: crate::model::DeleteApplicationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::list_locations].
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

    /// Implements [super::client::AppHub::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AppHub::test_iam_permissions].
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

    /// Implements [super::client::AppHub::list_operations].
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

    /// Implements [super::client::AppHub::get_operation].
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

    /// Implements [super::client::AppHub::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::AppHub::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
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
