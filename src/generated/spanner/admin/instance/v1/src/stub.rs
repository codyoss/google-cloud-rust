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

/// Defines the trait used to implement [super::client::InstanceAdmin].
///
/// Application developers may need to implement this trait to mock
/// `client::InstanceAdmin`.  In other use-cases, application developers only
/// use `client::InstanceAdmin` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait InstanceAdmin: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::InstanceAdmin::list_instance_configs].
    fn list_instance_configs(
        &self,
        _req: crate::model::ListInstanceConfigsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListInstanceConfigsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListInstanceConfigsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::InstanceAdmin::get_instance_config].
    fn get_instance_config(
        &self,
        _req: crate::model::GetInstanceConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::InstanceConfig>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::InstanceConfig>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::InstanceAdmin::create_instance_config].
    fn create_instance_config(
        &self,
        _req: crate::model::CreateInstanceConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::InstanceAdmin::update_instance_config].
    fn update_instance_config(
        &self,
        _req: crate::model::UpdateInstanceConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::InstanceAdmin::delete_instance_config].
    fn delete_instance_config(
        &self,
        _req: crate::model::DeleteInstanceConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::InstanceAdmin::list_instance_config_operations].
    fn list_instance_config_operations(
        &self,
        _req: crate::model::ListInstanceConfigOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<
            gax::response::Response<crate::model::ListInstanceConfigOperationsResponse>,
        >,
    > + Send {
        std::future::ready::<
            crate::Result<
                gax::response::Response<crate::model::ListInstanceConfigOperationsResponse>,
            >,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::InstanceAdmin::list_instances].
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

    /// Implements [super::client::InstanceAdmin::list_instance_partitions].
    fn list_instance_partitions(
        &self,
        _req: crate::model::ListInstancePartitionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<
            gax::response::Response<crate::model::ListInstancePartitionsResponse>,
        >,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<crate::model::ListInstancePartitionsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::InstanceAdmin::get_instance].
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

    /// Implements [super::client::InstanceAdmin::create_instance].
    fn create_instance(
        &self,
        _req: crate::model::CreateInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::InstanceAdmin::update_instance].
    fn update_instance(
        &self,
        _req: crate::model::UpdateInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::InstanceAdmin::delete_instance].
    fn delete_instance(
        &self,
        _req: crate::model::DeleteInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::InstanceAdmin::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<iam_v1::model::Policy>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::InstanceAdmin::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<iam_v1::model::Policy>>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::InstanceAdmin::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>,
    > + Send {
        std::future::ready::<
            crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::InstanceAdmin::get_instance_partition].
    fn get_instance_partition(
        &self,
        _req: crate::model::GetInstancePartitionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::InstancePartition>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<crate::model::InstancePartition>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::InstanceAdmin::create_instance_partition].
    fn create_instance_partition(
        &self,
        _req: crate::model::CreateInstancePartitionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::InstanceAdmin::delete_instance_partition].
    fn delete_instance_partition(
        &self,
        _req: crate::model::DeleteInstancePartitionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::InstanceAdmin::update_instance_partition].
    fn update_instance_partition(
        &self,
        _req: crate::model::UpdateInstancePartitionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::InstanceAdmin::list_instance_partition_operations].
    fn list_instance_partition_operations(
        &self,
        _req: crate::model::ListInstancePartitionOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<
            gax::response::Response<crate::model::ListInstancePartitionOperationsResponse>,
        >,
    > + Send {
        std::future::ready::<
            crate::Result<
                gax::response::Response<crate::model::ListInstancePartitionOperationsResponse>,
            >,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::InstanceAdmin::move_instance].
    fn move_instance(
        &self,
        _req: crate::model::MoveInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        std::future::ready::<crate::Result<gax::response::Response<longrunning::model::Operation>>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::InstanceAdmin::list_operations].
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

    /// Implements [super::client::InstanceAdmin::get_operation].
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

    /// Implements [super::client::InstanceAdmin::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::InstanceAdmin::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        std::future::ready::<crate::Result<gax::response::Response<()>>>(Err(Error::other(
            "unimplemented",
        )))
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
