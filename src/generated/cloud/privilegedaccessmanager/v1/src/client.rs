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

/// Implements a client for the Privileged Access Manager API.
///
/// # Service Description
///
/// This API allows customers to manage temporary, request based privileged
/// access to their resources.
///
/// It defines the following resource model:
///
/// * A collection of `Entitlement` resources. An entitlement allows configuring
///   (among other things):
///
///   * Some kind of privileged access that users can request.
///   * A set of users called _requesters_ who can request this access.
///   * A maximum duration for which the access can be requested.
///   * An optional approval workflow which must be satisfied before access is
///     granted.
/// * A collection of `Grant` resources. A grant is a request by a requester to
///   get the privileged access specified in an entitlement for some duration.
///
/// * After the approval workflow as specified in the entitlement is satisfied,
///   the specified access is given to the requester. The access is automatically
///   taken back after the requested duration is over.
///
///
/// # Configuration
///
/// `PrivilegedAccessManager` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `PrivilegedAccessManager` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `PrivilegedAccessManager` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct PrivilegedAccessManager {
    inner: Arc<dyn super::stubs::dynamic::PrivilegedAccessManager>,
}

impl PrivilegedAccessManager {
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
        T: super::stubs::PrivilegedAccessManager + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stubs::dynamic::PrivilegedAccessManager>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::PrivilegedAccessManager> {
        super::transport::PrivilegedAccessManager::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::PrivilegedAccessManager> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::PrivilegedAccessManager::new)
    }

    /// `CheckOnboardingStatus` reports the onboarding status for a
    /// project/folder/organization. Any findings reported by this API need to be
    /// fixed before PAM can be used on the resource.
    pub fn check_onboarding_status(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::CheckOnboardingStatus {
        super::builders::privileged_access_manager::CheckOnboardingStatus::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists entitlements in a given project/folder/organization and location.
    pub fn list_entitlements(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::ListEntitlements {
        super::builders::privileged_access_manager::ListEntitlements::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// `SearchEntitlements` returns entitlements on which the caller has the
    /// specified access.
    pub fn search_entitlements(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::SearchEntitlements {
        super::builders::privileged_access_manager::SearchEntitlements::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single entitlement.
    pub fn get_entitlement(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::GetEntitlement {
        super::builders::privileged_access_manager::GetEntitlement::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new entitlement in a given project/folder/organization and
    /// location.
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
    pub fn create_entitlement(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::CreateEntitlement {
        super::builders::privileged_access_manager::CreateEntitlement::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a single entitlement. This method can only be called when there
    /// are no in-progress (`ACTIVE`/`ACTIVATING`/`REVOKING`) grants under the
    /// entitlement.
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
    pub fn delete_entitlement(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::DeleteEntitlement {
        super::builders::privileged_access_manager::DeleteEntitlement::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates the entitlement specified in the request. Updated fields in the
    /// entitlement need to be specified in an update mask. The changes made to an
    /// entitlement are applicable only on future grants of the entitlement.
    /// However, if new approvers are added or existing approvers are removed from
    /// the approval workflow, the changes are effective on existing grants.
    ///
    /// The following fields are not supported for updates:
    ///
    /// * All immutable fields
    /// * Entitlement name
    /// * Resource name
    /// * Resource type
    /// * Adding an approval workflow in an entitlement which previously had no
    ///   approval workflow.
    /// * Deleting the approval workflow from an entitlement.
    /// * Adding or deleting a step in the approval workflow (only one step is
    ///   supported)
    ///
    /// Note that updates are allowed on the list of approvers in an approval
    /// workflow step.
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
    pub fn update_entitlement(
        &self,
        entitlement: impl Into<crate::model::Entitlement>,
    ) -> super::builders::privileged_access_manager::UpdateEntitlement {
        super::builders::privileged_access_manager::UpdateEntitlement::new(self.inner.clone())
            .set_entitlement(entitlement.into())
    }

    /// Lists grants for a given entitlement.
    pub fn list_grants(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::ListGrants {
        super::builders::privileged_access_manager::ListGrants::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// `SearchGrants` returns grants that are related to the calling user in the
    /// specified way.
    pub fn search_grants(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::SearchGrants {
        super::builders::privileged_access_manager::SearchGrants::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Get details of a single grant.
    pub fn get_grant(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::GetGrant {
        super::builders::privileged_access_manager::GetGrant::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new grant in a given project/folder/organization and
    /// location.
    pub fn create_grant(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::CreateGrant {
        super::builders::privileged_access_manager::CreateGrant::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// `ApproveGrant` is used to approve a grant. This method can only be called
    /// on a grant when it's in the `APPROVAL_AWAITED` state. This operation can't
    /// be undone.
    pub fn approve_grant(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::ApproveGrant {
        super::builders::privileged_access_manager::ApproveGrant::new(self.inner.clone())
            .set_name(name.into())
    }

    /// `DenyGrant` is used to deny a grant. This method can only be called on a
    /// grant when it's in the `APPROVAL_AWAITED` state. This operation can't be
    /// undone.
    pub fn deny_grant(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::DenyGrant {
        super::builders::privileged_access_manager::DenyGrant::new(self.inner.clone())
            .set_name(name.into())
    }

    /// `RevokeGrant` is used to immediately revoke access for a grant. This method
    /// can be called when the grant is in a non-terminal state.
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
    pub fn revoke_grant(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::RevokeGrant {
        super::builders::privileged_access_manager::RevokeGrant::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::ListLocations {
        super::builders::privileged_access_manager::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::GetLocation {
        super::builders::privileged_access_manager::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::ListOperations {
        super::builders::privileged_access_manager::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::GetOperation {
        super::builders::privileged_access_manager::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::privileged_access_manager::DeleteOperation {
        super::builders::privileged_access_manager::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
