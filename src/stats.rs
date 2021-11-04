use crate::models::PVStats;
use crate::models::Summary;
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
    pub async fn new() -> Result<Self, kube::Error> {
        let client = Client::try_default().await?;
        Ok(PVStatsCollector { client })
    }

    pub async fn get_pvs_stats(&self) -> Result<Vec<PVStats>, kube::Error> {
        let nodes = self.get_all_nodes().await?;

        let mut futures = Vec::new();

        for node in nodes {
            futures.push(self.get_node_summary(node.name()));
        }

        let nodes_summaries: Vec<Summary> = join_all(futures)
            .await
            .into_iter()
            .collect::<Result<Vec<Summary>, kube::Error>>()
            .unwrap();

        let pvs_stats = Self::build_pvs_stats(&nodes_summaries);

        Ok(pvs_stats)
    }

    async fn get_all_nodes(&self) -> Result<ObjectList<Node>, kube::Error> {
        let nodes_api: Api<Node> = Api::all(self.client.clone());
        nodes_api.list(&ListParams::default()).await
    }

    async fn get_node_summary(&self, node_name: String) -> Result<Summary, kube::Error> {
        let node_url = Node::url_path(&(), None);
        let req = Request::new(node_url)
            .get_subresource("proxy/stats/summary", &node_name)
            .unwrap();
        let res = self.client.clone().request::<Summary>(req).await?;
        Ok(res)
    }

    fn build_pvs_stats(summaries: &Vec<Summary>) -> Vec<PVStats> {
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
}
