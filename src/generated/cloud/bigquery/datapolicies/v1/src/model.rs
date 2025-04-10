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
#![no_implicit_prelude]
extern crate async_trait;
extern crate bytes;
extern crate gax;
extern crate gaxi;
extern crate iam_v1;
extern crate lazy_static;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate std;
extern crate tracing;
extern crate wkt;

/// Request message for the CreateDataPolicy method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct CreateDataPolicyRequest {
    /// Required. Resource name of the project that the data policy will belong to.
    /// The format is `projects/{project_number}/locations/{location_id}`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// Required. The data policy to create. The `name` field does not need to be
    /// provided for the data policy creation.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub data_policy: std::option::Option<crate::model::DataPolicy>,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl CreateDataPolicyRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::CreateDataPolicyRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [data_policy][crate::model::CreateDataPolicyRequest::data_policy].
    pub fn set_data_policy<T: std::convert::Into<std::option::Option<crate::model::DataPolicy>>>(
        mut self,
        v: T,
    ) -> Self {
        self.data_policy = v.into();
        self
    }
}

impl wkt::message::Message for CreateDataPolicyRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.bigquery.datapolicies.v1.CreateDataPolicyRequest"
    }
}

/// Response message for the UpdateDataPolicy method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct UpdateDataPolicyRequest {
    /// Required. Update the data policy's metadata.
    ///
    /// The target data policy is determined by the `name` field.
    /// Other fields are updated to the specified values based on the field masks.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub data_policy: std::option::Option<crate::model::DataPolicy>,

    /// The update mask applies to the resource. For the `FieldMask` definition,
    /// see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    /// If not set, defaults to all of the fields that are allowed to update.
    ///
    /// Updates to the `name` and `dataPolicyId` fields are not allowed.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub update_mask: std::option::Option<wkt::FieldMask>,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl UpdateDataPolicyRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [data_policy][crate::model::UpdateDataPolicyRequest::data_policy].
    pub fn set_data_policy<T: std::convert::Into<std::option::Option<crate::model::DataPolicy>>>(
        mut self,
        v: T,
    ) -> Self {
        self.data_policy = v.into();
        self
    }

    /// Sets the value of [update_mask][crate::model::UpdateDataPolicyRequest::update_mask].
    pub fn set_update_mask<T: std::convert::Into<std::option::Option<wkt::FieldMask>>>(
        mut self,
        v: T,
    ) -> Self {
        self.update_mask = v.into();
        self
    }
}

impl wkt::message::Message for UpdateDataPolicyRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.bigquery.datapolicies.v1.UpdateDataPolicyRequest"
    }
}

/// Request message for the RenameDataPolicy method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct RenameDataPolicyRequest {
    /// Required. Resource name of the data policy to rename. The format is
    /// `projects/{project_number}/locations/{location_id}/dataPolicies/{data_policy_id}`
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Required. The new data policy id.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub new_data_policy_id: std::string::String,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl RenameDataPolicyRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::RenameDataPolicyRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [new_data_policy_id][crate::model::RenameDataPolicyRequest::new_data_policy_id].
    pub fn set_new_data_policy_id<T: std::convert::Into<std::string::String>>(
        mut self,
        v: T,
    ) -> Self {
        self.new_data_policy_id = v.into();
        self
    }
}

impl wkt::message::Message for RenameDataPolicyRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.bigquery.datapolicies.v1.RenameDataPolicyRequest"
    }
}

/// Request message for the DeleteDataPolicy method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeleteDataPolicyRequest {
    /// Required. Resource name of the data policy to delete. Format is
    /// `projects/{project_number}/locations/{location_id}/dataPolicies/{data_policy_id}`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl DeleteDataPolicyRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::DeleteDataPolicyRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for DeleteDataPolicyRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.bigquery.datapolicies.v1.DeleteDataPolicyRequest"
    }
}

