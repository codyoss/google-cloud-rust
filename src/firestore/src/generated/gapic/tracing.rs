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
use crate::Result;

/// Implements a [Firestore](super::stubs::Firestore) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Firestore<T>
where
    T: super::stubs::Firestore + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Firestore<T>
where
    T: super::stubs::Firestore + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::Firestore for Firestore<T>
where
    T: super::stubs::Firestore + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn get_document(
        &self,
        req: crate::model::GetDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Document> {
        self.inner.get_document(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_documents(
        &self,
        req: crate::model::ListDocumentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDocumentsResponse> {
        self.inner.list_documents(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_document(
        &self,
        req: crate::model::UpdateDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Document> {
        self.inner.update_document(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_document(
        &self,
        req: crate::model::DeleteDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_document(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn begin_transaction(
        &self,
        req: crate::model::BeginTransactionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::BeginTransactionResponse> {
        self.inner.begin_transaction(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn commit(
        &self,
        req: crate::model::CommitRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::CommitResponse> {
        self.inner.commit(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn rollback(
        &self,
        req: crate::model::RollbackRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.rollback(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn partition_query(
        &self,
        req: crate::model::PartitionQueryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::PartitionQueryResponse> {
        self.inner.partition_query(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_collection_ids(
        &self,
        req: crate::model::ListCollectionIdsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListCollectionIdsResponse> {
        self.inner.list_collection_ids(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn batch_write(
        &self,
        req: crate::model::BatchWriteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::BatchWriteResponse> {
        self.inner.batch_write(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_document(
        &self,
        req: crate::model::CreateDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Document> {
        self.inner.create_document(req, options).await
    }
}
