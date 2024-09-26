use std::path::PathBuf;

use structopt::StructOpt;

use crate::{OUTPUT_DIR, TEMPLATES_PATH};

#[derive(Debug, StructOpt)]
pub struct Rendro {
    #[structopt(subcommand)]
    pub cmd: Option<SubCommand>,
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    /// Renders templates that follow the Jinja templating convention
    Render {
        #[structopt(parse(from_os_str), default_value=TEMPLATES_PATH, short, long)]
        input_dir: PathBuf,

        #[structopt(parse(from_os_str), default_value=OUTPUT_DIR, short, long)]
        output_dir: PathBuf,

        #[structopt(default_value = "j2", short = "-x", long)]
        file_extension: String,
    },
}
