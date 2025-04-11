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

/// Implements a [VideoStitcherService](super::stub::VideoStitcherService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct VideoStitcherService<T>
where
    T: super::stub::VideoStitcherService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> VideoStitcherService<T>
where
    T: super::stub::VideoStitcherService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::VideoStitcherService for VideoStitcherService<T>
where
    T: super::stub::VideoStitcherService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_cdn_key(
        &self,
        req: crate::model::CreateCdnKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_cdn_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_cdn_keys(
        &self,
        req: crate::model::ListCdnKeysRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListCdnKeysResponse>> {
        self.inner.list_cdn_keys(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_cdn_key(
        &self,
        req: crate::model::GetCdnKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::CdnKey>> {
        self.inner.get_cdn_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_cdn_key(
        &self,
        req: crate::model::DeleteCdnKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_cdn_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_cdn_key(
        &self,
        req: crate::model::UpdateCdnKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_cdn_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_vod_session(
        &self,
        req: crate::model::CreateVodSessionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::VodSession>> {
        self.inner.create_vod_session(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_vod_session(
        &self,
        req: crate::model::GetVodSessionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::VodSession>> {
        self.inner.get_vod_session(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_vod_stitch_details(
        &self,
        req: crate::model::ListVodStitchDetailsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListVodStitchDetailsResponse>> {
        self.inner.list_vod_stitch_details(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_vod_stitch_detail(
        &self,
        req: crate::model::GetVodStitchDetailRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::VodStitchDetail>> {
        self.inner.get_vod_stitch_detail(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_vod_ad_tag_details(
        &self,
        req: crate::model::ListVodAdTagDetailsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListVodAdTagDetailsResponse>> {
        self.inner.list_vod_ad_tag_details(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_vod_ad_tag_detail(
        &self,
        req: crate::model::GetVodAdTagDetailRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::VodAdTagDetail>> {
        self.inner.get_vod_ad_tag_detail(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_live_ad_tag_details(
        &self,
        req: crate::model::ListLiveAdTagDetailsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListLiveAdTagDetailsResponse>> {
        self.inner.list_live_ad_tag_details(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_live_ad_tag_detail(
        &self,
        req: crate::model::GetLiveAdTagDetailRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::LiveAdTagDetail>> {
        self.inner.get_live_ad_tag_detail(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_slate(
        &self,
        req: crate::model::CreateSlateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_slate(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_slates(
        &self,
        req: crate::model::ListSlatesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListSlatesResponse>> {
        self.inner.list_slates(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_slate(
        &self,
        req: crate::model::GetSlateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Slate>> {
        self.inner.get_slate(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_slate(
        &self,
        req: crate::model::UpdateSlateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_slate(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_slate(
        &self,
        req: crate::model::DeleteSlateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_slate(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_live_session(
        &self,
        req: crate::model::CreateLiveSessionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::LiveSession>> {
        self.inner.create_live_session(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_live_session(
        &self,
        req: crate::model::GetLiveSessionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::LiveSession>> {
        self.inner.get_live_session(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_live_config(
        &self,
        req: crate::model::CreateLiveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_live_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_live_configs(
        &self,
        req: crate::model::ListLiveConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListLiveConfigsResponse>> {
        self.inner.list_live_configs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_live_config(
        &self,
        req: crate::model::GetLiveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::LiveConfig>> {
        self.inner.get_live_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_live_config(
        &self,
        req: crate::model::DeleteLiveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_live_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_live_config(
        &self,
        req: crate::model::UpdateLiveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_live_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_vod_config(
        &self,
        req: crate::model::CreateVodConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_vod_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_vod_configs(
        &self,
        req: crate::model::ListVodConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListVodConfigsResponse>> {
        self.inner.list_vod_configs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_vod_config(
        &self,
        req: crate::model::GetVodConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::VodConfig>> {
        self.inner.get_vod_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_vod_config(
        &self,
        req: crate::model::DeleteVodConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_vod_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_vod_config(
        &self,
        req: crate::model::UpdateVodConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_vod_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.cancel_operation(req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        self.inner.get_polling_error_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
