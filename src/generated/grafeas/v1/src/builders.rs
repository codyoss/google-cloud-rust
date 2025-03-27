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

pub mod grafeas {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [super::super::client::Grafeas] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stubs::dynamic::Grafeas>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::Grafeas>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a Grafeas::get_occurrence call.
    #[derive(Clone, Debug)]
    pub struct GetOccurrence(RequestBuilder<crate::model::GetOccurrenceRequest>);

    impl GetOccurrence {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::Grafeas>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetOccurrenceRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Occurrence> {
            (*self.0.stub)
                .get_occurrence(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetOccurrenceRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetOccurrence {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a Grafeas::list_occurrences call.
    #[derive(Clone, Debug)]
    pub struct ListOccurrences(RequestBuilder<crate::model::ListOccurrencesRequest>);

    impl ListOccurrences {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::Grafeas>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListOccurrencesRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListOccurrencesResponse> {
            (*self.0.stub)
                .list_occurrences(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListOccurrencesResponse, gax::error::Error>
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

        /// Sets the value of [parent][crate::model::ListOccurrencesRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListOccurrencesRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListOccurrencesRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListOccurrencesRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListOccurrences {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a Grafeas::delete_occurrence call.
    #[derive(Clone, Debug)]
    pub struct DeleteOccurrence(RequestBuilder<crate::model::DeleteOccurrenceRequest>);

    impl DeleteOccurrence {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::Grafeas>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteOccurrenceRequest>>(
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
                .delete_occurrence(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteOccurrenceRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteOccurrence {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a Grafeas::create_occurrence call.
    #[derive(Clone, Debug)]
    pub struct CreateOccurrence(RequestBuilder<crate::model::CreateOccurrenceRequest>);

    impl CreateOccurrence {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::Grafeas>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateOccurrenceRequest>>(
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
        pub async fn send(self) -> Result<crate::model::Occurrence> {
            (*self.0.stub)
                .create_occurrence(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateOccurrenceRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [occurrence][crate::model::CreateOccurrenceRequest::occurrence].
        pub fn set_occurrence<T: Into<std::option::Option<crate::model::Occurrence>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.occurrence = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateOccurrence {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a Grafeas::batch_create_occurrences call.
    #[derive(Clone, Debug)]
    pub struct BatchCreateOccurrences(RequestBuilder<crate::model::BatchCreateOccurrencesRequest>);

    impl BatchCreateOccurrences {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::Grafeas>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::BatchCreateOccurrencesRequest>>(
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
        pub async fn send(self) -> Result<crate::model::BatchCreateOccurrencesResponse> {
            (*self.0.stub)
                .batch_create_occurrences(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::BatchCreateOccurrencesRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [occurrences][crate::model::BatchCreateOccurrencesRequest::occurrences].
        pub fn set_occurrences<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<crate::model::Occurrence>,
        {
            use std::iter::Iterator;
            self.0.request.occurrences = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for BatchCreateOccurrences {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a Grafeas::update_occurrence call.
    #[derive(Clone, Debug)]
    pub struct UpdateOccurrence(RequestBuilder<crate::model::UpdateOccurrenceRequest>);

    impl UpdateOccurrence {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::Grafeas>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateOccurrenceRequest>>(
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
        pub async fn send(self) -> Result<crate::model::Occurrence> {
            (*self.0.stub)
                .update_occurrence(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::UpdateOccurrenceRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [occurrence][crate::model::UpdateOccurrenceRequest::occurrence].
        pub fn set_occurrence<T: Into<std::option::Option<crate::model::Occurrence>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.occurrence = v.into();
            self
        }

        /// Sets the value of [update_mask][crate::model::UpdateOccurrenceRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for UpdateOccurrence {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a Grafeas::get_occurrence_note call.
    #[derive(Clone, Debug)]
    pub struct GetOccurrenceNote(RequestBuilder<crate::model::GetOccurrenceNoteRequest>);

    impl GetOccurrenceNote {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::Grafeas>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetOccurrenceNoteRequest>>(
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
        pub async fn send(self) -> Result<crate::model::Note> {
            (*self.0.stub)
                .get_occurrence_note(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetOccurrenceNoteRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetOccurrenceNote {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a Grafeas::get_note call.
    #[derive(Clone, Debug)]
    pub struct GetNote(RequestBuilder<crate::model::GetNoteRequest>);

    impl GetNote {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::Grafeas>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetNoteRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Note> {
            (*self.0.stub)
                .get_note(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetNoteRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetNote {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a Grafeas::list_notes call.
    #[derive(Clone, Debug)]
    pub struct ListNotes(RequestBuilder<crate::model::ListNotesRequest>);

    impl ListNotes {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::Grafeas>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListNotesRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListNotesResponse> {
            (*self.0.stub)
                .list_notes(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListNotesResponse, gax::error::Error> {
            use std::clone::Clone;
            let token = self.0.request.page_token.clone();
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListNotesRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListNotesRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListNotesRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListNotesRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListNotes {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a Grafeas::delete_note call.
    #[derive(Clone, Debug)]
    pub struct DeleteNote(RequestBuilder<crate::model::DeleteNoteRequest>);

    impl DeleteNote {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::Grafeas>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteNoteRequest>>(mut self, v: V) -> Self {
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
                .delete_note(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::DeleteNoteRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteNote {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a Grafeas::create_note call.
    #[derive(Clone, Debug)]
    pub struct CreateNote(RequestBuilder<crate::model::CreateNoteRequest>);

    impl CreateNote {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::Grafeas>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateNoteRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Note> {
            (*self.0.stub)
                .create_note(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateNoteRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [note_id][crate::model::CreateNoteRequest::note_id].
        pub fn set_note_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.note_id = v.into();
            self
        }

        /// Sets the value of [note][crate::model::CreateNoteRequest::note].
        pub fn set_note<T: Into<std::option::Option<crate::model::Note>>>(mut self, v: T) -> Self {
            self.0.request.note = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateNote {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a Grafeas::batch_create_notes call.
    #[derive(Clone, Debug)]
    pub struct BatchCreateNotes(RequestBuilder<crate::model::BatchCreateNotesRequest>);

    impl BatchCreateNotes {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::Grafeas>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::BatchCreateNotesRequest>>(
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
        pub async fn send(self) -> Result<crate::model::BatchCreateNotesResponse> {
            (*self.0.stub)
                .batch_create_notes(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::BatchCreateNotesRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [notes][crate::model::BatchCreateNotesRequest::notes].
        pub fn set_notes<T, K, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = (K, V)>,
            K: std::convert::Into<std::string::String>,
            V: std::convert::Into<crate::model::Note>,
        {
            self.0.request.notes = v.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for BatchCreateNotes {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a Grafeas::update_note call.
    #[derive(Clone, Debug)]
    pub struct UpdateNote(RequestBuilder<crate::model::UpdateNoteRequest>);

    impl UpdateNote {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::Grafeas>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateNoteRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Note> {
            (*self.0.stub)
                .update_note(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::UpdateNoteRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [note][crate::model::UpdateNoteRequest::note].
        pub fn set_note<T: Into<std::option::Option<crate::model::Note>>>(mut self, v: T) -> Self {
            self.0.request.note = v.into();
            self
        }

        /// Sets the value of [update_mask][crate::model::UpdateNoteRequest::update_mask].
        pub fn set_update_mask<T: Into<std::option::Option<wkt::FieldMask>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.update_mask = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for UpdateNote {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a Grafeas::list_note_occurrences call.
    #[derive(Clone, Debug)]
    pub struct ListNoteOccurrences(RequestBuilder<crate::model::ListNoteOccurrencesRequest>);

    impl ListNoteOccurrences {
        pub(crate) fn new(stub: Arc<dyn super::super::stubs::dynamic::Grafeas>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListNoteOccurrencesRequest>>(
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
        pub async fn send(self) -> Result<crate::model::ListNoteOccurrencesResponse> {
            (*self.0.stub)
                .list_note_occurrences(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListNoteOccurrencesResponse, gax::error::Error>
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

        /// Sets the value of [name][crate::model::ListNoteOccurrencesRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListNoteOccurrencesRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListNoteOccurrencesRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListNoteOccurrencesRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListNoteOccurrences {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
