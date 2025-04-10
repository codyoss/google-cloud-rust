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
extern crate longrunning;
extern crate lro;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate std;
extern crate tracing;
extern crate wkt;

/// A Cloud Shell environment, which is defined as the combination of a Docker
/// image specifying what is installed on the environment and a home directory
/// containing the user's data that will remain across sessions. Each user has
/// at least an environment with the ID "default".
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Environment {
    /// Immutable. Full name of this resource, in the format
    /// `users/{owner_email}/environments/{environment_id}`. `{owner_email}` is the
    /// email address of the user to whom this environment belongs, and
    /// `{environment_id}` is the identifier of this environment. For example,
    /// `users/someone@example.com/environments/default`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Output only. The environment's identifier, unique among the user's
    /// environments.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub id: std::string::String,

    /// Required. Immutable. Full path to the Docker image used to run this environment, e.g.
    /// "gcr.io/dev-con/cloud-devshell:latest".
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub docker_image: std::string::String,

    /// Output only. Current execution state of this environment.
    pub state: crate::model::environment::State,

    /// Output only. Host to which clients can connect to initiate HTTPS or WSS
    /// connections with the environment.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub web_host: std::string::String,

    /// Output only. Username that clients should use when initiating SSH sessions
    /// with the environment.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub ssh_username: std::string::String,

    /// Output only. Host to which clients can connect to initiate SSH sessions
    /// with the environment.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub ssh_host: std::string::String,

    /// Output only. Port to which clients can connect to initiate SSH sessions
    /// with the environment.
    pub ssh_port: i32,

    /// Output only. Public keys associated with the environment. Clients can
    /// connect to this environment via SSH only if they possess a private key
    /// corresponding to at least one of these public keys. Keys can be added to or
    /// removed from the environment using the AddPublicKey and RemovePublicKey
    /// methods.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub public_keys: std::vec::Vec<std::string::String>,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl Environment {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::Environment::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [id][crate::model::Environment::id].
    pub fn set_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.id = v.into();
        self
    }

    /// Sets the value of [docker_image][crate::model::Environment::docker_image].
    pub fn set_docker_image<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.docker_image = v.into();
        self
    }

    /// Sets the value of [state][crate::model::Environment::state].
    pub fn set_state<T: std::convert::Into<crate::model::environment::State>>(
        mut self,
        v: T,
    ) -> Self {
        self.state = v.into();
        self
    }

    /// Sets the value of [web_host][crate::model::Environment::web_host].
    pub fn set_web_host<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.web_host = v.into();
        self
    }

    /// Sets the value of [ssh_username][crate::model::Environment::ssh_username].
    pub fn set_ssh_username<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.ssh_username = v.into();
        self
    }

    /// Sets the value of [ssh_host][crate::model::Environment::ssh_host].
    pub fn set_ssh_host<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.ssh_host = v.into();
        self
    }

    /// Sets the value of [ssh_port][crate::model::Environment::ssh_port].
    pub fn set_ssh_port<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.ssh_port = v.into();
        self
    }

    /// Sets the value of [public_keys][crate::model::Environment::public_keys].
    pub fn set_public_keys<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.public_keys = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for Environment {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.shell.v1.Environment"
    }
}

/// Defines additional types related to [Environment].
pub mod environment {
    #[allow(unused_imports)]
    use super::*;

    /// Possible execution states for an environment.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct State(i32);

    impl State {
        /// The environment's states is unknown.
        pub const STATE_UNSPECIFIED: State = State::new(0);

        /// The environment is not running and can't be connected to. Starting the
        /// environment will transition it to the PENDING state.
        pub const SUSPENDED: State = State::new(1);

        /// The environment is being started but is not yet ready to accept
        /// connections.
        pub const PENDING: State = State::new(2);

        /// The environment is running and ready to accept connections. It will
        /// automatically transition back to DISABLED after a period of inactivity or
        /// if another environment is started.
        pub const RUNNING: State = State::new(3);

