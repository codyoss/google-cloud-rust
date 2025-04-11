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
use crate::Result;

/// Implements a [ParameterManager](super::stub::ParameterManager) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct ParameterManager<T>
where
    T: super::stub::ParameterManager + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> ParameterManager<T>
where
    T: super::stub::ParameterManager + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::ParameterManager for ParameterManager<T>
where
    T: super::stub::ParameterManager + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_parameters(
        &self,
        req: crate::model::ListParametersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListParametersResponse>> {
        self.inner.list_parameters(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_parameter(
        &self,
        req: crate::model::GetParameterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Parameter>> {
        self.inner.get_parameter(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_parameter(
        &self,
        req: crate::model::CreateParameterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Parameter>> {
        self.inner.create_parameter(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_parameter(
        &self,
        req: crate::model::UpdateParameterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Parameter>> {
        self.inner.update_parameter(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_parameter(
        &self,
        req: crate::model::DeleteParameterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_parameter(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_parameter_versions(
        &self,
        req: crate::model::ListParameterVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListParameterVersionsResponse>> {
        self.inner.list_parameter_versions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_parameter_version(
        &self,
        req: crate::model::GetParameterVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ParameterVersion>> {
        self.inner.get_parameter_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn render_parameter_version(
        &self,
        req: crate::model::RenderParameterVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::RenderParameterVersionResponse>> {
        self.inner.render_parameter_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_parameter_version(
        &self,
        req: crate::model::CreateParameterVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ParameterVersion>> {
        self.inner.create_parameter_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_parameter_version(
        &self,
        req: crate::model::UpdateParameterVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ParameterVersion>> {
        self.inner.update_parameter_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_parameter_version(
        &self,
        req: crate::model::DeleteParameterVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_parameter_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::ListLocationsResponse>> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::Location>> {
        self.inner.get_location(req, options).await
    }
}
