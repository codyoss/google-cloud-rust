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

#[cfg(all(test, feature = "run-integration-tests"))]
mod driver {
    use gax::error::*;
    use test_case::test_case;

    fn report(e: Error) -> Error {
        println!("\nERROR {e}\n");
        Error::other("test failed")
    }

    fn retry_policy() -> impl gax::retry_policy::RetryPolicy {
        use gax::retry_policy::RetryPolicyExt;
        use std::time::Duration;
        gax::retry_policy::AlwaysRetry
            .with_time_limit(Duration::from_secs(15))
            .with_attempt_limit(5)
    }

    #[test_case(firestore::client::Firestore::builder(); "default")]
    #[test_case(firestore::client::Firestore::builder().with_tracing(); "with tracing enabled")]
    #[test_case(firestore::client::Firestore::builder().with_retry_policy(retry_policy()); "with retry enabled")]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn run_firestore(
        builder: firestore::builder::firestore::ClientBuilder,
    ) -> integration_tests::Result<()> {
        integration_tests::firestore::basic(builder)
            .await
            .map_err(report)
    }

    #[test_case(sm::client::SecretManagerService::builder(); "default")]
    #[test_case(sm::client::SecretManagerService::builder().with_tracing(); "with tracing enabled")]
    #[test_case(sm::client::SecretManagerService::builder().with_retry_policy(retry_policy()); "with retry enabled")]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn run_secretmanager_protobuf(
        builder: sm::builder::secret_manager_service::ClientBuilder,
    ) -> integration_tests::Result<()> {
        integration_tests::secret_manager::protobuf::run(builder)
            .await
            .map_err(report)
    }

    #[test_case(smo::client::SecretManagerService::builder(); "default")]
    #[test_case(smo::client::SecretManagerService::builder().with_tracing(); "with tracing enabled")]
    #[test_case(smo::client::SecretManagerService::builder().with_retry_policy(retry_policy()); "with retry enabled")]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn run_secretmanager_openapi(
        builder: smo::builder::secret_manager_service::ClientBuilder,
    ) -> integration_tests::Result<()> {
        integration_tests::secret_manager::openapi::run(builder)
            .await
            .map_err(report)
    }

    #[test_case(smo::client::SecretManagerService::builder(); "default")]
    #[test_case(smo::client::SecretManagerService::builder().with_tracing(); "with tracing enabled")]
    #[test_case(smo::client::SecretManagerService::builder().with_retry_policy(retry_policy()); "with retry enabled")]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn run_secretmanager_openapi_locational(
        builder: smo::builder::secret_manager_service::ClientBuilder,
    ) -> integration_tests::Result<()> {
        integration_tests::secret_manager::openapi_locational::run(builder)
            .await
            .map_err(report)
    }

    #[test_case(wf::client::Workflows::builder(); "default")]
    #[test_case(wf::client::Workflows::builder().with_tracing(); "with tracing enabled")]
    #[test_case(wf::client::Workflows::builder().with_retry_policy(retry_policy()); "with retry enabled")]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn workflows_until_done(
        builder: wf::builder::workflows::ClientBuilder,
    ) -> integration_tests::Result<()> {
        integration_tests::workflows::until_done(builder)
            .await
            .map_err(report)
    }

    #[test_case(wf::client::Workflows::builder(); "default")]
    #[test_case(wf::client::Workflows::builder().with_tracing(); "with tracing enabled")]
    #[test_case(wf::client::Workflows::builder().with_retry_policy(retry_policy()); "with retry enabled")]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn workflows_explicit(
        builder: wf::builder::workflows::ClientBuilder,
    ) -> integration_tests::Result<()> {
        integration_tests::workflows::explicit_loop(builder)
            .await
            .map_err(report)
    }

    #[test_case(wf::client::Workflows::builder(); "default")]
    #[test_case(wf::client::Workflows::builder().with_tracing(); "with tracing enabled")]
    #[test_case(wf::client::Workflows::builder().with_retry_policy(retry_policy()); "with retry enabled")]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn workflows_manual(
        builder: wf::builder::workflows::ClientBuilder,
    ) -> integration_tests::Result<()> {
        integration_tests::workflows::until_done(builder)
            .await
            .map_err(report)
    }
}