        /// The environment is being deleted and can't be connected to.
        pub const DELETING: State = State::new(4);

        /// Creates a new State instance.
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
                0 => std::borrow::Cow::Borrowed("STATE_UNSPECIFIED"),
                1 => std::borrow::Cow::Borrowed("SUSPENDED"),
                2 => std::borrow::Cow::Borrowed("PENDING"),
                3 => std::borrow::Cow::Borrowed("RUNNING"),
                4 => std::borrow::Cow::Borrowed("DELETING"),
                _ => std::borrow::Cow::Owned(std::format!("UNKNOWN-VALUE:{}", self.0)),
            }
        }

        /// Creates an enum value from the value name.
        pub fn from_str_name(name: &str) -> std::option::Option<Self> {
            match name {
                "STATE_UNSPECIFIED" => std::option::Option::Some(Self::STATE_UNSPECIFIED),
                "SUSPENDED" => std::option::Option::Some(Self::SUSPENDED),
                "PENDING" => std::option::Option::Some(Self::PENDING),
                "RUNNING" => std::option::Option::Some(Self::RUNNING),
                "DELETING" => std::option::Option::Some(Self::DELETING),
                _ => std::option::Option::None,
            }
        }
    }

    impl std::convert::From<i32> for State {
        fn from(value: i32) -> Self {
            Self::new(value)
        }
    }

    impl std::default::Default for State {
        fn default() -> Self {
            Self::new(0)
        }
    }
}

/// Request message for
/// [GetEnvironment][google.cloud.shell.v1.CloudShellService.GetEnvironment].
///
/// [google.cloud.shell.v1.CloudShellService.GetEnvironment]: crate::client::CloudShellService::get_environment
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetEnvironmentRequest {
    /// Required. Name of the requested resource, for example `users/me/environments/default`
    /// or `users/someone@example.com/environments/default`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl GetEnvironmentRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::GetEnvironmentRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for GetEnvironmentRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.shell.v1.GetEnvironmentRequest"
    }
}

/// Message included in the metadata field of operations returned from
/// [CreateEnvironment][google.cloud.shell.v1.CloudShellService.CreateEnvironment].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct CreateEnvironmentMetadata {
    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl CreateEnvironmentMetadata {
    pub fn new() -> Self {
        std::default::Default::default()
    }
}

impl wkt::message::Message for CreateEnvironmentMetadata {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.shell.v1.CreateEnvironmentMetadata"
    }
}

/// Message included in the metadata field of operations returned from
/// [DeleteEnvironment][google.cloud.shell.v1.CloudShellService.DeleteEnvironment].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeleteEnvironmentMetadata {
    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl DeleteEnvironmentMetadata {
    pub fn new() -> Self {
        std::default::Default::default()
    }
}

impl wkt::message::Message for DeleteEnvironmentMetadata {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.shell.v1.DeleteEnvironmentMetadata"
    }
}

/// Request message for
/// [StartEnvironment][google.cloud.shell.v1.CloudShellService.StartEnvironment].
///
/// [google.cloud.shell.v1.CloudShellService.StartEnvironment]: crate::client::CloudShellService::start_environment
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct StartEnvironmentRequest {
    /// Name of the resource that should be started, for example
    /// `users/me/environments/default` or
    /// `users/someone@example.com/environments/default`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// The initial access token passed to the environment. If this is present and
    /// valid, the environment will be pre-authenticated with gcloud so that the
    /// user can run gcloud commands in Cloud Shell without having to log in. This
    /// code can be updated later by calling AuthorizeEnvironment.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub access_token: std::string::String,

    /// Public keys that should be added to the environment before it is started.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub public_keys: std::vec::Vec<std::string::String>,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl StartEnvironmentRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::StartEnvironmentRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [access_token][crate::model::StartEnvironmentRequest::access_token].
    pub fn set_access_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.access_token = v.into();
        self
    }

    /// Sets the value of [public_keys][crate::model::StartEnvironmentRequest::public_keys].
    pub fn set_public_keys<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.public_keys = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for StartEnvironmentRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.shell.v1.StartEnvironmentRequest"
    }
}

