#[cfg(target_os = "windows")]
const DEFAULT_CONFIG_PATH: &str = "Steam/userdata/{your_steam_id}/570/remote/dota_hero_grid_config.json";

#[cfg(target_os = "linux")]
const DEFAULT_CONFIG_PATH: &str = "~/.steam/steam/userdata/{your_steam_id}/570/remote/dota_hero_grid_config.json";

#[derive(clap::Parser)]
pub struct Cli {
    #[arg(short, help = "output file")]
    pub output: Option<String>,

    #[arg(short, long, help = format!("add to existing config (default path is {}", DEFAULT_CONFIG_PATH))]
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

        #[arg(short, long, help = "grid width in heroes", default_value_t = 256)]
        x: u32,
        #[arg(short, long, help = "grid height in heroes", default_value_t = 64)]
        y: u32,
    },
    #[cfg(feature = "dev")]
    DlVids,
}
