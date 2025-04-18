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

/// Implements a client for the Identity and Access Management (IAM) API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_iam_admin_v1::client::Iam;
/// let client = Iam::builder().build().await?;
/// // use `client` to make requests to the Identity and Access Management (IAM) API.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Creates and manages Identity and Access Management (IAM) resources.
///
/// You can use this service to work with all of the following resources:
///
/// * **Service accounts**, which identify an application or a virtual machine
///   (VM) instance rather than a person
/// * **Service account keys**, which service accounts use to authenticate with
///   Google APIs
/// * **IAM policies for service accounts**, which specify the roles that a
///   principal has for the service account
/// * **IAM custom roles**, which help you limit the number of permissions that
///   you grant to principals
///
/// In addition, you can use this service to complete the following tasks, among
/// others:
///
/// * Test whether a service account can use specific permissions
/// * Check which roles you can grant for a specific resource
/// * Lint, or validate, condition expressions in an IAM policy
///
/// When you read data from the IAM API, each read is eventually consistent. In
/// other words, if you write data with the IAM API, then immediately read that
/// data, the read operation might return an older version of the data. To deal
/// with this behavior, your application can retry the request with truncated
/// exponential backoff.
///
/// In contrast, writing data to the IAM API is sequentially consistent. In other
/// words, write operations are always processed in the order in which they were
/// received.
///
/// # Configuration
///
/// To configure `Iam` use the `with_*` methods in the type returned
/// by [builder()][Iam::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://iam.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::iam::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::iam::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `Iam` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Iam` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Iam {
    inner: Arc<dyn super::stub::dynamic::Iam>,
}

