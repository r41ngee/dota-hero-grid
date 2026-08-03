use clap::Parser;
#[cfg(feature = "dev")]
use std::fs;
use std::io::Write;
use dota_hero_grid::*;
use std::collections::HashMap;

mod cli;

mod dev;

fn main() -> Result<(), anyhow::Error> {
    let args = cli::Cli::parse();

    let grid = match &args.action {
        cli::Action::FromPic { input, grid_name, x, y } => {
            let grid_hero_size = (*x, *y);

            let mut grid = Grid::new(grid_name);
            let mut cat = Category::new("", (0.0, 0.0), (1200.0, 800.0));

            let img = image::open(input)?;
            let img = img.resize_exact(grid_hero_size.0, grid_hero_size.1, image::imageops::FilterType::Nearest);

            for x in 0..grid_hero_size.0 {
                for y in 0..grid_hero_size.1 {
                    use image::GenericImageView;

                    let rgb = img.get_pixel(x, y);
                    let nearest_id = get_nearest_id((rgb[0], rgb[1], rgb[2]))?;
                    cat.add_hero_id(nearest_id as u32);
                }
            }
            grid.add_category(cat);
            Some(grid)
        },

        // DEV
        #[cfg(feature = "dev")]
        cli::Action::DlVids => {
            let _ = fs::create_dir("portraits/");

            let dota_base = "/home/ryoshin/.steam/steam/steamapps/common/dota 2 beta/game/dota";
            let path = format!("{}/panorama/videos/heroes/", dota_base);

            let mut hpps: HashMap<String, dev::HeroPortraitPixels> = HashMap::new();

            for hero in rdotaconstants::Hero::all().iter() {
                const DODGE_LIST: &[&str] = &[
                    "npc_dota_hero_target_dummy",
                ];

                if DODGE_LIST.contains(&hero.name.as_str()) {
                    continue;
                }

                let file_str = format!("{}{}.webm", path, hero.name);
                let output = std::process::Command::new("ffmpeg")
                    .args([
                        "-hide_banner",
                        "-loglevel",
                        "error",
                        "-i",
                        &file_str,
                        "-vf",
                        "scale=8:8",
                        "-f",
                        "rawvideo",
                        "-pix_fmt",
                        "rgb24",
                        "-",
                    ])
                    .output()?;

                if !output.status.success() {
                    anyhow::bail!(
                        "ffmpeg failed for {}: {}",
                        hero.name,
                        String::from_utf8_lossy(&output.stderr)
                    );
                }

                let total = (output.stdout.len() / 3) as u64;
                if total == 0 {
                    anyhow::bail!("no frames extracted for {}", hero.name);
                }

                let mut sums = [0u64; 3];
                for (i, b) in output.stdout.iter().enumerate() {
                    sums[i % 3] += *b as u64;
                }

                hpps.insert(hero.name.clone(), dev::HeroPortraitPixels {
                    id: hero.id,
                    rgb: sums.map(|s| (s / total) as u8).to_vec(),
                });
            }

            let output_path = "crates/dotagrid-cli/src/portraits.toml";
            let toml_str = toml::to_string(&hpps)?;
            fs::write(output_path, toml_str)?;

            None
        }
    };

    if let Some(grid) = grid {
        if let Some(add_to) = args.add_to {
            if !std::fs::exists(&add_to)? {
                return Err(anyhow::anyhow!("File {} does not exist", add_to));
            }

            let mut map = dota_hero_grid::deserialize(
                &std::fs::read_to_string(&add_to)?
            )?;

            map.add_grid(grid);
            let mut file = std::fs::File::open(&add_to)?;
            file.write(serialize(&map)?.as_bytes())?;
        } else {
            let mut map = dota_hero_grid::GridMap::new();
            map.add_grid(grid);
            let output_path = args.output.unwrap_or("hero_grid_config.json".to_string());
            let mut output_file = std::fs::File::create(&output_path)?;
            output_file.write(serialize_pretty(&map)?.as_bytes())?;
        }
    }

    Ok(())
}

fn get_nearest_id(rgb: (u8, u8, u8)) -> Result<i64, anyhow::Error> {
    let hpps: HashMap<String, dev::HeroPortraitPixels> = toml::from_str(
        include_str!("portraits.toml")
    )?;

    let mut nearest_id = 0;
    let mut nearest_dist = f64::MAX;

    for (_, hpp) in hpps.iter() {
        let dist = ((hpp.rgb[0] as f64 - rgb.0 as f64).powi(2)
            + (hpp.rgb[1] as f64 - rgb.1 as f64).powi(2)
            + (hpp.rgb[2] as f64 - rgb.2 as f64).powi(2)).sqrt();

        if dist < nearest_dist {
            nearest_dist = dist;
            nearest_id = hpp.id;
        }
    }

    Ok(nearest_id)
}
