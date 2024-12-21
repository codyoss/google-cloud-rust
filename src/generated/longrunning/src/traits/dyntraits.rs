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

/// A dyn-compatible, crate-private version of `Operations`.
#[async_trait::async_trait]
pub trait Operations: std::fmt::Debug + Send + Sync {
    async fn list_operations(
        &self,
        req: crate::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListOperationsResponse>;

    async fn get_operation(
        &self,
        req: crate::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Operation>;

    async fn delete_operation(
        &self,
        req: crate::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn cancel_operation(
        &self,
        req: crate::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;
}

/// All implementations of [crate::traits::Operations] also implement [Operations].
#[async_trait::async_trait]
impl<T: crate::traits::Operations> Operations for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: crate::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListOperationsResponse> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: crate::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Operation> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: crate::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: crate::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::cancel_operation(self, req, options).await
    }
}