/// Request message for
/// [AuthorizeEnvironment][google.cloud.shell.v1.CloudShellService.AuthorizeEnvironment].
///
/// [google.cloud.shell.v1.CloudShellService.AuthorizeEnvironment]: crate::client::CloudShellService::authorize_environment
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct AuthorizeEnvironmentRequest {
    /// Name of the resource that should receive the credentials, for example
    /// `users/me/environments/default` or
    /// `users/someone@example.com/environments/default`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// The OAuth access token that should be sent to the environment.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub access_token: std::string::String,

    /// The OAuth ID token that should be sent to the environment.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub id_token: std::string::String,

    /// The time when the credentials expire. If not set, defaults to one hour from
    /// when the server received the request.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub expire_time: std::option::Option<wkt::Timestamp>,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl AuthorizeEnvironmentRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::AuthorizeEnvironmentRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [access_token][crate::model::AuthorizeEnvironmentRequest::access_token].
    pub fn set_access_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.access_token = v.into();
        self
    }

    /// Sets the value of [id_token][crate::model::AuthorizeEnvironmentRequest::id_token].
    pub fn set_id_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.id_token = v.into();
        self
    }

    /// Sets the value of [expire_time][crate::model::AuthorizeEnvironmentRequest::expire_time].
    pub fn set_expire_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.expire_time = v.into();
        self
    }
}

impl wkt::message::Message for AuthorizeEnvironmentRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.shell.v1.AuthorizeEnvironmentRequest"
    }
}

/// Response message for
/// [AuthorizeEnvironment][google.cloud.shell.v1.CloudShellService.AuthorizeEnvironment].
///
/// [google.cloud.shell.v1.CloudShellService.AuthorizeEnvironment]: crate::client::CloudShellService::authorize_environment
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct AuthorizeEnvironmentResponse {
    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl AuthorizeEnvironmentResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }
}

impl wkt::message::Message for AuthorizeEnvironmentResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.shell.v1.AuthorizeEnvironmentResponse"
    }
}

/// Message included in the metadata field of operations returned from
/// [AuthorizeEnvironment][google.cloud.shell.v1.CloudShellService.AuthorizeEnvironment].
///
/// [google.cloud.shell.v1.CloudShellService.AuthorizeEnvironment]: crate::client::CloudShellService::authorize_environment
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct AuthorizeEnvironmentMetadata {
    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl AuthorizeEnvironmentMetadata {
    pub fn new() -> Self {
        std::default::Default::default()
    }
}

impl wkt::message::Message for AuthorizeEnvironmentMetadata {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.shell.v1.AuthorizeEnvironmentMetadata"
    }
}

/// Message included in the metadata field of operations returned from
/// [StartEnvironment][google.cloud.shell.v1.CloudShellService.StartEnvironment].
///
/// [google.cloud.shell.v1.CloudShellService.StartEnvironment]: crate::client::CloudShellService::start_environment
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct StartEnvironmentMetadata {
    /// Current state of the environment being started.
    pub state: crate::model::start_environment_metadata::State,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl StartEnvironmentMetadata {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [state][crate::model::StartEnvironmentMetadata::state].
    pub fn set_state<T: std::convert::Into<crate::model::start_environment_metadata::State>>(
        mut self,
        v: T,
    ) -> Self {
        self.state = v.into();
        self
    }
}

impl wkt::message::Message for StartEnvironmentMetadata {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.shell.v1.StartEnvironmentMetadata"
    }
}

/// Defines additional types related to [StartEnvironmentMetadata].
pub mod start_environment_metadata {
    #[allow(unused_imports)]
    use super::*;

    /// Possible states an environment might transition between during startup.
    /// These states are not normally actionable by clients, but may be used to
    /// show a progress message to the user. An environment won't necessarily go
    /// through all of these states when starting. More states are likely to be
    /// added in the future.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct State(i32);

