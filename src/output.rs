use crate::models::PVStats;
use anyhow::{Context, Result};
use comfy_table::{presets::NOTHING, Cell, Table};
use std::{cell::RefCell, io::Write};

pub trait OutputFormatter {
    fn format(&self, pvs_stats: &[PVStats]) -> Result<String>;
}

pub struct OutputGenerator<S: OutputFormatter, W: Write> {
    strategy: S,
    writer: RefCell<W>,
}

impl<S: OutputFormatter, W: Write> OutputGenerator<S, W> {
    pub fn new(strategy: S, writer: W) -> Self {
        OutputGenerator {
            strategy,
            writer: RefCell::new(writer),
        }
    }

    pub fn generate(&self, pvs_stats: &[PVStats]) -> Result<()> {
        let formatted_output = self
            .strategy
            .format(pvs_stats)
            .context("Failed to format output")?;
        self.writer
            .borrow_mut()
            .write_all(formatted_output.as_bytes())
            .context("Failed write_all data")
    }
}

pub struct PrettyTableOutputFormatter;

impl OutputFormatter for PrettyTableOutputFormatter {
    fn format(&self, pvs_stats: &[PVStats]) -> Result<String> {
        let mut table = Table::new();
        table
            .set_header(vec![
                "PVC NAME",
                "POD NAMESPACE",
                "POD NAME",
                "CAPACITY",
                "USED",
                "INODES",
                "INODES USED",
                "INODES FREE",
            ])
            .load_preset(NOTHING);

        for pv_stats in pvs_stats {
            table.add_row(vec![
                Cell::new(pv_stats.pvc_name.clone()),
                Cell::new(pv_stats.pod_namespace.clone()),
                Cell::new(pv_stats.pod_name.clone()),
                Cell::new(pv_stats.capacity_bytes),
                Cell::new(pv_stats.used_bytes),
                Cell::new(pv_stats.inodes),
                Cell::new(pv_stats.inodes_used),
                Cell::new(pv_stats.inodes_free),
            ]);
        }

        let mut output = table.to_string();
        output.push('\n');
        Ok(output)
    }
}

pub struct JsonOutputFormatter;

impl OutputFormatter for JsonOutputFormatter {
    fn format(&self, pvs_stats: &[PVStats]) -> Result<String> {
        serde_json::to_string(pvs_stats).context("Failed serialize PVs statistics to JSON")
    }
}
