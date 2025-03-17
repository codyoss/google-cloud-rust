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

/// Implements a [AdvisoryNotificationsService](super::stubs::AdvisoryNotificationsService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct AdvisoryNotificationsService<T>
where
    T: super::stubs::AdvisoryNotificationsService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> AdvisoryNotificationsService<T>
where
    T: super::stubs::AdvisoryNotificationsService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::AdvisoryNotificationsService for AdvisoryNotificationsService<T>
where
    T: super::stubs::AdvisoryNotificationsService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_notifications(
        &self,
        req: crate::model::ListNotificationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListNotificationsResponse> {
        self.inner.list_notifications(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_notification(
        &self,
        req: crate::model::GetNotificationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Notification> {
        self.inner.get_notification(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_settings(
        &self,
        req: crate::model::GetSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Settings> {
        self.inner.get_settings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_settings(
        &self,
        req: crate::model::UpdateSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Settings> {
        self.inner.update_settings(req, options).await
    }
}
