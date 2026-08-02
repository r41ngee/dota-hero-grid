use clap::Parser;
use std::{fs, io::Write};
use dota_hero_grid::*;

mod cli;
mod types;

fn main() -> Result<(), anyhow::Error> {
    let args = cli::Cli::parse();

    match &args.action {
        cli::Action::Build { input } => {
            let file = fs::read_to_string(input)?;

            let sgm: types::SGridMap = toml::from_str(&file)?;
            let gm: GridMap = sgm.into();

            let json_gm = serialize_pretty(&gm)?;

            let output_path = args.output.unwrap_or_else(|| "hero_grid_config.json".to_string());
            let mut ofile = fs::File::create(&output_path)?;
            ofile.write_all(json_gm.as_bytes())?;
        },
    }



    Ok(())
}
