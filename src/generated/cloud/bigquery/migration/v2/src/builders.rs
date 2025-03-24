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

pub mod migration_service {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::MigrationService] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::MigrationService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::MigrationService>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a MigrationService::create_migration_workflow call.
    #[derive(Clone, Debug)]
    pub struct CreateMigrationWorkflow(
        RequestBuilder<crate::model::CreateMigrationWorkflowRequest>,
    );

    impl CreateMigrationWorkflow {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::MigrationService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateMigrationWorkflowRequest>>(
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
        pub async fn send(self) -> Result<crate::model::MigrationWorkflow> {
            (*self.0.stub)
                .create_migration_workflow(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateMigrationWorkflowRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [migration_workflow][crate::model::CreateMigrationWorkflowRequest::migration_workflow].
        pub fn set_migration_workflow<
            T: Into<std::option::Option<crate::model::MigrationWorkflow>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.migration_workflow = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateMigrationWorkflow {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a MigrationService::get_migration_workflow call.
    #[derive(Clone, Debug)]
    pub struct GetMigrationWorkflow(RequestBuilder<crate::model::GetMigrationWorkflowRequest>);

    impl GetMigrationWorkflow {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::MigrationService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetMigrationWorkflowRequest>>(
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
        pub async fn send(self) -> Result<crate::model::MigrationWorkflow> {
            (*self.0.stub)
                .get_migration_workflow(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetMigrationWorkflowRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [read_mask][crate::model::GetMigrationWorkflowRequest::read_mask].
        pub fn set_read_mask<T: Into<std::option::Option<wkt::FieldMask>>>(mut self, v: T) -> Self {
            self.0.request.read_mask = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetMigrationWorkflow {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a MigrationService::list_migration_workflows call.
    #[derive(Clone, Debug)]
    pub struct ListMigrationWorkflows(RequestBuilder<crate::model::ListMigrationWorkflowsRequest>);

    impl ListMigrationWorkflows {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::MigrationService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListMigrationWorkflowsRequest>>(
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
        pub async fn send(self) -> Result<crate::model::ListMigrationWorkflowsResponse> {
            (*self.0.stub)
                .list_migration_workflows(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<
            crate::model::ListMigrationWorkflowsResponse,
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

        /// Sets the value of [parent][crate::model::ListMigrationWorkflowsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [read_mask][crate::model::ListMigrationWorkflowsRequest::read_mask].
        pub fn set_read_mask<T: Into<std::option::Option<wkt::FieldMask>>>(mut self, v: T) -> Self {
            self.0.request.read_mask = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListMigrationWorkflowsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListMigrationWorkflowsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListMigrationWorkflows {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a MigrationService::delete_migration_workflow call.
    #[derive(Clone, Debug)]
    pub struct DeleteMigrationWorkflow(
        RequestBuilder<crate::model::DeleteMigrationWorkflowRequest>,
    );

    impl DeleteMigrationWorkflow {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::MigrationService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteMigrationWorkflowRequest>>(
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
                .delete_migration_workflow(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteMigrationWorkflowRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteMigrationWorkflow {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a MigrationService::start_migration_workflow call.
    #[derive(Clone, Debug)]
    pub struct StartMigrationWorkflow(RequestBuilder<crate::model::StartMigrationWorkflowRequest>);

    impl StartMigrationWorkflow {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::MigrationService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::StartMigrationWorkflowRequest>>(
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
                .start_migration_workflow(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::StartMigrationWorkflowRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for StartMigrationWorkflow {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a MigrationService::get_migration_subtask call.
    #[derive(Clone, Debug)]
    pub struct GetMigrationSubtask(RequestBuilder<crate::model::GetMigrationSubtaskRequest>);

    impl GetMigrationSubtask {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::MigrationService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetMigrationSubtaskRequest>>(
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
        pub async fn send(self) -> Result<crate::model::MigrationSubtask> {
            (*self.0.stub)
                .get_migration_subtask(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetMigrationSubtaskRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [read_mask][crate::model::GetMigrationSubtaskRequest::read_mask].
        pub fn set_read_mask<T: Into<std::option::Option<wkt::FieldMask>>>(mut self, v: T) -> Self {
            self.0.request.read_mask = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetMigrationSubtask {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a MigrationService::list_migration_subtasks call.
    #[derive(Clone, Debug)]
    pub struct ListMigrationSubtasks(RequestBuilder<crate::model::ListMigrationSubtasksRequest>);

    impl ListMigrationSubtasks {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::MigrationService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListMigrationSubtasksRequest>>(
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
        pub async fn send(self) -> Result<crate::model::ListMigrationSubtasksResponse> {
            (*self.0.stub)
                .list_migration_subtasks(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListMigrationSubtasksResponse, gax::error::Error>
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

        /// Sets the value of [parent][crate::model::ListMigrationSubtasksRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [read_mask][crate::model::ListMigrationSubtasksRequest::read_mask].
        pub fn set_read_mask<T: Into<std::option::Option<wkt::FieldMask>>>(mut self, v: T) -> Self {
            self.0.request.read_mask = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListMigrationSubtasksRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListMigrationSubtasksRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListMigrationSubtasksRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListMigrationSubtasks {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
