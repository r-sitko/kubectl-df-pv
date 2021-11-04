mod models;
mod stats;

use crate::stats::PVStatsCollector;
use tokio;

// kubectl get --raw /api/v1/nodes/minikube/proxy/stats/summary
// kubectl get --raw /api/v1/nodes/minikube-m02/proxy/stats/summary

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let pv_stats_collector = PVStatsCollector::new().await?;
    pv_stats_collector.get_pvs_stats().await?;

    Ok(())
}
