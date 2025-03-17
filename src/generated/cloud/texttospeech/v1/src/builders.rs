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

pub mod text_to_speech {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::TextToSpeech] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::TextToSpeech>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::TextToSpeech>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a TextToSpeech::list_voices call.
    #[derive(Clone, Debug)]
    pub struct ListVoices(RequestBuilder<crate::model::ListVoicesRequest>);

    impl ListVoices {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::TextToSpeech>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListVoicesRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListVoicesResponse> {
            (*self.0.stub)
                .list_voices(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [language_code][crate::model::ListVoicesRequest::language_code].
        pub fn set_language_code<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.language_code = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListVoices {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a TextToSpeech::synthesize_speech call.
    #[derive(Clone, Debug)]
    pub struct SynthesizeSpeech(RequestBuilder<crate::model::SynthesizeSpeechRequest>);

    impl SynthesizeSpeech {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::TextToSpeech>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::SynthesizeSpeechRequest>>(
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
        pub async fn send(self) -> Result<crate::model::SynthesizeSpeechResponse> {
            (*self.0.stub)
                .synthesize_speech(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [input][crate::model::SynthesizeSpeechRequest::input].
        pub fn set_input<T: Into<std::option::Option<crate::model::SynthesisInput>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.input = v.into();
            self
        }

        /// Sets the value of [voice][crate::model::SynthesizeSpeechRequest::voice].
        pub fn set_voice<T: Into<std::option::Option<crate::model::VoiceSelectionParams>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.voice = v.into();
            self
        }

        /// Sets the value of [audio_config][crate::model::SynthesizeSpeechRequest::audio_config].
        pub fn set_audio_config<T: Into<std::option::Option<crate::model::AudioConfig>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.audio_config = v.into();
            self
        }

        /// Sets the value of [advanced_voice_options][crate::model::SynthesizeSpeechRequest::advanced_voice_options].
        pub fn set_advanced_voice_options<
            T: Into<std::option::Option<crate::model::AdvancedVoiceOptions>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.advanced_voice_options = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for SynthesizeSpeech {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a TextToSpeech::list_operations call.
    #[derive(Clone, Debug)]
    pub struct ListOperations(RequestBuilder<longrunning::model::ListOperationsRequest>);

    impl ListOperations {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::TextToSpeech>) -> Self {
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
            let token = gax::paginator::extract_token(&self.0.request.page_token);
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

    /// The request builder for a TextToSpeech::get_operation call.
    #[derive(Clone, Debug)]
    pub struct GetOperation(RequestBuilder<longrunning::model::GetOperationRequest>);

    impl GetOperation {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::TextToSpeech>) -> Self {
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

pub mod text_to_speech_long_audio_synthesize {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::TextToSpeechLongAudioSynthesize] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::TextToSpeechLongAudioSynthesize>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::TextToSpeechLongAudioSynthesize>,
        ) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a TextToSpeechLongAudioSynthesize::synthesize_long_audio call.
    #[derive(Clone, Debug)]
    pub struct SynthesizeLongAudio(RequestBuilder<crate::model::SynthesizeLongAudioRequest>);

    impl SynthesizeLongAudio {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::TextToSpeechLongAudioSynthesize>,
        ) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::SynthesizeLongAudioRequest>>(
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
        /// on [synthesize_long_audio][super::super::client::TextToSpeechLongAudioSynthesize::synthesize_long_audio].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .synthesize_long_audio(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `synthesize_long_audio`.
        pub fn poller(
            self,
        ) -> impl lro::Poller<
            crate::model::SynthesizeLongAudioResponse,
            crate::model::SynthesizeLongAudioMetadata,
        > {
            type Operation = lro::Operation<
                crate::model::SynthesizeLongAudioResponse,
                crate::model::SynthesizeLongAudioMetadata,
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

        /// Sets the value of [parent][crate::model::SynthesizeLongAudioRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [input][crate::model::SynthesizeLongAudioRequest::input].
        pub fn set_input<T: Into<std::option::Option<crate::model::SynthesisInput>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.input = v.into();
            self
        }

        /// Sets the value of [audio_config][crate::model::SynthesizeLongAudioRequest::audio_config].
        pub fn set_audio_config<T: Into<std::option::Option<crate::model::AudioConfig>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.audio_config = v.into();
            self
        }

        /// Sets the value of [output_gcs_uri][crate::model::SynthesizeLongAudioRequest::output_gcs_uri].
        pub fn set_output_gcs_uri<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.output_gcs_uri = v.into();
            self
        }

        /// Sets the value of [voice][crate::model::SynthesizeLongAudioRequest::voice].
        pub fn set_voice<T: Into<std::option::Option<crate::model::VoiceSelectionParams>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.voice = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for SynthesizeLongAudio {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a TextToSpeechLongAudioSynthesize::list_operations call.
    #[derive(Clone, Debug)]
    pub struct ListOperations(RequestBuilder<longrunning::model::ListOperationsRequest>);

    impl ListOperations {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::TextToSpeechLongAudioSynthesize>,
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
            let token = gax::paginator::extract_token(&self.0.request.page_token);
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

    /// The request builder for a TextToSpeechLongAudioSynthesize::get_operation call.
    #[derive(Clone, Debug)]
    pub struct GetOperation(RequestBuilder<longrunning::model::GetOperationRequest>);

    impl GetOperation {
        pub(crate) fn new(
            stub: Arc<dyn super::super::stubs::dynamic::TextToSpeechLongAudioSynthesize>,
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
