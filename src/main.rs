mod models;
mod output;
mod stats;

use crate::output::{OutputGenerator, PrettyTable};
use crate::stats::PVStatsCollector;

// kubectl get --raw /api/v1/nodes/minikube/proxy/stats/summary
// kubectl get --raw /api/v1/nodes/minikube-m02/proxy/stats/summary

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let pv_stats_collector = PVStatsCollector::new().await?;
    let pvs_stats = pv_stats_collector.get_pvs_stats().await?;

    let output_generator = OutputGenerator::new(PrettyTable, std::io::stdout());
    output_generator.generate(&pvs_stats);

    Ok(())
}