    impl State {
        /// The environment's start state is unknown.
        pub const STATE_UNSPECIFIED: State = State::new(0);

        /// The environment is in the process of being started, but no additional
        /// details are available.
        pub const STARTING: State = State::new(1);

        /// Startup is waiting for the user's disk to be unarchived. This can happen
        /// when the user returns to Cloud Shell after not having used it for a
        /// while, and suggests that startup will take longer than normal.
        pub const UNARCHIVING_DISK: State = State::new(2);

        /// Startup is waiting for compute resources to be assigned to the
        /// environment. This should normally happen very quickly, but an environment
        /// might stay in this state for an extended period of time if the system is
        /// experiencing heavy load.
        pub const AWAITING_COMPUTE_RESOURCES: State = State::new(4);

        /// Startup has completed. If the start operation was successful, the user
        /// should be able to establish an SSH connection to their environment.
        /// Otherwise, the operation will contain details of the failure.
        pub const FINISHED: State = State::new(3);

        /// Creates a new State instance.
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
                0 => std::borrow::Cow::Borrowed("STATE_UNSPECIFIED"),
                1 => std::borrow::Cow::Borrowed("STARTING"),
                2 => std::borrow::Cow::Borrowed("UNARCHIVING_DISK"),
                3 => std::borrow::Cow::Borrowed("FINISHED"),
                4 => std::borrow::Cow::Borrowed("AWAITING_COMPUTE_RESOURCES"),
                _ => std::borrow::Cow::Owned(std::format!("UNKNOWN-VALUE:{}", self.0)),
            }
        }

        /// Creates an enum value from the value name.
        pub fn from_str_name(name: &str) -> std::option::Option<Self> {
            match name {
                "STATE_UNSPECIFIED" => std::option::Option::Some(Self::STATE_UNSPECIFIED),
                "STARTING" => std::option::Option::Some(Self::STARTING),
                "UNARCHIVING_DISK" => std::option::Option::Some(Self::UNARCHIVING_DISK),
                "AWAITING_COMPUTE_RESOURCES" => {
                    std::option::Option::Some(Self::AWAITING_COMPUTE_RESOURCES)
                }
                "FINISHED" => std::option::Option::Some(Self::FINISHED),
                _ => std::option::Option::None,
            }
        }
    }

    impl std::convert::From<i32> for State {
        fn from(value: i32) -> Self {
            Self::new(value)
        }
    }

    impl std::default::Default for State {
        fn default() -> Self {
            Self::new(0)
        }
    }
}

/// Message included in the response field of operations returned from
/// [StartEnvironment][google.cloud.shell.v1.CloudShellService.StartEnvironment]
/// once the operation is complete.
///
/// [google.cloud.shell.v1.CloudShellService.StartEnvironment]: crate::client::CloudShellService::start_environment
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct StartEnvironmentResponse {
    /// Environment that was started.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub environment: std::option::Option<crate::model::Environment>,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl StartEnvironmentResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [environment][crate::model::StartEnvironmentResponse::environment].
    pub fn set_environment<
        T: std::convert::Into<std::option::Option<crate::model::Environment>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.environment = v.into();
        self
    }
}

impl wkt::message::Message for StartEnvironmentResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.shell.v1.StartEnvironmentResponse"
    }
}

/// Request message for
/// [AddPublicKey][google.cloud.shell.v1.CloudShellService.AddPublicKey].
///
/// [google.cloud.shell.v1.CloudShellService.AddPublicKey]: crate::client::CloudShellService::add_public_key
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct AddPublicKeyRequest {
    /// Environment this key should be added to, e.g.
    /// `users/me/environments/default`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub environment: std::string::String,

    /// Key that should be added to the environment. Supported formats are
    /// `ssh-dss` (see RFC4253), `ssh-rsa` (see RFC4253), `ecdsa-sha2-nistp256`
    /// (see RFC5656), `ecdsa-sha2-nistp384` (see RFC5656) and
    /// `ecdsa-sha2-nistp521` (see RFC5656). It should be structured as
    /// &lt;format&gt; &lt;content&gt;, where &lt;content&gt; part is encoded with
    /// Base64.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub key: std::string::String,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl AddPublicKeyRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [environment][crate::model::AddPublicKeyRequest::environment].
    pub fn set_environment<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.environment = v.into();
        self
    }

    /// Sets the value of [key][crate::model::AddPublicKeyRequest::key].
    pub fn set_key<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.key = v.into();
        self
    }
}

