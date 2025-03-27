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

use std::sync::Arc;

/// A dyn-compatible, crate-private version of [super::DataCatalog].
#[async_trait::async_trait]
pub trait DataCatalog: std::fmt::Debug + Send + Sync {
    async fn search_catalog(
        &self,
        req: crate::model::SearchCatalogRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchCatalogResponse>;

    async fn create_entry_group(
        &self,
        req: crate::model::CreateEntryGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EntryGroup>;

    async fn get_entry_group(
        &self,
        req: crate::model::GetEntryGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EntryGroup>;

    async fn update_entry_group(
        &self,
        req: crate::model::UpdateEntryGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EntryGroup>;

    async fn delete_entry_group(
        &self,
        req: crate::model::DeleteEntryGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    async fn list_entry_groups(
        &self,
        req: crate::model::ListEntryGroupsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListEntryGroupsResponse>;

    async fn create_entry(
        &self,
        req: crate::model::CreateEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Entry>;

    async fn update_entry(
        &self,
        req: crate::model::UpdateEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Entry>;

    async fn delete_entry(
        &self,
        req: crate::model::DeleteEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    async fn get_entry(
        &self,
        req: crate::model::GetEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Entry>;

    async fn lookup_entry(
        &self,
        req: crate::model::LookupEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Entry>;

    async fn list_entries(
        &self,
        req: crate::model::ListEntriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListEntriesResponse>;

    async fn modify_entry_overview(
        &self,
        req: crate::model::ModifyEntryOverviewRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EntryOverview>;

    async fn modify_entry_contacts(
        &self,
        req: crate::model::ModifyEntryContactsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Contacts>;

    async fn create_tag_template(
        &self,
        req: crate::model::CreateTagTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TagTemplate>;

    async fn get_tag_template(
        &self,
        req: crate::model::GetTagTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TagTemplate>;

    async fn update_tag_template(
        &self,
        req: crate::model::UpdateTagTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TagTemplate>;

    async fn delete_tag_template(
        &self,
        req: crate::model::DeleteTagTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    async fn create_tag_template_field(
        &self,
        req: crate::model::CreateTagTemplateFieldRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TagTemplateField>;

    async fn update_tag_template_field(
        &self,
        req: crate::model::UpdateTagTemplateFieldRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TagTemplateField>;

    async fn rename_tag_template_field(
        &self,
        req: crate::model::RenameTagTemplateFieldRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TagTemplateField>;

    async fn rename_tag_template_field_enum_value(
        &self,
        req: crate::model::RenameTagTemplateFieldEnumValueRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TagTemplateField>;

    async fn delete_tag_template_field(
        &self,
        req: crate::model::DeleteTagTemplateFieldRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    async fn create_tag(
        &self,
        req: crate::model::CreateTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Tag>;

    async fn update_tag(
        &self,
        req: crate::model::UpdateTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Tag>;

    async fn delete_tag(
        &self,
        req: crate::model::DeleteTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    async fn list_tags(
        &self,
        req: crate::model::ListTagsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListTagsResponse>;

    async fn reconcile_tags(
        &self,
        req: crate::model::ReconcileTagsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn star_entry(
        &self,
        req: crate::model::StarEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::StarEntryResponse>;

    async fn unstar_entry(
        &self,
        req: crate::model::UnstarEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::UnstarEntryResponse>;

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;

    async fn import_entries(
        &self,
        req: crate::model::ImportEntriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn set_config(
        &self,
        req: crate::model::SetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MigrationConfig>;

    async fn retrieve_config(
        &self,
        req: crate::model::RetrieveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::OrganizationConfig>;

    async fn retrieve_effective_config(
        &self,
        req: crate::model::RetrieveEffectiveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MigrationConfig>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::DataCatalog] also implement [DataCatalog].
#[async_trait::async_trait]
impl<T: super::DataCatalog> DataCatalog for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn search_catalog(
        &self,
        req: crate::model::SearchCatalogRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::SearchCatalogResponse> {
        T::search_catalog(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_entry_group(
        &self,
        req: crate::model::CreateEntryGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EntryGroup> {
        T::create_entry_group(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_entry_group(
        &self,
        req: crate::model::GetEntryGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EntryGroup> {
        T::get_entry_group(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_entry_group(
        &self,
        req: crate::model::UpdateEntryGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EntryGroup> {
        T::update_entry_group(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_entry_group(
        &self,
        req: crate::model::DeleteEntryGroupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::delete_entry_group(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_entry_groups(
        &self,
        req: crate::model::ListEntryGroupsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListEntryGroupsResponse> {
        T::list_entry_groups(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_entry(
        &self,
        req: crate::model::CreateEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Entry> {
        T::create_entry(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_entry(
        &self,
        req: crate::model::UpdateEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Entry> {
        T::update_entry(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_entry(
        &self,
        req: crate::model::DeleteEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::delete_entry(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_entry(
        &self,
        req: crate::model::GetEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Entry> {
        T::get_entry(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn lookup_entry(
        &self,
        req: crate::model::LookupEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Entry> {
        T::lookup_entry(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_entries(
        &self,
        req: crate::model::ListEntriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListEntriesResponse> {
        T::list_entries(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn modify_entry_overview(
        &self,
        req: crate::model::ModifyEntryOverviewRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EntryOverview> {
        T::modify_entry_overview(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn modify_entry_contacts(
        &self,
        req: crate::model::ModifyEntryContactsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Contacts> {
        T::modify_entry_contacts(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_tag_template(
        &self,
        req: crate::model::CreateTagTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TagTemplate> {
        T::create_tag_template(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_tag_template(
        &self,
        req: crate::model::GetTagTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TagTemplate> {
        T::get_tag_template(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_tag_template(
        &self,
        req: crate::model::UpdateTagTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TagTemplate> {
        T::update_tag_template(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_tag_template(
        &self,
        req: crate::model::DeleteTagTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::delete_tag_template(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_tag_template_field(
        &self,
        req: crate::model::CreateTagTemplateFieldRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TagTemplateField> {
        T::create_tag_template_field(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_tag_template_field(
        &self,
        req: crate::model::UpdateTagTemplateFieldRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TagTemplateField> {
        T::update_tag_template_field(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn rename_tag_template_field(
        &self,
        req: crate::model::RenameTagTemplateFieldRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TagTemplateField> {
        T::rename_tag_template_field(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn rename_tag_template_field_enum_value(
        &self,
        req: crate::model::RenameTagTemplateFieldEnumValueRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TagTemplateField> {
        T::rename_tag_template_field_enum_value(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_tag_template_field(
        &self,
        req: crate::model::DeleteTagTemplateFieldRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::delete_tag_template_field(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_tag(
        &self,
        req: crate::model::CreateTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Tag> {
        T::create_tag(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_tag(
        &self,
        req: crate::model::UpdateTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Tag> {
        T::update_tag(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_tag(
        &self,
        req: crate::model::DeleteTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::delete_tag(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_tags(
        &self,
        req: crate::model::ListTagsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListTagsResponse> {
        T::list_tags(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn reconcile_tags(
        &self,
        req: crate::model::ReconcileTagsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::reconcile_tags(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn star_entry(
        &self,
        req: crate::model::StarEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::StarEntryResponse> {
        T::star_entry(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn unstar_entry(
        &self,
        req: crate::model::UnstarEntryRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::UnstarEntryResponse> {
        T::unstar_entry(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse> {
        T::test_iam_permissions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn import_entries(
        &self,
        req: crate::model::ImportEntriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::import_entries(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_config(
        &self,
        req: crate::model::SetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MigrationConfig> {
        T::set_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn retrieve_config(
        &self,
        req: crate::model::RetrieveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::OrganizationConfig> {
        T::retrieve_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn retrieve_effective_config(
        &self,
        req: crate::model::RetrieveEffectiveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::MigrationConfig> {
        T::retrieve_effective_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::cancel_operation(self, req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        T::get_polling_error_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}

/// A dyn-compatible, crate-private version of [super::PolicyTagManager].
#[async_trait::async_trait]
pub trait PolicyTagManager: std::fmt::Debug + Send + Sync {
    async fn create_taxonomy(
        &self,
        req: crate::model::CreateTaxonomyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Taxonomy>;

    async fn delete_taxonomy(
        &self,
        req: crate::model::DeleteTaxonomyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    async fn update_taxonomy(
        &self,
        req: crate::model::UpdateTaxonomyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Taxonomy>;

    async fn list_taxonomies(
        &self,
        req: crate::model::ListTaxonomiesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListTaxonomiesResponse>;

    async fn get_taxonomy(
        &self,
        req: crate::model::GetTaxonomyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Taxonomy>;

    async fn create_policy_tag(
        &self,
        req: crate::model::CreatePolicyTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PolicyTag>;

    async fn delete_policy_tag(
        &self,
        req: crate::model::DeletePolicyTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    async fn update_policy_tag(
        &self,
        req: crate::model::UpdatePolicyTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PolicyTag>;

    async fn list_policy_tags(
        &self,
        req: crate::model::ListPolicyTagsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPolicyTagsResponse>;

    async fn get_policy_tag(
        &self,
        req: crate::model::GetPolicyTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PolicyTag>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;
}

/// All implementations of [super::PolicyTagManager] also implement [PolicyTagManager].
#[async_trait::async_trait]
impl<T: super::PolicyTagManager> PolicyTagManager for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_taxonomy(
        &self,
        req: crate::model::CreateTaxonomyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Taxonomy> {
        T::create_taxonomy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_taxonomy(
        &self,
        req: crate::model::DeleteTaxonomyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::delete_taxonomy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_taxonomy(
        &self,
        req: crate::model::UpdateTaxonomyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Taxonomy> {
        T::update_taxonomy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_taxonomies(
        &self,
        req: crate::model::ListTaxonomiesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListTaxonomiesResponse> {
        T::list_taxonomies(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_taxonomy(
        &self,
        req: crate::model::GetTaxonomyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Taxonomy> {
        T::get_taxonomy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_policy_tag(
        &self,
        req: crate::model::CreatePolicyTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PolicyTag> {
        T::create_policy_tag(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_policy_tag(
        &self,
        req: crate::model::DeletePolicyTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::delete_policy_tag(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_policy_tag(
        &self,
        req: crate::model::UpdatePolicyTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PolicyTag> {
        T::update_policy_tag(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_policy_tags(
        &self,
        req: crate::model::ListPolicyTagsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPolicyTagsResponse> {
        T::list_policy_tags(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_policy_tag(
        &self,
        req: crate::model::GetPolicyTagRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::PolicyTag> {
        T::get_policy_tag(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse> {
        T::test_iam_permissions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::cancel_operation(self, req, options).await
    }
}

/// A dyn-compatible, crate-private version of [super::PolicyTagManagerSerialization].
#[async_trait::async_trait]
pub trait PolicyTagManagerSerialization: std::fmt::Debug + Send + Sync {
    async fn replace_taxonomy(
        &self,
        req: crate::model::ReplaceTaxonomyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Taxonomy>;

    async fn import_taxonomies(
        &self,
        req: crate::model::ImportTaxonomiesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ImportTaxonomiesResponse>;

    async fn export_taxonomies(
        &self,
        req: crate::model::ExportTaxonomiesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ExportTaxonomiesResponse>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;
}

/// All implementations of [super::PolicyTagManagerSerialization] also implement [PolicyTagManagerSerialization].
#[async_trait::async_trait]
impl<T: super::PolicyTagManagerSerialization> PolicyTagManagerSerialization for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn replace_taxonomy(
        &self,
        req: crate::model::ReplaceTaxonomyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Taxonomy> {
        T::replace_taxonomy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn import_taxonomies(
        &self,
        req: crate::model::ImportTaxonomiesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ImportTaxonomiesResponse> {
        T::import_taxonomies(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn export_taxonomies(
        &self,
        req: crate::model::ExportTaxonomiesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ExportTaxonomiesResponse> {
        T::export_taxonomies(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::cancel_operation(self, req, options).await
    }
}
