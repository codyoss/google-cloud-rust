// Copyright 2024 Google LLC
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

/// A dyn-compatible, crate-private version of `Locations`.
#[async_trait::async_trait]
pub trait Locations: std::fmt::Debug + Send + Sync {
    /// Lists information about the supported locations for this service.
    async fn list_locations(
        &self,
        req: crate::model::ListLocationsRequest
    ) -> crate::Result<crate::model::ListLocationsResponse>;

    /// Gets information about a location.
    async fn get_location(
        &self,
        req: crate::model::GetLocationRequest
    ) -> crate::Result<crate::model::Location>;

}

/// All implementations of [crate::traits::Locations] also implement [Locations].
#[async_trait::async_trait]
impl<T: crate::traits::Locations> Locations for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: crate::model::ListLocationsRequest
    ) -> crate::Result<crate::model::ListLocationsResponse> {
        let response = T::list_locations(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: crate::model::GetLocationRequest
    ) -> crate::Result<crate::model::Location> {
        let response = T::get_location(self, req).await?;
        Ok(response)
    }

}
