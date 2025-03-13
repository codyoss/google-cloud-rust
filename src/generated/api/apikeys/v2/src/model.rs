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
extern crate lazy_static;
extern crate longrunning;
extern crate lro;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate std;
extern crate tracing;
extern crate wkt;

/// Request message for `CreateKey` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct CreateKeyRequest {
    /// Required. The project in which the API key is created.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// Required. The API key fields to set at creation time.
    /// You can configure only the `display_name`, `restrictions`, and
    /// `annotations` fields.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub key: std::option::Option<crate::model::Key>,

    /// User specified key id (optional). If specified, it will become the final
    /// component of the key resource name.
    ///
    /// The id must be unique within the project, must conform with RFC-1034,
    /// is restricted to lower-cased letters, and has a maximum length of 63
    /// characters. In another word, the id must match the regular
    /// expression: `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`.
    ///
    /// The id must NOT be a UUID-like string.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub key_id: std::string::String,
}

impl CreateKeyRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::CreateKeyRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [key][crate::model::CreateKeyRequest::key].
    pub fn set_key<T: std::convert::Into<std::option::Option<crate::model::Key>>>(
        mut self,
        v: T,
    ) -> Self {
        self.key = v.into();
        self
    }

    /// Sets the value of [key_id][crate::model::CreateKeyRequest::key_id].
    pub fn set_key_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.key_id = v.into();
        self
    }
}

impl wkt::message::Message for CreateKeyRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.CreateKeyRequest"
    }
}

/// Request message for `ListKeys` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListKeysRequest {
    /// Required. Lists all API keys associated with this project.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// Optional. Specifies the maximum number of results to be returned at a time.
    pub page_size: i32,

    /// Optional. Requests a specific page of results.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub page_token: std::string::String,

    /// Optional. Indicate that keys deleted in the past 30 days should also be
    /// returned.
    pub show_deleted: bool,
}

impl ListKeysRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::ListKeysRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [page_size][crate::model::ListKeysRequest::page_size].
    pub fn set_page_size<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of [page_token][crate::model::ListKeysRequest::page_token].
    pub fn set_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }

    /// Sets the value of [show_deleted][crate::model::ListKeysRequest::show_deleted].
    pub fn set_show_deleted<T: std::convert::Into<bool>>(mut self, v: T) -> Self {
        self.show_deleted = v.into();
        self
    }
}

impl wkt::message::Message for ListKeysRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.ListKeysRequest"
    }
}

/// Response message for `ListKeys` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListKeysResponse {
    /// A list of API keys.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub keys: std::vec::Vec<crate::model::Key>,

    /// The pagination token for the next page of results.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub next_page_token: std::string::String,
}

impl ListKeysResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [next_page_token][crate::model::ListKeysResponse::next_page_token].
    pub fn set_next_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }

    /// Sets the value of [keys][crate::model::ListKeysResponse::keys].
    pub fn set_keys<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::Key>,
    {
        use std::iter::Iterator;
        self.keys = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for ListKeysResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.ListKeysResponse"
    }
}

impl gax::paginator::PageableResponse for ListKeysResponse {
    type PageItem = crate::model::Key;

    fn items(self) -> std::vec::Vec<Self::PageItem> {
        self.keys
    }

    fn next_page_token(&self) -> std::string::String {
        gax::paginator::extract_token(&self.next_page_token)
    }
}

/// Request message for `GetKey` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetKeyRequest {
    /// Required. The resource name of the API key to get.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,
}

impl GetKeyRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::GetKeyRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for GetKeyRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.GetKeyRequest"
    }
}

/// Request message for `GetKeyString` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetKeyStringRequest {
    /// Required. The resource name of the API key to be retrieved.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,
}

impl GetKeyStringRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::GetKeyStringRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for GetKeyStringRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.GetKeyStringRequest"
    }
}

/// Response message for `GetKeyString` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetKeyStringResponse {
    /// An encrypted and signed value of the key.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub key_string: std::string::String,
}

impl GetKeyStringResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [key_string][crate::model::GetKeyStringResponse::key_string].
    pub fn set_key_string<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.key_string = v.into();
        self
    }
}

impl wkt::message::Message for GetKeyStringResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.GetKeyStringResponse"
    }
}

