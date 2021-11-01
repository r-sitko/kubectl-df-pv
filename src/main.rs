mod models;

use kube::{api::{Api, ListParams, ResourceExt}, Client};
use k8s_openapi::api::core::v1::Node;
use tokio;
use kube::api::Request;
use crate::models::Summary;
use futures::future::join_all;
use kube::core::Resource;

// kubectl get --raw /api/v1/nodes/minikube/proxy/stats/summary
// kubectl get --raw /api/v1/nodes/minikube-m02/proxy/stats/summary

async fn get_nodes_summary(node_name: String) -> Result<Summary, kube::Error> {
    let client = Client::try_default().await?;
    let node_url = Node::url_path(&(), None);
    let req = Request::new(node_url).get_subresource("proxy/stats/summary", &node_name).unwrap();
    let res = client.request::<Summary>(req).await?;
    Ok(res)
}

fn print_pvcs(summary: &Summary) {
    for pod in &summary.pods {
        if let Some(volume_stats) = &pod.volume_stats {
            for volume in volume_stats {
                if let Some(_) = &volume.pvc_ref {
                    println!("PVC: {:?}", volume);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    // list nodes
    let client = Client::try_default().await?;
    let nodes_api: Api<Node> = Api::all(client);

    let nodes =  nodes_api.list(&ListParams::default()).await?;
    for node in &nodes {
        println!("Found node: {}", node.name());
    }

    // list stats for every node
    let mut futures = Vec::new();
    for node in nodes {
        futures.push(get_nodes_summary(node.name()));
    }

    let stats_result = join_all(futures).await;

    for stats in stats_result {
        print_pvcs(&stats.unwrap());
    }

    Ok(())
}
