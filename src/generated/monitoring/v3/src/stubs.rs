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

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::AlertPolicyService].
///
/// Application developers may need to implement this trait to mock
/// `client::AlertPolicyService`.  In other use-cases, application developers only
/// use `client::AlertPolicyService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait AlertPolicyService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::AlertPolicyService::list_alert_policies].
    fn list_alert_policies(
        &self,
        _req: crate::model::ListAlertPoliciesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListAlertPoliciesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListAlertPoliciesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::AlertPolicyService::get_alert_policy].
    fn get_alert_policy(
        &self,
        _req: crate::model::GetAlertPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AlertPolicy>> + Send {
        std::future::ready::<crate::Result<crate::model::AlertPolicy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlertPolicyService::create_alert_policy].
    fn create_alert_policy(
        &self,
        _req: crate::model::CreateAlertPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AlertPolicy>> + Send {
        std::future::ready::<crate::Result<crate::model::AlertPolicy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::AlertPolicyService::delete_alert_policy].
    fn delete_alert_policy(
        &self,
        _req: crate::model::DeleteAlertPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::AlertPolicyService::update_alert_policy].
    fn update_alert_policy(
        &self,
        _req: crate::model::UpdateAlertPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AlertPolicy>> + Send {
        std::future::ready::<crate::Result<crate::model::AlertPolicy>>(Err(Error::other(
            "unimplemented",
        )))
    }
}

/// Defines the trait used to implement [super::client::GroupService].
///
/// Application developers may need to implement this trait to mock
/// `client::GroupService`.  In other use-cases, application developers only
/// use `client::GroupService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait GroupService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::GroupService::list_groups].
    fn list_groups(
        &self,
        _req: crate::model::ListGroupsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListGroupsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListGroupsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::GroupService::get_group].
    fn get_group(
        &self,
        _req: crate::model::GetGroupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Group>> + Send {
        std::future::ready::<crate::Result<crate::model::Group>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::GroupService::create_group].
    fn create_group(
        &self,
        _req: crate::model::CreateGroupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Group>> + Send {
        std::future::ready::<crate::Result<crate::model::Group>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::GroupService::update_group].
    fn update_group(
        &self,
        _req: crate::model::UpdateGroupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Group>> + Send {
        std::future::ready::<crate::Result<crate::model::Group>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::GroupService::delete_group].
    fn delete_group(
        &self,
        _req: crate::model::DeleteGroupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::GroupService::list_group_members].
    fn list_group_members(
        &self,
        _req: crate::model::ListGroupMembersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListGroupMembersResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListGroupMembersResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }
}

/// Defines the trait used to implement [super::client::MetricService].
///
/// Application developers may need to implement this trait to mock
/// `client::MetricService`.  In other use-cases, application developers only
/// use `client::MetricService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait MetricService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::MetricService::list_monitored_resource_descriptors].
    fn list_monitored_resource_descriptors(
        &self,
        _req: crate::model::ListMonitoredResourceDescriptorsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListMonitoredResourceDescriptorsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListMonitoredResourceDescriptorsResponse>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::MetricService::get_monitored_resource_descriptor].
    fn get_monitored_resource_descriptor(
        &self,
        _req: crate::model::GetMonitoredResourceDescriptorRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<api::model::MonitoredResourceDescriptor>> + Send
    {
        std::future::ready::<crate::Result<api::model::MonitoredResourceDescriptor>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::MetricService::list_metric_descriptors].
    fn list_metric_descriptors(
        &self,
        _req: crate::model::ListMetricDescriptorsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListMetricDescriptorsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListMetricDescriptorsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::MetricService::get_metric_descriptor].
    fn get_metric_descriptor(
        &self,
        _req: crate::model::GetMetricDescriptorRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<api::model::MetricDescriptor>> + Send {
        std::future::ready::<crate::Result<api::model::MetricDescriptor>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::MetricService::create_metric_descriptor].
    fn create_metric_descriptor(
        &self,
        _req: crate::model::CreateMetricDescriptorRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<api::model::MetricDescriptor>> + Send {
        std::future::ready::<crate::Result<api::model::MetricDescriptor>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::MetricService::delete_metric_descriptor].
    fn delete_metric_descriptor(
        &self,
        _req: crate::model::DeleteMetricDescriptorRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::MetricService::list_time_series].
    fn list_time_series(
        &self,
        _req: crate::model::ListTimeSeriesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListTimeSeriesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListTimeSeriesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::MetricService::create_time_series].
    fn create_time_series(
        &self,
        _req: crate::model::CreateTimeSeriesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::MetricService::create_service_time_series].
    fn create_service_time_series(
        &self,
        _req: crate::model::CreateTimeSeriesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }
}

/// Defines the trait used to implement [super::client::NotificationChannelService].
///
/// Application developers may need to implement this trait to mock
/// `client::NotificationChannelService`.  In other use-cases, application developers only
/// use `client::NotificationChannelService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait NotificationChannelService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::NotificationChannelService::list_notification_channel_descriptors].
    fn list_notification_channel_descriptors(
        &self,
        _req: crate::model::ListNotificationChannelDescriptorsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListNotificationChannelDescriptorsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListNotificationChannelDescriptorsResponse>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::NotificationChannelService::get_notification_channel_descriptor].
    fn get_notification_channel_descriptor(
        &self,
        _req: crate::model::GetNotificationChannelDescriptorRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::NotificationChannelDescriptor>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::NotificationChannelDescriptor>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::NotificationChannelService::list_notification_channels].
    fn list_notification_channels(
        &self,
        _req: crate::model::ListNotificationChannelsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListNotificationChannelsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListNotificationChannelsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::NotificationChannelService::get_notification_channel].
    fn get_notification_channel(
        &self,
        _req: crate::model::GetNotificationChannelRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::NotificationChannel>> + Send
    {
        std::future::ready::<crate::Result<crate::model::NotificationChannel>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NotificationChannelService::create_notification_channel].
    fn create_notification_channel(
        &self,
        _req: crate::model::CreateNotificationChannelRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::NotificationChannel>> + Send
    {
        std::future::ready::<crate::Result<crate::model::NotificationChannel>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NotificationChannelService::update_notification_channel].
    fn update_notification_channel(
        &self,
        _req: crate::model::UpdateNotificationChannelRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::NotificationChannel>> + Send
    {
        std::future::ready::<crate::Result<crate::model::NotificationChannel>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::NotificationChannelService::delete_notification_channel].
    fn delete_notification_channel(
        &self,
        _req: crate::model::DeleteNotificationChannelRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::NotificationChannelService::send_notification_channel_verification_code].
    fn send_notification_channel_verification_code(
        &self,
        _req: crate::model::SendNotificationChannelVerificationCodeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::NotificationChannelService::get_notification_channel_verification_code].
    fn get_notification_channel_verification_code(
        &self,
        _req: crate::model::GetNotificationChannelVerificationCodeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::GetNotificationChannelVerificationCodeResponse>,
    > + Send {
        std::future::ready::<
            crate::Result<crate::model::GetNotificationChannelVerificationCodeResponse>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::NotificationChannelService::verify_notification_channel].
    fn verify_notification_channel(
        &self,
        _req: crate::model::VerifyNotificationChannelRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::NotificationChannel>> + Send
    {
        std::future::ready::<crate::Result<crate::model::NotificationChannel>>(Err(Error::other(
            "unimplemented",
        )))
    }
}

/// Defines the trait used to implement [super::client::QueryService].
///
/// Application developers may need to implement this trait to mock
/// `client::QueryService`.  In other use-cases, application developers only
/// use `client::QueryService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait QueryService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::QueryService::query_time_series].
    fn query_time_series(
        &self,
        _req: crate::model::QueryTimeSeriesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::QueryTimeSeriesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::QueryTimeSeriesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }
}

/// Defines the trait used to implement [super::client::ServiceMonitoringService].
///
/// Application developers may need to implement this trait to mock
/// `client::ServiceMonitoringService`.  In other use-cases, application developers only
/// use `client::ServiceMonitoringService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait ServiceMonitoringService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::ServiceMonitoringService::create_service].
    fn create_service(
        &self,
        _req: crate::model::CreateServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Service>> + Send {
        std::future::ready::<crate::Result<crate::model::Service>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ServiceMonitoringService::get_service].
    fn get_service(
        &self,
        _req: crate::model::GetServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Service>> + Send {
        std::future::ready::<crate::Result<crate::model::Service>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ServiceMonitoringService::list_services].
    fn list_services(
        &self,
        _req: crate::model::ListServicesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListServicesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListServicesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ServiceMonitoringService::update_service].
    fn update_service(
        &self,
        _req: crate::model::UpdateServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Service>> + Send {
        std::future::ready::<crate::Result<crate::model::Service>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ServiceMonitoringService::delete_service].
    fn delete_service(
        &self,
        _req: crate::model::DeleteServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::ServiceMonitoringService::create_service_level_objective].
    fn create_service_level_objective(
        &self,
        _req: crate::model::CreateServiceLevelObjectiveRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ServiceLevelObjective>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ServiceLevelObjective>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ServiceMonitoringService::get_service_level_objective].
    fn get_service_level_objective(
        &self,
        _req: crate::model::GetServiceLevelObjectiveRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ServiceLevelObjective>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ServiceLevelObjective>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ServiceMonitoringService::list_service_level_objectives].
    fn list_service_level_objectives(
        &self,
        _req: crate::model::ListServiceLevelObjectivesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListServiceLevelObjectivesResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListServiceLevelObjectivesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ServiceMonitoringService::update_service_level_objective].
    fn update_service_level_objective(
        &self,
        _req: crate::model::UpdateServiceLevelObjectiveRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ServiceLevelObjective>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ServiceLevelObjective>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ServiceMonitoringService::delete_service_level_objective].
    fn delete_service_level_objective(
        &self,
        _req: crate::model::DeleteServiceLevelObjectiveRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }
}

/// Defines the trait used to implement [super::client::SnoozeService].
///
/// Application developers may need to implement this trait to mock
/// `client::SnoozeService`.  In other use-cases, application developers only
/// use `client::SnoozeService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait SnoozeService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::SnoozeService::create_snooze].
    fn create_snooze(
        &self,
        _req: crate::model::CreateSnoozeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Snooze>> + Send {
        std::future::ready::<crate::Result<crate::model::Snooze>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::SnoozeService::list_snoozes].
    fn list_snoozes(
        &self,
        _req: crate::model::ListSnoozesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListSnoozesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListSnoozesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::SnoozeService::get_snooze].
    fn get_snooze(
        &self,
        _req: crate::model::GetSnoozeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Snooze>> + Send {
        std::future::ready::<crate::Result<crate::model::Snooze>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::SnoozeService::update_snooze].
    fn update_snooze(
        &self,
        _req: crate::model::UpdateSnoozeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Snooze>> + Send {
        std::future::ready::<crate::Result<crate::model::Snooze>>(Err(Error::other(
            "unimplemented",
        )))
    }
}

/// Defines the trait used to implement [super::client::UptimeCheckService].
///
/// Application developers may need to implement this trait to mock
/// `client::UptimeCheckService`.  In other use-cases, application developers only
/// use `client::UptimeCheckService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait UptimeCheckService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::UptimeCheckService::list_uptime_check_configs].
    fn list_uptime_check_configs(
        &self,
        _req: crate::model::ListUptimeCheckConfigsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListUptimeCheckConfigsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListUptimeCheckConfigsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::UptimeCheckService::get_uptime_check_config].
    fn get_uptime_check_config(
        &self,
        _req: crate::model::GetUptimeCheckConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::UptimeCheckConfig>> + Send
    {
        std::future::ready::<crate::Result<crate::model::UptimeCheckConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::UptimeCheckService::create_uptime_check_config].
    fn create_uptime_check_config(
        &self,
        _req: crate::model::CreateUptimeCheckConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::UptimeCheckConfig>> + Send
    {
        std::future::ready::<crate::Result<crate::model::UptimeCheckConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::UptimeCheckService::update_uptime_check_config].
    fn update_uptime_check_config(
        &self,
        _req: crate::model::UpdateUptimeCheckConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::UptimeCheckConfig>> + Send
    {
        std::future::ready::<crate::Result<crate::model::UptimeCheckConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::UptimeCheckService::delete_uptime_check_config].
    fn delete_uptime_check_config(
        &self,
        _req: crate::model::DeleteUptimeCheckConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        std::future::ready::<crate::Result<()>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::UptimeCheckService::list_uptime_check_ips].
    fn list_uptime_check_ips(
        &self,
        _req: crate::model::ListUptimeCheckIpsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListUptimeCheckIpsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListUptimeCheckIpsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }
}
