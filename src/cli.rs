use crate::config::{get_config_dir, get_data_dir};
use clap::Parser;
use clap_num::number_range;

#[derive(Parser, Debug)]
#[command(author, version = version(), about)]
pub struct Cli {
    /// Number of dinos hidden (max 99)
    #[arg(long, value_name = "INT", default_value_t = 10, value_parser=constraint_99)]
    pub num_dinos: usize,
    /// Height of the board (max 20)
    #[arg(long, value_name = "INT", default_value_t = 9, value_parser=constraint_20)]
    pub height: usize,
    /// Width of the board (max 30)
    #[arg(long, value_name = "INT", default_value_t = 9, value_parser=constraint_30)]
    pub width: usize,
}

const VERSION_MESSAGE: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "-",
    env!("VERGEN_GIT_DESCRIBE"),
    " (",
    env!("VERGEN_BUILD_DATE"),
    ")"
);

pub fn version() -> String {
    let author = clap::crate_authors!();

    // let current_exe_path = PathBuf::from(clap::crate_name!()).display().to_string();
    let config_dir_path = get_config_dir().display().to_string();
    let data_dir_path = get_data_dir().display().to_string();

    format!(
        "\
{VERSION_MESSAGE}

Authors: {author}

Config directory: {config_dir_path}
Data directory: {data_dir_path}"
    )
}

fn constraint_20(s: &str) -> Result<usize, String> {
    number_range(s, 5, 20)
}
fn constraint_30(s: &str) -> Result<usize, String> {
    number_range(s, 5, 30)
}
fn constraint_99(s: &str) -> Result<usize, String> {
    number_range(s, 0, 99)
}
