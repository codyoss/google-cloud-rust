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
#[allow(unused_imports)]
use gax::error::Error;
use std::sync::Arc;

const DEFAULT_HOST: &str = "https://firestore.googleapis.com";

mod info {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    lazy_static::lazy_static! {
        pub(crate) static ref X_GOOG_API_CLIENT_HEADER: String = {
            let ac = gax::api_header::XGoogApiClient{
                name:          NAME,
                version:       VERSION,
                library_type:  gax::api_header::GAPIC,
            };
            ac.header_value()
        };
    }
}

/// Implements [Firestore](super::stub::Firestore) using a Tonic-generated client.
#[derive(Clone)]
pub struct Firestore {
    inner: tonic::client::Grpc<tonic::transport::Channel>,
    cred: auth::credentials::Credentials,
    retry_policy: Option<Arc<dyn gax::retry_policy::RetryPolicy>>,
    backoff_policy: Option<Arc<dyn gax::backoff_policy::BackoffPolicy>>,
    retry_throttler: gax::retry_throttler::SharedRetryThrottler,
}

impl std::fmt::Debug for Firestore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Firestore")
            .field("inner", &self.inner)
            .finish()
    }
}

impl Firestore {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let cred = Self::make_credentials(&config).await?;
        let inner = Self::make_inner(config.endpoint).await?;
        Ok(Self {
            inner,
            cred,
            retry_policy: config.retry_policy,
            backoff_policy: config.backoff_policy,
            retry_throttler: config.retry_throttler,
        })
    }

    async fn make_inner(
        endpoint: Option<String>,
    ) -> Result<tonic::client::Grpc<tonic::transport::Channel>> {
        let endpoint =
            tonic::transport::Endpoint::new(endpoint.unwrap_or_else(|| DEFAULT_HOST.to_string()))
                .map_err(Error::other)?;
        let conn = endpoint.connect().await.map_err(Error::other)?;
        Ok(tonic::client::Grpc::new(conn))
    }

    async fn make_credentials(
        config: &gaxi::options::ClientConfig,
    ) -> Result<auth::credentials::Credentials> {
        if let Some(c) = config.cred.clone() {
            return Ok(c);
        }
        auth::credentials::create_access_token_credentials()
            .await
            .map_err(Error::authentication)
    }

    async fn make_headers(
        &self,
        options: &gax::options::RequestOptions,
        request_params: &str,
    ) -> Result<http::header::HeaderMap> {
        let mut headers = self.cred.headers().await.map_err(Error::authentication)?;
        headers.push((
            http::header::HeaderName::from_static("x-goog-api-client"),
            http::header::HeaderValue::from_static(&info::X_GOOG_API_CLIENT_HEADER),
        ));
        headers.push((
            http::header::HeaderName::from_static("x-goog-request-params"),
            http::header::HeaderValue::from_str(request_params).map_err(Error::other)?,
        ));
        if let Some(user_agent) = options.user_agent() {
            headers.push((
                http::header::USER_AGENT,
                http::header::HeaderValue::from_str(user_agent).map_err(Error::other)?,
            ));
        }
        Ok(http::header::HeaderMap::from_iter(headers))
    }

    async fn execute<Request, Response, F, RF>(
        &self,
        call: F,
        req: Request,
        options: &gax::options::RequestOptions,
        request_params: String,
    ) -> Result<Response>
    where
        F: Fn(Request, http::header::HeaderMap) -> RF + Send + Sync,
        RF: std::future::Future<Output = Result<Response>>,
        Request: std::clone::Clone,
    {
        match self.get_retry_policy(options) {
            None => {
                let headers = self.make_headers(options, &request_params).await?;
                call(req, headers).await
            }
            Some(policy) => {
                self.retry_loop(call, req, options, request_params, policy)
                    .await
            }
        }
    }

    async fn retry_loop<Request, Response, F, RF>(
        &self,
        call: F,
        req: Request,
        options: &gax::options::RequestOptions,
        request_params: String,
        retry_policy: Arc<dyn gax::retry_policy::RetryPolicy>,
    ) -> Result<Response>
    where
        F: Fn(Request, http::header::HeaderMap) -> RF + Send + Sync,
        RF: std::future::Future<Output = Result<Response>>,
        Request: std::clone::Clone,
    {
        let loop_start = std::time::Instant::now();
        let throttler = self.get_retry_throttler(options);
        let backoff = self.get_backoff_policy(options);
        let mut attempt_count = 0;
        loop {
            let request = req.clone();
            let remaining_time = retry_policy.remaining_time(loop_start, attempt_count);
            let throttle = if attempt_count == 0 {
                false
            } else {
                let t = throttler.lock().expect("retry throttler lock is poisoned");
                t.throttle_retry_attempt()
            };
            if throttle {
                // This counts as an error for the purposes of the retry policy.
                if let Some(error) = retry_policy.on_throttle(loop_start, attempt_count) {
                    return Err(error);
                }
                let delay = backoff.on_failure(loop_start, attempt_count);
                tokio::time::sleep(delay).await;
                continue;
            }
            attempt_count += 1;
            match self
                .request_attempt(&call, request, options, &request_params, remaining_time)
                .await
            {
                Ok(r) => {
                    throttler
                        .lock()
                        .expect("retry throttler lock is poisoned")
                        .on_success();
                    return Ok(r);
                }
                Err(e) => {
                    let flow = retry_policy.on_error(
                        loop_start,
                        attempt_count,
                        options.idempotent().unwrap_or(false),
                        e,
                    );
                    let delay = backoff.on_failure(loop_start, attempt_count);
                    {
                        throttler
                            .lock()
                            .expect("retry throttler lock is poisoned")
                            .on_retry_failure(&flow);
                    };
                    self.on_error(flow, delay).await?;
                }
            };
        }
    }

    async fn request_attempt<Request, Response, F, RF>(
        &self,
        call: &F,
        req: Request,
        options: &gax::options::RequestOptions,
        request_params: &str,
        _remaining_time: Option<std::time::Duration>,
    ) -> Result<Response>
    where
        F: Fn(Request, http::header::HeaderMap) -> RF + Send + Sync,
        RF: std::future::Future<Output = Result<Response>>,
        Request: std::clone::Clone,
    {
        let headers = self.make_headers(options, request_params).await?;
        call(req, headers).await
    }

    async fn on_error(
        &self,
        retry_flow: gax::loop_state::LoopState,
        backoff_delay: std::time::Duration,
    ) -> Result<()> {
        use gax::loop_state::LoopState;
        match retry_flow {
            LoopState::Permanent(e) | LoopState::Exhausted(e) => {
                return Err(e);
            }
            LoopState::Continue(_e) => {
                tokio::time::sleep(backoff_delay).await;
            }
        }
        Ok(())
    }

    fn get_retry_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Option<Arc<dyn gax::retry_policy::RetryPolicy>> {
        options
            .retry_policy()
            .clone()
            .or_else(|| self.retry_policy.clone())
    }

    fn get_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::backoff_policy::BackoffPolicy> {
        options
            .backoff_policy()
            .clone()
            .or_else(|| self.backoff_policy.clone())
            .unwrap_or_else(|| Arc::new(gax::exponential_backoff::ExponentialBackoff::default()))
    }

    fn get_retry_throttler(
        &self,
        options: &gax::options::RequestOptions,
    ) -> gax::retry_throttler::SharedRetryThrottler {
        options
            .retry_throttler()
            .clone()
            .unwrap_or_else(|| self.retry_throttler.clone())
    }
}