/// Request message for the GetDataPolicy method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetDataPolicyRequest {
    /// Required. Resource name of the requested data policy. Format is
    /// `projects/{project_number}/locations/{location_id}/dataPolicies/{data_policy_id}`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl GetDataPolicyRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::GetDataPolicyRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for GetDataPolicyRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.bigquery.datapolicies.v1.GetDataPolicyRequest"
    }
}

/// Request message for the ListDataPolicies method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListDataPoliciesRequest {
    /// Required. Resource name of the project for which to list data policies.
    /// Format is `projects/{project_number}/locations/{location_id}`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// The maximum number of data policies to return. Must be a value between 1
    /// and 1000.
    /// If not set, defaults to 50.
    pub page_size: i32,

    /// The `nextPageToken` value returned from a previous list request, if any. If
    /// not set, defaults to an empty string.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub page_token: std::string::String,

    /// Filters the data policies by policy tags that they
    /// are associated with. Currently filter only supports
    /// "policy\<span\>\</span\>_tag" based filtering and OR based predicates. Sample
    /// filter can be "policy\<span\>\</span\>_tag:
    /// projects/1/locations/us/taxonomies/2/policyTags/3".
    /// You may also use wildcard such as "policy\<span\>\</span\>_tag:
    /// projects/1/locations/us/taxonomies/2*". Please note that OR predicates
    /// cannot be used with wildcard filters.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub filter: std::string::String,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl ListDataPoliciesRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::ListDataPoliciesRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [page_size][crate::model::ListDataPoliciesRequest::page_size].
    pub fn set_page_size<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of [page_token][crate::model::ListDataPoliciesRequest::page_token].
    pub fn set_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }

    /// Sets the value of [filter][crate::model::ListDataPoliciesRequest::filter].
    pub fn set_filter<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }
}

impl wkt::message::Message for ListDataPoliciesRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.bigquery.datapolicies.v1.ListDataPoliciesRequest"
    }
}

/// Response message for the ListDataPolicies method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListDataPoliciesResponse {
    /// Data policies that belong to the requested project.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub data_policies: std::vec::Vec<crate::model::DataPolicy>,

    /// Token used to retrieve the next page of results, or empty if there are no
    /// more results.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub next_page_token: std::string::String,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl ListDataPoliciesResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [next_page_token][crate::model::ListDataPoliciesResponse::next_page_token].
    pub fn set_next_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }

    /// Sets the value of [data_policies][crate::model::ListDataPoliciesResponse::data_policies].
    pub fn set_data_policies<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::DataPolicy>,
    {
        use std::iter::Iterator;
        self.data_policies = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for ListDataPoliciesResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.bigquery.datapolicies.v1.ListDataPoliciesResponse"
    }
}

#[doc(hidden)]
impl gax::paginator::internal::PageableResponse for ListDataPoliciesResponse {
    type PageItem = crate::model::DataPolicy;

    fn items(self) -> std::vec::Vec<Self::PageItem> {
        self.data_policies
    }

    fn next_page_token(&self) -> std::string::String {
        use std::clone::Clone;
        self.next_page_token.clone()
    }
}

