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

use std::sync::Arc;

/// A dyn-compatible, crate-private version of [super::ApiGatewayService].
#[async_trait::async_trait]
pub trait ApiGatewayService: std::fmt::Debug + Send + Sync {
    async fn list_gateways(
        &self,
        req: crate::model::ListGatewaysRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListGatewaysResponse>>;

    async fn get_gateway(
        &self,
        req: crate::model::GetGatewayRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Gateway>>;

    async fn create_gateway(
        &self,
        req: crate::model::CreateGatewayRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn update_gateway(
        &self,
        req: crate::model::UpdateGatewayRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_gateway(
        &self,
        req: crate::model::DeleteGatewayRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn list_apis(
        &self,
        req: crate::model::ListApisRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListApisResponse>>;

    async fn get_api(
        &self,
        req: crate::model::GetApiRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Api>>;

    async fn create_api(
        &self,
        req: crate::model::CreateApiRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn update_api(
        &self,
        req: crate::model::UpdateApiRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_api(
        &self,
        req: crate::model::DeleteApiRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn list_api_configs(
        &self,
        req: crate::model::ListApiConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListApiConfigsResponse>>;

    async fn get_api_config(
        &self,
        req: crate::model::GetApiConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ApiConfig>>;

    async fn create_api_config(
        &self,
        req: crate::model::CreateApiConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn update_api_config(
        &self,
        req: crate::model::UpdateApiConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_api_config(
        &self,
        req: crate::model::DeleteApiConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::ApiGatewayService] also implement [ApiGatewayService].
#[async_trait::async_trait]
impl<T: super::ApiGatewayService> ApiGatewayService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_gateways(
        &self,
        req: crate::model::ListGatewaysRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListGatewaysResponse>> {
        T::list_gateways(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_gateway(
        &self,
        req: crate::model::GetGatewayRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Gateway>> {
        T::get_gateway(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_gateway(
        &self,
        req: crate::model::CreateGatewayRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_gateway(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_gateway(
        &self,
        req: crate::model::UpdateGatewayRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::update_gateway(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_gateway(
        &self,
        req: crate::model::DeleteGatewayRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_gateway(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_apis(
        &self,
        req: crate::model::ListApisRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListApisResponse>> {
        T::list_apis(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_api(
        &self,
        req: crate::model::GetApiRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Api>> {
        T::get_api(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_api(
        &self,
        req: crate::model::CreateApiRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_api(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_api(
        &self,
        req: crate::model::UpdateApiRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::update_api(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_api(
        &self,
        req: crate::model::DeleteApiRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_api(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_api_configs(
        &self,
        req: crate::model::ListApiConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListApiConfigsResponse>> {
        T::list_api_configs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_api_config(
        &self,
        req: crate::model::GetApiConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ApiConfig>> {
        T::get_api_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_api_config(
        &self,
        req: crate::model::CreateApiConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_api_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_api_config(
        &self,
        req: crate::model::UpdateApiConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::update_api_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_api_config(
        &self,
        req: crate::model::DeleteApiConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_api_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::cancel_operation(self, req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        T::get_polling_error_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}