impl Iam {
    /// Returns a builder for [Iam].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_iam_admin_v1::client::Iam;
    /// let client = Iam::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::iam::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::iam::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::Iam + 'static,
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
        conf: gaxi::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::Iam>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> Result<impl super::stub::Iam> {
        super::transport::Iam::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::Iam> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::Iam::new)
    }

    /// Lists every [ServiceAccount][google.iam.admin.v1.ServiceAccount] that belongs to a specific project.
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    pub fn list_service_accounts(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::ListServiceAccounts {
        super::builder::iam::ListServiceAccounts::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets a [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    pub fn get_service_account(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::GetServiceAccount {
        super::builder::iam::GetServiceAccount::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    pub fn create_service_account(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::CreateServiceAccount {
        super::builder::iam::CreateServiceAccount::new(self.inner.clone()).set_name(name.into())
    }

    /// **Note:** We are in the process of deprecating this method. Use
    /// [PatchServiceAccount][google.iam.admin.v1.IAM.PatchServiceAccount] instead.
    ///
    /// Updates a [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// You can update only the `display_name` field.
    ///
    /// [google.iam.admin.v1.IAM.PatchServiceAccount]: crate::client::Iam::patch_service_account
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    pub fn update_service_account(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::UpdateServiceAccount {
        super::builder::iam::UpdateServiceAccount::new(self.inner.clone()).set_name(name.into())
    }

    /// Patches a [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    pub fn patch_service_account(
        &self,
        service_account: impl Into<crate::model::ServiceAccount>,
    ) -> super::builder::iam::PatchServiceAccount {
        super::builder::iam::PatchServiceAccount::new(self.inner.clone())
            .set_service_account(service_account.into())
    }

    /// Deletes a [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// **Warning:** After you delete a service account, you might not be able to
    /// undelete it. If you know that you need to re-enable the service account in
    /// the future, use [DisableServiceAccount][google.iam.admin.v1.IAM.DisableServiceAccount] instead.
    ///
    /// If you delete a service account, IAM permanently removes the service
    /// account 30 days later. Google Cloud cannot recover the service account
    /// after it is permanently removed, even if you file a support request.
    ///
    /// To help avoid unplanned outages, we recommend that you disable the service
    /// account before you delete it. Use [DisableServiceAccount][google.iam.admin.v1.IAM.DisableServiceAccount] to disable the
    /// service account, then wait at least 24 hours and watch for unintended
    /// consequences. If there are no unintended consequences, you can delete the
    /// service account.
    ///
    /// [google.iam.admin.v1.IAM.DisableServiceAccount]: crate::client::Iam::disable_service_account
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    pub fn delete_service_account(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::DeleteServiceAccount {
        super::builder::iam::DeleteServiceAccount::new(self.inner.clone()).set_name(name.into())
    }

    /// Restores a deleted [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// **Important:** It is not always possible to restore a deleted service
    /// account. Use this method only as a last resort.
    ///
    /// After you delete a service account, IAM permanently removes the service
    /// account 30 days later. There is no way to restore a deleted service account
    /// that has been permanently removed.
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    pub fn undelete_service_account(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::UndeleteServiceAccount {
        super::builder::iam::UndeleteServiceAccount::new(self.inner.clone()).set_name(name.into())
    }

    /// Enables a [ServiceAccount][google.iam.admin.v1.ServiceAccount] that was disabled by
    /// [DisableServiceAccount][google.iam.admin.v1.IAM.DisableServiceAccount].
    ///
    /// If the service account is already enabled, then this method has no effect.
    ///
    /// If the service account was disabled by other means—for example, if Google
    /// disabled the service account because it was compromised—you cannot use this
    /// method to enable the service account.
    ///
    /// [google.iam.admin.v1.IAM.DisableServiceAccount]: crate::client::Iam::disable_service_account
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    pub fn enable_service_account(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::EnableServiceAccount {
        super::builder::iam::EnableServiceAccount::new(self.inner.clone()).set_name(name.into())
    }

    /// Disables a [ServiceAccount][google.iam.admin.v1.ServiceAccount] immediately.
    ///
    /// If an application uses the service account to authenticate, that
    /// application can no longer call Google APIs or access Google Cloud
    /// resources. Existing access tokens for the service account are rejected, and
    /// requests for new access tokens will fail.
    ///
    /// To re-enable the service account, use [EnableServiceAccount][google.iam.admin.v1.IAM.EnableServiceAccount]. After you
    /// re-enable the service account, its existing access tokens will be accepted,
    /// and you can request new access tokens.
    ///
    /// To help avoid unplanned outages, we recommend that you disable the service
    /// account before you delete it. Use this method to disable the service
    /// account, then wait at least 24 hours and watch for unintended consequences.
    /// If there are no unintended consequences, you can delete the service account
    /// with [DeleteServiceAccount][google.iam.admin.v1.IAM.DeleteServiceAccount].
    ///
    /// [google.iam.admin.v1.IAM.DeleteServiceAccount]: crate::client::Iam::delete_service_account
    /// [google.iam.admin.v1.IAM.EnableServiceAccount]: crate::client::Iam::enable_service_account
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    pub fn disable_service_account(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::DisableServiceAccount {
        super::builder::iam::DisableServiceAccount::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists every [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey] for a service account.
    ///
    /// [google.iam.admin.v1.ServiceAccountKey]: crate::model::ServiceAccountKey
    pub fn list_service_account_keys(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::ListServiceAccountKeys {
        super::builder::iam::ListServiceAccountKeys::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey].
    ///
    /// [google.iam.admin.v1.ServiceAccountKey]: crate::model::ServiceAccountKey
    pub fn get_service_account_key(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::GetServiceAccountKey {
        super::builder::iam::GetServiceAccountKey::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey].
    ///
    /// [google.iam.admin.v1.ServiceAccountKey]: crate::model::ServiceAccountKey
    pub fn create_service_account_key(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::CreateServiceAccountKey {
        super::builder::iam::CreateServiceAccountKey::new(self.inner.clone()).set_name(name.into())
    }

    /// Uploads the public key portion of a key pair that you manage, and
    /// associates the public key with a [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// After you upload the public key, you can use the private key from the key
    /// pair as a service account key.
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    pub fn upload_service_account_key(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::UploadServiceAccountKey {
        super::builder::iam::UploadServiceAccountKey::new(self.inner.clone()).set_name(name.into())
    }

    /// Deletes a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey]. Deleting a service account key does not
    /// revoke short-lived credentials that have been issued based on the service
    /// account key.
    ///
    /// [google.iam.admin.v1.ServiceAccountKey]: crate::model::ServiceAccountKey
    pub fn delete_service_account_key(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::DeleteServiceAccountKey {
        super::builder::iam::DeleteServiceAccountKey::new(self.inner.clone()).set_name(name.into())
    }

    /// Disable a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey]. A disabled service account key can be
    /// re-enabled with [EnableServiceAccountKey][google.iam.admin.v1.IAM.EnableServiceAccountKey].
    ///
    /// [google.iam.admin.v1.IAM.EnableServiceAccountKey]: crate::client::Iam::enable_service_account_key
    /// [google.iam.admin.v1.ServiceAccountKey]: crate::model::ServiceAccountKey
    pub fn disable_service_account_key(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::DisableServiceAccountKey {
        super::builder::iam::DisableServiceAccountKey::new(self.inner.clone()).set_name(name.into())
    }

    /// Enable a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey].
    ///
    /// [google.iam.admin.v1.ServiceAccountKey]: crate::model::ServiceAccountKey
    pub fn enable_service_account_key(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::EnableServiceAccountKey {
        super::builder::iam::EnableServiceAccountKey::new(self.inner.clone()).set_name(name.into())
    }

    /// **Note:** This method is deprecated. Use the
    /// [`signBlob`](https://cloud.google.com/iam/help/rest-credentials/v1/projects.serviceAccounts/signBlob)
    /// method in the IAM Service Account Credentials API instead. If you currently
    /// use this method, see the [migration
    /// guide](https://cloud.google.com/iam/help/credentials/migrate-api) for
    /// instructions.
    ///
    /// Signs a blob using the system-managed private key for a [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    pub fn sign_blob(&self, name: impl Into<std::string::String>) -> super::builder::iam::SignBlob {
        super::builder::iam::SignBlob::new(self.inner.clone()).set_name(name.into())
    }

    /// **Note:** This method is deprecated. Use the
    /// [`signJwt`](https://cloud.google.com/iam/help/rest-credentials/v1/projects.serviceAccounts/signJwt)
    /// method in the IAM Service Account Credentials API instead. If you currently
    /// use this method, see the [migration
    /// guide](https://cloud.google.com/iam/help/credentials/migrate-api) for
    /// instructions.
    ///
    /// Signs a JSON Web Token (JWT) using the system-managed private key for a
    /// [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    pub fn sign_jwt(&self, name: impl Into<std::string::String>) -> super::builder::iam::SignJwt {
        super::builder::iam::SignJwt::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets the IAM policy that is attached to a [ServiceAccount][google.iam.admin.v1.ServiceAccount]. This IAM
    /// policy specifies which principals have access to the service account.
    ///
    /// This method does not tell you whether the service account has been granted
    /// any roles on other resources. To check whether a service account has role
    /// grants on a resource, use the `getIamPolicy` method for that resource. For
    /// example, to view the role grants for a project, call the Resource Manager
    /// API's
    /// [`projects.getIamPolicy`](https://cloud.google.com/resource-manager/reference/rest/v1/projects/getIamPolicy)
    /// method.
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::iam::GetIamPolicy {
        super::builder::iam::GetIamPolicy::new(self.inner.clone()).set_resource(resource.into())
    }

    /// Sets the IAM policy that is attached to a [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// Use this method to grant or revoke access to the service account. For
    /// example, you could grant a principal the ability to impersonate the service
    /// account.
    ///
    /// This method does not enable the service account to access other resources.
    /// To grant roles to a service account on a resource, follow these steps:
    ///
    /// . Call the resource's `getIamPolicy` method to get its current IAM policy.
    /// . Edit the policy so that it binds the service account to an IAM role for
    ///   the resource.
    /// . Call the resource's `setIamPolicy` method to update its IAM policy.
    ///
    /// For detailed instructions, see
    /// [Manage access to project, folders, and
    /// organizations](https://cloud.google.com/iam/help/service-accounts/granting-access-to-service-accounts)
    /// or [Manage access to other
    /// resources](https://cloud.google.com/iam/help/access/manage-other-resources).
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::iam::SetIamPolicy {
        super::builder::iam::SetIamPolicy::new(self.inner.clone()).set_resource(resource.into())
    }

    /// Tests whether the caller has the specified permissions on a
    /// [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    pub fn test_iam_permissions(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::iam::TestIamPermissions {
        super::builder::iam::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Lists roles that can be granted on a Google Cloud resource. A role is
    /// grantable if the IAM policy for the resource can contain bindings to the
    /// role.
    pub fn query_grantable_roles(&self) -> super::builder::iam::QueryGrantableRoles {
        super::builder::iam::QueryGrantableRoles::new(self.inner.clone())
    }

    /// Lists every predefined [Role][google.iam.admin.v1.Role] that IAM supports, or every custom role
    /// that is defined for an organization or project.
    ///
    /// [google.iam.admin.v1.Role]: crate::model::Role
    pub fn list_roles(&self) -> super::builder::iam::ListRoles {
        super::builder::iam::ListRoles::new(self.inner.clone())
    }

    /// Gets the definition of a [Role][google.iam.admin.v1.Role].
    ///
    /// [google.iam.admin.v1.Role]: crate::model::Role
    pub fn get_role(&self, name: impl Into<std::string::String>) -> super::builder::iam::GetRole {
        super::builder::iam::GetRole::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a new custom [Role][google.iam.admin.v1.Role].
    ///
    /// [google.iam.admin.v1.Role]: crate::model::Role
    pub fn create_role(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::iam::CreateRole {
        super::builder::iam::CreateRole::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Updates the definition of a custom [Role][google.iam.admin.v1.Role].
    ///
    /// [google.iam.admin.v1.Role]: crate::model::Role
    pub fn update_role(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::UpdateRole {
        super::builder::iam::UpdateRole::new(self.inner.clone()).set_name(name.into())
    }

    /// Deletes a custom [Role][google.iam.admin.v1.Role].
    ///
    /// When you delete a custom role, the following changes occur immediately:
    ///
    /// * You cannot bind a principal to the custom role in an IAM
    ///   [Policy][google.iam.v1.Policy].
    /// * Existing bindings to the custom role are not changed, but they have no
    ///   effect.
    /// * By default, the response from [ListRoles][google.iam.admin.v1.IAM.ListRoles] does not include the custom
    ///   role.
    ///
    /// You have 7 days to undelete the custom role. After 7 days, the following
    /// changes occur:
    ///
    /// * The custom role is permanently deleted and cannot be recovered.
    /// * If an IAM policy contains a binding to the custom role, the binding is
    ///   permanently removed.
    ///
    /// [google.iam.admin.v1.IAM.ListRoles]: crate::client::Iam::list_roles
    /// [google.iam.admin.v1.Role]: crate::model::Role
    /// [google.iam.v1.Policy]: iam_v1::model::Policy
    pub fn delete_role(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::DeleteRole {
        super::builder::iam::DeleteRole::new(self.inner.clone()).set_name(name.into())
    }

    /// Undeletes a custom [Role][google.iam.admin.v1.Role].
    ///
    /// [google.iam.admin.v1.Role]: crate::model::Role
    pub fn undelete_role(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::iam::UndeleteRole {
        super::builder::iam::UndeleteRole::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists every permission that you can test on a resource. A permission is
    /// testable if you can check whether a principal has that permission on the
    /// resource.
    pub fn query_testable_permissions(&self) -> super::builder::iam::QueryTestablePermissions {
        super::builder::iam::QueryTestablePermissions::new(self.inner.clone())
    }

    /// Returns a list of services that allow you to opt into audit logs that are
    /// not generated by default.
    ///
    /// To learn more about audit logs, see the [Logging
    /// documentation](https://cloud.google.com/logging/docs/audit).
    pub fn query_auditable_services(&self) -> super::builder::iam::QueryAuditableServices {
        super::builder::iam::QueryAuditableServices::new(self.inner.clone())
    }

    /// Lints, or validates, an IAM policy. Currently checks the
    /// [google.iam.v1.Binding.condition][google.iam.v1.Binding.condition] field, which contains a condition
    /// expression for a role binding.
    ///
    /// Successful calls to this method always return an HTTP `200 OK` status code,
    /// even if the linter detects an issue in the IAM policy.
    ///
    /// [google.iam.v1.Binding.condition]: iam_v1::model::Binding::condition
    pub fn lint_policy(&self) -> super::builder::iam::LintPolicy {
        super::builder::iam::LintPolicy::new(self.inner.clone())
    }
}