impl super::stub::Firestore for Firestore {
    async fn get_document(
        &self,
        req: crate::model::GetDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Document>> {
        use wkt::prost::Convert;
        let inner = self.inner.clone();
        let call = |r: crate::model::GetDocumentRequest, h: http::header::HeaderMap| async {
            let extensions = {
                let mut e = tonic::Extensions::new();
                e.insert(tonic::GrpcMethod::new(
                    "google.firestore.v1.Firestore",
                    "GetDocument",
                ));
                e
            };
            let metadata = tonic::metadata::MetadataMap::from_headers(h);
            let request = tonic::Request::from_parts(metadata, extensions, r.cnv());
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.firestore.v1.Firestore/GetDocument");
            let mut inner = inner.clone();
            inner.ready().await.map_err(Error::rpc)?;
            let response: tonic::Response<crate::google::firestore::v1::Document> = inner
                .unary(request, path, codec)
                .await
                .map_err(Error::rpc)?;
            let (metadata, body, _extensions) = response.into_parts();
            Ok(gax::response::Response::from_parts(
                gax::response::Parts::new().set_headers(metadata.into_headers()),
                body.cnv(),
            ))
        };
        let x_goog_request_params = [format!("name={}", req.name)]
            .into_iter()
            .fold(String::new(), |b, p| b + "&" + &p);

        self.execute(call, req, &options, x_goog_request_params)
            .await
    }

    async fn list_documents(
        &self,
        req: crate::model::ListDocumentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDocumentsResponse>> {
        use wkt::prost::Convert;
        let inner = self.inner.clone();
        let call = |r: crate::model::ListDocumentsRequest, h: http::header::HeaderMap| async {
            let extensions = {
                let mut e = tonic::Extensions::new();
                e.insert(tonic::GrpcMethod::new(
                    "google.firestore.v1.Firestore",
                    "ListDocuments",
                ));
                e
            };
            let metadata = tonic::metadata::MetadataMap::from_headers(h);
            let request = tonic::Request::from_parts(metadata, extensions, r.cnv());
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.v1.Firestore/ListDocuments",
            );
            let mut inner = inner.clone();
            inner.ready().await.map_err(Error::rpc)?;
            let response: tonic::Response<crate::google::firestore::v1::ListDocumentsResponse> =
                inner
                    .unary(request, path, codec)
                    .await
                    .map_err(Error::rpc)?;
            let (metadata, body, _extensions) = response.into_parts();
            Ok(gax::response::Response::from_parts(
                gax::response::Parts::new().set_headers(metadata.into_headers()),
                body.cnv(),
            ))
        };
        let x_goog_request_params = [
            format!("parent={}", req.parent),
            format!("collection_id={}", req.collection_id),
        ]
        .into_iter()
        .fold(String::new(), |b, p| b + "&" + &p);

        self.execute(call, req, &options, x_goog_request_params)
            .await
    }

    async fn update_document(
        &self,
        req: crate::model::UpdateDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Document>> {
        use wkt::prost::Convert;
        let inner = self.inner.clone();
        let call = |r: crate::model::UpdateDocumentRequest, h: http::header::HeaderMap| async {
            let extensions = {
                let mut e = tonic::Extensions::new();
                e.insert(tonic::GrpcMethod::new(
                    "google.firestore.v1.Firestore",
                    "UpdateDocument",
                ));
                e
            };
            let metadata = tonic::metadata::MetadataMap::from_headers(h);
            let request = tonic::Request::from_parts(metadata, extensions, r.cnv());
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.v1.Firestore/UpdateDocument",
            );
            let mut inner = inner.clone();
            inner.ready().await.map_err(Error::rpc)?;
            let response: tonic::Response<crate::google::firestore::v1::Document> = inner
                .unary(request, path, codec)
                .await
                .map_err(Error::rpc)?;
            let (metadata, body, _extensions) = response.into_parts();
            Ok(gax::response::Response::from_parts(
                gax::response::Parts::new().set_headers(metadata.into_headers()),
                body.cnv(),
            ))
        };
        let x_goog_request_params = [format!(
            "document.name={}",
            req.document
                .as_ref()
                .ok_or_else(|| gaxi::path_parameter::missing("document"))?
                .name
        )]
        .into_iter()
        .fold(String::new(), |b, p| b + "&" + &p);

        self.execute(call, req, &options, x_goog_request_params)
            .await
    }

    async fn delete_document(
        &self,
        req: crate::model::DeleteDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        use wkt::prost::Convert;
        let inner = self.inner.clone();
        let call = |r: crate::model::DeleteDocumentRequest, h: http::header::HeaderMap| async {
            let extensions = {
                let mut e = tonic::Extensions::new();
                e.insert(tonic::GrpcMethod::new(
                    "google.firestore.v1.Firestore",
                    "DeleteDocument",
                ));
                e
            };
            let metadata = tonic::metadata::MetadataMap::from_headers(h);
            let request = tonic::Request::from_parts(metadata, extensions, r.cnv());
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.v1.Firestore/DeleteDocument",
            );
            let mut inner = inner.clone();
            inner.ready().await.map_err(Error::rpc)?;
            let response: tonic::Response<()> = inner
                .unary(request, path, codec)
                .await
                .map_err(Error::rpc)?;
            let (metadata, _body, _extensions) = response.into_parts();
            Ok(gax::response::Response::from_parts(
                gax::response::Parts::new().set_headers(metadata.into_headers()),
                (),
            ))
        };
        let x_goog_request_params = [format!("name={}", req.name)]
            .into_iter()
            .fold(String::new(), |b, p| b + "&" + &p);

        self.execute(call, req, &options, x_goog_request_params)
            .await
    }

