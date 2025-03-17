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

/// Implements a [AttachedClusters](super::stubs::AttachedClusters) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct AttachedClusters<T>
where
    T: super::stubs::AttachedClusters + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> AttachedClusters<T>
where
    T: super::stubs::AttachedClusters + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::AttachedClusters for AttachedClusters<T>
where
    T: super::stubs::AttachedClusters + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_attached_cluster(
        &self,
        req: crate::model::CreateAttachedClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_attached_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_attached_cluster(
        &self,
        req: crate::model::UpdateAttachedClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_attached_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn import_attached_cluster(
        &self,
        req: crate::model::ImportAttachedClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.import_attached_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_attached_cluster(
        &self,
        req: crate::model::GetAttachedClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AttachedCluster> {
        self.inner.get_attached_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_attached_clusters(
        &self,
        req: crate::model::ListAttachedClustersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAttachedClustersResponse> {
        self.inner.list_attached_clusters(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_attached_cluster(
        &self,
        req: crate::model::DeleteAttachedClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_attached_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_attached_server_config(
        &self,
        req: crate::model::GetAttachedServerConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AttachedServerConfig> {
        self.inner.get_attached_server_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn generate_attached_cluster_install_manifest(
        &self,
        req: crate::model::GenerateAttachedClusterInstallManifestRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GenerateAttachedClusterInstallManifestResponse> {
        self.inner
            .generate_attached_cluster_install_manifest(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn generate_attached_cluster_agent_token(
        &self,
        req: crate::model::GenerateAttachedClusterAgentTokenRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GenerateAttachedClusterAgentTokenResponse> {
        self.inner
            .generate_attached_cluster_agent_token(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_operation(req, options).await
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

/// Implements a [AwsClusters](super::stubs::AwsClusters) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct AwsClusters<T>
where
    T: super::stubs::AwsClusters + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> AwsClusters<T>
where
    T: super::stubs::AwsClusters + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::AwsClusters for AwsClusters<T>
where
    T: super::stubs::AwsClusters + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_aws_cluster(
        &self,
        req: crate::model::CreateAwsClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_aws_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_aws_cluster(
        &self,
        req: crate::model::UpdateAwsClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_aws_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_aws_cluster(
        &self,
        req: crate::model::GetAwsClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AwsCluster> {
        self.inner.get_aws_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_aws_clusters(
        &self,
        req: crate::model::ListAwsClustersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAwsClustersResponse> {
        self.inner.list_aws_clusters(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_aws_cluster(
        &self,
        req: crate::model::DeleteAwsClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_aws_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn generate_aws_cluster_agent_token(
        &self,
        req: crate::model::GenerateAwsClusterAgentTokenRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GenerateAwsClusterAgentTokenResponse> {
        self.inner
            .generate_aws_cluster_agent_token(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn generate_aws_access_token(
        &self,
        req: crate::model::GenerateAwsAccessTokenRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GenerateAwsAccessTokenResponse> {
        self.inner.generate_aws_access_token(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_aws_node_pool(
        &self,
        req: crate::model::CreateAwsNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_aws_node_pool(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_aws_node_pool(
        &self,
        req: crate::model::UpdateAwsNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_aws_node_pool(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn rollback_aws_node_pool_update(
        &self,
        req: crate::model::RollbackAwsNodePoolUpdateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.rollback_aws_node_pool_update(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_aws_node_pool(
        &self,
        req: crate::model::GetAwsNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AwsNodePool> {
        self.inner.get_aws_node_pool(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_aws_node_pools(
        &self,
        req: crate::model::ListAwsNodePoolsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAwsNodePoolsResponse> {
        self.inner.list_aws_node_pools(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_aws_node_pool(
        &self,
        req: crate::model::DeleteAwsNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_aws_node_pool(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_aws_open_id_config(
        &self,
        req: crate::model::GetAwsOpenIdConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AwsOpenIdConfig> {
        self.inner.get_aws_open_id_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_aws_json_web_keys(
        &self,
        req: crate::model::GetAwsJsonWebKeysRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AwsJsonWebKeys> {
        self.inner.get_aws_json_web_keys(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_aws_server_config(
        &self,
        req: crate::model::GetAwsServerConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AwsServerConfig> {
        self.inner.get_aws_server_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_operation(req, options).await
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

/// Implements a [AzureClusters](super::stubs::AzureClusters) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct AzureClusters<T>
where
    T: super::stubs::AzureClusters + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> AzureClusters<T>
where
    T: super::stubs::AzureClusters + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stubs::AzureClusters for AzureClusters<T>
where
    T: super::stubs::AzureClusters + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_azure_client(
        &self,
        req: crate::model::CreateAzureClientRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_azure_client(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_azure_client(
        &self,
        req: crate::model::GetAzureClientRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AzureClient> {
        self.inner.get_azure_client(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_azure_clients(
        &self,
        req: crate::model::ListAzureClientsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAzureClientsResponse> {
        self.inner.list_azure_clients(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_azure_client(
        &self,
        req: crate::model::DeleteAzureClientRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_azure_client(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_azure_cluster(
        &self,
        req: crate::model::CreateAzureClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_azure_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_azure_cluster(
        &self,
        req: crate::model::UpdateAzureClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_azure_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_azure_cluster(
        &self,
        req: crate::model::GetAzureClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AzureCluster> {
        self.inner.get_azure_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_azure_clusters(
        &self,
        req: crate::model::ListAzureClustersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAzureClustersResponse> {
        self.inner.list_azure_clusters(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_azure_cluster(
        &self,
        req: crate::model::DeleteAzureClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_azure_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn generate_azure_cluster_agent_token(
        &self,
        req: crate::model::GenerateAzureClusterAgentTokenRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GenerateAzureClusterAgentTokenResponse> {
        self.inner
            .generate_azure_cluster_agent_token(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn generate_azure_access_token(
        &self,
        req: crate::model::GenerateAzureAccessTokenRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GenerateAzureAccessTokenResponse> {
        self.inner.generate_azure_access_token(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_azure_node_pool(
        &self,
        req: crate::model::CreateAzureNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_azure_node_pool(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_azure_node_pool(
        &self,
        req: crate::model::UpdateAzureNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_azure_node_pool(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_azure_node_pool(
        &self,
        req: crate::model::GetAzureNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AzureNodePool> {
        self.inner.get_azure_node_pool(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_azure_node_pools(
        &self,
        req: crate::model::ListAzureNodePoolsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAzureNodePoolsResponse> {
        self.inner.list_azure_node_pools(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_azure_node_pool(
        &self,
        req: crate::model::DeleteAzureNodePoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_azure_node_pool(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_azure_open_id_config(
        &self,
        req: crate::model::GetAzureOpenIdConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AzureOpenIdConfig> {
        self.inner.get_azure_open_id_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_azure_json_web_keys(
        &self,
        req: crate::model::GetAzureJsonWebKeysRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AzureJsonWebKeys> {
        self.inner.get_azure_json_web_keys(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_azure_server_config(
        &self,
        req: crate::model::GetAzureServerConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AzureServerConfig> {
        self.inner.get_azure_server_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_operation(req, options).await
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