/// Represents the label-policy binding.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct DataPolicy {
    /// Output only. Resource name of this data policy, in the format of
    /// `projects/{project_number}/locations/{location_id}/dataPolicies/{data_policy_id}`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Type of data policy.
    pub data_policy_type: crate::model::data_policy::DataPolicyType,

    /// User-assigned (human readable) ID of the data policy that needs to be
    /// unique within a project. Used as {data_policy_id} in part of the resource
    /// name.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub data_policy_id: std::string::String,

    /// Label that is bound to this data policy.
    #[serde(flatten, skip_serializing_if = "std::option::Option::is_none")]
    pub matching_label: std::option::Option<crate::model::data_policy::MatchingLabel>,

    /// The policy that is bound to this data policy.
    #[serde(flatten, skip_serializing_if = "std::option::Option::is_none")]
    pub policy: std::option::Option<crate::model::data_policy::Policy>,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl DataPolicy {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::DataPolicy::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [data_policy_type][crate::model::DataPolicy::data_policy_type].
    pub fn set_data_policy_type<
        T: std::convert::Into<crate::model::data_policy::DataPolicyType>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.data_policy_type = v.into();
        self
    }

    /// Sets the value of [data_policy_id][crate::model::DataPolicy::data_policy_id].
    pub fn set_data_policy_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.data_policy_id = v.into();
        self
    }

    /// Sets the value of `matching_label`.
    pub fn set_matching_label<
        T: std::convert::Into<std::option::Option<crate::model::data_policy::MatchingLabel>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.matching_label = v.into();
        self
    }

    /// The value of [matching_label][crate::model::DataPolicy::matching_label]
    /// if it holds a `PolicyTag`, `None` if the field is not set or
    /// holds a different branch.
    pub fn get_policy_tag(&self) -> std::option::Option<&std::string::String> {
        #[allow(unreachable_patterns)]
        self.matching_label.as_ref().and_then(|v| match v {
            crate::model::data_policy::MatchingLabel::PolicyTag(v) => std::option::Option::Some(v),
            _ => std::option::Option::None,
        })
    }

    /// Sets the value of [matching_label][crate::model::DataPolicy::matching_label]
    /// to hold a `PolicyTag`.
    ///
    /// Note that all the setters affecting `matching_label` are
    /// mutually exclusive.
    pub fn set_policy_tag<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.matching_label = std::option::Option::Some(
            crate::model::data_policy::MatchingLabel::PolicyTag(v.into()),
        );
        self
    }

    /// Sets the value of `policy`.
    pub fn set_policy<
        T: std::convert::Into<std::option::Option<crate::model::data_policy::Policy>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.policy = v.into();
        self
    }

    /// The value of [policy][crate::model::DataPolicy::policy]
    /// if it holds a `DataMaskingPolicy`, `None` if the field is not set or
    /// holds a different branch.
    pub fn get_data_masking_policy(
        &self,
    ) -> std::option::Option<&std::boxed::Box<crate::model::DataMaskingPolicy>> {
        #[allow(unreachable_patterns)]
        self.policy.as_ref().and_then(|v| match v {
            crate::model::data_policy::Policy::DataMaskingPolicy(v) => std::option::Option::Some(v),
            _ => std::option::Option::None,
        })
    }

    /// Sets the value of [policy][crate::model::DataPolicy::policy]
    /// to hold a `DataMaskingPolicy`.
    ///
    /// Note that all the setters affecting `policy` are
    /// mutually exclusive.
    pub fn set_data_masking_policy<
        T: std::convert::Into<std::boxed::Box<crate::model::DataMaskingPolicy>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.policy = std::option::Option::Some(
            crate::model::data_policy::Policy::DataMaskingPolicy(v.into()),
        );
        self
    }
}

impl wkt::message::Message for DataPolicy {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.bigquery.datapolicies.v1.DataPolicy"
    }
}

/// Defines additional types related to [DataPolicy].
pub mod data_policy {
    #[allow(unused_imports)]
    use super::*;

    /// A list of supported data policy types.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct DataPolicyType(i32);

    impl DataPolicyType {
        /// Default value for the data policy type. This should not be used.
        pub const DATA_POLICY_TYPE_UNSPECIFIED: DataPolicyType = DataPolicyType::new(0);

        /// Used to create a data policy for column-level security, without data
        /// masking.
        pub const COLUMN_LEVEL_SECURITY_POLICY: DataPolicyType = DataPolicyType::new(3);

        /// Used to create a data policy for data masking.
        pub const DATA_MASKING_POLICY: DataPolicyType = DataPolicyType::new(2);

        /// Creates a new DataPolicyType instance.
        pub(crate) const fn new(value: i32) -> Self {
            Self(value)
        }

