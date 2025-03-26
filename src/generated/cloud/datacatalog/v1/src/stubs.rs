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

/// Defines the trait used to implement [super::client::DataCatalog].
///
/// Application developers may need to implement this trait to mock
/// `client::DataCatalog`.  In other use-cases, application developers only
/// use `client::DataCatalog` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait DataCatalog: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::DataCatalog::search_catalog].
    fn search_catalog(
        &self,
        _req: crate::model::SearchCatalogRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SearchCatalogResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::SearchCatalogResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::create_entry_group].
    fn create_entry_group(
        &self,
        _req: crate::model::CreateEntryGroupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::EntryGroup>> + Send {
        std::future::ready::<crate::Result<crate::model::EntryGroup>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::get_entry_group].
    fn get_entry_group(
        &self,
        _req: crate::model::GetEntryGroupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::EntryGroup>> + Send {
        std::future::ready::<crate::Result<crate::model::EntryGroup>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::update_entry_group].
    fn update_entry_group(
        &self,
        _req: crate::model::UpdateEntryGroupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::EntryGroup>> + Send {
        std::future::ready::<crate::Result<crate::model::EntryGroup>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::delete_entry_group].
    fn delete_entry_group(
        &self,
        _req: crate::model::DeleteEntryGroupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::DataCatalog::list_entry_groups].
    fn list_entry_groups(
        &self,
        _req: crate::model::ListEntryGroupsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListEntryGroupsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListEntryGroupsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::DataCatalog::create_entry].
    fn create_entry(
        &self,
        _req: crate::model::CreateEntryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Entry>> + Send {
        std::future::ready::<crate::Result<crate::model::Entry>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::DataCatalog::update_entry].
    fn update_entry(
        &self,
        _req: crate::model::UpdateEntryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Entry>> + Send {
        std::future::ready::<crate::Result<crate::model::Entry>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::DataCatalog::delete_entry].
    fn delete_entry(
        &self,
        _req: crate::model::DeleteEntryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::DataCatalog::get_entry].
    fn get_entry(
        &self,
        _req: crate::model::GetEntryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Entry>> + Send {
        std::future::ready::<crate::Result<crate::model::Entry>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::DataCatalog::lookup_entry].
    fn lookup_entry(
        &self,
        _req: crate::model::LookupEntryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Entry>> + Send {
        std::future::ready::<crate::Result<crate::model::Entry>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::DataCatalog::list_entries].
    fn list_entries(
        &self,
        _req: crate::model::ListEntriesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListEntriesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListEntriesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::modify_entry_overview].
    fn modify_entry_overview(
        &self,
        _req: crate::model::ModifyEntryOverviewRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::EntryOverview>> + Send {
        std::future::ready::<crate::Result<crate::model::EntryOverview>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::modify_entry_contacts].
    fn modify_entry_contacts(
        &self,
        _req: crate::model::ModifyEntryContactsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Contacts>> + Send {
        std::future::ready::<crate::Result<crate::model::Contacts>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::create_tag_template].
    fn create_tag_template(
        &self,
        _req: crate::model::CreateTagTemplateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TagTemplate>> + Send {
        std::future::ready::<crate::Result<crate::model::TagTemplate>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::get_tag_template].
    fn get_tag_template(
        &self,
        _req: crate::model::GetTagTemplateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TagTemplate>> + Send {
        std::future::ready::<crate::Result<crate::model::TagTemplate>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::update_tag_template].
    fn update_tag_template(
        &self,
        _req: crate::model::UpdateTagTemplateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TagTemplate>> + Send {
        std::future::ready::<crate::Result<crate::model::TagTemplate>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::delete_tag_template].
    fn delete_tag_template(
        &self,
        _req: crate::model::DeleteTagTemplateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::DataCatalog::create_tag_template_field].
    fn create_tag_template_field(
        &self,
        _req: crate::model::CreateTagTemplateFieldRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TagTemplateField>> + Send
    {
        std::future::ready::<crate::Result<crate::model::TagTemplateField>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::update_tag_template_field].
    fn update_tag_template_field(
        &self,
        _req: crate::model::UpdateTagTemplateFieldRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TagTemplateField>> + Send
    {
        std::future::ready::<crate::Result<crate::model::TagTemplateField>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::rename_tag_template_field].
    fn rename_tag_template_field(
        &self,
        _req: crate::model::RenameTagTemplateFieldRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TagTemplateField>> + Send
    {
        std::future::ready::<crate::Result<crate::model::TagTemplateField>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::rename_tag_template_field_enum_value].
    fn rename_tag_template_field_enum_value(
        &self,
        _req: crate::model::RenameTagTemplateFieldEnumValueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TagTemplateField>> + Send
    {
        std::future::ready::<crate::Result<crate::model::TagTemplateField>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::delete_tag_template_field].
    fn delete_tag_template_field(
        &self,
        _req: crate::model::DeleteTagTemplateFieldRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::DataCatalog::create_tag].
    fn create_tag(
        &self,
        _req: crate::model::CreateTagRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Tag>> + Send {
        std::future::ready::<crate::Result<crate::model::Tag>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::DataCatalog::update_tag].
    fn update_tag(
        &self,
        _req: crate::model::UpdateTagRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Tag>> + Send {
        std::future::ready::<crate::Result<crate::model::Tag>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::DataCatalog::delete_tag].
    fn delete_tag(
        &self,
        _req: crate::model::DeleteTagRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::DataCatalog::list_tags].
    fn list_tags(
        &self,
        _req: crate::model::ListTagsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListTagsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListTagsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::reconcile_tags].
    fn reconcile_tags(
        &self,
        _req: crate::model::ReconcileTagsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::star_entry].
    fn star_entry(
        &self,
        _req: crate::model::StarEntryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::StarEntryResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::StarEntryResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::unstar_entry].
    fn unstar_entry(
        &self,
        _req: crate::model::UnstarEntryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::UnstarEntryResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::UnstarEntryResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::test_iam_permissions].
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

    /// Implements [super::client::DataCatalog::import_entries].
    fn import_entries(
        &self,
        _req: crate::model::ImportEntriesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::set_config].
    fn set_config(
        &self,
        _req: crate::model::SetConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::MigrationConfig>> + Send
    {
        std::future::ready::<crate::Result<crate::model::MigrationConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::retrieve_config].
    fn retrieve_config(
        &self,
        _req: crate::model::RetrieveConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::OrganizationConfig>> + Send
    {
        std::future::ready::<crate::Result<crate::model::OrganizationConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::retrieve_effective_config].
    fn retrieve_effective_config(
        &self,
        _req: crate::model::RetrieveEffectiveConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::MigrationConfig>> + Send
    {
        std::future::ready::<crate::Result<crate::model::MigrationConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::DataCatalog::list_operations].
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

    /// Implements [super::client::DataCatalog::get_operation].
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

    /// Implements [super::client::DataCatalog::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::DataCatalog::cancel_operation].
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

/// Defines the trait used to implement [super::client::PolicyTagManager].
///
/// Application developers may need to implement this trait to mock
/// `client::PolicyTagManager`.  In other use-cases, application developers only
/// use `client::PolicyTagManager` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait PolicyTagManager: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::PolicyTagManager::create_taxonomy].
    fn create_taxonomy(
        &self,
        _req: crate::model::CreateTaxonomyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Taxonomy>> + Send {
        std::future::ready::<crate::Result<crate::model::Taxonomy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::PolicyTagManager::delete_taxonomy].
    fn delete_taxonomy(
        &self,
        _req: crate::model::DeleteTaxonomyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::PolicyTagManager::update_taxonomy].
    fn update_taxonomy(
        &self,
        _req: crate::model::UpdateTaxonomyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Taxonomy>> + Send {
        std::future::ready::<crate::Result<crate::model::Taxonomy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::PolicyTagManager::list_taxonomies].
    fn list_taxonomies(
        &self,
        _req: crate::model::ListTaxonomiesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListTaxonomiesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListTaxonomiesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::PolicyTagManager::get_taxonomy].
    fn get_taxonomy(
        &self,
        _req: crate::model::GetTaxonomyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Taxonomy>> + Send {
        std::future::ready::<crate::Result<crate::model::Taxonomy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::PolicyTagManager::create_policy_tag].
    fn create_policy_tag(
        &self,
        _req: crate::model::CreatePolicyTagRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::PolicyTag>> + Send {
        std::future::ready::<crate::Result<crate::model::PolicyTag>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::PolicyTagManager::delete_policy_tag].
    fn delete_policy_tag(
        &self,
        _req: crate::model::DeletePolicyTagRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::PolicyTagManager::update_policy_tag].
    fn update_policy_tag(
        &self,
        _req: crate::model::UpdatePolicyTagRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::PolicyTag>> + Send {
        std::future::ready::<crate::Result<crate::model::PolicyTag>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::PolicyTagManager::list_policy_tags].
    fn list_policy_tags(
        &self,
        _req: crate::model::ListPolicyTagsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListPolicyTagsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListPolicyTagsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::PolicyTagManager::get_policy_tag].
    fn get_policy_tag(
        &self,
        _req: crate::model::GetPolicyTagRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::PolicyTag>> + Send {
        std::future::ready::<crate::Result<crate::model::PolicyTag>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::PolicyTagManager::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::PolicyTagManager::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::PolicyTagManager::test_iam_permissions].
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

    /// Implements [super::client::PolicyTagManager::list_operations].
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

    /// Implements [super::client::PolicyTagManager::get_operation].
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

    /// Implements [super::client::PolicyTagManager::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::PolicyTagManager::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }
}

/// Defines the trait used to implement [super::client::PolicyTagManagerSerialization].
///
/// Application developers may need to implement this trait to mock
/// `client::PolicyTagManagerSerialization`.  In other use-cases, application developers only
/// use `client::PolicyTagManagerSerialization` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait PolicyTagManagerSerialization: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::PolicyTagManagerSerialization::replace_taxonomy].
    fn replace_taxonomy(
        &self,
        _req: crate::model::ReplaceTaxonomyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Taxonomy>> + Send {
        std::future::ready::<crate::Result<crate::model::Taxonomy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::PolicyTagManagerSerialization::import_taxonomies].
    fn import_taxonomies(
        &self,
        _req: crate::model::ImportTaxonomiesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ImportTaxonomiesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ImportTaxonomiesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::PolicyTagManagerSerialization::export_taxonomies].
    fn export_taxonomies(
        &self,
        _req: crate::model::ExportTaxonomiesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ExportTaxonomiesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ExportTaxonomiesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::PolicyTagManagerSerialization::list_operations].
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

    /// Implements [super::client::PolicyTagManagerSerialization::get_operation].
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

    /// Implements [super::client::PolicyTagManagerSerialization::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::PolicyTagManagerSerialization::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }
}
