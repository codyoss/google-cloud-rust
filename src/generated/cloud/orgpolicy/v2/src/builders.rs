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

pub mod org_policy {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::OrgPolicy] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::OrgPolicy>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::OrgPolicy>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a OrgPolicy::list_constraints call.
    #[derive(Clone, Debug)]
    pub struct ListConstraints(RequestBuilder<crate::model::ListConstraintsRequest>);

    impl ListConstraints {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::OrgPolicy>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListConstraintsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListConstraintsResponse> {
            (*self.0.stub)
                .list_constraints(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListConstraintsResponse, gax::error::Error>
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

        /// Sets the value of [parent][crate::model::ListConstraintsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListConstraintsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListConstraintsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListConstraints {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a OrgPolicy::list_policies call.
    #[derive(Clone, Debug)]
    pub struct ListPolicies(RequestBuilder<crate::model::ListPoliciesRequest>);

    impl ListPolicies {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::OrgPolicy>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListPoliciesRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListPoliciesResponse> {
            (*self.0.stub)
                .list_policies(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListPoliciesResponse, gax::error::Error>
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

        /// Sets the value of [parent][crate::model::ListPoliciesRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListPoliciesRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListPoliciesRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListPolicies {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a OrgPolicy::get_policy call.
    #[derive(Clone, Debug)]
    pub struct GetPolicy(RequestBuilder<crate::model::GetPolicyRequest>);

    impl GetPolicy {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::OrgPolicy>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetPolicyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Policy> {
            (*self.0.stub)
                .get_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetPolicyRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a OrgPolicy::get_effective_policy call.
    #[derive(Clone, Debug)]
    pub struct GetEffectivePolicy(RequestBuilder<crate::model::GetEffectivePolicyRequest>);

    impl GetEffectivePolicy {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::OrgPolicy>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetEffectivePolicyRequest>>(
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
        pub async fn send(self) -> Result<crate::model::Policy> {
            (*self.0.stub)
                .get_effective_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetEffectivePolicyRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetEffectivePolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a OrgPolicy::create_policy call.
    #[derive(Clone, Debug)]
    pub struct CreatePolicy(RequestBuilder<crate::model::CreatePolicyRequest>);

    impl CreatePolicy {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::OrgPolicy>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreatePolicyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Policy> {
            (*self.0.stub)
                .create_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreatePolicyRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [policy][crate::model::CreatePolicyRequest::policy].
        pub fn set_policy<T: Into<std::option::Option<crate::model::Policy>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.policy = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreatePolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a OrgPolicy::update_policy call.
    #[derive(Clone, Debug)]
    pub struct UpdatePolicy(RequestBuilder<crate::model::UpdatePolicyRequest>);

    impl UpdatePolicy {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::OrgPolicy>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdatePolicyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Policy> {
            (*self.0.stub)
                .update_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [policy][crate::model::UpdatePolicyRequest::policy].
        pub fn set_policy<T: Into<std::option::Option<crate::model::Policy>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.policy = v.into();
            self
        }

        /// Sets the value of [update_mask][crate::model::UpdatePolicyRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for UpdatePolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a OrgPolicy::delete_policy call.
    #[derive(Clone, Debug)]
    pub struct DeletePolicy(RequestBuilder<crate::model::DeletePolicyRequest>);

    impl DeletePolicy {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::OrgPolicy>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeletePolicyRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<wkt::Empty> {
            (*self.0.stub)
                .delete_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeletePolicyRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [etag][crate::model::DeletePolicyRequest::etag].
        pub fn set_etag<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.etag = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeletePolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a OrgPolicy::create_custom_constraint call.
    #[derive(Clone, Debug)]
    pub struct CreateCustomConstraint(RequestBuilder<crate::model::CreateCustomConstraintRequest>);

    impl CreateCustomConstraint {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::OrgPolicy>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateCustomConstraintRequest>>(
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
        pub async fn send(self) -> Result<crate::model::CustomConstraint> {
            (*self.0.stub)
                .create_custom_constraint(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateCustomConstraintRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [custom_constraint][crate::model::CreateCustomConstraintRequest::custom_constraint].
        pub fn set_custom_constraint<
            T: Into<std::option::Option<crate::model::CustomConstraint>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.custom_constraint = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateCustomConstraint {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a OrgPolicy::update_custom_constraint call.
    #[derive(Clone, Debug)]
    pub struct UpdateCustomConstraint(RequestBuilder<crate::model::UpdateCustomConstraintRequest>);

    impl UpdateCustomConstraint {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::OrgPolicy>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateCustomConstraintRequest>>(
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
        pub async fn send(self) -> Result<crate::model::CustomConstraint> {
            (*self.0.stub)
                .update_custom_constraint(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [custom_constraint][crate::model::UpdateCustomConstraintRequest::custom_constraint].
        pub fn set_custom_constraint<
            T: Into<std::option::Option<crate::model::CustomConstraint>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.custom_constraint = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for UpdateCustomConstraint {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a OrgPolicy::get_custom_constraint call.
    #[derive(Clone, Debug)]
    pub struct GetCustomConstraint(RequestBuilder<crate::model::GetCustomConstraintRequest>);

    impl GetCustomConstraint {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::OrgPolicy>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetCustomConstraintRequest>>(
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
        pub async fn send(self) -> Result<crate::model::CustomConstraint> {
            (*self.0.stub)
                .get_custom_constraint(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetCustomConstraintRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetCustomConstraint {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a OrgPolicy::list_custom_constraints call.
    #[derive(Clone, Debug)]
    pub struct ListCustomConstraints(RequestBuilder<crate::model::ListCustomConstraintsRequest>);

    impl ListCustomConstraints {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::OrgPolicy>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListCustomConstraintsRequest>>(
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
        pub async fn send(self) -> Result<crate::model::ListCustomConstraintsResponse> {
            (*self.0.stub)
                .list_custom_constraints(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListCustomConstraintsResponse, gax::error::Error>
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

        /// Sets the value of [parent][crate::model::ListCustomConstraintsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListCustomConstraintsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListCustomConstraintsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListCustomConstraints {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a OrgPolicy::delete_custom_constraint call.
    #[derive(Clone, Debug)]
    pub struct DeleteCustomConstraint(RequestBuilder<crate::model::DeleteCustomConstraintRequest>);

    impl DeleteCustomConstraint {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::OrgPolicy>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteCustomConstraintRequest>>(
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
        pub async fn send(self) -> Result<wkt::Empty> {
            (*self.0.stub)
                .delete_custom_constraint(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteCustomConstraintRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteCustomConstraint {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
