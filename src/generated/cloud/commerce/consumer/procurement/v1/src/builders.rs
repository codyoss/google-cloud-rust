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

pub mod license_management_service {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::LicenseManagementService] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::LicenseManagementService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::LicenseManagementService>,
        ) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a LicenseManagementService::get_license_pool call.
    #[derive(Clone, Debug)]
    pub struct GetLicensePool(RequestBuilder<crate::model::GetLicensePoolRequest>);

    impl GetLicensePool {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::LicenseManagementService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetLicensePoolRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::LicensePool> {
            (*self.0.stub)
                .get_license_pool(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetLicensePoolRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetLicensePool {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a LicenseManagementService::update_license_pool call.
    #[derive(Clone, Debug)]
    pub struct UpdateLicensePool(RequestBuilder<crate::model::UpdateLicensePoolRequest>);

    impl UpdateLicensePool {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::LicenseManagementService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateLicensePoolRequest>>(
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
        pub async fn send(self) -> Result<crate::model::LicensePool> {
            (*self.0.stub)
                .update_license_pool(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [license_pool][crate::model::UpdateLicensePoolRequest::license_pool].
        pub fn set_license_pool<T: Into<std::option::Option<crate::model::LicensePool>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.license_pool = v.into();
            self
        }

        /// Sets the value of [update_mask][crate::model::UpdateLicensePoolRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for UpdateLicensePool {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a LicenseManagementService::assign call.
    #[derive(Clone, Debug)]
    pub struct Assign(RequestBuilder<crate::model::AssignRequest>);

    impl Assign {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::LicenseManagementService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::AssignRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::AssignResponse> {
            (*self.0.stub).assign(self.0.request, self.0.options).await
        }

        /// Sets the value of [parent][crate::model::AssignRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [usernames][crate::model::AssignRequest::usernames].
        pub fn set_usernames<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<std::string::String>,
        {
            use std::iter::Iterator;
            self.0.request.usernames = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for Assign {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a LicenseManagementService::unassign call.
    #[derive(Clone, Debug)]
    pub struct Unassign(RequestBuilder<crate::model::UnassignRequest>);

    impl Unassign {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::LicenseManagementService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UnassignRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::UnassignResponse> {
            (*self.0.stub)
                .unassign(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::UnassignRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [usernames][crate::model::UnassignRequest::usernames].
        pub fn set_usernames<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<std::string::String>,
        {
            use std::iter::Iterator;
            self.0.request.usernames = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for Unassign {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a LicenseManagementService::enumerate_licensed_users call.
    #[derive(Clone, Debug)]
    pub struct EnumerateLicensedUsers(RequestBuilder<crate::model::EnumerateLicensedUsersRequest>);

    impl EnumerateLicensedUsers {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::LicenseManagementService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::EnumerateLicensedUsersRequest>>(
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
        pub async fn send(self) -> Result<crate::model::EnumerateLicensedUsersResponse> {
            (*self.0.stub)
                .enumerate_licensed_users(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<
            crate::model::EnumerateLicensedUsersResponse,
            gax::error::Error,
        > {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::EnumerateLicensedUsersRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::EnumerateLicensedUsersRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::EnumerateLicensedUsersRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for EnumerateLicensedUsers {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a LicenseManagementService::get_operation call.
    #[derive(Clone, Debug)]
    pub struct GetOperation(RequestBuilder<longrunning::model::GetOperationRequest>);

    impl GetOperation {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::LicenseManagementService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<longrunning::model::GetOperationRequest>>(
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
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .get_operation(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][longrunning::model::GetOperationRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetOperation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}

pub mod consumer_procurement_service {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::ConsumerProcurementService] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::ConsumerProcurementService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::ConsumerProcurementService>,
        ) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a ConsumerProcurementService::place_order call.
    #[derive(Clone, Debug)]
    pub struct PlaceOrder(RequestBuilder<crate::model::PlaceOrderRequest>);

    impl PlaceOrder {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::ConsumerProcurementService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::PlaceOrderRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        ///
        /// # Long running operations
        ///
        /// This starts, but does not poll, a longrunning operation. More information
        /// on [place_order][super::super::client::ConsumerProcurementService::place_order].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .place_order(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `place_order`.
        pub fn poller(
            self,
        ) -> impl lro::Poller<crate::model::Order, crate::model::PlaceOrderMetadata> {
            type Operation = lro::Operation<crate::model::Order, crate::model::PlaceOrderMetadata>;
            let polling_policy = self.0.stub.get_polling_policy(&self.0.options);
            let polling_backoff_policy = self.0.stub.get_polling_backoff_policy(&self.0.options);

            let stub = self.0.stub.clone();
            let mut options = self.0.options.clone();
            options.set_retry_policy(gax::retry_policy::NeverRetry);
            let query = move |name| {
                let stub = stub.clone();
                let options = options.clone();
                async {
                    let op = GetOperation::new(stub)
                        .set_name(name)
                        .with_options(options)
                        .send()
                        .await?;
                    Ok(Operation::new(op))
                }
            };

            let start = move || async {
                let op = self.send().await?;
                Ok(Operation::new(op))
            };

            lro::new_poller(polling_policy, polling_backoff_policy, start, query)
        }

        /// Sets the value of [parent][crate::model::PlaceOrderRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [display_name][crate::model::PlaceOrderRequest::display_name].
        pub fn set_display_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.display_name = v.into();
            self
        }

        /// Sets the value of [request_id][crate::model::PlaceOrderRequest::request_id].
        pub fn set_request_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.request_id = v.into();
            self
        }

        /// Sets the value of [line_item_info][crate::model::PlaceOrderRequest::line_item_info].
        pub fn set_line_item_info<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<crate::model::LineItemInfo>,
        {
            use std::iter::Iterator;
            self.0.request.line_item_info = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for PlaceOrder {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ConsumerProcurementService::get_order call.
    #[derive(Clone, Debug)]
    pub struct GetOrder(RequestBuilder<crate::model::GetOrderRequest>);

    impl GetOrder {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::ConsumerProcurementService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetOrderRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Order> {
            (*self.0.stub)
                .get_order(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetOrderRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetOrder {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ConsumerProcurementService::list_orders call.
    #[derive(Clone, Debug)]
    pub struct ListOrders(RequestBuilder<crate::model::ListOrdersRequest>);

    impl ListOrders {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::ConsumerProcurementService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListOrdersRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListOrdersResponse> {
            (*self.0.stub)
                .list_orders(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListOrdersResponse, gax::error::Error>
        {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListOrdersRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListOrdersRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListOrdersRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListOrdersRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListOrders {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ConsumerProcurementService::modify_order call.
    #[derive(Clone, Debug)]
    pub struct ModifyOrder(RequestBuilder<crate::model::ModifyOrderRequest>);

    impl ModifyOrder {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::ConsumerProcurementService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ModifyOrderRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        ///
        /// # Long running operations
        ///
        /// This starts, but does not poll, a longrunning operation. More information
        /// on [modify_order][super::super::client::ConsumerProcurementService::modify_order].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .modify_order(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `modify_order`.
        pub fn poller(
            self,
        ) -> impl lro::Poller<crate::model::Order, crate::model::ModifyOrderMetadata> {
            type Operation = lro::Operation<crate::model::Order, crate::model::ModifyOrderMetadata>;
            let polling_policy = self.0.stub.get_polling_policy(&self.0.options);
            let polling_backoff_policy = self.0.stub.get_polling_backoff_policy(&self.0.options);

            let stub = self.0.stub.clone();
            let mut options = self.0.options.clone();
            options.set_retry_policy(gax::retry_policy::NeverRetry);
            let query = move |name| {
                let stub = stub.clone();
                let options = options.clone();
                async {
                    let op = GetOperation::new(stub)
                        .set_name(name)
                        .with_options(options)
                        .send()
                        .await?;
                    Ok(Operation::new(op))
                }
            };

            let start = move || async {
                let op = self.send().await?;
                Ok(Operation::new(op))
            };

            lro::new_poller(polling_policy, polling_backoff_policy, start, query)
        }

        /// Sets the value of [name][crate::model::ModifyOrderRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [display_name][crate::model::ModifyOrderRequest::display_name].
        pub fn set_display_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.display_name = v.into();
            self
        }

        /// Sets the value of [etag][crate::model::ModifyOrderRequest::etag].
        pub fn set_etag<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.etag = v.into();
            self
        }

        /// Sets the value of [modifications][crate::model::ModifyOrderRequest::modifications].
        pub fn set_modifications<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<crate::model::modify_order_request::Modification>,
        {
            use std::iter::Iterator;
            self.0.request.modifications = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for ModifyOrder {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ConsumerProcurementService::cancel_order call.
    #[derive(Clone, Debug)]
    pub struct CancelOrder(RequestBuilder<crate::model::CancelOrderRequest>);

    impl CancelOrder {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::ConsumerProcurementService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CancelOrderRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        ///
        /// # Long running operations
        ///
        /// This starts, but does not poll, a longrunning operation. More information
        /// on [cancel_order][super::super::client::ConsumerProcurementService::cancel_order].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .cancel_order(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `cancel_order`.
        pub fn poller(
            self,
        ) -> impl lro::Poller<crate::model::Order, crate::model::CancelOrderMetadata> {
            type Operation = lro::Operation<crate::model::Order, crate::model::CancelOrderMetadata>;
            let polling_policy = self.0.stub.get_polling_policy(&self.0.options);
            let polling_backoff_policy = self.0.stub.get_polling_backoff_policy(&self.0.options);

            let stub = self.0.stub.clone();
            let mut options = self.0.options.clone();
            options.set_retry_policy(gax::retry_policy::NeverRetry);
            let query = move |name| {
                let stub = stub.clone();
                let options = options.clone();
                async {
                    let op = GetOperation::new(stub)
                        .set_name(name)
                        .with_options(options)
                        .send()
                        .await?;
                    Ok(Operation::new(op))
                }
            };

            let start = move || async {
                let op = self.send().await?;
                Ok(Operation::new(op))
            };

            lro::new_poller(polling_policy, polling_backoff_policy, start, query)
        }

        /// Sets the value of [name][crate::model::CancelOrderRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [etag][crate::model::CancelOrderRequest::etag].
        pub fn set_etag<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.etag = v.into();
            self
        }

        /// Sets the value of [cancellation_policy][crate::model::CancelOrderRequest::cancellation_policy].
        pub fn set_cancellation_policy<
            T: Into<crate::model::cancel_order_request::CancellationPolicy>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.cancellation_policy = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CancelOrder {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ConsumerProcurementService::get_operation call.
    #[derive(Clone, Debug)]
    pub struct GetOperation(RequestBuilder<longrunning::model::GetOperationRequest>);

    impl GetOperation {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::ConsumerProcurementService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<longrunning::model::GetOperationRequest>>(
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
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .get_operation(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][longrunning::model::GetOperationRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetOperation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
