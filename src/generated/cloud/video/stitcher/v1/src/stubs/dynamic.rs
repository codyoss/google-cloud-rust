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

/// A dyn-compatible, crate-private version of [super::VideoStitcherService].
#[async_trait::async_trait]
pub trait VideoStitcherService: std::fmt::Debug + Send + Sync {
    async fn create_cdn_key(
        &self,
        req: crate::model::CreateCdnKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_cdn_keys(
        &self,
        req: crate::model::ListCdnKeysRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListCdnKeysResponse>;

    async fn get_cdn_key(
        &self,
        req: crate::model::GetCdnKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CdnKey>;

    async fn delete_cdn_key(
        &self,
        req: crate::model::DeleteCdnKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_cdn_key(
        &self,
        req: crate::model::UpdateCdnKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_vod_session(
        &self,
        req: crate::model::CreateVodSessionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VodSession>;

    async fn get_vod_session(
        &self,
        req: crate::model::GetVodSessionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VodSession>;

    async fn list_vod_stitch_details(
        &self,
        req: crate::model::ListVodStitchDetailsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListVodStitchDetailsResponse>;

    async fn get_vod_stitch_detail(
        &self,
        req: crate::model::GetVodStitchDetailRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VodStitchDetail>;

    async fn list_vod_ad_tag_details(
        &self,
        req: crate::model::ListVodAdTagDetailsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListVodAdTagDetailsResponse>;

    async fn get_vod_ad_tag_detail(
        &self,
        req: crate::model::GetVodAdTagDetailRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VodAdTagDetail>;

    async fn list_live_ad_tag_details(
        &self,
        req: crate::model::ListLiveAdTagDetailsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListLiveAdTagDetailsResponse>;

    async fn get_live_ad_tag_detail(
        &self,
        req: crate::model::GetLiveAdTagDetailRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LiveAdTagDetail>;

    async fn create_slate(
        &self,
        req: crate::model::CreateSlateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_slates(
        &self,
        req: crate::model::ListSlatesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListSlatesResponse>;

    async fn get_slate(
        &self,
        req: crate::model::GetSlateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Slate>;

    async fn update_slate(
        &self,
        req: crate::model::UpdateSlateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_slate(
        &self,
        req: crate::model::DeleteSlateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_live_session(
        &self,
        req: crate::model::CreateLiveSessionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LiveSession>;

    async fn get_live_session(
        &self,
        req: crate::model::GetLiveSessionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LiveSession>;

    async fn create_live_config(
        &self,
        req: crate::model::CreateLiveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_live_configs(
        &self,
        req: crate::model::ListLiveConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListLiveConfigsResponse>;

    async fn get_live_config(
        &self,
        req: crate::model::GetLiveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LiveConfig>;

    async fn delete_live_config(
        &self,
        req: crate::model::DeleteLiveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_live_config(
        &self,
        req: crate::model::UpdateLiveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn create_vod_config(
        &self,
        req: crate::model::CreateVodConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_vod_configs(
        &self,
        req: crate::model::ListVodConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListVodConfigsResponse>;

    async fn get_vod_config(
        &self,
        req: crate::model::GetVodConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VodConfig>;

    async fn delete_vod_config(
        &self,
        req: crate::model::DeleteVodConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_vod_config(
        &self,
        req: crate::model::UpdateVodConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::VideoStitcherService] also implement [VideoStitcherService].
#[async_trait::async_trait]
impl<T: super::VideoStitcherService> VideoStitcherService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_cdn_key(
        &self,
        req: crate::model::CreateCdnKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_cdn_key(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_cdn_keys(
        &self,
        req: crate::model::ListCdnKeysRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListCdnKeysResponse> {
        T::list_cdn_keys(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_cdn_key(
        &self,
        req: crate::model::GetCdnKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CdnKey> {
        T::get_cdn_key(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_cdn_key(
        &self,
        req: crate::model::DeleteCdnKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_cdn_key(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_cdn_key(
        &self,
        req: crate::model::UpdateCdnKeyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_cdn_key(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_vod_session(
        &self,
        req: crate::model::CreateVodSessionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VodSession> {
        T::create_vod_session(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_vod_session(
        &self,
        req: crate::model::GetVodSessionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VodSession> {
        T::get_vod_session(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_vod_stitch_details(
        &self,
        req: crate::model::ListVodStitchDetailsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListVodStitchDetailsResponse> {
        T::list_vod_stitch_details(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_vod_stitch_detail(
        &self,
        req: crate::model::GetVodStitchDetailRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VodStitchDetail> {
        T::get_vod_stitch_detail(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_vod_ad_tag_details(
        &self,
        req: crate::model::ListVodAdTagDetailsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListVodAdTagDetailsResponse> {
        T::list_vod_ad_tag_details(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_vod_ad_tag_detail(
        &self,
        req: crate::model::GetVodAdTagDetailRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VodAdTagDetail> {
        T::get_vod_ad_tag_detail(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_live_ad_tag_details(
        &self,
        req: crate::model::ListLiveAdTagDetailsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListLiveAdTagDetailsResponse> {
        T::list_live_ad_tag_details(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_live_ad_tag_detail(
        &self,
        req: crate::model::GetLiveAdTagDetailRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LiveAdTagDetail> {
        T::get_live_ad_tag_detail(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_slate(
        &self,
        req: crate::model::CreateSlateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_slate(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_slates(
        &self,
        req: crate::model::ListSlatesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListSlatesResponse> {
        T::list_slates(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_slate(
        &self,
        req: crate::model::GetSlateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Slate> {
        T::get_slate(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_slate(
        &self,
        req: crate::model::UpdateSlateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_slate(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_slate(
        &self,
        req: crate::model::DeleteSlateRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_slate(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_live_session(
        &self,
        req: crate::model::CreateLiveSessionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LiveSession> {
        T::create_live_session(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_live_session(
        &self,
        req: crate::model::GetLiveSessionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LiveSession> {
        T::get_live_session(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_live_config(
        &self,
        req: crate::model::CreateLiveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_live_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_live_configs(
        &self,
        req: crate::model::ListLiveConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListLiveConfigsResponse> {
        T::list_live_configs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_live_config(
        &self,
        req: crate::model::GetLiveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LiveConfig> {
        T::get_live_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_live_config(
        &self,
        req: crate::model::DeleteLiveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_live_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_live_config(
        &self,
        req: crate::model::UpdateLiveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_live_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_vod_config(
        &self,
        req: crate::model::CreateVodConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_vod_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_vod_configs(
        &self,
        req: crate::model::ListVodConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListVodConfigsResponse> {
        T::list_vod_configs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_vod_config(
        &self,
        req: crate::model::GetVodConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::VodConfig> {
        T::get_vod_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_vod_config(
        &self,
        req: crate::model::DeleteVodConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_vod_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_vod_config(
        &self,
        req: crate::model::UpdateVodConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_vod_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
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
