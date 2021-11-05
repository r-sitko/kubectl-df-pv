use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames, VariantNames};

const DEFAULT_FORMAT: &str = "table";

#[derive(EnumString, EnumVariantNames, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum Format {
    Table,
    Json,
}

#[derive(StructOpt, Debug)]
pub struct ApplicationArgs {
    #[structopt(short, long, possible_values = Format::VARIANTS, case_insensitive = true, default_value = DEFAULT_FORMAT)]
    pub format: Format,
}
