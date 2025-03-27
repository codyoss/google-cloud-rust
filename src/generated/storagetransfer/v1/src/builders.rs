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

pub mod storage_transfer_service {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::StorageTransferService] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        ) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a StorageTransferService::get_google_service_account call.
    #[derive(Clone, Debug)]
    pub struct GetGoogleServiceAccount(
        RequestBuilder<crate::model::GetGoogleServiceAccountRequest>,
    );

    impl GetGoogleServiceAccount {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetGoogleServiceAccountRequest>>(
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
        pub async fn send(self) -> Result<crate::model::GoogleServiceAccount> {
            (*self.0.stub)
                .get_google_service_account(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [project_id][crate::model::GetGoogleServiceAccountRequest::project_id].
        pub fn set_project_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.project_id = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetGoogleServiceAccount {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a StorageTransferService::create_transfer_job call.
    #[derive(Clone, Debug)]
    pub struct CreateTransferJob(RequestBuilder<crate::model::CreateTransferJobRequest>);

    impl CreateTransferJob {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateTransferJobRequest>>(
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
        pub async fn send(self) -> Result<crate::model::TransferJob> {
            (*self.0.stub)
                .create_transfer_job(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [transfer_job][crate::model::CreateTransferJobRequest::transfer_job].
        pub fn set_transfer_job<T: Into<std::option::Option<crate::model::TransferJob>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.transfer_job = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for CreateTransferJob {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a StorageTransferService::update_transfer_job call.
    #[derive(Clone, Debug)]
    pub struct UpdateTransferJob(RequestBuilder<crate::model::UpdateTransferJobRequest>);

    impl UpdateTransferJob {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateTransferJobRequest>>(
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
        pub async fn send(self) -> Result<crate::model::TransferJob> {
            (*self.0.stub)
                .update_transfer_job(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [job_name][crate::model::UpdateTransferJobRequest::job_name].
        pub fn set_job_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.job_name = v.into();
            self
        }

        /// Sets the value of [project_id][crate::model::UpdateTransferJobRequest::project_id].
        pub fn set_project_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.project_id = v.into();
            self
        }

        /// Sets the value of [transfer_job][crate::model::UpdateTransferJobRequest::transfer_job].
        pub fn set_transfer_job<T: Into<std::option::Option<crate::model::TransferJob>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.transfer_job = v.into();
            self
        }

        /// Sets the value of [update_transfer_job_field_mask][crate::model::UpdateTransferJobRequest::update_transfer_job_field_mask].
        pub fn set_update_transfer_job_field_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_transfer_job_field_mask = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for UpdateTransferJob {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a StorageTransferService::get_transfer_job call.
    #[derive(Clone, Debug)]
    pub struct GetTransferJob(RequestBuilder<crate::model::GetTransferJobRequest>);

    impl GetTransferJob {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetTransferJobRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::TransferJob> {
            (*self.0.stub)
                .get_transfer_job(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [job_name][crate::model::GetTransferJobRequest::job_name].
        pub fn set_job_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.job_name = v.into();
            self
        }

        /// Sets the value of [project_id][crate::model::GetTransferJobRequest::project_id].
        pub fn set_project_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.project_id = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetTransferJob {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a StorageTransferService::list_transfer_jobs call.
    #[derive(Clone, Debug)]
    pub struct ListTransferJobs(RequestBuilder<crate::model::ListTransferJobsRequest>);

    impl ListTransferJobs {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListTransferJobsRequest>>(
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
        pub async fn send(self) -> Result<crate::model::ListTransferJobsResponse> {
            (*self.0.stub)
                .list_transfer_jobs(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListTransferJobsResponse, gax::error::Error>
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

        /// Sets the value of [filter][crate::model::ListTransferJobsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListTransferJobsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListTransferJobsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListTransferJobs {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a StorageTransferService::pause_transfer_operation call.
    #[derive(Clone, Debug)]
    pub struct PauseTransferOperation(RequestBuilder<crate::model::PauseTransferOperationRequest>);

    impl PauseTransferOperation {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::PauseTransferOperationRequest>>(
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
                .pause_transfer_operation(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::PauseTransferOperationRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for PauseTransferOperation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a StorageTransferService::resume_transfer_operation call.
    #[derive(Clone, Debug)]
    pub struct ResumeTransferOperation(
        RequestBuilder<crate::model::ResumeTransferOperationRequest>,
    );

    impl ResumeTransferOperation {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ResumeTransferOperationRequest>>(
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
                .resume_transfer_operation(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::ResumeTransferOperationRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ResumeTransferOperation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a StorageTransferService::run_transfer_job call.
    #[derive(Clone, Debug)]
    pub struct RunTransferJob(RequestBuilder<crate::model::RunTransferJobRequest>);

    impl RunTransferJob {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::RunTransferJobRequest>>(mut self, v: V) -> Self {
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
        /// on [run_transfer_job][super::super::client::StorageTransferService::run_transfer_job].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .run_transfer_job(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `run_transfer_job`.
        pub fn poller(self) -> impl lro::Poller<wkt::Empty, crate::model::TransferOperation> {
            type Operation = lro::Operation<wkt::Empty, crate::model::TransferOperation>;
            let polling_error_policy = self.0.stub.get_polling_error_policy(&self.0.options);
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

            lro::new_poller(polling_error_policy, polling_backoff_policy, start, query)
        }

        /// Sets the value of [job_name][crate::model::RunTransferJobRequest::job_name].
        pub fn set_job_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.job_name = v.into();
            self
        }

        /// Sets the value of [project_id][crate::model::RunTransferJobRequest::project_id].
        pub fn set_project_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.project_id = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for RunTransferJob {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a StorageTransferService::delete_transfer_job call.
    #[derive(Clone, Debug)]
    pub struct DeleteTransferJob(RequestBuilder<crate::model::DeleteTransferJobRequest>);

    impl DeleteTransferJob {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteTransferJobRequest>>(
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
                .delete_transfer_job(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [job_name][crate::model::DeleteTransferJobRequest::job_name].
        pub fn set_job_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.job_name = v.into();
            self
        }

        /// Sets the value of [project_id][crate::model::DeleteTransferJobRequest::project_id].
        pub fn set_project_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.project_id = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for DeleteTransferJob {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a StorageTransferService::create_agent_pool call.
    #[derive(Clone, Debug)]
    pub struct CreateAgentPool(RequestBuilder<crate::model::CreateAgentPoolRequest>);

    impl CreateAgentPool {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateAgentPoolRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::AgentPool> {
            (*self.0.stub)
                .create_agent_pool(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [project_id][crate::model::CreateAgentPoolRequest::project_id].
        pub fn set_project_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.project_id = v.into();
            self
        }

        /// Sets the value of [agent_pool][crate::model::CreateAgentPoolRequest::agent_pool].
        pub fn set_agent_pool<T: Into<std::option::Option<crate::model::AgentPool>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.agent_pool = v.into();
            self
        }

        /// Sets the value of [agent_pool_id][crate::model::CreateAgentPoolRequest::agent_pool_id].
        pub fn set_agent_pool_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.agent_pool_id = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for CreateAgentPool {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a StorageTransferService::update_agent_pool call.
    #[derive(Clone, Debug)]
    pub struct UpdateAgentPool(RequestBuilder<crate::model::UpdateAgentPoolRequest>);

    impl UpdateAgentPool {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateAgentPoolRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::AgentPool> {
            (*self.0.stub)
                .update_agent_pool(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [agent_pool][crate::model::UpdateAgentPoolRequest::agent_pool].
        pub fn set_agent_pool<T: Into<std::option::Option<crate::model::AgentPool>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.agent_pool = v.into();
            self
        }

        /// Sets the value of [update_mask][crate::model::UpdateAgentPoolRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for UpdateAgentPool {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a StorageTransferService::get_agent_pool call.
    #[derive(Clone, Debug)]
    pub struct GetAgentPool(RequestBuilder<crate::model::GetAgentPoolRequest>);

    impl GetAgentPool {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetAgentPoolRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::AgentPool> {
            (*self.0.stub)
                .get_agent_pool(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetAgentPoolRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetAgentPool {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a StorageTransferService::list_agent_pools call.
    #[derive(Clone, Debug)]
    pub struct ListAgentPools(RequestBuilder<crate::model::ListAgentPoolsRequest>);

    impl ListAgentPools {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListAgentPoolsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListAgentPoolsResponse> {
            (*self.0.stub)
                .list_agent_pools(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListAgentPoolsResponse, gax::error::Error>
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

        /// Sets the value of [project_id][crate::model::ListAgentPoolsRequest::project_id].
        pub fn set_project_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.project_id = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListAgentPoolsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListAgentPoolsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListAgentPoolsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListAgentPools {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a StorageTransferService::delete_agent_pool call.
    #[derive(Clone, Debug)]
    pub struct DeleteAgentPool(RequestBuilder<crate::model::DeleteAgentPoolRequest>);

    impl DeleteAgentPool {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteAgentPoolRequest>>(mut self, v: V) -> Self {
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
                .delete_agent_pool(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteAgentPoolRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for DeleteAgentPool {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a StorageTransferService::list_operations call.
    #[derive(Clone, Debug)]
    pub struct ListOperations(RequestBuilder<longrunning::model::ListOperationsRequest>);

    impl ListOperations {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        ) -> Self {
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

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListOperations {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a StorageTransferService::get_operation call.
    #[derive(Clone, Debug)]
    pub struct GetOperation(RequestBuilder<longrunning::model::GetOperationRequest>);

    impl GetOperation {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
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

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetOperation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a StorageTransferService::cancel_operation call.
    #[derive(Clone, Debug)]
    pub struct CancelOperation(RequestBuilder<longrunning::model::CancelOperationRequest>);

    impl CancelOperation {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::StorageTransferService>,
        ) -> Self {
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
        pub async fn send(self) -> Result<()> {
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

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for CancelOperation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
