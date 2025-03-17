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
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Cloud Build API.
///
/// # Service Description
///
/// Manages connections to source code repositories.
///
/// # Configuration
///
/// `RepositoryManager` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `RepositoryManager` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `RepositoryManager` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct RepositoryManager {
    inner: Arc<dyn super::stubs::dynamic::RepositoryManager>,
}

impl RepositoryManager {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stubs::RepositoryManager + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stubs::dynamic::RepositoryManager>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::RepositoryManager> {
        super::transport::RepositoryManager::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::RepositoryManager> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::RepositoryManager::new)
    }

    /// Creates a Connection.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_connection(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::CreateConnection {
        super::builders::repository_manager::CreateConnection::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single connection.
    pub fn get_connection(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::GetConnection {
        super::builders::repository_manager::GetConnection::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists Connections in a given project and location.
    pub fn list_connections(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::ListConnections {
        super::builders::repository_manager::ListConnections::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a single connection.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_connection(
        &self,
        connection: impl Into<crate::model::Connection>,
    ) -> super::builders::repository_manager::UpdateConnection {
        super::builders::repository_manager::UpdateConnection::new(self.inner.clone())
            .set_connection(connection.into())
    }

    /// Deletes a single connection.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_connection(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::DeleteConnection {
        super::builders::repository_manager::DeleteConnection::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a Repository.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_repository(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::CreateRepository {
        super::builders::repository_manager::CreateRepository::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates multiple repositories inside a connection.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn batch_create_repositories(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::BatchCreateRepositories {
        super::builders::repository_manager::BatchCreateRepositories::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single repository.
    pub fn get_repository(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::GetRepository {
        super::builders::repository_manager::GetRepository::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists Repositories in a given connection.
    pub fn list_repositories(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::ListRepositories {
        super::builders::repository_manager::ListRepositories::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a single repository.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_repository(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::DeleteRepository {
        super::builders::repository_manager::DeleteRepository::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Fetches read/write token of a given repository.
    pub fn fetch_read_write_token(
        &self,
        repository: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::FetchReadWriteToken {
        super::builders::repository_manager::FetchReadWriteToken::new(self.inner.clone())
            .set_repository(repository.into())
    }

    /// Fetches read token of a given repository.
    pub fn fetch_read_token(
        &self,
        repository: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::FetchReadToken {
        super::builders::repository_manager::FetchReadToken::new(self.inner.clone())
            .set_repository(repository.into())
    }

    /// FetchLinkableRepositories get repositories from SCM that are
    /// accessible and could be added to the connection.
    pub fn fetch_linkable_repositories(
        &self,
        connection: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::FetchLinkableRepositories {
        super::builders::repository_manager::FetchLinkableRepositories::new(self.inner.clone())
            .set_connection(connection.into())
    }

    /// Fetch the list of branches or tags for a given repository.
    pub fn fetch_git_refs(
        &self,
        repository: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::FetchGitRefs {
        super::builders::repository_manager::FetchGitRefs::new(self.inner.clone())
            .set_repository(repository.into())
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::SetIamPolicy {
        super::builders::repository_manager::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::GetIamPolicy {
        super::builders::repository_manager::GetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    pub fn test_iam_permissions(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::TestIamPermissions {
        super::builders::repository_manager::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::GetOperation {
        super::builders::repository_manager::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::repository_manager::CancelOperation {
        super::builders::repository_manager::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