        /// Gets the enum value.
        pub fn value(&self) -> i32 {
            self.0
        }

        /// Gets the enum value as a string.
        pub fn as_str_name(&self) -> std::borrow::Cow<'static, str> {
            match self.0 {
                0 => std::borrow::Cow::Borrowed("DATA_POLICY_TYPE_UNSPECIFIED"),
                2 => std::borrow::Cow::Borrowed("DATA_MASKING_POLICY"),
                3 => std::borrow::Cow::Borrowed("COLUMN_LEVEL_SECURITY_POLICY"),
                _ => std::borrow::Cow::Owned(std::format!("UNKNOWN-VALUE:{}", self.0)),
            }
        }

        /// Creates an enum value from the value name.
        pub fn from_str_name(name: &str) -> std::option::Option<Self> {
            match name {
                "DATA_POLICY_TYPE_UNSPECIFIED" => {
                    std::option::Option::Some(Self::DATA_POLICY_TYPE_UNSPECIFIED)
                }
                "COLUMN_LEVEL_SECURITY_POLICY" => {
                    std::option::Option::Some(Self::COLUMN_LEVEL_SECURITY_POLICY)
                }
                "DATA_MASKING_POLICY" => std::option::Option::Some(Self::DATA_MASKING_POLICY),
                _ => std::option::Option::None,
            }
        }
    }

    impl std::convert::From<i32> for DataPolicyType {
        fn from(value: i32) -> Self {
            Self::new(value)
        }
    }

    impl std::default::Default for DataPolicyType {
        fn default() -> Self {
            Self::new(0)
        }
    }

    /// Label that is bound to this data policy.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub enum MatchingLabel {
        /// Policy tag resource name, in the format of
        /// `projects/{project_number}/locations/{location_id}/taxonomies/{taxonomy_id}/policyTags/{policyTag_id}`.
        PolicyTag(std::string::String),
    }

    /// The policy that is bound to this data policy.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub enum Policy {
        /// The data masking policy that specifies the data masking rule to use.
        DataMaskingPolicy(std::boxed::Box<crate::model::DataMaskingPolicy>),
    }
}

/// The data masking policy that is used to specify data masking rule.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct DataMaskingPolicy {
    /// A masking expression to bind to the data masking rule.
    #[serde(flatten, skip_serializing_if = "std::option::Option::is_none")]
    pub masking_expression:
        std::option::Option<crate::model::data_masking_policy::MaskingExpression>,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl DataMaskingPolicy {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of `masking_expression`.
    pub fn set_masking_expression<
        T: std::convert::Into<
                std::option::Option<crate::model::data_masking_policy::MaskingExpression>,
            >,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.masking_expression = v.into();
        self
    }

    /// The value of [masking_expression][crate::model::DataMaskingPolicy::masking_expression]
    /// if it holds a `PredefinedExpression`, `None` if the field is not set or
    /// holds a different branch.
    pub fn get_predefined_expression(
        &self,
    ) -> std::option::Option<&crate::model::data_masking_policy::PredefinedExpression> {
        #[allow(unreachable_patterns)]
        self.masking_expression.as_ref().and_then(|v| match v {
            crate::model::data_masking_policy::MaskingExpression::PredefinedExpression(v) => {
                std::option::Option::Some(v)
            }
            _ => std::option::Option::None,
        })
    }

    /// The value of [masking_expression][crate::model::DataMaskingPolicy::masking_expression]
    /// if it holds a `Routine`, `None` if the field is not set or
    /// holds a different branch.
    pub fn get_routine(&self) -> std::option::Option<&std::string::String> {
        #[allow(unreachable_patterns)]
        self.masking_expression.as_ref().and_then(|v| match v {
            crate::model::data_masking_policy::MaskingExpression::Routine(v) => {
                std::option::Option::Some(v)
            }
            _ => std::option::Option::None,
        })
    }

    /// Sets the value of [masking_expression][crate::model::DataMaskingPolicy::masking_expression]
    /// to hold a `PredefinedExpression`.
    ///
    /// Note that all the setters affecting `masking_expression` are
    /// mutually exclusive.
    pub fn set_predefined_expression<
        T: std::convert::Into<crate::model::data_masking_policy::PredefinedExpression>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.masking_expression = std::option::Option::Some(
            crate::model::data_masking_policy::MaskingExpression::PredefinedExpression(v.into()),
        );
        self
    }

    /// Sets the value of [masking_expression][crate::model::DataMaskingPolicy::masking_expression]
    /// to hold a `Routine`.
    ///
    /// Note that all the setters affecting `masking_expression` are
    /// mutually exclusive.
    pub fn set_routine<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.masking_expression = std::option::Option::Some(
            crate::model::data_masking_policy::MaskingExpression::Routine(v.into()),
        );
        self
    }
}