/// Request message for `UpdateKey` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct UpdateKeyRequest {
    /// Required. Set the `name` field to the resource name of the API key to be
    /// updated. You can update only the `display_name`, `restrictions`, and
    /// `annotations` fields.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub key: std::option::Option<crate::model::Key>,

    /// The field mask specifies which fields to be updated as part of this
    /// request. All other fields are ignored.
    /// Mutable fields are: `display_name`, `restrictions`, and `annotations`.
    /// If an update mask is not provided, the service treats it as an implied mask
    /// equivalent to all allowed fields that are set on the wire. If the field
    /// mask has a special value "*", the service treats it equivalent to replace
    /// all allowed mutable fields.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub update_mask: std::option::Option<wkt::FieldMask>,
}

impl UpdateKeyRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [key][crate::model::UpdateKeyRequest::key].
    pub fn set_key<T: std::convert::Into<std::option::Option<crate::model::Key>>>(
        mut self,
        v: T,
    ) -> Self {
        self.key = v.into();
        self
    }

    /// Sets the value of [update_mask][crate::model::UpdateKeyRequest::update_mask].
    pub fn set_update_mask<T: std::convert::Into<std::option::Option<wkt::FieldMask>>>(
        mut self,
        v: T,
    ) -> Self {
        self.update_mask = v.into();
        self
    }
}

impl wkt::message::Message for UpdateKeyRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.UpdateKeyRequest"
    }
}

/// Request message for `DeleteKey` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeleteKeyRequest {
    /// Required. The resource name of the API key to be deleted.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Optional. The etag known to the client for the expected state of the key.
    /// This is to be used for optimistic concurrency.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub etag: std::string::String,
}

impl DeleteKeyRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::DeleteKeyRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [etag][crate::model::DeleteKeyRequest::etag].
    pub fn set_etag<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.etag = v.into();
        self
    }
}

impl wkt::message::Message for DeleteKeyRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.DeleteKeyRequest"
    }
}

/// Request message for `UndeleteKey` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct UndeleteKeyRequest {
    /// Required. The resource name of the API key to be undeleted.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,
}

impl UndeleteKeyRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::UndeleteKeyRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for UndeleteKeyRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.UndeleteKeyRequest"
    }
}

/// Request message for `LookupKey` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct LookupKeyRequest {
    /// Required. Finds the project that owns the key string value.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub key_string: std::string::String,
}

impl LookupKeyRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [key_string][crate::model::LookupKeyRequest::key_string].
    pub fn set_key_string<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.key_string = v.into();
        self
    }
}

impl wkt::message::Message for LookupKeyRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.LookupKeyRequest"
    }
}

/// Response message for `LookupKey` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct LookupKeyResponse {
    /// The project that owns the key with the value specified in the request.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// The resource name of the API key. If the API key has been purged,
    /// resource name is empty.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,
}

impl LookupKeyResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::LookupKeyResponse::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [name][crate::model::LookupKeyResponse::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for LookupKeyResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.LookupKeyResponse"
    }
}

/// The representation of a key managed by the API Keys API.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Key {
    /// Output only. The resource name of the key.
    /// The `name` has the form:
    /// `projects/<PROJECT_NUMBER>/locations/global/keys/<KEY_ID>`.
    /// For example:
    /// `projects/123456867718/locations/global/keys/b7ff1f9f-8275-410a-94dd-3855ee9b5dd2`
    ///
    /// NOTE: Key is a global resource; hence the only supported value for
    /// location is `global`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Output only. Unique id in UUID4 format.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub uid: std::string::String,

    /// Human-readable display name of this key that you can modify.
    /// The maximum length is 63 characters.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub display_name: std::string::String,

    /// Output only. An encrypted and signed value held by this key.
    /// This field can be accessed only through the `GetKeyString` method.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub key_string: std::string::String,

    /// Output only. A timestamp identifying the time this key was originally
    /// created.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub create_time: std::option::Option<wkt::Timestamp>,

    /// Output only. A timestamp identifying the time this key was last
    /// updated.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub update_time: std::option::Option<wkt::Timestamp>,

    /// Output only. A timestamp when this key was deleted. If the resource is not
    /// deleted, this must be empty.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub delete_time: std::option::Option<wkt::Timestamp>,

    /// Annotations is an unstructured key-value map stored with a policy that
    /// may be set by external tools to store and retrieve arbitrary metadata.
    /// They are not queryable and should be preserved when modifying objects.
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub annotations: std::collections::HashMap<std::string::String, std::string::String>,

    /// Key restrictions.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub restrictions: std::option::Option<crate::model::Restrictions>,

    /// Output only. A checksum computed by the server based on the current value
    /// of the Key resource. This may be sent on update and delete requests to
    /// ensure the client has an up-to-date value before proceeding. See
    /// <https://google.aip.dev/154>.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub etag: std::string::String,
}

