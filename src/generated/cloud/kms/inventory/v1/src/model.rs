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
extern crate gclient;
extern crate kms;
extern crate lazy_static;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate std;
extern crate tracing;
extern crate wkt;

/// Request message for
/// [KeyDashboardService.ListCryptoKeys][google.cloud.kms.inventory.v1.KeyDashboardService.ListCryptoKeys].
///
/// [google.cloud.kms.inventory.v1.KeyDashboardService.ListCryptoKeys]: crate::client::KeyDashboardService::list_crypto_keys
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListCryptoKeysRequest {
    /// Required. The Google Cloud project for which to retrieve key metadata, in
    /// the format `projects/*`
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// Optional. The maximum number of keys to return. The service may return
    /// fewer than this value. If unspecified, at most 1000 keys will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    pub page_size: i32,

    /// Optional. Pass this into a subsequent request in order to receive the next
    /// page of results.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub page_token: std::string::String,
}

impl ListCryptoKeysRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::ListCryptoKeysRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [page_size][crate::model::ListCryptoKeysRequest::page_size].
    pub fn set_page_size<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of [page_token][crate::model::ListCryptoKeysRequest::page_token].
    pub fn set_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }
}

impl wkt::message::Message for ListCryptoKeysRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.kms.inventory.v1.ListCryptoKeysRequest"
    }
}

/// Response message for
/// [KeyDashboardService.ListCryptoKeys][google.cloud.kms.inventory.v1.KeyDashboardService.ListCryptoKeys].
///
/// [google.cloud.kms.inventory.v1.KeyDashboardService.ListCryptoKeys]: crate::client::KeyDashboardService::list_crypto_keys
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListCryptoKeysResponse {
    /// The list of [CryptoKeys][google.cloud.kms.v1.CryptoKey].
    ///
    /// [google.cloud.kms.v1.CryptoKey]: kms::model::CryptoKey
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub crypto_keys: std::vec::Vec<kms::model::CryptoKey>,

    /// The page token returned from the previous response if the next page is
    /// desired.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub next_page_token: std::string::String,
}

impl ListCryptoKeysResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [next_page_token][crate::model::ListCryptoKeysResponse::next_page_token].
    pub fn set_next_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }

    /// Sets the value of [crypto_keys][crate::model::ListCryptoKeysResponse::crypto_keys].
    pub fn set_crypto_keys<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<kms::model::CryptoKey>,
    {
        use std::iter::Iterator;
        self.crypto_keys = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for ListCryptoKeysResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.kms.inventory.v1.ListCryptoKeysResponse"
    }
}

impl gax::paginator::PageableResponse for ListCryptoKeysResponse {
    type PageItem = kms::model::CryptoKey;

    fn items(self) -> std::vec::Vec<Self::PageItem> {
        self.crypto_keys
    }

    fn next_page_token(&self) -> std::string::String {
        use std::clone::Clone;
        self.next_page_token.clone()
    }
}

/// Request message for
/// [KeyTrackingService.GetProtectedResourcesSummary][google.cloud.kms.inventory.v1.KeyTrackingService.GetProtectedResourcesSummary].
///
/// [google.cloud.kms.inventory.v1.KeyTrackingService.GetProtectedResourcesSummary]: crate::client::KeyTrackingService::get_protected_resources_summary
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetProtectedResourcesSummaryRequest {
    /// Required. The resource name of the
    /// [CryptoKey][google.cloud.kms.v1.CryptoKey].
    ///
    /// [google.cloud.kms.v1.CryptoKey]: kms::model::CryptoKey
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,
}

impl GetProtectedResourcesSummaryRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::GetProtectedResourcesSummaryRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for GetProtectedResourcesSummaryRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.kms.inventory.v1.GetProtectedResourcesSummaryRequest"
    }
}

/// Aggregate information about the resources protected by a Cloud KMS key in the
/// same Cloud organization as the key.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ProtectedResourcesSummary {
    /// The full name of the ProtectedResourcesSummary resource.
    /// Example:
    /// projects/test-project/locations/us/keyRings/test-keyring/cryptoKeys/test-key/protectedResourcesSummary
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// The total number of protected resources in the same Cloud organization as
    /// the key.
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub resource_count: i64,

    /// The number of distinct Cloud projects in the same Cloud organization as the
    /// key that have resources protected by the key.
    pub project_count: i32,

    /// The number of resources protected by the key grouped by resource type.
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[serde_as(as = "std::collections::HashMap<_, serde_with::DisplayFromStr>")]
    pub resource_types: std::collections::HashMap<std::string::String, i64>,

    /// The number of resources protected by the key grouped by Cloud product.
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[serde_as(as = "std::collections::HashMap<_, serde_with::DisplayFromStr>")]
    pub cloud_products: std::collections::HashMap<std::string::String, i64>,

    /// The number of resources protected by the key grouped by region.
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[serde_as(as = "std::collections::HashMap<_, serde_with::DisplayFromStr>")]
    pub locations: std::collections::HashMap<std::string::String, i64>,
}

