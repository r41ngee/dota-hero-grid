#[derive(clap::Parser)]
pub struct Cli {
    #[arg(short, help = "output file")]
    pub output: Option<String>,

    #[arg(short, long, help = "add to existing config")]
    pub add_to: Option<String>,

    #[command(subcommand)]
    pub action: Action,
}

#[derive(clap::Subcommand)]
pub enum Action {
    FromPic {
        #[arg(help = "input file")]
        input: String,
        #[arg(help = "hero name", default_value = "")]
        grid_name: String,
    },
    #[cfg(feature = "dev")]
    DlVids,
}
