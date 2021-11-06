use crate::models::PVStats;
use crate::models::Summary;
use anyhow::{Context, Result};
use futures::future::join_all;
use k8s_openapi::api::core::v1::Node;
use kube::core::Resource;
use kube::{
    api::{Api, ListParams, ObjectList, Request},
    Client, ResourceExt,
};

pub struct PVStatsCollector {
    client: Client,
}

impl PVStatsCollector {
    pub async fn new() -> Result<Self> {
        let client = Client::try_default()
            .await
            .context("Failed to create K8S client")?;
        Ok(PVStatsCollector { client })
    }

    pub async fn get_pvs_stats(&self, namespace: Option<&str>) -> Result<Vec<PVStats>> {
        let nodes = self
            .get_all_nodes()
            .await
            .context("Failed to get Kubernetes nodes")?;

        let mut futures = Vec::new();

        for node in nodes {
            futures.push(self.get_node_summary(node.name()));
        }

        let nodes_summaries: Vec<Summary> = join_all(futures)
            .await
            .into_iter()
            .collect::<Result<Vec<Summary>>>()
            .context("Failed to get Summary result from Kubernetes")?;

        let mut pvs_stats = Self::build_pvs_stats(&nodes_summaries);

        if let Some(namespace) = namespace {
            pvs_stats = Self::filter_pvs_stats_by_namespace(pvs_stats, namespace);
        }

        Ok(pvs_stats)
    }

    async fn get_all_nodes(&self) -> Result<ObjectList<Node>> {
        let nodes_api: Api<Node> = Api::all(self.client.clone());
        nodes_api
            .list(&ListParams::default())
            .await
            .context("Failed to list nodes")
    }

    async fn get_node_summary(&self, node_name: String) -> Result<Summary> {
        let node_url = Node::url_path(&(), None);
        let req = Request::new(node_url)
            .get_subresource("proxy/stats/summary", &node_name)
            .context("Failed to get an instance of subresource")?;
        let res = self
            .client
            .request::<Summary>(req)
            .await
            .with_context(|| {
                format!(
                    "Failed to get Summary result from Kubernetes node {}",
                    node_name
                )
            })?;
        Ok(res)
    }

    fn build_pvs_stats(summaries: &[Summary]) -> Vec<PVStats> {
        summaries
            .iter()
            .map(|summary| &summary.pods_stats)
            .flat_map(|pods| {
                pods.iter()
                    .filter(|pod_stats| pod_stats.volume_stats.is_some())
                    .flat_map(|pod_stats| {
                        pod_stats.volume_stats.iter().flat_map(move |volume_stats| {
                            volume_stats
                                .iter()
                                .filter(|volume_stats| volume_stats.pvc_ref.is_some())
                                .map(move |volume_stats| PVStats {
                                    pvc_name: volume_stats.pvc_ref.as_ref().unwrap().name.clone(),
                                    pod_namespace: pod_stats.pod_ref.namespace.clone(),
                                    pod_name: pod_stats.pod_ref.name.clone(),
                                    capacity_bytes: volume_stats.fs_stats.capacity_bytes,
                                    used_bytes: volume_stats.fs_stats.used_bytes,
                                    inodes_free: volume_stats.fs_stats.inodes_free,
                                    inodes: volume_stats.fs_stats.inodes,
                                    inodes_used: volume_stats.fs_stats.inodes_used,
                                })
                        })
                    })
            })
            .collect()
    }

    fn filter_pvs_stats_by_namespace(pvs_stats: Vec<PVStats>, namespace: &str) -> Vec<PVStats> {
        pvs_stats
            .into_iter()
            .filter(|stats| stats.pod_namespace == namespace)
            .collect()
    }
}
