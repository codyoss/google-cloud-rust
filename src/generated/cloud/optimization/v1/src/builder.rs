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

pub mod fleet_routing {
    use crate::Result;
    use std::sync::Arc;

    /// A builder for [FleetRouting][super::super::client::FleetRouting].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_optimization_v1::*;
    /// # use builder::fleet_routing::ClientBuilder;
    /// # use client::FleetRouting;
    /// let builder : ClientBuilder = FleetRouting::builder();
    /// let client = builder
    ///     .with_endpoint("https://cloudoptimization.googleapis.com")
    ///     .build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::FleetRouting;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = FleetRouting;
            type Credentials = gaxi::options::Credentials;
            async fn build(self, config: gaxi::options::ClientConfig) -> gax::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [super::super::client::FleetRouting] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stub::dynamic::FleetRouting>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::FleetRouting>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [FleetRouting::optimize_tours][super::super::client::FleetRouting::optimize_tours] calls.
    #[derive(Clone, Debug)]
    pub struct OptimizeTours(RequestBuilder<crate::model::OptimizeToursRequest>);

    impl OptimizeTours {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::FleetRouting>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::OptimizeToursRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::OptimizeToursResponse> {
            (*self.0.stub)
                .optimize_tours(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Sets the value of [parent][crate::model::OptimizeToursRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [timeout][crate::model::OptimizeToursRequest::timeout].
        pub fn set_timeout<T: Into<std::option::Option<wkt::Duration>>>(mut self, v: T) -> Self {
            self.0.request.timeout = v.into();
            self
        }

        /// Sets the value of [model][crate::model::OptimizeToursRequest::model].
        pub fn set_model<T: Into<std::option::Option<crate::model::ShipmentModel>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.model = v.into();
            self
        }

        /// Sets the value of [solving_mode][crate::model::OptimizeToursRequest::solving_mode].
        pub fn set_solving_mode<T: Into<crate::model::optimize_tours_request::SolvingMode>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.solving_mode = v.into();
            self
        }

        /// Sets the value of [search_mode][crate::model::OptimizeToursRequest::search_mode].
        pub fn set_search_mode<T: Into<crate::model::optimize_tours_request::SearchMode>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.search_mode = v.into();
            self
        }

        /// Sets the value of [injected_solution_constraint][crate::model::OptimizeToursRequest::injected_solution_constraint].
        pub fn set_injected_solution_constraint<
            T: Into<std::option::Option<crate::model::InjectedSolutionConstraint>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.injected_solution_constraint = v.into();
            self
        }