impl wkt::message::Message for AddPublicKeyRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.shell.v1.AddPublicKeyRequest"
    }
}

/// Response message for
/// [AddPublicKey][google.cloud.shell.v1.CloudShellService.AddPublicKey].
///
/// [google.cloud.shell.v1.CloudShellService.AddPublicKey]: crate::client::CloudShellService::add_public_key
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct AddPublicKeyResponse {
    /// Key that was added to the environment.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub key: std::string::String,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl AddPublicKeyResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [key][crate::model::AddPublicKeyResponse::key].
    pub fn set_key<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.key = v.into();
        self
    }
}

impl wkt::message::Message for AddPublicKeyResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.shell.v1.AddPublicKeyResponse"
    }
}

/// Message included in the metadata field of operations returned from
/// [AddPublicKey][google.cloud.shell.v1.CloudShellService.AddPublicKey].
///
/// [google.cloud.shell.v1.CloudShellService.AddPublicKey]: crate::client::CloudShellService::add_public_key
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct AddPublicKeyMetadata {
    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl AddPublicKeyMetadata {
    pub fn new() -> Self {
        std::default::Default::default()
    }
}

impl wkt::message::Message for AddPublicKeyMetadata {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.shell.v1.AddPublicKeyMetadata"
    }
}

/// Request message for
/// [RemovePublicKey][google.cloud.shell.v1.CloudShellService.RemovePublicKey].
///
/// [google.cloud.shell.v1.CloudShellService.RemovePublicKey]: crate::client::CloudShellService::remove_public_key
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct RemovePublicKeyRequest {
    /// Environment this key should be removed from, e.g.
    /// `users/me/environments/default`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub environment: std::string::String,

    /// Key that should be removed from the environment.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub key: std::string::String,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl RemovePublicKeyRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [environment][crate::model::RemovePublicKeyRequest::environment].
    pub fn set_environment<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.environment = v.into();
        self
    }

    /// Sets the value of [key][crate::model::RemovePublicKeyRequest::key].
    pub fn set_key<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.key = v.into();
        self
    }
}

impl wkt::message::Message for RemovePublicKeyRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.shell.v1.RemovePublicKeyRequest"
    }
}

/// Response message for
/// [RemovePublicKey][google.cloud.shell.v1.CloudShellService.RemovePublicKey].
///
/// [google.cloud.shell.v1.CloudShellService.RemovePublicKey]: crate::client::CloudShellService::remove_public_key
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct RemovePublicKeyResponse {
    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl RemovePublicKeyResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }
}

impl wkt::message::Message for RemovePublicKeyResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.shell.v1.RemovePublicKeyResponse"
    }
}

/// Message included in the metadata field of operations returned from
/// [RemovePublicKey][google.cloud.shell.v1.CloudShellService.RemovePublicKey].
///
/// [google.cloud.shell.v1.CloudShellService.RemovePublicKey]: crate::client::CloudShellService::remove_public_key
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct RemovePublicKeyMetadata {
    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl RemovePublicKeyMetadata {
    pub fn new() -> Self {
        std::default::Default::default()
    }
}

impl wkt::message::Message for RemovePublicKeyMetadata {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.shell.v1.RemovePublicKeyMetadata"
    }
}

/// Cloud-shell specific information that will be included as details in failure
/// responses.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct CloudShellErrorDetails {
    /// Code indicating the specific error the occurred.
    pub code: crate::model::cloud_shell_error_details::CloudShellErrorCode,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl CloudShellErrorDetails {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [code][crate::model::CloudShellErrorDetails::code].
    pub fn set_code<
        T: std::convert::Into<crate::model::cloud_shell_error_details::CloudShellErrorCode>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.code = v.into();
        self
    }
}

