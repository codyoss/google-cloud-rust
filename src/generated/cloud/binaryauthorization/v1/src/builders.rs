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

pub mod binauthz_management_service_v_1 {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::BinauthzManagementServiceV1] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::BinauthzManagementServiceV1>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::BinauthzManagementServiceV1>,
        ) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a BinauthzManagementServiceV1::get_policy call.
    #[derive(Clone, Debug)]
    pub struct GetPolicy(RequestBuilder<crate::model::GetPolicyRequest>);

    impl GetPolicy {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::BinauthzManagementServiceV1>,
        ) -> Self {
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

    /// The request builder for a BinauthzManagementServiceV1::update_policy call.
    #[derive(Clone, Debug)]
    pub struct UpdatePolicy(RequestBuilder<crate::model::UpdatePolicyRequest>);

    impl UpdatePolicy {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::BinauthzManagementServiceV1>,
        ) -> Self {
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
    }

    impl gax::options::RequestBuilder for UpdatePolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a BinauthzManagementServiceV1::create_attestor call.
    #[derive(Clone, Debug)]
    pub struct CreateAttestor(RequestBuilder<crate::model::CreateAttestorRequest>);

    impl CreateAttestor {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::BinauthzManagementServiceV1>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateAttestorRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Attestor> {
            (*self.0.stub)
                .create_attestor(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateAttestorRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [attestor_id][crate::model::CreateAttestorRequest::attestor_id].
        pub fn set_attestor_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.attestor_id = v.into();
            self
        }

        /// Sets the value of [attestor][crate::model::CreateAttestorRequest::attestor].
        pub fn set_attestor<T: Into<std::option::Option<crate::model::Attestor>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.attestor = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateAttestor {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a BinauthzManagementServiceV1::get_attestor call.
    #[derive(Clone, Debug)]
    pub struct GetAttestor(RequestBuilder<crate::model::GetAttestorRequest>);

    impl GetAttestor {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::BinauthzManagementServiceV1>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetAttestorRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Attestor> {
            (*self.0.stub)
                .get_attestor(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetAttestorRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetAttestor {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a BinauthzManagementServiceV1::update_attestor call.
    #[derive(Clone, Debug)]
    pub struct UpdateAttestor(RequestBuilder<crate::model::UpdateAttestorRequest>);

    impl UpdateAttestor {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::BinauthzManagementServiceV1>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateAttestorRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Attestor> {
            (*self.0.stub)
                .update_attestor(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [attestor][crate::model::UpdateAttestorRequest::attestor].
        pub fn set_attestor<T: Into<std::option::Option<crate::model::Attestor>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.attestor = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for UpdateAttestor {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a BinauthzManagementServiceV1::list_attestors call.
    #[derive(Clone, Debug)]
    pub struct ListAttestors(RequestBuilder<crate::model::ListAttestorsRequest>);

    impl ListAttestors {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::BinauthzManagementServiceV1>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListAttestorsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListAttestorsResponse> {
            (*self.0.stub)
                .list_attestors(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListAttestorsResponse, gax::error::Error>
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

        /// Sets the value of [parent][crate::model::ListAttestorsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListAttestorsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListAttestorsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListAttestors {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a BinauthzManagementServiceV1::delete_attestor call.
    #[derive(Clone, Debug)]
    pub struct DeleteAttestor(RequestBuilder<crate::model::DeleteAttestorRequest>);

    impl DeleteAttestor {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::BinauthzManagementServiceV1>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteAttestorRequest>>(mut self, v: V) -> Self {
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
                .delete_attestor(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteAttestorRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteAttestor {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}

pub mod system_policy_v_1 {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::SystemPolicyV1] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::SystemPolicyV1>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::SystemPolicyV1>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a SystemPolicyV1::get_system_policy call.
    #[derive(Clone, Debug)]
    pub struct GetSystemPolicy(RequestBuilder<crate::model::GetSystemPolicyRequest>);

    impl GetSystemPolicy {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::SystemPolicyV1>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetSystemPolicyRequest>>(mut self, v: V) -> Self {
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
                .get_system_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetSystemPolicyRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetSystemPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}

pub mod validation_helper_v_1 {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::ValidationHelperV1] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::ValidationHelperV1>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::ValidationHelperV1>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a ValidationHelperV1::validate_attestation_occurrence call.
    #[derive(Clone, Debug)]
    pub struct ValidateAttestationOccurrence(
        RequestBuilder<crate::model::ValidateAttestationOccurrenceRequest>,
    );

    impl ValidateAttestationOccurrence {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::ValidationHelperV1>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ValidateAttestationOccurrenceRequest>>(
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
        pub async fn send(self) -> Result<crate::model::ValidateAttestationOccurrenceResponse> {
            (*self.0.stub)
                .validate_attestation_occurrence(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [attestor][crate::model::ValidateAttestationOccurrenceRequest::attestor].
        pub fn set_attestor<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.attestor = v.into();
            self
        }

        /// Sets the value of [attestation][crate::model::ValidateAttestationOccurrenceRequest::attestation].
        pub fn set_attestation<
            T: Into<std::option::Option<grafeas::model::AttestationOccurrence>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.attestation = v.into();
            self
        }

        /// Sets the value of [occurrence_note][crate::model::ValidateAttestationOccurrenceRequest::occurrence_note].
        pub fn set_occurrence_note<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.occurrence_note = v.into();
            self
        }

        /// Sets the value of [occurrence_resource_uri][crate::model::ValidateAttestationOccurrenceRequest::occurrence_resource_uri].
        pub fn set_occurrence_resource_uri<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.occurrence_resource_uri = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ValidateAttestationOccurrence {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