        /// Sets the value of [interpret_injected_solutions_using_labels][crate::model::OptimizeToursRequest::interpret_injected_solutions_using_labels].
        pub fn set_interpret_injected_solutions_using_labels<T: Into<bool>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.interpret_injected_solutions_using_labels = v.into();
            self
        }

        /// Sets the value of [consider_road_traffic][crate::model::OptimizeToursRequest::consider_road_traffic].
        pub fn set_consider_road_traffic<T: Into<bool>>(mut self, v: T) -> Self {
            self.0.request.consider_road_traffic = v.into();
            self
        }

        /// Sets the value of [populate_polylines][crate::model::OptimizeToursRequest::populate_polylines].
        pub fn set_populate_polylines<T: Into<bool>>(mut self, v: T) -> Self {
            self.0.request.populate_polylines = v.into();
            self
        }

        /// Sets the value of [populate_transition_polylines][crate::model::OptimizeToursRequest::populate_transition_polylines].
        pub fn set_populate_transition_polylines<T: Into<bool>>(mut self, v: T) -> Self {
            self.0.request.populate_transition_polylines = v.into();
            self
        }

        /// Sets the value of [allow_large_deadline_despite_interruption_risk][crate::model::OptimizeToursRequest::allow_large_deadline_despite_interruption_risk].
        pub fn set_allow_large_deadline_despite_interruption_risk<T: Into<bool>>(
            mut self,
            v: T,
        ) -> Self {
            self.0
                .request
                .allow_large_deadline_despite_interruption_risk = v.into();
            self
        }

        /// Sets the value of [use_geodesic_distances][crate::model::OptimizeToursRequest::use_geodesic_distances].
        pub fn set_use_geodesic_distances<T: Into<bool>>(mut self, v: T) -> Self {
            self.0.request.use_geodesic_distances = v.into();
            self
        }

        /// Sets the value of [geodesic_meters_per_second][crate::model::OptimizeToursRequest::geodesic_meters_per_second].
        pub fn set_geodesic_meters_per_second<T: Into<std::option::Option<f64>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.geodesic_meters_per_second = v.into();
            self
        }

        /// Sets the value of [max_validation_errors][crate::model::OptimizeToursRequest::max_validation_errors].
        pub fn set_max_validation_errors<T: Into<std::option::Option<i32>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.max_validation_errors = v.into();
            self
        }

        /// Sets the value of [label][crate::model::OptimizeToursRequest::label].
        pub fn set_label<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.label = v.into();
            self
        }

        /// Sets the value of [populate_travel_step_polylines][crate::model::OptimizeToursRequest::populate_travel_step_polylines].
        pub fn set_populate_travel_step_polylines<T: Into<bool>>(mut self, v: T) -> Self {
            self.0.request.populate_travel_step_polylines = v.into();
            self
        }

        /// Sets the value of [injected_first_solution_routes][crate::model::OptimizeToursRequest::injected_first_solution_routes].
        pub fn set_injected_first_solution_routes<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<crate::model::ShipmentRoute>,
        {
            use std::iter::Iterator;
            self.0.request.injected_first_solution_routes =
                v.into_iter().map(|i| i.into()).collect();
            self
        }

        /// Sets the value of [refresh_details_routes][crate::model::OptimizeToursRequest::refresh_details_routes].
        pub fn set_refresh_details_routes<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<crate::model::ShipmentRoute>,
        {
            use std::iter::Iterator;
            self.0.request.refresh_details_routes = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for OptimizeTours {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [FleetRouting::batch_optimize_tours][super::super::client::FleetRouting::batch_optimize_tours] calls.
    #[derive(Clone, Debug)]
    pub struct BatchOptimizeTours(RequestBuilder<crate::model::BatchOptimizeToursRequest>);

    impl BatchOptimizeTours {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::FleetRouting>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::BatchOptimizeToursRequest>>(
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
        ///
        /// # Long running operations
        ///
        /// This starts, but does not poll, a longrunning operation. More information
        /// on [batch_optimize_tours][super::super::client::FleetRouting::batch_optimize_tours].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .batch_optimize_tours(self.0.request, self.0.options)
                .await
                .map(gax::response::Response::into_body)
        }

        /// Creates a [Poller][lro::Poller] to work with `batch_optimize_tours`.
        pub fn poller(
            self,
        ) -> impl lro::Poller<crate::model::BatchOptimizeToursResponse, crate::model::AsyncModelMetadata>
        {
            type Operation = lro::Operation<
                crate::model::BatchOptimizeToursResponse,
                crate::model::AsyncModelMetadata,
            >;
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

        /// Sets the value of [parent][crate::model::BatchOptimizeToursRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [model_configs][crate::model::BatchOptimizeToursRequest::model_configs].
        pub fn set_model_configs<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<crate::model::batch_optimize_tours_request::AsyncModelConfig>,
        {
            use std::iter::Iterator;
            self.0.request.model_configs = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for BatchOptimizeTours {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [FleetRouting::get_operation][super::super::client::FleetRouting::get_operation] calls.
    #[derive(Clone, Debug)]
    pub struct GetOperation(RequestBuilder<longrunning::model::GetOperationRequest>);

    impl GetOperation {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::FleetRouting>) -> Self {
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
                .map(gax::response::Response::into_body)
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
}