impl wkt::message::Message for DataMaskingPolicy {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.bigquery.datapolicies.v1.DataMaskingPolicy"
    }
}

/// Defines additional types related to [DataMaskingPolicy].
pub mod data_masking_policy {
    #[allow(unused_imports)]
    use super::*;

    /// The available masking rules. Learn more here:
    /// <https://cloud.google.com/bigquery/docs/column-data-masking-intro#masking_options>.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct PredefinedExpression(i32);

    impl PredefinedExpression {
        /// Default, unspecified predefined expression. No masking will take place
        /// since no expression is specified.
        pub const PREDEFINED_EXPRESSION_UNSPECIFIED: PredefinedExpression =
            PredefinedExpression::new(0);

        /// Masking expression to replace data with SHA-256 hash.
        pub const SHA256: PredefinedExpression = PredefinedExpression::new(3);

        /// Masking expression to replace data with NULLs.
        pub const ALWAYS_NULL: PredefinedExpression = PredefinedExpression::new(5);

        /// Masking expression to replace data with their default masking values.
        /// The default masking values for each type listed as below:
        ///
        /// * STRING: ""
        /// * BYTES: b''
        /// * INTEGER: 0
        /// * FLOAT: 0.0
        /// * NUMERIC: 0
        /// * BOOLEAN: FALSE
        /// * TIMESTAMP: 1970-01-01 00:00:00 UTC
        /// * DATE: 1970-01-01
        /// * TIME: 00:00:00
        /// * DATETIME: 1970-01-01T00:00:00
        /// * GEOGRAPHY: POINT(0 0)
        /// * BIGNUMERIC: 0
        /// * ARRAY: []
        /// * STRUCT: NOT_APPLICABLE
        /// * JSON: NULL
        pub const DEFAULT_MASKING_VALUE: PredefinedExpression = PredefinedExpression::new(7);

        /// Masking expression shows the last four characters of text.
        /// The masking behavior is as follows:
        ///
        /// * If text length > 4 characters: Replace text with XXXXX, append last
        ///   four characters of original text.
        /// * If text length <= 4 characters: Apply SHA-256 hash.
        pub const LAST_FOUR_CHARACTERS: PredefinedExpression = PredefinedExpression::new(9);

        /// Masking expression shows the first four characters of text.
        /// The masking behavior is as follows:
        ///
        /// * If text length > 4 characters: Replace text with XXXXX, prepend first
        ///   four characters of original text.
        /// * If text length <= 4 characters: Apply SHA-256 hash.
        pub const FIRST_FOUR_CHARACTERS: PredefinedExpression = PredefinedExpression::new(10);

        /// Masking expression for email addresses.
        /// The masking behavior is as follows:
        ///
        /// * Syntax-valid email address: Replace username with XXXXX. For example,
        ///   cloudysanfrancisco@gmail.com becomes XXXXX@gmail.com.
        /// * Syntax-invalid email address: Apply SHA-256 hash.
        ///
        /// For more information, see [Email
        /// mask](https://cloud.google.com/bigquery/docs/column-data-masking-intro#masking_options).
        pub const EMAIL_MASK: PredefinedExpression = PredefinedExpression::new(12);

