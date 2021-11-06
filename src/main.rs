mod args;
mod models;
mod output;
mod stats;

use crate::args::{ApplicationArgs, Format};
use crate::output::{JsonOutputFormatter, OutputGenerator, PrettyTableOutputFormatter};
use crate::stats::PVStatsCollector;
use anyhow::{Context, Result};
use structopt::StructOpt;

// kubectl get --raw /api/v1/nodes/minikube/proxy/stats/summary
// kubectl get --raw /api/v1/nodes/minikube-m02/proxy/stats/summary

#[tokio::main]
async fn main() -> Result<()> {
    let app_args = ApplicationArgs::from_args();

    let pv_stats_collector = PVStatsCollector::new()
        .await
        .context("Failed to create PVStatsCollector")?;
    let pvs_stats = pv_stats_collector
        .get_pvs_stats(app_args.namespace.as_deref())
        .await
        .context("Failed to get PVs statistics")?;

    match app_args.format {
        Format::Table => {
            let output_generator =
                OutputGenerator::new(PrettyTableOutputFormatter, std::io::stdout());
            output_generator
                .generate(&pvs_stats)
                .context("Failed to generate PrettyTable output")?;
        }
        Format::Json => {
            let output_generator = OutputGenerator::new(JsonOutputFormatter, std::io::stdout());
            output_generator
                .generate(&pvs_stats)
                .context("Failed to generate JSON output")?;
        }
    }

    Ok(())
}
