// Copyright 2024 Google LLC
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
extern crate lazy_static;
extern crate reqwest;
extern crate rpc;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate std;
extern crate tracing;
extern crate wkt;

/// This resource represents a long-running operation that is the result of a
/// network API call.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Operation {
    /// The server-assigned name, which is only unique within the same service that
    /// originally returns it. If you use the default HTTP mapping, the
    /// `name` should be a resource name ending with `operations/{unique_id}`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Service-specific metadata associated with the operation.  It typically
    /// contains progress information and common metadata such as create time.
    /// Some services might not provide such metadata.  Any method that returns a
    /// long-running operation should document the metadata type, if any.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub metadata: std::option::Option<wkt::Any>,

    /// If the value is `false`, it means the operation is still in progress.
    /// If `true`, the operation is completed, and either `error` or `response` is
    /// available.
    pub done: bool,

    /// The operation result, which can be either an `error` or a valid `response`.
    /// If `done` == `false`, neither `error` nor `response` is set.
    /// If `done` == `true`, exactly one of `error` or `response` can be set.
    /// Some services might not provide the result.
    #[serde(flatten, skip_serializing_if = "std::option::Option::is_none")]
    pub result: std::option::Option<crate::model::operation::Result>,
}

impl Operation {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::Operation::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [metadata][crate::model::Operation::metadata].
    pub fn set_metadata<T: std::convert::Into<std::option::Option<wkt::Any>>>(
        mut self,
        v: T,
    ) -> Self {
        self.metadata = v.into();
        self
    }

    /// Sets the value of [done][crate::model::Operation::done].
    pub fn set_done<T: std::convert::Into<bool>>(mut self, v: T) -> Self {
        self.done = v.into();
        self
    }

    /// Sets the value of `result`.
    pub fn set_result<
        T: std::convert::Into<std::option::Option<crate::model::operation::Result>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.result = v.into();
        self
    }

    /// The value of [result][crate::model::Operation::result]
    /// if it holds a `Error`, `None` if the field is not set or
    /// holds a different branch.
    pub fn get_error(&self) -> std::option::Option<&std::boxed::Box<rpc::model::Status>> {
        #[allow(unreachable_patterns)]
        self.result.as_ref().and_then(|v| match v {
            crate::model::operation::Result::Error(v) => std::option::Option::Some(v),
            _ => std::option::Option::None,
        })
    }

    /// The value of [result][crate::model::Operation::result]
    /// if it holds a `Response`, `None` if the field is not set or
    /// holds a different branch.
    pub fn get_response(&self) -> std::option::Option<&std::boxed::Box<wkt::Any>> {
        #[allow(unreachable_patterns)]
        self.result.as_ref().and_then(|v| match v {
            crate::model::operation::Result::Response(v) => std::option::Option::Some(v),
            _ => std::option::Option::None,
        })
    }

    /// Sets the value of [result][crate::model::Operation::result]
    /// to hold a `Error`.
    ///
    /// Note that all the setters affecting `result` are
    /// mutually exclusive.
    pub fn set_error<T: std::convert::Into<std::boxed::Box<rpc::model::Status>>>(
        mut self,
        v: T,
    ) -> Self {
        self.result = std::option::Option::Some(crate::model::operation::Result::Error(v.into()));
        self
    }

    /// Sets the value of [result][crate::model::Operation::result]
    /// to hold a `Response`.
    ///
    /// Note that all the setters affecting `result` are
    /// mutually exclusive.
    pub fn set_response<T: std::convert::Into<std::boxed::Box<wkt::Any>>>(mut self, v: T) -> Self {
        self.result =
            std::option::Option::Some(crate::model::operation::Result::Response(v.into()));
        self
    }
}

impl wkt::message::Message for Operation {
    fn typename() -> &'static str {
        "type.googleapis.com/google.longrunning.Operation"
    }
}

/// Defines additional types related to Operation
pub mod operation {
    #[allow(unused_imports)]
    use super::*;

    /// The operation result, which can be either an `error` or a valid `response`.
    /// If `done` == `false`, neither `error` nor `response` is set.
    /// If `done` == `true`, exactly one of `error` or `response` can be set.
    /// Some services might not provide the result.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub enum Result {
        /// The error result of the operation in case of failure or cancellation.
        Error(std::boxed::Box<rpc::model::Status>),
        /// The normal, successful response of the operation.  If the original
        /// method returns no data on success, such as `Delete`, the response is
        /// `google.protobuf.Empty`.  If the original method is standard
        /// `Get`/`Create`/`Update`, the response should be the resource.  For other
        /// methods, the response should have the type `XxxResponse`, where `Xxx`
        /// is the original method name.  For example, if the original method name
        /// is `TakeSnapshot()`, the inferred response type is
        /// `TakeSnapshotResponse`.
        Response(std::boxed::Box<wkt::Any>),
    }
}

/// The request message for
/// [Operations.GetOperation][google.longrunning.Operations.GetOperation].
///
/// [google.longrunning.Operations.GetOperation]: crate::client::Operations::get_operation
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetOperationRequest {
    /// The name of the operation resource.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,
}

impl GetOperationRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::GetOperationRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for GetOperationRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.longrunning.GetOperationRequest"
    }
}

