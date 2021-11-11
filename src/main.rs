mod args;
mod models;
mod output;
mod stats;

use crate::{
    args::{ApplicationArgs, Format},
    output::{JsonOutputFormatter, OutputGenerator, PrettyTableOutputFormatter},
    stats::{Filter, PVSStatsNamespaceFilter, SummaryCollector, SummaryConverter},
};
use anyhow::{Context, Result};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    let app_args = ApplicationArgs::from_args();

    let summary_collector = SummaryCollector::new()
        .await
        .context("Failed to create SummaryCollector")?;
    let summaries = summary_collector
        .get_summaries()
        .await
        .context("Failed to get summaries")?;

    let mut pvs_stats = SummaryConverter::convert(summaries);

    if let Some(namespace) = app_args.namespace {
        pvs_stats = PVSStatsNamespaceFilter::new(&namespace).apply(pvs_stats);
    }

    match app_args.format {
        Format::Table => {
            OutputGenerator::new(PrettyTableOutputFormatter, std::io::stdout())
                .generate(&pvs_stats)
                .context("Failed to generate PrettyTable output")?;
        }
        Format::Json => {
            OutputGenerator::new(JsonOutputFormatter, std::io::stdout())
                .generate(&pvs_stats)
                .context("Failed to generate JSON output")?;
        }
    }

    Ok(())
}
