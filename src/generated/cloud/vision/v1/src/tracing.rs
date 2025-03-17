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

/// Implements a [ImageAnnotator](super::stubs::ImageAnnotator) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct ImageAnnotator<T>
where
    T: super::stubs::ImageAnnotator + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> ImageAnnotator<T>
where
    T: super::stubs::ImageAnnotator + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::ImageAnnotator for ImageAnnotator<T>
where
    T: super::stubs::ImageAnnotator + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn batch_annotate_images(
        &self,
        req: crate::model::BatchAnnotateImagesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::BatchAnnotateImagesResponse> {
        self.inner.batch_annotate_images(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn batch_annotate_files(
        &self,
        req: crate::model::BatchAnnotateFilesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::BatchAnnotateFilesResponse> {
        self.inner.batch_annotate_files(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn async_batch_annotate_images(
        &self,
        req: crate::model::AsyncBatchAnnotateImagesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.async_batch_annotate_images(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn async_batch_annotate_files(
        &self,
        req: crate::model::AsyncBatchAnnotateFilesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.async_batch_annotate_files(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy> {
        self.inner.get_polling_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}

/// Implements a [ProductSearch](super::stubs::ProductSearch) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct ProductSearch<T>
where
    T: super::stubs::ProductSearch + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> ProductSearch<T>
where
    T: super::stubs::ProductSearch + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::ProductSearch for ProductSearch<T>
where
    T: super::stubs::ProductSearch + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_product_set(
        &self,
        req: crate::model::CreateProductSetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ProductSet> {
        self.inner.create_product_set(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_product_sets(
        &self,
        req: crate::model::ListProductSetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListProductSetsResponse> {
        self.inner.list_product_sets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_product_set(
        &self,
        req: crate::model::GetProductSetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ProductSet> {
        self.inner.get_product_set(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_product_set(
        &self,
        req: crate::model::UpdateProductSetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ProductSet> {
        self.inner.update_product_set(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_product_set(
        &self,
        req: crate::model::DeleteProductSetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_product_set(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_product(
        &self,
        req: crate::model::CreateProductRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Product> {
        self.inner.create_product(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_products(
        &self,
        req: crate::model::ListProductsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListProductsResponse> {
        self.inner.list_products(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_product(
        &self,
        req: crate::model::GetProductRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Product> {
        self.inner.get_product(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_product(
        &self,
        req: crate::model::UpdateProductRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Product> {
        self.inner.update_product(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_product(
        &self,
        req: crate::model::DeleteProductRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_product(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_reference_image(
        &self,
        req: crate::model::CreateReferenceImageRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ReferenceImage> {
        self.inner.create_reference_image(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_reference_image(
        &self,
        req: crate::model::DeleteReferenceImageRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_reference_image(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_reference_images(
        &self,
        req: crate::model::ListReferenceImagesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListReferenceImagesResponse> {
        self.inner.list_reference_images(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_reference_image(
        &self,
        req: crate::model::GetReferenceImageRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ReferenceImage> {
        self.inner.get_reference_image(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn add_product_to_product_set(
        &self,
        req: crate::model::AddProductToProductSetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.add_product_to_product_set(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn remove_product_from_product_set(
        &self,
        req: crate::model::RemoveProductFromProductSetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner
            .remove_product_from_product_set(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn list_products_in_product_set(
        &self,
        req: crate::model::ListProductsInProductSetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListProductsInProductSetResponse> {
        self.inner.list_products_in_product_set(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn import_product_sets(
        &self,
        req: crate::model::ImportProductSetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.import_product_sets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn purge_products(
        &self,
        req: crate::model::PurgeProductsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.purge_products(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy> {
        self.inner.get_polling_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
