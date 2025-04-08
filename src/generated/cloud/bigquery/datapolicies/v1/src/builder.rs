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

pub mod data_policy_service {
    use crate::Result;
    use std::sync::Arc;

    /// A builder for [DataPolicyService][super::super::client::DataPolicyService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_bigquery_datapolicies_v1::*;
    /// # use builder::data_policy_service::ClientBuilder;
    /// # use client::DataPolicyService;
    /// let builder : ClientBuilder = DataPolicyService::builder();
    /// let client = builder
    ///     .with_endpoint("https://bigquerydatapolicy.googleapis.com")
    ///     .build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::DataPolicyService;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = DataPolicyService;
            type Credentials = gaxi::options::Credentials;
            async fn build(self, config: gaxi::options::ClientConfig) -> gax::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [super::super::client::DataPolicyService] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stub::dynamic::DataPolicyService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::DataPolicyService>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [DataPolicyService::create_data_policy][super::super::client::DataPolicyService::create_data_policy] calls.
    #[derive(Clone, Debug)]
    pub struct CreateDataPolicy(RequestBuilder<crate::model::CreateDataPolicyRequest>);

    impl CreateDataPolicy {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::DataPolicyService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateDataPolicyRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::DataPolicy> {
            (*self.0.stub)
                .create_data_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateDataPolicyRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [data_policy][crate::model::CreateDataPolicyRequest::data_policy].
        pub fn set_data_policy<T: Into<std::option::Option<crate::model::DataPolicy>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.data_policy = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for CreateDataPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [DataPolicyService::update_data_policy][super::super::client::DataPolicyService::update_data_policy] calls.
    #[derive(Clone, Debug)]
    pub struct UpdateDataPolicy(RequestBuilder<crate::model::UpdateDataPolicyRequest>);

    impl UpdateDataPolicy {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::DataPolicyService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateDataPolicyRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::DataPolicy> {
            (*self.0.stub)
                .update_data_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [data_policy][crate::model::UpdateDataPolicyRequest::data_policy].
        pub fn set_data_policy<T: Into<std::option::Option<crate::model::DataPolicy>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.data_policy = v.into();
            self
        }

        /// Sets the value of [update_mask][crate::model::UpdateDataPolicyRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for UpdateDataPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [DataPolicyService::rename_data_policy][super::super::client::DataPolicyService::rename_data_policy] calls.
    #[derive(Clone, Debug)]
    pub struct RenameDataPolicy(RequestBuilder<crate::model::RenameDataPolicyRequest>);

    impl RenameDataPolicy {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::DataPolicyService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::RenameDataPolicyRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::DataPolicy> {
            (*self.0.stub)
                .rename_data_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::RenameDataPolicyRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [new_data_policy_id][crate::model::RenameDataPolicyRequest::new_data_policy_id].
        pub fn set_new_data_policy_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.new_data_policy_id = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for RenameDataPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [DataPolicyService::delete_data_policy][super::super::client::DataPolicyService::delete_data_policy] calls.
    #[derive(Clone, Debug)]
    pub struct DeleteDataPolicy(RequestBuilder<crate::model::DeleteDataPolicyRequest>);

    impl DeleteDataPolicy {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::DataPolicyService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteDataPolicyRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<()> {
            (*self.0.stub)
                .delete_data_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteDataPolicyRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for DeleteDataPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [DataPolicyService::get_data_policy][super::super::client::DataPolicyService::get_data_policy] calls.
    #[derive(Clone, Debug)]
    pub struct GetDataPolicy(RequestBuilder<crate::model::GetDataPolicyRequest>);

    impl GetDataPolicy {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::DataPolicyService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetDataPolicyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::DataPolicy> {
            (*self.0.stub)
                .get_data_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetDataPolicyRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetDataPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [DataPolicyService::list_data_policies][super::super::client::DataPolicyService::list_data_policies] calls.
    #[derive(Clone, Debug)]
    pub struct ListDataPolicies(RequestBuilder<crate::model::ListDataPoliciesRequest>);

    impl ListDataPolicies {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::DataPolicyService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListDataPoliciesRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListDataPoliciesResponse> {
            (*self.0.stub)
                .list_data_policies(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> impl gax::paginator::Paginator<crate::model::ListDataPoliciesResponse, gax::error::Error>
        {
            use std::clone::Clone;
            let token = self.0.request.page_token.clone();
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::internal::new_paginator(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListDataPoliciesRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListDataPoliciesRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListDataPoliciesRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListDataPoliciesRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListDataPolicies {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [DataPolicyService::get_iam_policy][super::super::client::DataPolicyService::get_iam_policy] calls.
    #[derive(Clone, Debug)]
    pub struct GetIamPolicy(RequestBuilder<iam_v1::model::GetIamPolicyRequest>);

    impl GetIamPolicy {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::DataPolicyService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<iam_v1::model::GetIamPolicyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<iam_v1::model::Policy> {
            (*self.0.stub)
                .get_iam_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [resource][iam_v1::model::GetIamPolicyRequest::resource].
        pub fn set_resource<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.resource = v.into();
            self
        }

        /// Sets the value of [options][iam_v1::model::GetIamPolicyRequest::options].
        pub fn set_options<T: Into<std::option::Option<iam_v1::model::GetPolicyOptions>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.options = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetIamPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [DataPolicyService::set_iam_policy][super::super::client::DataPolicyService::set_iam_policy] calls.
    #[derive(Clone, Debug)]
    pub struct SetIamPolicy(RequestBuilder<iam_v1::model::SetIamPolicyRequest>);

    impl SetIamPolicy {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::DataPolicyService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<iam_v1::model::SetIamPolicyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<iam_v1::model::Policy> {
            (*self.0.stub)
                .set_iam_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [resource][iam_v1::model::SetIamPolicyRequest::resource].
        pub fn set_resource<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.resource = v.into();
            self
        }

        /// Sets the value of [policy][iam_v1::model::SetIamPolicyRequest::policy].
        pub fn set_policy<T: Into<std::option::Option<iam_v1::model::Policy>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.policy = v.into();
            self
        }

        /// Sets the value of [update_mask][iam_v1::model::SetIamPolicyRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for SetIamPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [DataPolicyService::test_iam_permissions][super::super::client::DataPolicyService::test_iam_permissions] calls.
    #[derive(Clone, Debug)]
    pub struct TestIamPermissions(RequestBuilder<iam_v1::model::TestIamPermissionsRequest>);

    impl TestIamPermissions {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::DataPolicyService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<iam_v1::model::TestIamPermissionsRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<iam_v1::model::TestIamPermissionsResponse> {
            (*self.0.stub)
                .test_iam_permissions(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [resource][iam_v1::model::TestIamPermissionsRequest::resource].
        pub fn set_resource<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.resource = v.into();
            self
        }

        /// Sets the value of [permissions][iam_v1::model::TestIamPermissionsRequest::permissions].
        pub fn set_permissions<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<std::string::String>,
        {
            use std::iter::Iterator;
            self.0.request.permissions = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for TestIamPermissions {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
