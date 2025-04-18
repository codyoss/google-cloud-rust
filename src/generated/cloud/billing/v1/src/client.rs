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

/// Implements a client for the Cloud Billing API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_billing_v1::client::CloudBilling;
/// let client = CloudBilling::builder().build().await?;
/// // use `client` to make requests to the Cloud Billing API.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Retrieves the Google Cloud Console billing accounts and associates them with
/// projects.
///
/// # Configuration
///
/// To configure `CloudBilling` use the `with_*` methods in the type returned
/// by [builder()][CloudBilling::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://cloudbilling.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::cloud_billing::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::cloud_billing::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `CloudBilling` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `CloudBilling` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct CloudBilling {
    inner: Arc<dyn super::stub::dynamic::CloudBilling>,
}

impl CloudBilling {
    /// Returns a builder for [CloudBilling].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_billing_v1::client::CloudBilling;
    /// let client = CloudBilling::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::cloud_billing::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::cloud_billing::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::CloudBilling + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::CloudBilling>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::CloudBilling> {
        super::transport::CloudBilling::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::CloudBilling> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::CloudBilling::new)
    }

    /// Gets information about a billing account. The current authenticated user
    /// must be a [viewer of the billing
    /// account](https://cloud.google.com/billing/docs/how-to/billing-access).
    pub fn get_billing_account(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_billing::GetBillingAccount {
        super::builder::cloud_billing::GetBillingAccount::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists the billing accounts that the current authenticated user has
    /// permission to
    /// [view](https://cloud.google.com/billing/docs/how-to/billing-access).
    pub fn list_billing_accounts(&self) -> super::builder::cloud_billing::ListBillingAccounts {
        super::builder::cloud_billing::ListBillingAccounts::new(self.inner.clone())
    }

    /// Updates a billing account's fields.
    /// Currently the only field that can be edited is `display_name`.
    /// The current authenticated user must have the `billing.accounts.update`
    /// IAM permission, which is typically given to the
    /// [administrator](https://cloud.google.com/billing/docs/how-to/billing-access)
    /// of the billing account.
    pub fn update_billing_account(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_billing::UpdateBillingAccount {
        super::builder::cloud_billing::UpdateBillingAccount::new(self.inner.clone())
            .set_name(name.into())
    }

    /// This method creates [billing
    /// subaccounts](https://cloud.google.com/billing/docs/concepts#subaccounts).
    ///
    /// Google Cloud resellers should use the
    /// Channel Services APIs,
    /// [accounts.customers.create](https://cloud.google.com/channel/docs/reference/rest/v1/accounts.customers/create)
    /// and
    /// [accounts.customers.entitlements.create](https://cloud.google.com/channel/docs/reference/rest/v1/accounts.customers.entitlements/create).
    ///
    /// When creating a subaccount, the current authenticated user must have the
    /// `billing.accounts.update` IAM permission on the parent account, which is
    /// typically given to billing account
    /// [administrators](https://cloud.google.com/billing/docs/how-to/billing-access).
    /// This method will return an error if the parent account has not been
    /// provisioned for subaccounts.
    pub fn create_billing_account(&self) -> super::builder::cloud_billing::CreateBillingAccount {
        super::builder::cloud_billing::CreateBillingAccount::new(self.inner.clone())
    }

    /// Lists the projects associated with a billing account. The current
    /// authenticated user must have the `billing.resourceAssociations.list` IAM
    /// permission, which is often given to billing account
    /// [viewers](https://cloud.google.com/billing/docs/how-to/billing-access).
    pub fn list_project_billing_info(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_billing::ListProjectBillingInfo {
        super::builder::cloud_billing::ListProjectBillingInfo::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets the billing information for a project. The current authenticated user
    /// must have the `resourcemanager.projects.get` permission for the project,
    /// which can be granted by assigning the [Project
    /// Viewer](https://cloud.google.com/iam/docs/understanding-roles#predefined_roles)
    /// role.
    pub fn get_project_billing_info(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_billing::GetProjectBillingInfo {
        super::builder::cloud_billing::GetProjectBillingInfo::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Sets or updates the billing account associated with a project. You specify
    /// the new billing account by setting the `billing_account_name` in the
    /// `ProjectBillingInfo` resource to the resource name of a billing account.
    /// Associating a project with an open billing account enables billing on the
    /// project and allows charges for resource usage. If the project already had a
    /// billing account, this method changes the billing account used for resource
    /// usage charges.
    ///
    /// *Note:* Incurred charges that have not yet been reported in the transaction
    /// history of the Google Cloud Console might be billed to the new billing
    /// account, even if the charge occurred before the new billing account was
    /// assigned to the project.
    ///
    /// The current authenticated user must have ownership privileges for both
    /// the
    /// [project](<https://cloud.google.com/docs/permissions-overview#h.bgs0oxofvnoo>
    /// ) and the [billing
    /// account](https://cloud.google.com/billing/docs/how-to/billing-access).
    ///
    /// You can disable billing on the project by setting the
    /// `billing_account_name` field to empty. This action disassociates the
    /// current billing account from the project. Any billable activity of your
    /// in-use services will stop, and your application could stop functioning as
    /// expected. Any unbilled charges to date will be billed to the previously
    /// associated account. The current authenticated user must be either an owner
    /// of the project or an owner of the billing account for the project.
    ///
    /// Note that associating a project with a *closed* billing account will have
    /// much the same effect as disabling billing on the project: any paid
    /// resources used by the project will be shut down. Thus, unless you wish to
    /// disable billing, you should always call this method with the name of an
    /// *open* billing account.
    pub fn update_project_billing_info(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_billing::UpdateProjectBillingInfo {
        super::builder::cloud_billing::UpdateProjectBillingInfo::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets the access control policy for a billing account.
    /// The caller must have the `billing.accounts.getIamPolicy` permission on the
    /// account, which is often given to billing account
    /// [viewers](https://cloud.google.com/billing/docs/how-to/billing-access).
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::cloud_billing::GetIamPolicy {
        super::builder::cloud_billing::GetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Sets the access control policy for a billing account. Replaces any existing
    /// policy.
    /// The caller must have the `billing.accounts.setIamPolicy` permission on the
    /// account, which is often given to billing account
    /// [administrators](https://cloud.google.com/billing/docs/how-to/billing-access).
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::cloud_billing::SetIamPolicy {
        super::builder::cloud_billing::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Tests the access control policy for a billing account. This method takes
    /// the resource and a set of permissions as input and returns the subset of
    /// the input permissions that the caller is allowed for that resource.
    pub fn test_iam_permissions(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::cloud_billing::TestIamPermissions {
        super::builder::cloud_billing::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Changes which parent organization a billing account belongs to.
    pub fn move_billing_account(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_billing::MoveBillingAccount {
        super::builder::cloud_billing::MoveBillingAccount::new(self.inner.clone())
            .set_name(name.into())
    }
}

/// Implements a client for the Cloud Billing API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_billing_v1::client::CloudCatalog;
/// let client = CloudCatalog::builder().build().await?;
/// // use `client` to make requests to the Cloud Billing API.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// A catalog of Google Cloud Platform services and SKUs.
/// Provides pricing information and metadata on Google Cloud Platform services
/// and SKUs.
///
/// # Configuration
///
/// To configure `CloudCatalog` use the `with_*` methods in the type returned
/// by [builder()][CloudCatalog::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://cloudbilling.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::cloud_catalog::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::cloud_catalog::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `CloudCatalog` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `CloudCatalog` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct CloudCatalog {
    inner: Arc<dyn super::stub::dynamic::CloudCatalog>,
}

impl CloudCatalog {
    /// Returns a builder for [CloudCatalog].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_billing_v1::client::CloudCatalog;
    /// let client = CloudCatalog::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::cloud_catalog::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::cloud_catalog::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::CloudCatalog + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::CloudCatalog>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::CloudCatalog> {
        super::transport::CloudCatalog::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::CloudCatalog> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::CloudCatalog::new)
    }

    /// Lists all public cloud services.
    pub fn list_services(&self) -> super::builder::cloud_catalog::ListServices {
        super::builder::cloud_catalog::ListServices::new(self.inner.clone())
    }

    /// Lists all publicly available SKUs for a given cloud service.
    pub fn list_skus(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::cloud_catalog::ListSkus {
        super::builder::cloud_catalog::ListSkus::new(self.inner.clone()).set_parent(parent.into())
    }
}