impl Key {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::Key::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [uid][crate::model::Key::uid].
    pub fn set_uid<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.uid = v.into();
        self
    }

    /// Sets the value of [display_name][crate::model::Key::display_name].
    pub fn set_display_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.display_name = v.into();
        self
    }

    /// Sets the value of [key_string][crate::model::Key::key_string].
    pub fn set_key_string<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.key_string = v.into();
        self
    }

    /// Sets the value of [create_time][crate::model::Key::create_time].
    pub fn set_create_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.create_time = v.into();
        self
    }

    /// Sets the value of [update_time][crate::model::Key::update_time].
    pub fn set_update_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.update_time = v.into();
        self
    }

    /// Sets the value of [delete_time][crate::model::Key::delete_time].
    pub fn set_delete_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.delete_time = v.into();
        self
    }

    /// Sets the value of [restrictions][crate::model::Key::restrictions].
    pub fn set_restrictions<
        T: std::convert::Into<std::option::Option<crate::model::Restrictions>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.restrictions = v.into();
        self
    }

    /// Sets the value of [etag][crate::model::Key::etag].
    pub fn set_etag<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.etag = v.into();
        self
    }

    /// Sets the value of [annotations][crate::model::Key::annotations].
    pub fn set_annotations<T, K, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = (K, V)>,
        K: std::convert::Into<std::string::String>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.annotations = v.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        self
    }
}

impl wkt::message::Message for Key {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.Key"
    }
}

/// Describes the restrictions on the key.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Restrictions {
    /// A restriction for a specific service and optionally one or
    /// more specific methods. Requests are allowed if they
    /// match any of these restrictions. If no restrictions are
    /// specified, all targets are allowed.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub api_targets: std::vec::Vec<crate::model::ApiTarget>,

    /// The websites, IP addresses, Android apps, or iOS apps (the clients) that
    /// are allowed to use the key. You can specify only one type of client
    /// restrictions per key.
    #[serde(flatten, skip_serializing_if = "std::option::Option::is_none")]
    pub client_restrictions: std::option::Option<crate::model::restrictions::ClientRestrictions>,
}

impl Restrictions {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [api_targets][crate::model::Restrictions::api_targets].
    pub fn set_api_targets<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::ApiTarget>,
    {
        use std::iter::Iterator;
        self.api_targets = v.into_iter().map(|i| i.into()).collect();
        self
    }