    async fn begin_transaction(
        &self,
        req: crate::model::BeginTransactionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::BeginTransactionResponse>> {
        use wkt::prost::Convert;
        let inner = self.inner.clone();
        let call = |r: crate::model::BeginTransactionRequest, h: http::header::HeaderMap| async {
            let extensions = {
                let mut e = tonic::Extensions::new();
                e.insert(tonic::GrpcMethod::new(
                    "google.firestore.v1.Firestore",
                    "BeginTransaction",
                ));
                e
            };
            let metadata = tonic::metadata::MetadataMap::from_headers(h);
            let request = tonic::Request::from_parts(metadata, extensions, r.cnv());
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.v1.Firestore/BeginTransaction",
            );
            let mut inner = inner.clone();
            inner.ready().await.map_err(Error::rpc)?;
            let response: tonic::Response<crate::google::firestore::v1::BeginTransactionResponse> =
                inner
                    .unary(request, path, codec)
                    .await
                    .map_err(Error::rpc)?;
            let (metadata, body, _extensions) = response.into_parts();
            Ok(gax::response::Response::from_parts(
                gax::response::Parts::new().set_headers(metadata.into_headers()),
                body.cnv(),
            ))
        };
        let x_goog_request_params = [format!("database={}", req.database)]
            .into_iter()
            .fold(String::new(), |b, p| b + "&" + &p);

        self.execute(call, req, &options, x_goog_request_params)
            .await
    }

