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

/// A dyn-compatible, crate-private version of [super::Executions].
#[async_trait::async_trait]
pub trait Executions: std::fmt::Debug + Send + Sync {
    async fn list_executions(
        &self,
        req: crate::model::ListExecutionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListExecutionsResponse>;

    async fn create_execution(
        &self,
        req: crate::model::CreateExecutionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Execution>;

    async fn get_execution(
        &self,
        req: crate::model::GetExecutionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Execution>;

    async fn cancel_execution(
        &self,
        req: crate::model::CancelExecutionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Execution>;
}

/// All implementations of [super::Executions] also implement [Executions].
#[async_trait::async_trait]
impl<T: super::Executions> Executions for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_executions(
        &self,
        req: crate::model::ListExecutionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListExecutionsResponse> {
        T::list_executions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_execution(
        &self,
        req: crate::model::CreateExecutionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Execution> {
        T::create_execution(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_execution(
        &self,
        req: crate::model::GetExecutionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Execution> {
        T::get_execution(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_execution(
        &self,
        req: crate::model::CancelExecutionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Execution> {
        T::cancel_execution(self, req, options).await
    }
}