/// The request message for
/// [Operations.ListOperations][google.longrunning.Operations.ListOperations].
///
/// [google.longrunning.Operations.ListOperations]: crate::client::Operations::list_operations
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListOperationsRequest {
    /// The name of the operation's parent resource.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// The standard list filter.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub filter: std::string::String,

    /// The standard list page size.
    pub page_size: i32,

    /// The standard list page token.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub page_token: std::string::String,
}

impl ListOperationsRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::ListOperationsRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [filter][crate::model::ListOperationsRequest::filter].
    pub fn set_filter<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }

    /// Sets the value of [page_size][crate::model::ListOperationsRequest::page_size].
    pub fn set_page_size<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of [page_token][crate::model::ListOperationsRequest::page_token].
    pub fn set_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }
}

impl wkt::message::Message for ListOperationsRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.longrunning.ListOperationsRequest"
    }
}

/// The response message for
/// [Operations.ListOperations][google.longrunning.Operations.ListOperations].
///
/// [google.longrunning.Operations.ListOperations]: crate::client::Operations::list_operations
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListOperationsResponse {
    /// A list of operations that matches the specified filter in the request.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub operations: std::vec::Vec<crate::model::Operation>,

    /// The standard List next-page token.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub next_page_token: std::string::String,
}

impl ListOperationsResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [next_page_token][crate::model::ListOperationsResponse::next_page_token].
    pub fn set_next_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }

    /// Sets the value of [operations][crate::model::ListOperationsResponse::operations].
    pub fn set_operations<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::Operation>,
    {
        use std::iter::Iterator;
        self.operations = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for ListOperationsResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.longrunning.ListOperationsResponse"
    }
}

impl gax::paginator::PageableResponse for ListOperationsResponse {
    type PageItem = crate::model::Operation;

    fn items(self) -> std::vec::Vec<Self::PageItem> {
        self.operations
    }

    fn next_page_token(&self) -> std::string::String {
        use std::clone::Clone;
        self.next_page_token.clone()
    }
}

/// The request message for
/// [Operations.CancelOperation][google.longrunning.Operations.CancelOperation].
///
/// [google.longrunning.Operations.CancelOperation]: crate::client::Operations::cancel_operation
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct CancelOperationRequest {
    /// The name of the operation resource to be cancelled.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,
}

impl CancelOperationRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::CancelOperationRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for CancelOperationRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.longrunning.CancelOperationRequest"
    }
}

/// The request message for
/// [Operations.DeleteOperation][google.longrunning.Operations.DeleteOperation].
///
/// [google.longrunning.Operations.DeleteOperation]: crate::client::Operations::delete_operation
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeleteOperationRequest {
    /// The name of the operation resource to be deleted.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,
}

impl DeleteOperationRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::DeleteOperationRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for DeleteOperationRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.longrunning.DeleteOperationRequest"
    }
}

/// The request message for
/// [Operations.WaitOperation][google.longrunning.Operations.WaitOperation].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct WaitOperationRequest {
    /// The name of the operation resource to wait on.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// The maximum duration to wait before timing out. If left blank, the wait
    /// will be at most the time permitted by the underlying HTTP/RPC protocol.
    /// If RPC context deadline is also specified, the shorter one will be used.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub timeout: std::option::Option<wkt::Duration>,
}

impl WaitOperationRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::WaitOperationRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [timeout][crate::model::WaitOperationRequest::timeout].
    pub fn set_timeout<T: std::convert::Into<std::option::Option<wkt::Duration>>>(
        mut self,
        v: T,
    ) -> Self {
        self.timeout = v.into();
        self
    }
}

impl wkt::message::Message for WaitOperationRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.longrunning.WaitOperationRequest"
    }
}

/// A message representing the message types used by a long-running operation.
///
/// Example:
///
/// ```norust
/// rpc Export(ExportRequest) returns (google.longrunning.Operation) {
///   option (google.longrunning.operation_info) = {
///     response_type: "ExportResponse"
///     metadata_type: "ExportMetadata"
///   };
/// }
/// ```
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct OperationInfo {
    /// Required. The message name of the primary return type for this
    /// long-running operation.
    /// This type will be used to deserialize the LRO's response.
    ///
    /// If the response is in a different package from the rpc, a fully-qualified
    /// message name must be used (e.g. `google.protobuf.Struct`).
    ///
    /// Note: Altering this value constitutes a breaking change.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub response_type: std::string::String,

    /// Required. The message name of the metadata type for this long-running
    /// operation.
    ///
    /// If the response is in a different package from the rpc, a fully-qualified
    /// message name must be used (e.g. `google.protobuf.Struct`).
    ///
    /// Note: Altering this value constitutes a breaking change.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub metadata_type: std::string::String,
}

impl OperationInfo {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [response_type][crate::model::OperationInfo::response_type].
    pub fn set_response_type<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.response_type = v.into();
        self
    }

    /// Sets the value of [metadata_type][crate::model::OperationInfo::metadata_type].
    pub fn set_metadata_type<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.metadata_type = v.into();
        self
    }
}

impl wkt::message::Message for OperationInfo {
    fn typename() -> &'static str {
        "type.googleapis.com/google.longrunning.OperationInfo"
    }
}
