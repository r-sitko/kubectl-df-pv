mod args;
mod models;
mod output;
mod stats;

use crate::args::{ApplicationArgs, Format};
use crate::output::{JsonOutputFormatter, OutputGenerator, PrettyTableOutputFormatter};
use crate::stats::PVStatsCollector;
use structopt::StructOpt;

// kubectl get --raw /api/v1/nodes/minikube/proxy/stats/summary
// kubectl get --raw /api/v1/nodes/minikube-m02/proxy/stats/summary

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let app_args = ApplicationArgs::from_args();

    let pv_stats_collector = PVStatsCollector::new().await?;
    let pvs_stats = pv_stats_collector
        .get_pvs_stats(app_args.namespace.as_deref())
        .await?;

    match app_args.format {
        Format::Table => {
            let output_generator =
                OutputGenerator::new(PrettyTableOutputFormatter, std::io::stdout());
            output_generator.generate(&pvs_stats).unwrap();
        }
        Format::Json => {
            let output_generator = OutputGenerator::new(JsonOutputFormatter, std::io::stdout());
            output_generator.generate(&pvs_stats).unwrap();
        }
    }

    Ok(())
}
