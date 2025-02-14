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
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate std;
extern crate tracing;
extern crate wkt;

/// CreateProfileRequest describes a profile resource online creation request.
/// The deployment field must be populated. The profile_type specifies the list
/// of profile types supported by the agent. The creation call will hang until a
/// profile of one of these types needs to be collected.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct CreateProfileRequest {
    /// Parent project to create the profile in.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// Deployment details.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub deployment: std::option::Option<crate::model::Deployment>,

    /// One or more profile types that the agent is capable of providing.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub profile_type: std::vec::Vec<crate::model::ProfileType>,
}

impl CreateProfileRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::CreateProfileRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [deployment][crate::model::CreateProfileRequest::deployment].
    pub fn set_deployment<T: std::convert::Into<std::option::Option<crate::model::Deployment>>>(
        mut self,
        v: T,
    ) -> Self {
        self.deployment = v.into();
        self
    }

    /// Sets the value of [profile_type][crate::model::CreateProfileRequest::profile_type].
    pub fn set_profile_type<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::ProfileType>,
    {
        use std::iter::Iterator;
        self.profile_type = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for CreateProfileRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.devtools.cloudprofiler.v2.CreateProfileRequest"
    }
}

/// CreateOfflineProfileRequest describes a profile resource offline creation
/// request.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct CreateOfflineProfileRequest {
    /// Parent project to create the profile in.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// Contents of the profile to create.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub profile: std::option::Option<crate::model::Profile>,
}

impl CreateOfflineProfileRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::CreateOfflineProfileRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [profile][crate::model::CreateOfflineProfileRequest::profile].
    pub fn set_profile<T: std::convert::Into<std::option::Option<crate::model::Profile>>>(
        mut self,
        v: T,
    ) -> Self {
        self.profile = v.into();
        self
    }
}

impl wkt::message::Message for CreateOfflineProfileRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.devtools.cloudprofiler.v2.CreateOfflineProfileRequest"
    }
}

/// UpdateProfileRequest contains the profile to update.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct UpdateProfileRequest {
    /// Profile to update.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub profile: std::option::Option<crate::model::Profile>,

    /// Field mask used to specify the fields to be overwritten. Currently only
    /// profile_bytes and labels fields are supported by UpdateProfile, so only
    /// those fields can be specified in the mask. When no mask is provided, all
    /// fields are overwritten.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub update_mask: std::option::Option<wkt::FieldMask>,
}

impl UpdateProfileRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [profile][crate::model::UpdateProfileRequest::profile].
    pub fn set_profile<T: std::convert::Into<std::option::Option<crate::model::Profile>>>(
        mut self,
        v: T,
    ) -> Self {
        self.profile = v.into();
        self
    }

    /// Sets the value of [update_mask][crate::model::UpdateProfileRequest::update_mask].
    pub fn set_update_mask<T: std::convert::Into<std::option::Option<wkt::FieldMask>>>(
        mut self,
        v: T,
    ) -> Self {
        self.update_mask = v.into();
        self
    }
}

impl wkt::message::Message for UpdateProfileRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.devtools.cloudprofiler.v2.UpdateProfileRequest"
    }
}

/// Profile resource.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Profile {
    /// Output only. Opaque, server-assigned, unique ID for this profile.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Type of profile.
    /// For offline mode, this must be specified when creating the profile. For
    /// online mode it is assigned and returned by the server.
    pub profile_type: crate::model::ProfileType,

    /// Deployment this profile corresponds to.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub deployment: std::option::Option<crate::model::Deployment>,

    /// Duration of the profiling session.
    /// Input (for the offline mode) or output (for the online mode).
    /// The field represents requested profiling duration. It may slightly differ
    /// from the effective profiling duration, which is recorded in the profile
    /// data, in case the profiling can't be stopped immediately (e.g. in case
    /// stopping the profiling is handled asynchronously).
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub duration: std::option::Option<wkt::Duration>,

    /// Input only. Profile bytes, as a gzip compressed serialized proto, the
    /// format is <https://github.com/google/pprof/blob/master/proto/profile.proto>.
    #[serde(skip_serializing_if = "bytes::Bytes::is_empty")]
    #[serde_as(as = "serde_with::base64::Base64")]
    pub profile_bytes: bytes::Bytes,

    /// Input only. Labels associated to this specific profile. These labels will
    /// get merged with the deployment labels for the final data set. See
    /// documentation on deployment labels for validation rules and limits.
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub labels: std::collections::HashMap<std::string::String, std::string::String>,

    /// Output only. Start time for the profile.
    /// This output is only present in response from the ListProfiles method.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub start_time: std::option::Option<wkt::Timestamp>,
}

