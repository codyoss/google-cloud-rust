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

/// Implements a client for the Essential Contacts API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_essentialcontacts_v1::client::EssentialContactsService;
/// let client = EssentialContactsService::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Manages contacts for important Google Cloud notifications.
///
/// # Configuration
///
/// To configure `EssentialContactsService` use the `with_*` methods in the type returned
/// by [builder()][EssentialContactsService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://essentialcontacts.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::essential_contacts_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::essential_contacts_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `EssentialContactsService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `EssentialContactsService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct EssentialContactsService {
    inner: Arc<dyn super::stub::dynamic::EssentialContactsService>,
}

impl EssentialContactsService {
    /// Returns a builder for [EssentialContactsService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_essentialcontacts_v1::client::EssentialContactsService;
    /// let client = EssentialContactsService::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::essential_contacts_service::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::essential_contacts_service::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::EssentialContactsService + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::EssentialContactsService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::EssentialContactsService> {
        super::transport::EssentialContactsService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::EssentialContactsService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::EssentialContactsService::new)
    }

    /// Adds a new contact for a resource.
    pub fn create_contact(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::essential_contacts_service::CreateContact {
        super::builder::essential_contacts_service::CreateContact::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a contact.
    /// Note: A contact's email address cannot be changed.
    pub fn update_contact(
        &self,
        contact: impl Into<crate::model::Contact>,
    ) -> super::builder::essential_contacts_service::UpdateContact {
        super::builder::essential_contacts_service::UpdateContact::new(self.inner.clone())
            .set_contact(contact.into())
    }

    /// Lists the contacts that have been set on a resource.
    pub fn list_contacts(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::essential_contacts_service::ListContacts {
        super::builder::essential_contacts_service::ListContacts::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a single contact.
    pub fn get_contact(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::essential_contacts_service::GetContact {
        super::builder::essential_contacts_service::GetContact::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes a contact.
    pub fn delete_contact(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::essential_contacts_service::DeleteContact {
        super::builder::essential_contacts_service::DeleteContact::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists all contacts for the resource that are subscribed to the
    /// specified notification categories, including contacts inherited from
    /// any parent resources.
    pub fn compute_contacts(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::essential_contacts_service::ComputeContacts {
        super::builder::essential_contacts_service::ComputeContacts::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Allows a contact admin to send a test message to contact to verify that it
    /// has been configured correctly.
    pub fn send_test_message(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::essential_contacts_service::SendTestMessage {
        super::builder::essential_contacts_service::SendTestMessage::new(self.inner.clone())
            .set_resource(resource.into())
    }
}