    /// Sets the value of `client_restrictions`.
    pub fn set_client_restrictions<
        T: std::convert::Into<std::option::Option<crate::model::restrictions::ClientRestrictions>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.client_restrictions = v.into();
        self
    }

    /// The value of [client_restrictions][crate::model::Restrictions::client_restrictions]
    /// if it holds a `BrowserKeyRestrictions`, `None` if the field is not set or
    /// holds a different branch.
    pub fn get_browser_key_restrictions(
        &self,
    ) -> std::option::Option<&std::boxed::Box<crate::model::BrowserKeyRestrictions>> {
        #[allow(unreachable_patterns)]
        self.client_restrictions.as_ref().and_then(|v| match v {
            crate::model::restrictions::ClientRestrictions::BrowserKeyRestrictions(v) => {
                std::option::Option::Some(v)
            }
            _ => std::option::Option::None,
        })
    }

    /// The value of [client_restrictions][crate::model::Restrictions::client_restrictions]
    /// if it holds a `ServerKeyRestrictions`, `None` if the field is not set or
    /// holds a different branch.
    pub fn get_server_key_restrictions(
        &self,
    ) -> std::option::Option<&std::boxed::Box<crate::model::ServerKeyRestrictions>> {
        #[allow(unreachable_patterns)]
        self.client_restrictions.as_ref().and_then(|v| match v {
            crate::model::restrictions::ClientRestrictions::ServerKeyRestrictions(v) => {
                std::option::Option::Some(v)
            }
            _ => std::option::Option::None,
        })
    }

    /// The value of [client_restrictions][crate::model::Restrictions::client_restrictions]
    /// if it holds a `AndroidKeyRestrictions`, `None` if the field is not set or
    /// holds a different branch.
    pub fn get_android_key_restrictions(
        &self,
    ) -> std::option::Option<&std::boxed::Box<crate::model::AndroidKeyRestrictions>> {
        #[allow(unreachable_patterns)]
        self.client_restrictions.as_ref().and_then(|v| match v {
            crate::model::restrictions::ClientRestrictions::AndroidKeyRestrictions(v) => {
                std::option::Option::Some(v)
            }
            _ => std::option::Option::None,
        })
    }

    /// The value of [client_restrictions][crate::model::Restrictions::client_restrictions]
    /// if it holds a `IosKeyRestrictions`, `None` if the field is not set or
    /// holds a different branch.
    pub fn get_ios_key_restrictions(
        &self,
    ) -> std::option::Option<&std::boxed::Box<crate::model::IosKeyRestrictions>> {
        #[allow(unreachable_patterns)]
        self.client_restrictions.as_ref().and_then(|v| match v {
            crate::model::restrictions::ClientRestrictions::IosKeyRestrictions(v) => {
                std::option::Option::Some(v)
            }
            _ => std::option::Option::None,
        })
    }

    /// Sets the value of [client_restrictions][crate::model::Restrictions::client_restrictions]
    /// to hold a `BrowserKeyRestrictions`.
    ///
    /// Note that all the setters affecting `client_restrictions` are
    /// mutually exclusive.
    pub fn set_browser_key_restrictions<
        T: std::convert::Into<std::boxed::Box<crate::model::BrowserKeyRestrictions>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.client_restrictions = std::option::Option::Some(
            crate::model::restrictions::ClientRestrictions::BrowserKeyRestrictions(v.into()),
        );
        self
    }

    /// Sets the value of [client_restrictions][crate::model::Restrictions::client_restrictions]
    /// to hold a `ServerKeyRestrictions`.
    ///
    /// Note that all the setters affecting `client_restrictions` are
    /// mutually exclusive.
    pub fn set_server_key_restrictions<
        T: std::convert::Into<std::boxed::Box<crate::model::ServerKeyRestrictions>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.client_restrictions = std::option::Option::Some(
            crate::model::restrictions::ClientRestrictions::ServerKeyRestrictions(v.into()),
        );
        self
    }

    /// Sets the value of [client_restrictions][crate::model::Restrictions::client_restrictions]
    /// to hold a `AndroidKeyRestrictions`.
    ///
    /// Note that all the setters affecting `client_restrictions` are
    /// mutually exclusive.
    pub fn set_android_key_restrictions<
        T: std::convert::Into<std::boxed::Box<crate::model::AndroidKeyRestrictions>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.client_restrictions = std::option::Option::Some(
            crate::model::restrictions::ClientRestrictions::AndroidKeyRestrictions(v.into()),
        );
        self
    }

    /// Sets the value of [client_restrictions][crate::model::Restrictions::client_restrictions]
    /// to hold a `IosKeyRestrictions`.
    ///
    /// Note that all the setters affecting `client_restrictions` are
    /// mutually exclusive.
    pub fn set_ios_key_restrictions<
        T: std::convert::Into<std::boxed::Box<crate::model::IosKeyRestrictions>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.client_restrictions = std::option::Option::Some(
            crate::model::restrictions::ClientRestrictions::IosKeyRestrictions(v.into()),
        );
        self
    }
}

impl wkt::message::Message for Restrictions {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.Restrictions"
    }
}

/// Defines additional types related to Restrictions
pub mod restrictions {
    #[allow(unused_imports)]
    use super::*;

    /// The websites, IP addresses, Android apps, or iOS apps (the clients) that
    /// are allowed to use the key. You can specify only one type of client
    /// restrictions per key.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub enum ClientRestrictions {
        /// The HTTP referrers (websites) that are allowed to use the key.
        BrowserKeyRestrictions(std::boxed::Box<crate::model::BrowserKeyRestrictions>),
        /// The IP addresses of callers that are allowed to use the key.
        ServerKeyRestrictions(std::boxed::Box<crate::model::ServerKeyRestrictions>),
        /// The Android apps that are allowed to use the key.
        AndroidKeyRestrictions(std::boxed::Box<crate::model::AndroidKeyRestrictions>),
        /// The iOS apps that are allowed to use the key.
        IosKeyRestrictions(std::boxed::Box<crate::model::IosKeyRestrictions>),
    }
}

