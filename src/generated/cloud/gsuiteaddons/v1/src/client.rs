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

/// Implements a client for the Google Workspace add-ons API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_gsuiteaddons_v1::client::GSuiteAddOns;
/// let client = GSuiteAddOns::builder().build().await?;
/// // use `client` to make requests to the Google Workspace add-ons API.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// A service for managing Google Workspace add-ons deployments.
///
/// A Google Workspace add-on is a third-party embedded component that can be
/// installed in Google Workspace Applications like Gmail, Calendar, Drive, and
/// the Google Docs, Sheets, and Slides editors. Google Workspace add-ons can
/// display UI cards, receive contextual information from the host application,
/// and perform actions in the host application (See:
/// <https://developers.google.com/gsuite/add-ons/overview> for more information).
///
/// A Google Workspace add-on deployment resource specifies metadata about the
/// add-on, including a specification of the entry points in the host application
/// that trigger add-on executions (see:
/// <https://developers.google.com/gsuite/add-ons/concepts/gsuite-manifests>).
/// Add-on deployments defined via the Google Workspace add-ons API define their
/// entrypoints using HTTPS URLs (See:
/// <https://developers.google.com/gsuite/add-ons/guides/alternate-runtimes>),
///
/// A Google Workspace add-on deployment can be installed in developer mode,
/// which allows an add-on developer to test the experience an end-user would see
/// when installing and running the add-on in their G Suite applications.  When
/// running in developer mode, more detailed error messages are exposed in the
/// add-on UI to aid in debugging.
///
/// A Google Workspace add-on deployment can be published to Google Workspace
/// Marketplace, which allows other Google Workspace users to discover and
/// install the add-on.  See:
/// <https://developers.google.com/gsuite/add-ons/how-tos/publish-add-on-overview>
/// for details.
///
/// # Configuration
///
/// To configure `GSuiteAddOns` use the `with_*` methods in the type returned
/// by [builder()][GSuiteAddOns::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://gsuiteaddons.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::g_suite_add_ons::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::g_suite_add_ons::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `GSuiteAddOns` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `GSuiteAddOns` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct GSuiteAddOns {
    inner: Arc<dyn super::stub::dynamic::GSuiteAddOns>,
}

impl GSuiteAddOns {
    /// Returns a builder for [GSuiteAddOns].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_gsuiteaddons_v1::client::GSuiteAddOns;
    /// let client = GSuiteAddOns::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::g_suite_add_ons::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::g_suite_add_ons::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::GSuiteAddOns + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::GSuiteAddOns>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::GSuiteAddOns> {
        super::transport::GSuiteAddOns::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::GSuiteAddOns> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::GSuiteAddOns::new)
    }

    /// Gets the authorization information for deployments in a given project.
    pub fn get_authorization(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::g_suite_add_ons::GetAuthorization {
        super::builder::g_suite_add_ons::GetAuthorization::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a deployment with the specified name and configuration.
    pub fn create_deployment(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::g_suite_add_ons::CreateDeployment {
        super::builder::g_suite_add_ons::CreateDeployment::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates or replaces a deployment with the specified name.
    pub fn replace_deployment(
        &self,
        deployment: impl Into<crate::model::Deployment>,
    ) -> super::builder::g_suite_add_ons::ReplaceDeployment {
        super::builder::g_suite_add_ons::ReplaceDeployment::new(self.inner.clone())
            .set_deployment(deployment.into())
    }

    /// Gets the deployment with the specified name.
    pub fn get_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::g_suite_add_ons::GetDeployment {
        super::builder::g_suite_add_ons::GetDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists all deployments in a particular project.
    pub fn list_deployments(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::g_suite_add_ons::ListDeployments {
        super::builder::g_suite_add_ons::ListDeployments::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes the deployment with the given name.
    pub fn delete_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::g_suite_add_ons::DeleteDeployment {
        super::builder::g_suite_add_ons::DeleteDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Installs a deployment in developer mode.
    /// See:
    /// <https://developers.google.com/gsuite/add-ons/how-tos/testing-gsuite-addons>.
    pub fn install_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::g_suite_add_ons::InstallDeployment {
        super::builder::g_suite_add_ons::InstallDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Uninstalls a developer mode deployment.
    /// See:
    /// <https://developers.google.com/gsuite/add-ons/how-tos/testing-gsuite-addons>.
    pub fn uninstall_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::g_suite_add_ons::UninstallDeployment {
        super::builder::g_suite_add_ons::UninstallDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Fetches the install status of a developer mode deployment.
    pub fn get_install_status(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::g_suite_add_ons::GetInstallStatus {
        super::builder::g_suite_add_ons::GetInstallStatus::new(self.inner.clone())
            .set_name(name.into())
    }
}