impl wkt::message::Message for CloudShellErrorDetails {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.shell.v1.CloudShellErrorDetails"
    }
}

/// Defines additional types related to [CloudShellErrorDetails].
pub mod cloud_shell_error_details {
    #[allow(unused_imports)]
    use super::*;

    /// Set of possible errors returned from API calls.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct CloudShellErrorCode(i32);

    impl CloudShellErrorCode {
        /// An unknown error occurred.
        pub const CLOUD_SHELL_ERROR_CODE_UNSPECIFIED: CloudShellErrorCode =
            CloudShellErrorCode::new(0);

        /// The image used by the Cloud Shell environment either does not exist or
        /// the user does not have access to it.
        pub const IMAGE_UNAVAILABLE: CloudShellErrorCode = CloudShellErrorCode::new(1);

        /// Cloud Shell has been disabled by an administrator for the user making the
        /// request.
        pub const CLOUD_SHELL_DISABLED: CloudShellErrorCode = CloudShellErrorCode::new(2);

        /// Cloud Shell has been permanently disabled due to a Terms of Service
        /// violation by the user.
        pub const TOS_VIOLATION: CloudShellErrorCode = CloudShellErrorCode::new(4);

        /// The user has exhausted their weekly Cloud Shell quota, and Cloud Shell
        /// will be disabled until the quota resets.
        pub const QUOTA_EXCEEDED: CloudShellErrorCode = CloudShellErrorCode::new(5);

        /// The Cloud Shell environment is unavailable and cannot be connected to at
        /// the moment.
        pub const ENVIRONMENT_UNAVAILABLE: CloudShellErrorCode = CloudShellErrorCode::new(6);

        /// Creates a new CloudShellErrorCode instance.
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
                0 => std::borrow::Cow::Borrowed("CLOUD_SHELL_ERROR_CODE_UNSPECIFIED"),
                1 => std::borrow::Cow::Borrowed("IMAGE_UNAVAILABLE"),
                2 => std::borrow::Cow::Borrowed("CLOUD_SHELL_DISABLED"),
                4 => std::borrow::Cow::Borrowed("TOS_VIOLATION"),
                5 => std::borrow::Cow::Borrowed("QUOTA_EXCEEDED"),
                6 => std::borrow::Cow::Borrowed("ENVIRONMENT_UNAVAILABLE"),
                _ => std::borrow::Cow::Owned(std::format!("UNKNOWN-VALUE:{}", self.0)),
            }
        }

        /// Creates an enum value from the value name.
        pub fn from_str_name(name: &str) -> std::option::Option<Self> {
            match name {
                "CLOUD_SHELL_ERROR_CODE_UNSPECIFIED" => {
                    std::option::Option::Some(Self::CLOUD_SHELL_ERROR_CODE_UNSPECIFIED)
                }
                "IMAGE_UNAVAILABLE" => std::option::Option::Some(Self::IMAGE_UNAVAILABLE),
                "CLOUD_SHELL_DISABLED" => std::option::Option::Some(Self::CLOUD_SHELL_DISABLED),
                "TOS_VIOLATION" => std::option::Option::Some(Self::TOS_VIOLATION),
                "QUOTA_EXCEEDED" => std::option::Option::Some(Self::QUOTA_EXCEEDED),
                "ENVIRONMENT_UNAVAILABLE" => {
                    std::option::Option::Some(Self::ENVIRONMENT_UNAVAILABLE)
                }
                _ => std::option::Option::None,
            }
        }
    }

    impl std::convert::From<i32> for CloudShellErrorCode {
        fn from(value: i32) -> Self {
            Self::new(value)
        }
    }

    impl std::default::Default for CloudShellErrorCode {
        fn default() -> Self {
            Self::new(0)
        }
    }
}
