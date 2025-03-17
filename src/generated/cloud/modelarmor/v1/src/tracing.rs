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

/// Implements a [ModelArmor](super::stubs::ModelArmor) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct ModelArmor<T>
where
    T: super::stubs::ModelArmor + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> ModelArmor<T>
where
    T: super::stubs::ModelArmor + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::ModelArmor for ModelArmor<T>
where
    T: super::stubs::ModelArmor + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_templates(
        &self,
        req: crate::model::ListTemplatesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTemplatesResponse> {
        self.inner.list_templates(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_template(
        &self,
        req: crate::model::GetTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Template> {
        self.inner.get_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_template(
        &self,
        req: crate::model::CreateTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Template> {
        self.inner.create_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_template(
        &self,
        req: crate::model::UpdateTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Template> {
        self.inner.update_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_template(
        &self,
        req: crate::model::DeleteTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_floor_setting(
        &self,
        req: crate::model::GetFloorSettingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::FloorSetting> {
        self.inner.get_floor_setting(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_floor_setting(
        &self,
        req: crate::model::UpdateFloorSettingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::FloorSetting> {
        self.inner.update_floor_setting(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn sanitize_user_prompt(
        &self,
        req: crate::model::SanitizeUserPromptRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SanitizeUserPromptResponse> {
        self.inner.sanitize_user_prompt(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn sanitize_model_response(
        &self,
        req: crate::model::SanitizeModelResponseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SanitizeModelResponseResponse> {
        self.inner.sanitize_model_response(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
    }
}
