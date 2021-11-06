use lazy_static::lazy_static;
use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames, IntoStaticStr, VariantNames};

lazy_static! {
    static ref DEFAULT_FORMAT: &'static str = Format::Table.into();
}

#[derive(EnumString, EnumVariantNames, IntoStaticStr, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum Format {
    Table,
    Json,
}

#[derive(StructOpt, Debug)]
pub struct ApplicationArgs {
    #[structopt(short, long, possible_values = Format::VARIANTS, case_insensitive = true, default_value = &DEFAULT_FORMAT)]
    pub format: Format,
    #[structopt(short, long)]
    pub namespace: Option<String>,
}
