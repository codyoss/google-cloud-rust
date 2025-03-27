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

pub mod iam_checker {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::IamChecker] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::IamChecker>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::IamChecker>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a IamChecker::troubleshoot_iam_policy call.
    #[derive(Clone, Debug)]
    pub struct TroubleshootIamPolicy(RequestBuilder<crate::model::TroubleshootIamPolicyRequest>);

    impl TroubleshootIamPolicy {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::IamChecker>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::TroubleshootIamPolicyRequest>>(
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
        pub async fn send(self) -> Result<crate::model::TroubleshootIamPolicyResponse> {
            (*self.0.stub)
                .troubleshoot_iam_policy(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [access_tuple][crate::model::TroubleshootIamPolicyRequest::access_tuple].
        pub fn set_access_tuple<T: Into<std::option::Option<crate::model::AccessTuple>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.access_tuple = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for TroubleshootIamPolicy {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