impl ProtectedResourcesSummary {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::ProtectedResourcesSummary::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [resource_count][crate::model::ProtectedResourcesSummary::resource_count].
    pub fn set_resource_count<T: std::convert::Into<i64>>(mut self, v: T) -> Self {
        self.resource_count = v.into();
        self
    }

    /// Sets the value of [project_count][crate::model::ProtectedResourcesSummary::project_count].
    pub fn set_project_count<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.project_count = v.into();
        self
    }

    /// Sets the value of [resource_types][crate::model::ProtectedResourcesSummary::resource_types].
    pub fn set_resource_types<T, K, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = (K, V)>,
        K: std::convert::Into<std::string::String>,
        V: std::convert::Into<i64>,
    {
        use std::iter::Iterator;
        self.resource_types = v.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        self
    }

    /// Sets the value of [cloud_products][crate::model::ProtectedResourcesSummary::cloud_products].
    pub fn set_cloud_products<T, K, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = (K, V)>,
        K: std::convert::Into<std::string::String>,
        V: std::convert::Into<i64>,
    {
        use std::iter::Iterator;
        self.cloud_products = v.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        self
    }

    /// Sets the value of [locations][crate::model::ProtectedResourcesSummary::locations].
    pub fn set_locations<T, K, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = (K, V)>,
        K: std::convert::Into<std::string::String>,
        V: std::convert::Into<i64>,
    {
        use std::iter::Iterator;
        self.locations = v.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        self
    }
}

impl wkt::message::Message for ProtectedResourcesSummary {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.kms.inventory.v1.ProtectedResourcesSummary"
    }
}

/// Request message for
/// [KeyTrackingService.SearchProtectedResources][google.cloud.kms.inventory.v1.KeyTrackingService.SearchProtectedResources].
///
/// [google.cloud.kms.inventory.v1.KeyTrackingService.SearchProtectedResources]: crate::client::KeyTrackingService::search_protected_resources
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct SearchProtectedResourcesRequest {
    /// Required. Resource name of the organization.
    /// Example: organizations/123
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub scope: std::string::String,

    /// Required. The resource name of the
    /// [CryptoKey][google.cloud.kms.v1.CryptoKey].
    ///
    /// [google.cloud.kms.v1.CryptoKey]: kms::model::CryptoKey
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub crypto_key: std::string::String,

    /// The maximum number of resources to return. The service may return fewer
    /// than this value.
    /// If unspecified, at most 500 resources will be returned.
    /// The maximum value is 500; values above 500 will be coerced to 500.
    pub page_size: i32,

    /// A page token, received from a previous
    /// [KeyTrackingService.SearchProtectedResources][google.cloud.kms.inventory.v1.KeyTrackingService.SearchProtectedResources]
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// [KeyTrackingService.SearchProtectedResources][google.cloud.kms.inventory.v1.KeyTrackingService.SearchProtectedResources]
    /// must match the call that provided the page token.
    ///
    /// [google.cloud.kms.inventory.v1.KeyTrackingService.SearchProtectedResources]: crate::client::KeyTrackingService::search_protected_resources
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub page_token: std::string::String,

    /// Optional. A list of resource types that this request searches for. If
    /// empty, it will search all the [trackable resource
    /// types](https://cloud.google.com/kms/docs/view-key-usage#tracked-resource-types).
    ///
    /// Regular expressions are also supported. For example:
    ///
    /// * `compute.googleapis.com.*` snapshots resources whose type starts
    ///   with `compute.googleapis.com`.
    /// * `.*Image` snapshots resources whose type ends with `Image`.
    /// * `.*Image.*` snapshots resources whose type contains `Image`.
    ///
    /// See [RE2](https://github.com/google/re2/wiki/Syntax) for all supported
    /// regular expression syntax. If the regular expression does not match any
    /// supported resource type, an INVALID_ARGUMENT error will be returned.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub resource_types: std::vec::Vec<std::string::String>,
}

impl SearchProtectedResourcesRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [scope][crate::model::SearchProtectedResourcesRequest::scope].
    pub fn set_scope<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.scope = v.into();
        self
    }

    /// Sets the value of [crypto_key][crate::model::SearchProtectedResourcesRequest::crypto_key].
    pub fn set_crypto_key<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.crypto_key = v.into();
        self
    }

    /// Sets the value of [page_size][crate::model::SearchProtectedResourcesRequest::page_size].
    pub fn set_page_size<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of [page_token][crate::model::SearchProtectedResourcesRequest::page_token].
    pub fn set_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }

    /// Sets the value of [resource_types][crate::model::SearchProtectedResourcesRequest::resource_types].
    pub fn set_resource_types<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.resource_types = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for SearchProtectedResourcesRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.kms.inventory.v1.SearchProtectedResourcesRequest"
    }
}

