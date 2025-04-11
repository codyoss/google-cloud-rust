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

/// Defines the trait used to implement [super::client::Applications].
///
/// Application developers may need to implement this trait to mock
/// `client::Applications`.  In other use-cases, application developers only
/// use `client::Applications` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait Applications: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::Applications::get_application].
    fn get_application(
        &self,
        _req: crate::model::GetApplicationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Application>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Application>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Applications::create_application].
    fn create_application(
        &self,
        _req: crate::model::CreateApplicationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Applications::update_application].
    fn update_application(
        &self,
        _req: crate::model::UpdateApplicationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Applications::repair_application].
    fn repair_application(
        &self,
        _req: crate::model::RepairApplicationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Applications::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Applications::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
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

/// Defines the trait used to implement [super::client::Services].
///
/// Application developers may need to implement this trait to mock
/// `client::Services`.  In other use-cases, application developers only
/// use `client::Services` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait Services: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::Services::list_services].
    fn list_services(
        &self,
        _req: crate::model::ListServicesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListServicesResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListServicesResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Services::get_service].
    fn get_service(
        &self,
        _req: crate::model::GetServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Service>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Service>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Services::update_service].
    fn update_service(
        &self,
        _req: crate::model::UpdateServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Services::delete_service].
    fn delete_service(
        &self,
        _req: crate::model::DeleteServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Services::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Services::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
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

/// Defines the trait used to implement [super::client::Versions].
///
/// Application developers may need to implement this trait to mock
/// `client::Versions`.  In other use-cases, application developers only
/// use `client::Versions` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait Versions: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::Versions::list_versions].
    fn list_versions(
        &self,
        _req: crate::model::ListVersionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListVersionsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListVersionsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Versions::get_version].
    fn get_version(
        &self,
        _req: crate::model::GetVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Version>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Version>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Versions::create_version].
    fn create_version(
        &self,
        _req: crate::model::CreateVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Versions::update_version].
    fn update_version(
        &self,
        _req: crate::model::UpdateVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Versions::delete_version].
    fn delete_version(
        &self,
        _req: crate::model::DeleteVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Versions::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Versions::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
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

/// Defines the trait used to implement [super::client::Instances].
///
/// Application developers may need to implement this trait to mock
/// `client::Instances`.  In other use-cases, application developers only
/// use `client::Instances` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait Instances: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::Instances::list_instances].
    fn list_instances(
        &self,
        _req: crate::model::ListInstancesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListInstancesResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListInstancesResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Instances::get_instance].
    fn get_instance(
        &self,
        _req: crate::model::GetInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Instance>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::Instance>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::Instances::delete_instance].
    fn delete_instance(
        &self,
        _req: crate::model::DeleteInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Instances::debug_instance].
    fn debug_instance(
        &self,
        _req: crate::model::DebugInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Instances::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Instances::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
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

/// Defines the trait used to implement [super::client::Firewall].
///
/// Application developers may need to implement this trait to mock
/// `client::Firewall`.  In other use-cases, application developers only
/// use `client::Firewall` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait Firewall: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::Firewall::list_ingress_rules].
    fn list_ingress_rules(
        &self,
        _req: crate::model::ListIngressRulesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListIngressRulesResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListIngressRulesResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Firewall::batch_update_ingress_rules].
    fn batch_update_ingress_rules(
        &self,
        _req: crate::model::BatchUpdateIngressRulesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<
            gax::response::Response<crate::model::BatchUpdateIngressRulesResponse>,
        >,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::BatchUpdateIngressRulesResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Firewall::create_ingress_rule].
    fn create_ingress_rule(
        &self,
        _req: crate::model::CreateIngressRuleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::FirewallRule>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::FirewallRule>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Firewall::get_ingress_rule].
    fn get_ingress_rule(
        &self,
        _req: crate::model::GetIngressRuleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::FirewallRule>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::FirewallRule>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Firewall::update_ingress_rule].
    fn update_ingress_rule(
        &self,
        _req: crate::model::UpdateIngressRuleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::FirewallRule>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::FirewallRule>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::Firewall::delete_ingress_rule].
    fn delete_ingress_rule(
        &self,
        _req: crate::model::DeleteIngressRuleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::Firewall::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::Firewall::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }
}

/// Defines the trait used to implement [super::client::AuthorizedDomains].
///
/// Application developers may need to implement this trait to mock
/// `client::AuthorizedDomains`.  In other use-cases, application developers only
/// use `client::AuthorizedDomains` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait AuthorizedDomains: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::AuthorizedDomains::list_authorized_domains].
    fn list_authorized_domains(
        &self,
        _req: crate::model::ListAuthorizedDomainsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<
            gax::response::Response<crate::model::ListAuthorizedDomainsResponse>,
        >,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListAuthorizedDomainsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::AuthorizedDomains::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::AuthorizedDomains::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }
}

/// Defines the trait used to implement [super::client::AuthorizedCertificates].
///
/// Application developers may need to implement this trait to mock
/// `client::AuthorizedCertificates`.  In other use-cases, application developers only
/// use `client::AuthorizedCertificates` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait AuthorizedCertificates: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::AuthorizedCertificates::list_authorized_certificates].
    fn list_authorized_certificates(
        &self,
        _req: crate::model::ListAuthorizedCertificatesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<
            gax::response::Response<crate::model::ListAuthorizedCertificatesResponse>,
        >,
    > + Send {
        std::future::ready::<
            crate::Result<
                gax::response::Response<crate::model::ListAuthorizedCertificatesResponse>,
            >,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::AuthorizedCertificates::get_authorized_certificate].
    fn get_authorized_certificate(
        &self,
        _req: crate::model::GetAuthorizedCertificateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::AuthorizedCertificate>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::AuthorizedCertificate>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::AuthorizedCertificates::create_authorized_certificate].
    fn create_authorized_certificate(
        &self,
        _req: crate::model::CreateAuthorizedCertificateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::AuthorizedCertificate>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::AuthorizedCertificate>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::AuthorizedCertificates::update_authorized_certificate].
    fn update_authorized_certificate(
        &self,
        _req: crate::model::UpdateAuthorizedCertificateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::AuthorizedCertificate>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::AuthorizedCertificate>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::AuthorizedCertificates::delete_authorized_certificate].
    fn delete_authorized_certificate(
        &self,
        _req: crate::model::DeleteAuthorizedCertificateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AuthorizedCertificates::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::AuthorizedCertificates::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }
}

/// Defines the trait used to implement [super::client::DomainMappings].
///
/// Application developers may need to implement this trait to mock
/// `client::DomainMappings`.  In other use-cases, application developers only
/// use `client::DomainMappings` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait DomainMappings: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::DomainMappings::list_domain_mappings].
    fn list_domain_mappings(
        &self,
        _req: crate::model::ListDomainMappingsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListDomainMappingsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListDomainMappingsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::DomainMappings::get_domain_mapping].
    fn get_domain_mapping(
        &self,
        _req: crate::model::GetDomainMappingRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::DomainMapping>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::DomainMapping>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::DomainMappings::create_domain_mapping].
    fn create_domain_mapping(
        &self,
        _req: crate::model::CreateDomainMappingRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::DomainMappings::update_domain_mapping].
    fn update_domain_mapping(
        &self,
        _req: crate::model::UpdateDomainMappingRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::DomainMappings::delete_domain_mapping].
    fn delete_domain_mapping(
        &self,
        _req: crate::model::DeleteDomainMappingRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::DomainMappings::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::DomainMappings::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
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