    async fn commit(
        &self,
        req: crate::model::CommitRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::CommitResponse>> {
        use wkt::prost::Convert;
        let inner = self.inner.clone();
        let call = |r: crate::model::CommitRequest, h: http::header::HeaderMap| async {
            let extensions = {
                let mut e = tonic::Extensions::new();
                e.insert(tonic::GrpcMethod::new(
                    "google.firestore.v1.Firestore",
                    "Commit",
                ));
                e
            };
            let metadata = tonic::metadata::MetadataMap::from_headers(h);
            let request = tonic::Request::from_parts(metadata, extensions, r.cnv());
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.firestore.v1.Firestore/Commit");
            let mut inner = inner.clone();
            inner.ready().await.map_err(Error::rpc)?;
            let response: tonic::Response<crate::google::firestore::v1::CommitResponse> = inner
                .unary(request, path, codec)
                .await
                .map_err(Error::rpc)?;
            let (metadata, body, _extensions) = response.into_parts();
            Ok(gax::response::Response::from_parts(
                gax::response::Parts::new().set_headers(metadata.into_headers()),
                body.cnv(),
            ))
        };
        let x_goog_request_params = [format!("database={}", req.database)]
            .into_iter()
            .fold(String::new(), |b, p| b + "&" + &p);

        self.execute(call, req, &options, x_goog_request_params)
            .await
    }

    async fn rollback(
        &self,
        req: crate::model::RollbackRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        use wkt::prost::Convert;
        let inner = self.inner.clone();
        let call = |r: crate::model::RollbackRequest, h: http::header::HeaderMap| async {
            let extensions = {
                let mut e = tonic::Extensions::new();
                e.insert(tonic::GrpcMethod::new(
                    "google.firestore.v1.Firestore",
                    "Rollback",
                ));
                e
            };
            let metadata = tonic::metadata::MetadataMap::from_headers(h);
            let request = tonic::Request::from_parts(metadata, extensions, r.cnv());
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.firestore.v1.Firestore/Rollback");
            let mut inner = inner.clone();
            inner.ready().await.map_err(Error::rpc)?;
            let response: tonic::Response<()> = inner
                .unary(request, path, codec)
                .await
                .map_err(Error::rpc)?;
            let (metadata, _body, _extensions) = response.into_parts();
            Ok(gax::response::Response::from_parts(
                gax::response::Parts::new().set_headers(metadata.into_headers()),
                (),
            ))
        };
        let x_goog_request_params = [format!("database={}", req.database)]
            .into_iter()
            .fold(String::new(), |b, p| b + "&" + &p);

        self.execute(call, req, &options, x_goog_request_params)
            .await
    }

    async fn partition_query(
        &self,
        req: crate::model::PartitionQueryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::PartitionQueryResponse>> {
        use wkt::prost::Convert;
        let inner = self.inner.clone();
        let call = |r: crate::model::PartitionQueryRequest, h: http::header::HeaderMap| async {
            let extensions = {
                let mut e = tonic::Extensions::new();
                e.insert(tonic::GrpcMethod::new(
                    "google.firestore.v1.Firestore",
                    "PartitionQuery",
                ));
                e
            };
            let metadata = tonic::metadata::MetadataMap::from_headers(h);
            let request = tonic::Request::from_parts(metadata, extensions, r.cnv());
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.v1.Firestore/PartitionQuery",
            );
            let mut inner = inner.clone();
            inner.ready().await.map_err(Error::rpc)?;
            let response: tonic::Response<crate::google::firestore::v1::PartitionQueryResponse> =
                inner
                    .unary(request, path, codec)
                    .await
                    .map_err(Error::rpc)?;
            let (metadata, body, _extensions) = response.into_parts();
            Ok(gax::response::Response::from_parts(
                gax::response::Parts::new().set_headers(metadata.into_headers()),
                body.cnv(),
            ))
        };
        let x_goog_request_params = [format!("parent={}", req.parent)]
            .into_iter()
            .fold(String::new(), |b, p| b + "&" + &p);

        self.execute(call, req, &options, x_goog_request_params)
            .await
    }

    async fn list_collection_ids(
        &self,
        req: crate::model::ListCollectionIdsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListCollectionIdsResponse>> {
        use wkt::prost::Convert;
        let inner = self.inner.clone();
        let call = |r: crate::model::ListCollectionIdsRequest, h: http::header::HeaderMap| async {
            let extensions = {
                let mut e = tonic::Extensions::new();
                e.insert(tonic::GrpcMethod::new(
                    "google.firestore.v1.Firestore",
                    "ListCollectionIds",
                ));
                e
            };
            let metadata = tonic::metadata::MetadataMap::from_headers(h);
            let request = tonic::Request::from_parts(metadata, extensions, r.cnv());
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.v1.Firestore/ListCollectionIds",
            );
            let mut inner = inner.clone();
            inner.ready().await.map_err(Error::rpc)?;
            let response: tonic::Response<crate::google::firestore::v1::ListCollectionIdsResponse> =
                inner
                    .unary(request, path, codec)
                    .await
                    .map_err(Error::rpc)?;
            let (metadata, body, _extensions) = response.into_parts();
            Ok(gax::response::Response::from_parts(
                gax::response::Parts::new().set_headers(metadata.into_headers()),
                body.cnv(),
            ))
        };
        let x_goog_request_params = [format!("parent={}", req.parent)]
            .into_iter()
            .fold(String::new(), |b, p| b + "&" + &p);

        self.execute(call, req, &options, x_goog_request_params)
            .await
    }

    async fn batch_write(
        &self,
        req: crate::model::BatchWriteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::BatchWriteResponse>> {
        use wkt::prost::Convert;
        let inner = self.inner.clone();
        let call = |r: crate::model::BatchWriteRequest, h: http::header::HeaderMap| async {
            let extensions = {
                let mut e = tonic::Extensions::new();
                e.insert(tonic::GrpcMethod::new(
                    "google.firestore.v1.Firestore",
                    "BatchWrite",
                ));
                e
            };
            let metadata = tonic::metadata::MetadataMap::from_headers(h);
            let request = tonic::Request::from_parts(metadata, extensions, r.cnv());
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.firestore.v1.Firestore/BatchWrite");
            let mut inner = inner.clone();
            inner.ready().await.map_err(Error::rpc)?;
            let response: tonic::Response<crate::google::firestore::v1::BatchWriteResponse> = inner
                .unary(request, path, codec)
                .await
                .map_err(Error::rpc)?;
            let (metadata, body, _extensions) = response.into_parts();
            Ok(gax::response::Response::from_parts(
                gax::response::Parts::new().set_headers(metadata.into_headers()),
                body.cnv(),
            ))
        };
        let x_goog_request_params = [format!("database={}", req.database)]
            .into_iter()
            .fold(String::new(), |b, p| b + "&" + &p);

        self.execute(call, req, &options, x_goog_request_params)
            .await
    }

    async fn create_document(
        &self,
        req: crate::model::CreateDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Document>> {
        use wkt::prost::Convert;
        let inner = self.inner.clone();
        let call = |r: crate::model::CreateDocumentRequest, h: http::header::HeaderMap| async {
            let extensions = {
                let mut e = tonic::Extensions::new();
                e.insert(tonic::GrpcMethod::new(
                    "google.firestore.v1.Firestore",
                    "CreateDocument",
                ));
                e
            };
            let metadata = tonic::metadata::MetadataMap::from_headers(h);
            let request = tonic::Request::from_parts(metadata, extensions, r.cnv());
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.v1.Firestore/CreateDocument",
            );
            let mut inner = inner.clone();
            inner.ready().await.map_err(Error::rpc)?;
            let response: tonic::Response<crate::google::firestore::v1::Document> = inner
                .unary(request, path, codec)
                .await
                .map_err(Error::rpc)?;
            let (metadata, body, _extensions) = response.into_parts();
            Ok(gax::response::Response::from_parts(
                gax::response::Parts::new().set_headers(metadata.into_headers()),
                body.cnv(),
            ))
        };
        let x_goog_request_params = [
            format!("parent={}", req.parent),
            format!("collection_id={}", req.collection_id),
        ]
        .into_iter()
        .fold(String::new(), |b, p| b + "&" + &p);

        self.execute(call, req, &options, x_goog_request_params)
            .await
    }
}
