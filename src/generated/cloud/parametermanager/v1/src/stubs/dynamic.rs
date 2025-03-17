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

/// A dyn-compatible, crate-private version of [super::ParameterManager].
#[async_trait::async_trait]
pub trait ParameterManager: std::fmt::Debug + Send + Sync {
    async fn list_parameters(
        &self,
        req: crate::model::ListParametersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListParametersResponse>;

    async fn get_parameter(
        &self,
        req: crate::model::GetParameterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Parameter>;

    async fn create_parameter(
        &self,
        req: crate::model::CreateParameterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Parameter>;

    async fn update_parameter(
        &self,
        req: crate::model::UpdateParameterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Parameter>;

    async fn delete_parameter(
        &self,
        req: crate::model::DeleteParameterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn list_parameter_versions(
        &self,
        req: crate::model::ListParameterVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListParameterVersionsResponse>;

    async fn get_parameter_version(
        &self,
        req: crate::model::GetParameterVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ParameterVersion>;

    async fn render_parameter_version(
        &self,
        req: crate::model::RenderParameterVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::RenderParameterVersionResponse>;

    async fn create_parameter_version(
        &self,
        req: crate::model::CreateParameterVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ParameterVersion>;

    async fn update_parameter_version(
        &self,
        req: crate::model::UpdateParameterVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ParameterVersion>;

    async fn delete_parameter_version(
        &self,
        req: crate::model::DeleteParameterVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

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

/// All implementations of [super::ParameterManager] also implement [ParameterManager].
#[async_trait::async_trait]
impl<T: super::ParameterManager> ParameterManager for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_parameters(
        &self,
        req: crate::model::ListParametersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListParametersResponse> {
        T::list_parameters(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_parameter(
        &self,
        req: crate::model::GetParameterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Parameter> {
        T::get_parameter(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_parameter(
        &self,
        req: crate::model::CreateParameterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Parameter> {
        T::create_parameter(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_parameter(
        &self,
        req: crate::model::UpdateParameterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Parameter> {
        T::update_parameter(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_parameter(
        &self,
        req: crate::model::DeleteParameterRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_parameter(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_parameter_versions(
        &self,
        req: crate::model::ListParameterVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListParameterVersionsResponse> {
        T::list_parameter_versions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_parameter_version(
        &self,
        req: crate::model::GetParameterVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ParameterVersion> {
        T::get_parameter_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn render_parameter_version(
        &self,
        req: crate::model::RenderParameterVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::RenderParameterVersionResponse> {
        T::render_parameter_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_parameter_version(
        &self,
        req: crate::model::CreateParameterVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ParameterVersion> {
        T::create_parameter_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_parameter_version(
        &self,
        req: crate::model::UpdateParameterVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ParameterVersion> {
        T::update_parameter_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_parameter_version(
        &self,
        req: crate::model::DeleteParameterVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_parameter_version(self, req, options).await
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