impl Profile {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::Profile::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [profile_type][crate::model::Profile::profile_type].
    pub fn set_profile_type<T: std::convert::Into<crate::model::ProfileType>>(
        mut self,
        v: T,
    ) -> Self {
        self.profile_type = v.into();
        self
    }

    /// Sets the value of [deployment][crate::model::Profile::deployment].
    pub fn set_deployment<T: std::convert::Into<std::option::Option<crate::model::Deployment>>>(
        mut self,
        v: T,
    ) -> Self {
        self.deployment = v.into();
        self
    }

    /// Sets the value of [duration][crate::model::Profile::duration].
    pub fn set_duration<T: std::convert::Into<std::option::Option<wkt::Duration>>>(
        mut self,
        v: T,
    ) -> Self {
        self.duration = v.into();
        self
    }

    /// Sets the value of [profile_bytes][crate::model::Profile::profile_bytes].
    pub fn set_profile_bytes<T: std::convert::Into<bytes::Bytes>>(mut self, v: T) -> Self {
        self.profile_bytes = v.into();
        self
    }

    /// Sets the value of [start_time][crate::model::Profile::start_time].
    pub fn set_start_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.start_time = v.into();
        self
    }

    /// Sets the value of [labels][crate::model::Profile::labels].
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

impl wkt::message::Message for Profile {
    fn typename() -> &'static str {
        "type.googleapis.com/google.devtools.cloudprofiler.v2.Profile"
    }
}

/// Deployment contains the deployment identification information.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Deployment {
    /// Project ID is the ID of a cloud project.
    /// Validation regex: `^[a-z][-a-z0-9:.]{4,61}[a-z0-9]$`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub project_id: std::string::String,

    /// Target is the service name used to group related deployments:
    ///
    /// * Service name for App Engine Flex / Standard.
    /// * Cluster and container name for GKE.
    /// * User-specified string for direct Compute Engine profiling (e.g. Java).
    /// * Job name for Dataflow.
    ///   Validation regex: `^[a-z0-9]([-a-z0-9_.]{0,253}[a-z0-9])?$`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub target: std::string::String,

    /// Labels identify the deployment within the user universe and same target.
    /// Validation regex for label names: `^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$`.
    /// Value for an individual label must be <= 512 bytes, the total
    /// size of all label names and values must be <= 1024 bytes.
    ///
    /// Label named "language" can be used to record the programming language of
    /// the profiled deployment. The standard choices for the value include "java",
    /// "go", "python", "ruby", "nodejs", "php", "dotnet".
    ///
    /// For deployments running on Google Cloud Platform, "zone" or "region" label
    /// should be present describing the deployment location. An example of a zone
    /// is "us-central1-a", an example of a region is "us-central1" or
    /// "us-central".
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub labels: std::collections::HashMap<std::string::String, std::string::String>,
}

impl Deployment {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [project_id][crate::model::Deployment::project_id].
    pub fn set_project_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.project_id = v.into();
        self
    }

    /// Sets the value of [target][crate::model::Deployment::target].
    pub fn set_target<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.target = v.into();
        self
    }

    /// Sets the value of [labels][crate::model::Deployment::labels].
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

impl wkt::message::Message for Deployment {
    fn typename() -> &'static str {
        "type.googleapis.com/google.devtools.cloudprofiler.v2.Deployment"
    }
}

/// ListProfilesRequest contains request parameters for listing profiles for
/// deployments in projects which the user has permissions to view.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListProfilesRequest {
    /// Required. The parent, which owns this collection of profiles.
    /// Format: projects/{user_project_id}
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// The maximum number of items to return.
    /// Default page_size is 1000.
    /// Max limit is 1000.
    pub page_size: i32,

    /// The token to continue pagination and get profiles from a particular page.
    /// When paginating, all other parameters provided to `ListProfiles` must match
    /// the call that provided the page token.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub page_token: std::string::String,
}

