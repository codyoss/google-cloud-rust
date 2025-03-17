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

/// A dyn-compatible, crate-private version of [super::CloudTasks].
#[async_trait::async_trait]
pub trait CloudTasks: std::fmt::Debug + Send + Sync {
    async fn list_queues(
        &self,
        req: crate::model::ListQueuesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListQueuesResponse>;

    async fn get_queue(
        &self,
        req: crate::model::GetQueueRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Queue>;

    async fn create_queue(
        &self,
        req: crate::model::CreateQueueRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Queue>;

    async fn update_queue(
        &self,
        req: crate::model::UpdateQueueRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Queue>;

    async fn delete_queue(
        &self,
        req: crate::model::DeleteQueueRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn purge_queue(
        &self,
        req: crate::model::PurgeQueueRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Queue>;

    async fn pause_queue(
        &self,
        req: crate::model::PauseQueueRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Queue>;

    async fn resume_queue(
        &self,
        req: crate::model::ResumeQueueRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Queue>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;

    async fn list_tasks(
        &self,
        req: crate::model::ListTasksRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListTasksResponse>;

    async fn get_task(
        &self,
        req: crate::model::GetTaskRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Task>;

    async fn create_task(
        &self,
        req: crate::model::CreateTaskRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Task>;

    async fn delete_task(
        &self,
        req: crate::model::DeleteTaskRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn run_task(
        &self,
        req: crate::model::RunTaskRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Task>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location>;
}

/// All implementations of [super::CloudTasks] also implement [CloudTasks].
#[async_trait::async_trait]
impl<T: super::CloudTasks> CloudTasks for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_queues(
        &self,
        req: crate::model::ListQueuesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListQueuesResponse> {
        T::list_queues(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_queue(
        &self,
        req: crate::model::GetQueueRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Queue> {
        T::get_queue(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_queue(
        &self,
        req: crate::model::CreateQueueRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Queue> {
        T::create_queue(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_queue(
        &self,
        req: crate::model::UpdateQueueRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Queue> {
        T::update_queue(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_queue(
        &self,
        req: crate::model::DeleteQueueRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_queue(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn purge_queue(
        &self,
        req: crate::model::PurgeQueueRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Queue> {
        T::purge_queue(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn pause_queue(
        &self,
        req: crate::model::PauseQueueRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Queue> {
        T::pause_queue(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn resume_queue(
        &self,
        req: crate::model::ResumeQueueRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Queue> {
        T::resume_queue(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse> {
        T::test_iam_permissions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_tasks(
        &self,
        req: crate::model::ListTasksRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListTasksResponse> {
        T::list_tasks(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_task(
        &self,
        req: crate::model::GetTaskRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Task> {
        T::get_task(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_task(
        &self,
        req: crate::model::CreateTaskRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Task> {
        T::create_task(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_task(
        &self,
        req: crate::model::DeleteTaskRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_task(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn run_task(
        &self,
        req: crate::model::RunTaskRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Task> {
        T::run_task(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location> {
        T::get_location(self, req, options).await
    }
}