/// The HTTP referrers (websites) that are allowed to use the key.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct BrowserKeyRestrictions {
    /// A list of regular expressions for the referrer URLs that are allowed
    /// to make API calls with this key.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub allowed_referrers: std::vec::Vec<std::string::String>,
}

impl BrowserKeyRestrictions {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [allowed_referrers][crate::model::BrowserKeyRestrictions::allowed_referrers].
    pub fn set_allowed_referrers<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.allowed_referrers = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for BrowserKeyRestrictions {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.BrowserKeyRestrictions"
    }
}

/// The IP addresses of callers that are allowed to use the key.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ServerKeyRestrictions {
    /// A list of the caller IP addresses that are allowed to make API calls
    /// with this key.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub allowed_ips: std::vec::Vec<std::string::String>,
}

impl ServerKeyRestrictions {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [allowed_ips][crate::model::ServerKeyRestrictions::allowed_ips].
    pub fn set_allowed_ips<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.allowed_ips = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for ServerKeyRestrictions {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.ServerKeyRestrictions"
    }
}

/// The Android apps that are allowed to use the key.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct AndroidKeyRestrictions {
    /// A list of Android applications that are allowed to make API calls with
    /// this key.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub allowed_applications: std::vec::Vec<crate::model::AndroidApplication>,
}

impl AndroidKeyRestrictions {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [allowed_applications][crate::model::AndroidKeyRestrictions::allowed_applications].
    pub fn set_allowed_applications<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::AndroidApplication>,
    {
        use std::iter::Iterator;
        self.allowed_applications = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for AndroidKeyRestrictions {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.AndroidKeyRestrictions"
    }
}

/// Identifier of an Android application for key use.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct AndroidApplication {
    /// The SHA1 fingerprint of the application. For example, both sha1 formats are
    /// acceptable : DA:39:A3:EE:5E:6B:4B:0D:32:55:BF:EF:95:60:18:90:AF:D8:07:09 or
    /// DA39A3EE5E6B4B0D3255BFEF95601890AFD80709.
    /// Output format is the latter.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub sha1_fingerprint: std::string::String,

    /// The package name of the application.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub package_name: std::string::String,
}

impl AndroidApplication {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [sha1_fingerprint][crate::model::AndroidApplication::sha1_fingerprint].
    pub fn set_sha1_fingerprint<T: std::convert::Into<std::string::String>>(
        mut self,
        v: T,
    ) -> Self {
        self.sha1_fingerprint = v.into();
        self
    }

    /// Sets the value of [package_name][crate::model::AndroidApplication::package_name].
    pub fn set_package_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.package_name = v.into();
        self
    }
}

impl wkt::message::Message for AndroidApplication {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.AndroidApplication"
    }
}

/// The iOS apps that are allowed to use the key.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct IosKeyRestrictions {
    /// A list of bundle IDs that are allowed when making API calls with this key.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub allowed_bundle_ids: std::vec::Vec<std::string::String>,
}

impl IosKeyRestrictions {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [allowed_bundle_ids][crate::model::IosKeyRestrictions::allowed_bundle_ids].
    pub fn set_allowed_bundle_ids<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.allowed_bundle_ids = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for IosKeyRestrictions {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.IosKeyRestrictions"
    }
}

/// A restriction for a specific service and optionally one or multiple
/// specific methods. Both fields are case insensitive.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ApiTarget {
    /// The service for this restriction. It should be the canonical
    /// service name, for example: `translate.googleapis.com`.
    /// You can use [`gcloud services list`](/sdk/gcloud/reference/services/list)
    /// to get a list of services that are enabled in the project.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub service: std::string::String,

    /// Optional. List of one or more methods that can be called.
    /// If empty, all methods for the service are allowed. A wildcard
    /// (*) can be used as the last symbol.
    /// Valid examples:
    /// `google.cloud.translate.v2.TranslateService.GetSupportedLanguage`
    /// `TranslateText`
    /// `Get*`
    /// `translate.googleapis.com.Get*`
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub methods: std::vec::Vec<std::string::String>,
}

impl ApiTarget {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [service][crate::model::ApiTarget::service].
    pub fn set_service<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.service = v.into();
        self
    }

    /// Sets the value of [methods][crate::model::ApiTarget::methods].
    pub fn set_methods<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.methods = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for ApiTarget {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.apikeys.v2.ApiTarget"
    }
}
