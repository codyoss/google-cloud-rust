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
extern crate lazy_static;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate std;
extern crate tracing;
extern crate wkt;

/// A request for connection information for a particular membership.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GenerateCredentialsRequest {
    /// Required. The Fleet membership resource.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Optional. Whether to force the use of Connect Agent-based transport.
    ///
    /// This will return a configuration that uses Connect Agent as the underlying
    /// transport mechanism for cluster types that would otherwise have used a
    /// different transport. Requires that Connect Agent be installed on the
    /// cluster. Setting this field to false is equivalent to not setting it.
    pub force_use_agent: bool,

    /// Optional. The Connect Gateway version to be used in the resulting
    /// configuration.
    ///
    /// Leave this field blank to let the server choose the version (recommended).
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub version: std::string::String,

    /// Optional. The namespace to use in the kubeconfig context.
    ///
    /// If this field is specified, the server will set the `namespace` field in
    /// kubeconfig context. If not specified, the `namespace` field is omitted.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub kubernetes_namespace: std::string::String,

    /// Optional. The operating system where the kubeconfig will be used.
    pub operating_system: crate::model::generate_credentials_request::OperatingSystem,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl GenerateCredentialsRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::GenerateCredentialsRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [force_use_agent][crate::model::GenerateCredentialsRequest::force_use_agent].
    pub fn set_force_use_agent<T: std::convert::Into<bool>>(mut self, v: T) -> Self {
        self.force_use_agent = v.into();
        self
    }

    /// Sets the value of [version][crate::model::GenerateCredentialsRequest::version].
    pub fn set_version<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.version = v.into();
        self
    }

    /// Sets the value of [kubernetes_namespace][crate::model::GenerateCredentialsRequest::kubernetes_namespace].
    pub fn set_kubernetes_namespace<T: std::convert::Into<std::string::String>>(
        mut self,
        v: T,
    ) -> Self {
        self.kubernetes_namespace = v.into();
        self
    }

    /// Sets the value of [operating_system][crate::model::GenerateCredentialsRequest::operating_system].
    pub fn set_operating_system<
        T: std::convert::Into<crate::model::generate_credentials_request::OperatingSystem>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.operating_system = v.into();
        self
    }
}

impl wkt::message::Message for GenerateCredentialsRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.gkeconnect.gateway.v1.GenerateCredentialsRequest"
    }
}

/// Defines additional types related to [GenerateCredentialsRequest].
pub mod generate_credentials_request {
    #[allow(unused_imports)]
    use super::*;

    /// Operating systems requiring specialized kubeconfigs.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct OperatingSystem(i32);

    impl OperatingSystem {
        /// Generates a kubeconfig that works for all operating systems not defined
        /// below.
        pub const OPERATING_SYSTEM_UNSPECIFIED: OperatingSystem = OperatingSystem::new(0);

        /// Generates a kubeconfig that is specifically designed to work with
        /// Windows.
        pub const OPERATING_SYSTEM_WINDOWS: OperatingSystem = OperatingSystem::new(1);

        /// Creates a new OperatingSystem instance.
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
                0 => std::borrow::Cow::Borrowed("OPERATING_SYSTEM_UNSPECIFIED"),
                1 => std::borrow::Cow::Borrowed("OPERATING_SYSTEM_WINDOWS"),
                _ => std::borrow::Cow::Owned(std::format!("UNKNOWN-VALUE:{}", self.0)),
            }
        }

        /// Creates an enum value from the value name.
        pub fn from_str_name(name: &str) -> std::option::Option<Self> {
            match name {
                "OPERATING_SYSTEM_UNSPECIFIED" => {
                    std::option::Option::Some(Self::OPERATING_SYSTEM_UNSPECIFIED)
                }
                "OPERATING_SYSTEM_WINDOWS" => {
                    std::option::Option::Some(Self::OPERATING_SYSTEM_WINDOWS)
                }
                _ => std::option::Option::None,
            }
        }
    }

    impl std::convert::From<i32> for OperatingSystem {
        fn from(value: i32) -> Self {
            Self::new(value)
        }
    }

    impl std::default::Default for OperatingSystem {
        fn default() -> Self {
            Self::new(0)
        }
    }
}

/// Connection information for a particular membership.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GenerateCredentialsResponse {
    /// A full YAML kubeconfig in serialized format.
    #[serde(skip_serializing_if = "::bytes::Bytes::is_empty")]
    #[serde_as(as = "serde_with::base64::Base64")]
    pub kubeconfig: ::bytes::Bytes,

    /// The generated URI of the cluster as accessed through the Connect Gateway
    /// API.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub endpoint: std::string::String,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl GenerateCredentialsResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [kubeconfig][crate::model::GenerateCredentialsResponse::kubeconfig].
    pub fn set_kubeconfig<T: std::convert::Into<::bytes::Bytes>>(mut self, v: T) -> Self {
        self.kubeconfig = v.into();
        self
    }

    /// Sets the value of [endpoint][crate::model::GenerateCredentialsResponse::endpoint].
    pub fn set_endpoint<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.endpoint = v.into();
        self
    }
}

impl wkt::message::Message for GenerateCredentialsResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.gkeconnect.gateway.v1.GenerateCredentialsResponse"
    }
}
