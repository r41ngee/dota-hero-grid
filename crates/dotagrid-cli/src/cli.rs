#[derive(clap::Parser)]
pub struct Cli {
    #[arg(short, help = "output file")]
    pub output: Option<String>,

    #[command(subcommand)]
    pub action: Action,
}

#[derive(clap::Subcommand)]
pub enum Action {
    Build {
        #[arg(help = "input file")]
        input: String,
    },
    FromPic {
        #[arg(help = "input file")]
        input: String,
        #[arg(help = "hero name")]
        grid_name: String,
    },
    #[cfg(feature = "dev")]
    DlVids,
}
