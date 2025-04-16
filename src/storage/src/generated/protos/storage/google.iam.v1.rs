// This file is @generated by prost-build.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetPolicyOptions {
    #[prost(int32, tag = "1")]
    pub requested_policy_version: i32,
}
impl ::prost::Name for GetPolicyOptions {
    const NAME: &'static str = "GetPolicyOptions";
    const PACKAGE: &'static str = "google.iam.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "google.iam.v1.GetPolicyOptions".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.iam.v1.GetPolicyOptions".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Policy {
    #[prost(int32, tag = "1")]
    pub version: i32,
    #[prost(message, repeated, tag = "4")]
    pub bindings: ::prost::alloc::vec::Vec<Binding>,
    #[prost(message, repeated, tag = "6")]
    pub audit_configs: ::prost::alloc::vec::Vec<AuditConfig>,
    #[prost(bytes = "bytes", tag = "3")]
    pub etag: ::prost::bytes::Bytes,
}
impl ::prost::Name for Policy {
    const NAME: &'static str = "Policy";
    const PACKAGE: &'static str = "google.iam.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "google.iam.v1.Policy".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.iam.v1.Policy".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Binding {
    #[prost(string, tag = "1")]
    pub role: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub members: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub condition: ::core::option::Option<super::super::r#type::Expr>,
}
impl ::prost::Name for Binding {
    const NAME: &'static str = "Binding";
    const PACKAGE: &'static str = "google.iam.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "google.iam.v1.Binding".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.iam.v1.Binding".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditConfig {
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub audit_log_configs: ::prost::alloc::vec::Vec<AuditLogConfig>,
}
impl ::prost::Name for AuditConfig {
    const NAME: &'static str = "AuditConfig";
    const PACKAGE: &'static str = "google.iam.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "google.iam.v1.AuditConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.iam.v1.AuditConfig".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditLogConfig {
    #[prost(enumeration = "audit_log_config::LogType", tag = "1")]
    pub log_type: i32,
    #[prost(string, repeated, tag = "2")]
    pub exempted_members: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `AuditLogConfig`.
pub mod audit_log_config {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum LogType {
        Unspecified = 0,
        AdminRead = 1,
        DataWrite = 2,
        DataRead = 3,
    }
    impl LogType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "LOG_TYPE_UNSPECIFIED",
                Self::AdminRead => "ADMIN_READ",
                Self::DataWrite => "DATA_WRITE",
                Self::DataRead => "DATA_READ",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LOG_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ADMIN_READ" => Some(Self::AdminRead),
                "DATA_WRITE" => Some(Self::DataWrite),
                "DATA_READ" => Some(Self::DataRead),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for AuditLogConfig {
    const NAME: &'static str = "AuditLogConfig";
    const PACKAGE: &'static str = "google.iam.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "google.iam.v1.AuditLogConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.iam.v1.AuditLogConfig".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyDelta {
    #[prost(message, repeated, tag = "1")]
    pub binding_deltas: ::prost::alloc::vec::Vec<BindingDelta>,
    #[prost(message, repeated, tag = "2")]
    pub audit_config_deltas: ::prost::alloc::vec::Vec<AuditConfigDelta>,
}
impl ::prost::Name for PolicyDelta {
    const NAME: &'static str = "PolicyDelta";
    const PACKAGE: &'static str = "google.iam.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "google.iam.v1.PolicyDelta".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.iam.v1.PolicyDelta".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindingDelta {
    #[prost(enumeration = "binding_delta::Action", tag = "1")]
    pub action: i32,
    #[prost(string, tag = "2")]
    pub role: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub member: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub condition: ::core::option::Option<super::super::r#type::Expr>,
}
/// Nested message and enum types in `BindingDelta`.
pub mod binding_delta {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Action {
        Unspecified = 0,
        Add = 1,
        Remove = 2,
    }
    impl Action {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "ACTION_UNSPECIFIED",
                Self::Add => "ADD",
                Self::Remove => "REMOVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACTION_UNSPECIFIED" => Some(Self::Unspecified),
                "ADD" => Some(Self::Add),
                "REMOVE" => Some(Self::Remove),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for BindingDelta {
    const NAME: &'static str = "BindingDelta";
    const PACKAGE: &'static str = "google.iam.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "google.iam.v1.BindingDelta".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.iam.v1.BindingDelta".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditConfigDelta {
    #[prost(enumeration = "audit_config_delta::Action", tag = "1")]
    pub action: i32,
    #[prost(string, tag = "2")]
    pub service: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub exempted_member: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub log_type: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AuditConfigDelta`.
pub mod audit_config_delta {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Action {
        Unspecified = 0,
        Add = 1,
        Remove = 2,
    }
    impl Action {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "ACTION_UNSPECIFIED",
                Self::Add => "ADD",
                Self::Remove => "REMOVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACTION_UNSPECIFIED" => Some(Self::Unspecified),
                "ADD" => Some(Self::Add),
                "REMOVE" => Some(Self::Remove),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for AuditConfigDelta {
    const NAME: &'static str = "AuditConfigDelta";
    const PACKAGE: &'static str = "google.iam.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "google.iam.v1.AuditConfigDelta".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.iam.v1.AuditConfigDelta".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetIamPolicyRequest {
    #[prost(string, tag = "1")]
    pub resource: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub policy: ::core::option::Option<Policy>,
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
impl ::prost::Name for SetIamPolicyRequest {
    const NAME: &'static str = "SetIamPolicyRequest";
    const PACKAGE: &'static str = "google.iam.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "google.iam.v1.SetIamPolicyRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.iam.v1.SetIamPolicyRequest".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIamPolicyRequest {
    #[prost(string, tag = "1")]
    pub resource: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub options: ::core::option::Option<GetPolicyOptions>,
}
impl ::prost::Name for GetIamPolicyRequest {
    const NAME: &'static str = "GetIamPolicyRequest";
    const PACKAGE: &'static str = "google.iam.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "google.iam.v1.GetIamPolicyRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.iam.v1.GetIamPolicyRequest".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestIamPermissionsRequest {
    #[prost(string, tag = "1")]
    pub resource: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for TestIamPermissionsRequest {
    const NAME: &'static str = "TestIamPermissionsRequest";
    const PACKAGE: &'static str = "google.iam.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "google.iam.v1.TestIamPermissionsRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.iam.v1.TestIamPermissionsRequest".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestIamPermissionsResponse {
    #[prost(string, repeated, tag = "1")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for TestIamPermissionsResponse {
    const NAME: &'static str = "TestIamPermissionsResponse";
    const PACKAGE: &'static str = "google.iam.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "google.iam.v1.TestIamPermissionsResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.iam.v1.TestIamPermissionsResponse".into()
    }
}
