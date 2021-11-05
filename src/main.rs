mod models;
mod output;
mod stats;

use crate::output::{JsonOutputFormatter, OutputGenerator, PrettyTableOutputFormatter};
use crate::stats::PVStatsCollector;

// kubectl get --raw /api/v1/nodes/minikube/proxy/stats/summary
// kubectl get --raw /api/v1/nodes/minikube-m02/proxy/stats/summary

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let pv_stats_collector = PVStatsCollector::new().await?;
    let pvs_stats = pv_stats_collector.get_pvs_stats().await?;

    let output_generator = OutputGenerator::new(PrettyTableOutputFormatter, std::io::stdout());
    output_generator.generate(&pvs_stats).unwrap();

    Ok(())
}
