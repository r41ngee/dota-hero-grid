#[derive(clap::Parser)]
pub struct Cli {
    #[arg(help = "TOML config file path")]
    pub input: String,

    #[arg(short, help = "output file")]
    pub output: Option<String>,
}
