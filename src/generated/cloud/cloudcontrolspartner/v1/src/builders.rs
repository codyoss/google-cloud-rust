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

pub mod cloud_controls_partner_core {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::CloudControlsPartnerCore] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::CloudControlsPartnerCore>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::CloudControlsPartnerCore>,
        ) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a CloudControlsPartnerCore::get_workload call.
    #[derive(Clone, Debug)]
    pub struct GetWorkload(RequestBuilder<crate::model::GetWorkloadRequest>);

    impl GetWorkload {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::CloudControlsPartnerCore>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetWorkloadRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Workload> {
            (*self.0.stub)
                .get_workload(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetWorkloadRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetWorkload {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudControlsPartnerCore::list_workloads call.
    #[derive(Clone, Debug)]
    pub struct ListWorkloads(RequestBuilder<crate::model::ListWorkloadsRequest>);

    impl ListWorkloads {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::CloudControlsPartnerCore>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListWorkloadsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListWorkloadsResponse> {
            (*self.0.stub)
                .list_workloads(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListWorkloadsResponse, gax::error::Error>
        {
            use std::clone::Clone;
            let token = self.0.request.page_token.clone();
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListWorkloadsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListWorkloadsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListWorkloadsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListWorkloadsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [order_by][crate::model::ListWorkloadsRequest::order_by].
        pub fn set_order_by<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.order_by = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListWorkloads {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudControlsPartnerCore::get_customer call.
    #[derive(Clone, Debug)]
    pub struct GetCustomer(RequestBuilder<crate::model::GetCustomerRequest>);

    impl GetCustomer {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::CloudControlsPartnerCore>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetCustomerRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Customer> {
            (*self.0.stub)
                .get_customer(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetCustomerRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetCustomer {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudControlsPartnerCore::list_customers call.
    #[derive(Clone, Debug)]
    pub struct ListCustomers(RequestBuilder<crate::model::ListCustomersRequest>);

    impl ListCustomers {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::CloudControlsPartnerCore>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListCustomersRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListCustomersResponse> {
            (*self.0.stub)
                .list_customers(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListCustomersResponse, gax::error::Error>
        {
            use std::clone::Clone;
            let token = self.0.request.page_token.clone();
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListCustomersRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListCustomersRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListCustomersRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListCustomersRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [order_by][crate::model::ListCustomersRequest::order_by].
        pub fn set_order_by<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.order_by = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListCustomers {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudControlsPartnerCore::get_ekm_connections call.
    #[derive(Clone, Debug)]
    pub struct GetEkmConnections(RequestBuilder<crate::model::GetEkmConnectionsRequest>);

    impl GetEkmConnections {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::CloudControlsPartnerCore>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetEkmConnectionsRequest>>(
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
        pub async fn send(self) -> Result<crate::model::EkmConnections> {
            (*self.0.stub)
                .get_ekm_connections(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetEkmConnectionsRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetEkmConnections {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudControlsPartnerCore::get_partner_permissions call.
    #[derive(Clone, Debug)]
    pub struct GetPartnerPermissions(RequestBuilder<crate::model::GetPartnerPermissionsRequest>);

    impl GetPartnerPermissions {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::CloudControlsPartnerCore>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetPartnerPermissionsRequest>>(
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
        pub async fn send(self) -> Result<crate::model::PartnerPermissions> {
            (*self.0.stub)
                .get_partner_permissions(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetPartnerPermissionsRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetPartnerPermissions {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudControlsPartnerCore::list_access_approval_requests call.
    #[derive(Clone, Debug)]
    pub struct ListAccessApprovalRequests(
        RequestBuilder<crate::model::ListAccessApprovalRequestsRequest>,
    );

    impl ListAccessApprovalRequests {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::CloudControlsPartnerCore>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListAccessApprovalRequestsRequest>>(
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
        pub async fn send(self) -> Result<crate::model::ListAccessApprovalRequestsResponse> {
            (*self.0.stub)
                .list_access_approval_requests(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<
            crate::model::ListAccessApprovalRequestsResponse,
            gax::error::Error,
        > {
            use std::clone::Clone;
            let token = self.0.request.page_token.clone();
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListAccessApprovalRequestsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListAccessApprovalRequestsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListAccessApprovalRequestsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListAccessApprovalRequestsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [order_by][crate::model::ListAccessApprovalRequestsRequest::order_by].
        pub fn set_order_by<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.order_by = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListAccessApprovalRequests {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudControlsPartnerCore::get_partner call.
    #[derive(Clone, Debug)]
    pub struct GetPartner(RequestBuilder<crate::model::GetPartnerRequest>);

    impl GetPartner {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::CloudControlsPartnerCore>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetPartnerRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Partner> {
            (*self.0.stub)
                .get_partner(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetPartnerRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetPartner {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}

pub mod cloud_controls_partner_monitoring {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::CloudControlsPartnerMonitoring] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::CloudControlsPartnerMonitoring>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::CloudControlsPartnerMonitoring>,
        ) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a CloudControlsPartnerMonitoring::list_violations call.
    #[derive(Clone, Debug)]
    pub struct ListViolations(RequestBuilder<crate::model::ListViolationsRequest>);

    impl ListViolations {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::CloudControlsPartnerMonitoring>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListViolationsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListViolationsResponse> {
            (*self.0.stub)
                .list_violations(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListViolationsResponse, gax::error::Error>
        {
            use std::clone::Clone;
            let token = self.0.request.page_token.clone();
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListViolationsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListViolationsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListViolationsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListViolationsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [order_by][crate::model::ListViolationsRequest::order_by].
        pub fn set_order_by<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.order_by = v.into();
            self
        }

        /// Sets the value of [interval][crate::model::ListViolationsRequest::interval].
        pub fn set_interval<T: Into<std::option::Option<gtype::model::Interval>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.interval = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListViolations {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a CloudControlsPartnerMonitoring::get_violation call.
    #[derive(Clone, Debug)]
    pub struct GetViolation(RequestBuilder<crate::model::GetViolationRequest>);

    impl GetViolation {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::CloudControlsPartnerMonitoring>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetViolationRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Violation> {
            (*self.0.stub)
                .get_violation(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetViolationRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetViolation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
