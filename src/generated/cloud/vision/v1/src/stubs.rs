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

//! Traits to mock the clients in this library.
//!
//! Application developers may need to mock the clients in this library to test
//! how their application works with different (and sometimes hard to trigger)
//! client and service behavior. Such test can define mocks implementing the
//! trait(s) defined in this module, initialize the client with an instance of
//! this mock in their tests, and verify their application responds as expected.

#![allow(rustdoc::broken_intra_doc_links)]

use gax::error::Error;
use std::sync::Arc;

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::ImageAnnotator].
///
/// Application developers may need to implement this trait to mock
/// `client::ImageAnnotator`.  In other use-cases, application developers only
/// use `client::ImageAnnotator` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait ImageAnnotator: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::ImageAnnotator::batch_annotate_images].
    fn batch_annotate_images(
        &self,
        _req: crate::model::BatchAnnotateImagesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::BatchAnnotateImagesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::BatchAnnotateImagesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ImageAnnotator::batch_annotate_files].
    fn batch_annotate_files(
        &self,
        _req: crate::model::BatchAnnotateFilesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::BatchAnnotateFilesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::BatchAnnotateFilesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ImageAnnotator::async_batch_annotate_images].
    fn async_batch_annotate_images(
        &self,
        _req: crate::model::AsyncBatchAnnotateImagesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ImageAnnotator::async_batch_annotate_files].
    fn async_batch_annotate_files(
        &self,
        _req: crate::model::AsyncBatchAnnotateFilesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ImageAnnotator::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Returns the polling policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        Arc::new(gax::polling_policy::Aip194Strict)
    }

    /// Returns the polling backoff policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_backoff_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        Arc::new(gax::exponential_backoff::ExponentialBackoff::default())
    }
}

/// Defines the trait used to implement [super::client::ProductSearch].
///
/// Application developers may need to implement this trait to mock
/// `client::ProductSearch`.  In other use-cases, application developers only
/// use `client::ProductSearch` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait ProductSearch: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::ProductSearch::create_product_set].
    fn create_product_set(
        &self,
        _req: crate::model::CreateProductSetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ProductSet>> + Send {
        std::future::ready::<crate::Result<crate::model::ProductSet>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ProductSearch::list_product_sets].
    fn list_product_sets(
        &self,
        _req: crate::model::ListProductSetsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListProductSetsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListProductSetsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ProductSearch::get_product_set].
    fn get_product_set(
        &self,
        _req: crate::model::GetProductSetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ProductSet>> + Send {
        std::future::ready::<crate::Result<crate::model::ProductSet>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ProductSearch::update_product_set].
    fn update_product_set(
        &self,
        _req: crate::model::UpdateProductSetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ProductSet>> + Send {
        std::future::ready::<crate::Result<crate::model::ProductSet>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ProductSearch::delete_product_set].
    fn delete_product_set(
        &self,
        _req: crate::model::DeleteProductSetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::ProductSearch::create_product].
    fn create_product(
        &self,
        _req: crate::model::CreateProductRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Product>> + Send {
        std::future::ready::<crate::Result<crate::model::Product>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ProductSearch::list_products].
    fn list_products(
        &self,
        _req: crate::model::ListProductsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListProductsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListProductsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ProductSearch::get_product].
    fn get_product(
        &self,
        _req: crate::model::GetProductRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Product>> + Send {
        std::future::ready::<crate::Result<crate::model::Product>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ProductSearch::update_product].
    fn update_product(
        &self,
        _req: crate::model::UpdateProductRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Product>> + Send {
        std::future::ready::<crate::Result<crate::model::Product>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ProductSearch::delete_product].
    fn delete_product(
        &self,
        _req: crate::model::DeleteProductRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::ProductSearch::create_reference_image].
    fn create_reference_image(
        &self,
        _req: crate::model::CreateReferenceImageRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ReferenceImage>> + Send {
        std::future::ready::<crate::Result<crate::model::ReferenceImage>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ProductSearch::delete_reference_image].
    fn delete_reference_image(
        &self,
        _req: crate::model::DeleteReferenceImageRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::ProductSearch::list_reference_images].
    fn list_reference_images(
        &self,
        _req: crate::model::ListReferenceImagesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListReferenceImagesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListReferenceImagesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ProductSearch::get_reference_image].
    fn get_reference_image(
        &self,
        _req: crate::model::GetReferenceImageRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ReferenceImage>> + Send {
        std::future::ready::<crate::Result<crate::model::ReferenceImage>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ProductSearch::add_product_to_product_set].
    fn add_product_to_product_set(
        &self,
        _req: crate::model::AddProductToProductSetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::ProductSearch::remove_product_from_product_set].
    fn remove_product_from_product_set(
        &self,
        _req: crate::model::RemoveProductFromProductSetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::ProductSearch::list_products_in_product_set].
    fn list_products_in_product_set(
        &self,
        _req: crate::model::ListProductsInProductSetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListProductsInProductSetResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListProductsInProductSetResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ProductSearch::import_product_sets].
    fn import_product_sets(
        &self,
        _req: crate::model::ImportProductSetsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ProductSearch::purge_products].
    fn purge_products(
        &self,
        _req: crate::model::PurgeProductsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ProductSearch::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Returns the polling policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        Arc::new(gax::polling_policy::Aip194Strict)
    }

    /// Returns the polling backoff policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_backoff_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        Arc::new(gax::exponential_backoff::ExponentialBackoff::default())
    }
}
