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

pub mod datastore_admin {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::DatastoreAdmin] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::DatastoreAdmin>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::DatastoreAdmin>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a DatastoreAdmin::export_entities call.
    #[derive(Clone, Debug)]
    pub struct ExportEntities(RequestBuilder<crate::model::ExportEntitiesRequest>);

    impl ExportEntities {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::DatastoreAdmin>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ExportEntitiesRequest>>(mut self, v: V) -> Self {
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
        /// on [export_entities][super::super::client::DatastoreAdmin::export_entities].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .export_entities(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `export_entities`.
        pub fn poller(
            self,
        ) -> impl lro::Poller<crate::model::ExportEntitiesResponse, crate::model::ExportEntitiesMetadata>
        {
            type Operation = lro::Operation<
                crate::model::ExportEntitiesResponse,
                crate::model::ExportEntitiesMetadata,
            >;
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

        /// Sets the value of [project_id][crate::model::ExportEntitiesRequest::project_id].
        pub fn set_project_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.project_id = v.into();
            self
        }

        /// Sets the value of [entity_filter][crate::model::ExportEntitiesRequest::entity_filter].
        pub fn set_entity_filter<T: Into<std::option::Option<crate::model::EntityFilter>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.entity_filter = v.into();
            self
        }

        /// Sets the value of [output_url_prefix][crate::model::ExportEntitiesRequest::output_url_prefix].
        pub fn set_output_url_prefix<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.output_url_prefix = v.into();
            self
        }

        /// Sets the value of [labels][crate::model::ExportEntitiesRequest::labels].
        pub fn set_labels<T, K, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = (K, V)>,
            K: std::convert::Into<std::string::String>,
            V: std::convert::Into<std::string::String>,
        {
            self.0.request.labels = v.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for ExportEntities {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a DatastoreAdmin::import_entities call.
    #[derive(Clone, Debug)]
    pub struct ImportEntities(RequestBuilder<crate::model::ImportEntitiesRequest>);

    impl ImportEntities {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::DatastoreAdmin>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ImportEntitiesRequest>>(mut self, v: V) -> Self {
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
        /// on [import_entities][super::super::client::DatastoreAdmin::import_entities].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .import_entities(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `import_entities`.
        pub fn poller(self) -> impl lro::Poller<wkt::Empty, crate::model::ImportEntitiesMetadata> {
            type Operation = lro::Operation<wkt::Empty, crate::model::ImportEntitiesMetadata>;
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

        /// Sets the value of [project_id][crate::model::ImportEntitiesRequest::project_id].
        pub fn set_project_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.project_id = v.into();
            self
        }

        /// Sets the value of [input_url][crate::model::ImportEntitiesRequest::input_url].
        pub fn set_input_url<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.input_url = v.into();
            self
        }

        /// Sets the value of [entity_filter][crate::model::ImportEntitiesRequest::entity_filter].
        pub fn set_entity_filter<T: Into<std::option::Option<crate::model::EntityFilter>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.entity_filter = v.into();
            self
        }

        /// Sets the value of [labels][crate::model::ImportEntitiesRequest::labels].
        pub fn set_labels<T, K, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = (K, V)>,
            K: std::convert::Into<std::string::String>,
            V: std::convert::Into<std::string::String>,
        {
            self.0.request.labels = v.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for ImportEntities {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a DatastoreAdmin::create_index call.
    #[derive(Clone, Debug)]
    pub struct CreateIndex(RequestBuilder<crate::model::CreateIndexRequest>);

    impl CreateIndex {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::DatastoreAdmin>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateIndexRequest>>(mut self, v: V) -> Self {
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
        /// on [create_index][super::super::client::DatastoreAdmin::create_index].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .create_index(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `create_index`.
        pub fn poller(
            self,
        ) -> impl lro::Poller<crate::model::Index, crate::model::IndexOperationMetadata> {
            type Operation =
                lro::Operation<crate::model::Index, crate::model::IndexOperationMetadata>;
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

        /// Sets the value of [project_id][crate::model::CreateIndexRequest::project_id].
        pub fn set_project_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.project_id = v.into();
            self
        }

        /// Sets the value of [index][crate::model::CreateIndexRequest::index].
        pub fn set_index<T: Into<std::option::Option<crate::model::Index>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.index = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateIndex {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a DatastoreAdmin::delete_index call.
    #[derive(Clone, Debug)]
    pub struct DeleteIndex(RequestBuilder<crate::model::DeleteIndexRequest>);

    impl DeleteIndex {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::DatastoreAdmin>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteIndexRequest>>(mut self, v: V) -> Self {
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
        /// on [delete_index][super::super::client::DatastoreAdmin::delete_index].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .delete_index(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `delete_index`.
        pub fn poller(
            self,
        ) -> impl lro::Poller<crate::model::Index, crate::model::IndexOperationMetadata> {
            type Operation =
                lro::Operation<crate::model::Index, crate::model::IndexOperationMetadata>;
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

        /// Sets the value of [project_id][crate::model::DeleteIndexRequest::project_id].
        pub fn set_project_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.project_id = v.into();
            self
        }

        /// Sets the value of [index_id][crate::model::DeleteIndexRequest::index_id].
        pub fn set_index_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.index_id = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteIndex {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a DatastoreAdmin::get_index call.
    #[derive(Clone, Debug)]
    pub struct GetIndex(RequestBuilder<crate::model::GetIndexRequest>);

    impl GetIndex {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::DatastoreAdmin>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetIndexRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Index> {
            (*self.0.stub)
                .get_index(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [project_id][crate::model::GetIndexRequest::project_id].
        pub fn set_project_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.project_id = v.into();
            self
        }

        /// Sets the value of [index_id][crate::model::GetIndexRequest::index_id].
        pub fn set_index_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.index_id = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetIndex {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a DatastoreAdmin::list_indexes call.
    #[derive(Clone, Debug)]
    pub struct ListIndexes(RequestBuilder<crate::model::ListIndexesRequest>);

    impl ListIndexes {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::DatastoreAdmin>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListIndexesRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListIndexesResponse> {
            (*self.0.stub)
                .list_indexes(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListIndexesResponse, gax::error::Error>
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

        /// Sets the value of [project_id][crate::model::ListIndexesRequest::project_id].
        pub fn set_project_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.project_id = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListIndexesRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListIndexesRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListIndexesRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListIndexes {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a DatastoreAdmin::list_operations call.
    #[derive(Clone, Debug)]
    pub struct ListOperations(RequestBuilder<longrunning::model::ListOperationsRequest>);

    impl ListOperations {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::DatastoreAdmin>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<longrunning::model::ListOperationsRequest>>(
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
        pub async fn send(self) -> Result<longrunning::model::ListOperationsResponse> {
            (*self.0.stub)
                .list_operations(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<longrunning::model::ListOperationsResponse, gax::error::Error>
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

        /// Sets the value of [name][longrunning::model::ListOperationsRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [filter][longrunning::model::ListOperationsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [page_size][longrunning::model::ListOperationsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][longrunning::model::ListOperationsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListOperations {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a DatastoreAdmin::get_operation call.
    #[derive(Clone, Debug)]
    pub struct GetOperation(RequestBuilder<longrunning::model::GetOperationRequest>);

    impl GetOperation {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::DatastoreAdmin>) -> Self {
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

    /// The request builder for a DatastoreAdmin::delete_operation call.
    #[derive(Clone, Debug)]
    pub struct DeleteOperation(RequestBuilder<longrunning::model::DeleteOperationRequest>);

    impl DeleteOperation {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::DatastoreAdmin>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<longrunning::model::DeleteOperationRequest>>(
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
                .delete_operation(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][longrunning::model::DeleteOperationRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteOperation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a DatastoreAdmin::cancel_operation call.
    #[derive(Clone, Debug)]
    pub struct CancelOperation(RequestBuilder<longrunning::model::CancelOperationRequest>);

    impl CancelOperation {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::DatastoreAdmin>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<longrunning::model::CancelOperationRequest>>(
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
                .cancel_operation(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][longrunning::model::CancelOperationRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CancelOperation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