        /// Masking expression to only show the year of `Date`,
        /// `DateTime` and `TimeStamp`. For example, with the
        /// year 2076:
        ///
        /// * DATE         :  2076-01-01
        /// * DATETIME     :  2076-01-01T00:00:00
        /// * TIMESTAMP    :  2076-01-01 00:00:00 UTC
        ///
        /// Truncation occurs according to the UTC time zone. To change this, adjust
        /// the default time zone using the `time_zone` system variable.
        /// For more information, see the <a
        /// href="https://cloud.google.com/bigquery/docs/reference/system-variables">System
        /// variables reference</a>.
        pub const DATE_YEAR_MASK: PredefinedExpression = PredefinedExpression::new(13);

        /// Creates a new PredefinedExpression instance.
        pub(crate) const fn new(value: i32) -> Self {
            Self(value)
        }

        /// Gets the enum value.
        pub fn value(&self) -> i32 {
            self.0
        }

        /// Gets the enum value as a string.
        pub fn as_str_name(&self) -> std::borrow::Cow<'static, str> {
            match self.0 {
                0 => std::borrow::Cow::Borrowed("PREDEFINED_EXPRESSION_UNSPECIFIED"),
                3 => std::borrow::Cow::Borrowed("SHA256"),
                5 => std::borrow::Cow::Borrowed("ALWAYS_NULL"),
                7 => std::borrow::Cow::Borrowed("DEFAULT_MASKING_VALUE"),
                9 => std::borrow::Cow::Borrowed("LAST_FOUR_CHARACTERS"),
                10 => std::borrow::Cow::Borrowed("FIRST_FOUR_CHARACTERS"),
                12 => std::borrow::Cow::Borrowed("EMAIL_MASK"),
                13 => std::borrow::Cow::Borrowed("DATE_YEAR_MASK"),
                _ => std::borrow::Cow::Owned(std::format!("UNKNOWN-VALUE:{}", self.0)),
            }
        }

        /// Creates an enum value from the value name.
        pub fn from_str_name(name: &str) -> std::option::Option<Self> {
            match name {
                "PREDEFINED_EXPRESSION_UNSPECIFIED" => {
                    std::option::Option::Some(Self::PREDEFINED_EXPRESSION_UNSPECIFIED)
                }
                "SHA256" => std::option::Option::Some(Self::SHA256),
                "ALWAYS_NULL" => std::option::Option::Some(Self::ALWAYS_NULL),
                "DEFAULT_MASKING_VALUE" => std::option::Option::Some(Self::DEFAULT_MASKING_VALUE),
                "LAST_FOUR_CHARACTERS" => std::option::Option::Some(Self::LAST_FOUR_CHARACTERS),
                "FIRST_FOUR_CHARACTERS" => std::option::Option::Some(Self::FIRST_FOUR_CHARACTERS),
                "EMAIL_MASK" => std::option::Option::Some(Self::EMAIL_MASK),
                "DATE_YEAR_MASK" => std::option::Option::Some(Self::DATE_YEAR_MASK),
                _ => std::option::Option::None,
            }
        }
    }

    impl std::convert::From<i32> for PredefinedExpression {
        fn from(value: i32) -> Self {
            Self::new(value)
        }
    }

    impl std::default::Default for PredefinedExpression {
        fn default() -> Self {
            Self::new(0)
        }
    }

    /// A masking expression to bind to the data masking rule.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub enum MaskingExpression {
        /// A predefined masking expression.
        PredefinedExpression(crate::model::data_masking_policy::PredefinedExpression),
        /// The name of the BigQuery routine that contains the custom masking
        /// routine, in the format of
        /// `projects/{project_number}/datasets/{dataset_id}/routines/{routine_id}`.
        Routine(std::string::String),
    }
}