impl ListProfilesRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::ListProfilesRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [page_size][crate::model::ListProfilesRequest::page_size].
    pub fn set_page_size<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of [page_token][crate::model::ListProfilesRequest::page_token].
    pub fn set_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }
}

impl wkt::message::Message for ListProfilesRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.devtools.cloudprofiler.v2.ListProfilesRequest"
    }
}

/// ListProfileResponse contains the list of collected profiles for deployments
/// in projects which the user has permissions to view.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListProfilesResponse {
    /// List of profiles fetched.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub profiles: std::vec::Vec<crate::model::Profile>,

    /// Token to receive the next page of results.
    /// This field maybe empty if there are no more profiles to fetch.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub next_page_token: std::string::String,

    /// Number of profiles that were skipped in the current page since they were
    /// not able to be fetched successfully. This should typically be zero. A
    /// non-zero value may indicate a transient failure, in which case if the
    /// number is too high for your use case, the call may be retried.
    pub skipped_profiles: i32,
}

impl ListProfilesResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [next_page_token][crate::model::ListProfilesResponse::next_page_token].
    pub fn set_next_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }

    /// Sets the value of [skipped_profiles][crate::model::ListProfilesResponse::skipped_profiles].
    pub fn set_skipped_profiles<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.skipped_profiles = v.into();
        self
    }

    /// Sets the value of [profiles][crate::model::ListProfilesResponse::profiles].
    pub fn set_profiles<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::Profile>,
    {
        use std::iter::Iterator;
        self.profiles = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for ListProfilesResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.devtools.cloudprofiler.v2.ListProfilesResponse"
    }
}

#[cfg(feature = "unstable-stream")]
impl gax::paginator::PageableResponse for ListProfilesResponse {
    type PageItem = crate::model::Profile;

    fn items(self) -> std::vec::Vec<Self::PageItem> {
        self.profiles
    }

    fn next_page_token(&self) -> std::string::String {
        gax::paginator::extract_token(&self.next_page_token)
    }
}

/// ProfileType is type of profiling data.
/// NOTE: the enumeration member names are used (in lowercase) as unique string
/// identifiers of profile types, so they must not be renamed.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ProfileType(std::borrow::Cow<'static, str>);

impl ProfileType {
    /// Creates a new ProfileType instance.
    pub const fn new(v: &'static str) -> Self {
        Self(std::borrow::Cow::Borrowed(v))
    }

    /// Gets the enum value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Useful constants to work with [ProfileType](ProfileType)
pub mod profile_type {
    use super::ProfileType;

    /// Unspecified profile type.
    pub const PROFILE_TYPE_UNSPECIFIED: ProfileType = ProfileType::new("PROFILE_TYPE_UNSPECIFIED");

    /// Thread CPU time sampling.
    pub const CPU: ProfileType = ProfileType::new("CPU");

    /// Wallclock time sampling. More expensive as stops all threads.
    pub const WALL: ProfileType = ProfileType::new("WALL");

    /// In-use heap profile. Represents a snapshot of the allocations that are
    /// live at the time of the profiling.
    pub const HEAP: ProfileType = ProfileType::new("HEAP");

    /// Single-shot collection of all thread stacks.
    pub const THREADS: ProfileType = ProfileType::new("THREADS");

    /// Synchronization contention profile.
    pub const CONTENTION: ProfileType = ProfileType::new("CONTENTION");

    /// Peak heap profile.
    pub const PEAK_HEAP: ProfileType = ProfileType::new("PEAK_HEAP");

    /// Heap allocation profile. It represents the aggregation of all allocations
    /// made over the duration of the profile. All allocations are included,
    /// including those that might have been freed by the end of the profiling
    /// interval. The profile is in particular useful for garbage collecting
    /// languages to understand which parts of the code create most of the garbage
    /// collection pressure to see if those can be optimized.
    pub const HEAP_ALLOC: ProfileType = ProfileType::new("HEAP_ALLOC");
}

impl std::convert::From<std::string::String> for ProfileType {
    fn from(value: std::string::String) -> Self {
        Self(std::borrow::Cow::Owned(value))
    }
}