/// Response message for
/// [KeyTrackingService.SearchProtectedResources][google.cloud.kms.inventory.v1.KeyTrackingService.SearchProtectedResources].
///
/// [google.cloud.kms.inventory.v1.KeyTrackingService.SearchProtectedResources]: crate::client::KeyTrackingService::search_protected_resources
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct SearchProtectedResourcesResponse {
    /// Protected resources for this page.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub protected_resources: std::vec::Vec<crate::model::ProtectedResource>,

    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub next_page_token: std::string::String,
}

impl SearchProtectedResourcesResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [next_page_token][crate::model::SearchProtectedResourcesResponse::next_page_token].
    pub fn set_next_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }

    /// Sets the value of [protected_resources][crate::model::SearchProtectedResourcesResponse::protected_resources].
    pub fn set_protected_resources<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::ProtectedResource>,
    {
        use std::iter::Iterator;
        self.protected_resources = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for SearchProtectedResourcesResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.kms.inventory.v1.SearchProtectedResourcesResponse"
    }
}

impl gax::paginator::PageableResponse for SearchProtectedResourcesResponse {
    type PageItem = crate::model::ProtectedResource;

    fn items(self) -> std::vec::Vec<Self::PageItem> {
        self.protected_resources
    }

    fn next_page_token(&self) -> std::string::String {
        use std::clone::Clone;
        self.next_page_token.clone()
    }
}

/// Metadata about a resource protected by a Cloud KMS key.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ProtectedResource {
    /// The full resource name of the resource.
    /// Example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Format: `projects/{PROJECT_NUMBER}`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub project: std::string::String,

    /// The ID of the project that owns the resource.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub project_id: std::string::String,

    /// The Cloud product that owns the resource.
    /// Example: `compute`
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub cloud_product: std::string::String,

    /// Example: `compute.googleapis.com/Disk`
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub resource_type: std::string::String,

    /// Location can be `global`, regional like `us-east1`, or zonal like
    /// `us-west1-b`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub location: std::string::String,

    /// A key-value pair of the resource's labels (v1) to their values.
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub labels: std::collections::HashMap<std::string::String, std::string::String>,

    /// The name of the Cloud KMS
    /// [CryptoKeyVersion](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys.cryptoKeyVersions?hl=en)
    /// used to protect this resource via CMEK. This field is empty if the
    /// Google Cloud product owning the resource does not provide key version data
    /// to Asset Inventory. If there are multiple key versions protecting the
    /// resource, then this is same value as the first element of
    /// crypto_key_versions.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub crypto_key_version: std::string::String,

    /// The names of the Cloud KMS
    /// [CryptoKeyVersion](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys.cryptoKeyVersions?hl=en)
    /// used to protect this resource via CMEK. This field is empty if the
    /// Google Cloud product owning the resource does not provide key versions data
    /// to Asset Inventory. The first element of this field is stored in
    /// crypto_key_version.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub crypto_key_versions: std::vec::Vec<std::string::String>,

    /// Output only. The time at which this resource was created. The granularity
    /// is in seconds. Timestamp.nanos will always be 0.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub create_time: std::option::Option<wkt::Timestamp>,
}

impl ProtectedResource {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::ProtectedResource::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [project][crate::model::ProtectedResource::project].
    pub fn set_project<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of [project_id][crate::model::ProtectedResource::project_id].
    pub fn set_project_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.project_id = v.into();
        self
    }

    /// Sets the value of [cloud_product][crate::model::ProtectedResource::cloud_product].
    pub fn set_cloud_product<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.cloud_product = v.into();
        self
    }

    /// Sets the value of [resource_type][crate::model::ProtectedResource::resource_type].
    pub fn set_resource_type<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.resource_type = v.into();
        self
    }

    /// Sets the value of [location][crate::model::ProtectedResource::location].
    pub fn set_location<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }

    /// Sets the value of [crypto_key_version][crate::model::ProtectedResource::crypto_key_version].
    pub fn set_crypto_key_version<T: std::convert::Into<std::string::String>>(
        mut self,
        v: T,
    ) -> Self {
        self.crypto_key_version = v.into();
        self
    }

    /// Sets the value of [create_time][crate::model::ProtectedResource::create_time].
    pub fn set_create_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.create_time = v.into();
        self
    }

    /// Sets the value of [crypto_key_versions][crate::model::ProtectedResource::crypto_key_versions].
    pub fn set_crypto_key_versions<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.crypto_key_versions = v.into_iter().map(|i| i.into()).collect();
        self
    }

    /// Sets the value of [labels][crate::model::ProtectedResource::labels].
    pub fn set_labels<T, K, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = (K, V)>,
        K: std::convert::Into<std::string::String>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.labels = v.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        self
    }
}

impl wkt::message::Message for ProtectedResource {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.kms.inventory.v1.ProtectedResource"
    }
}
