#[derive(clap::Parser)]
pub struct Cli {
    #[arg(short, help = "output file")]
    pub output: Option<String>,

    #[command(subcommand)]
    pub action: Action,
}

#[derive(clap::Subcommand)]
pub enum Action {
    #[command()]
    Build {
        #[arg(help = "input file")]
        input: String,
    }
}
