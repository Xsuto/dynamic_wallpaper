use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use chrono::prelude::*;
use clap::Parser;
use rand::prelude::*;

mod api;

fn get_wallpapers<T: AsRef<Path>>(path: T) -> anyhow::Result<Vec<PathBuf>> {
    let mut wallpapers = Vec::new();
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            let file_path = entry.path();
            let extension = file_path.extension();
            if let Some(ext) = extension {
                if let Some("jpg") | Some("jpeg") | Some("png") = ext.to_str() {
                    wallpapers.push(entry.path());
                }
            }
        }
    }

    Ok(wallpapers)
}

fn set_random_wallpaper(wallpapers: &[PathBuf]) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let wallpaper_path = wallpapers.choose(&mut rng).unwrap();
    let command = format!(
        "tell application \"System Events\" to set picture of every desktop to \"{}\"",
        wallpaper_path.display()
    );
    match Command::new("osascript").arg("-e").arg(command).output() {
        Ok(_) => Ok(()),
        Err(_) => {
            anyhow::bail!(
                "Failed to change wallpaper. Probably program does not have permission to finder"
            )
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    day_wallpapers_path: String,

    #[arg(long)]
    night_wallpapers_path: String,

    #[arg(long, default_value_t = 30)]
    change_wallpaper_every_nth_minutes: u64,

    #[arg(long)]
    latitude: f64,

    #[arg(long)]
    longitude: f64,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let day_wallpapers = get_wallpapers(args.day_wallpapers_path)?;
    let night_wallpapers = get_wallpapers(args.night_wallpapers_path)?;
    let change_wallpaper_every_nth_minutes =
        std::time::Duration::from_secs(60 * args.change_wallpaper_every_nth_minutes);

    // Don't want to crash the program just because we could not fetch info about current day
    // We will try to refetch it after some time
    let mut day_info = api::get_day_info(args.latitude, args.longitude);

    loop {
        let Ok(ref mut day_info) = day_info else {
            day_info = api::get_day_info(args.latitude, args.longitude);
            sleep(Duration::from_secs(60 * 5));
            continue;
        };
        let now = Local::now();
        if now.day() != day_info.sunset.day() {
            if let Ok(info) = api::get_day_info(args.latitude, args.longitude) {
                *day_info = info;
            }
        }
        if (now.hour() >= day_info.sunset.hour()) || (now.hour() < day_info.sunrise.hour()) {
            set_random_wallpaper(&night_wallpapers)?;
        } else {
            set_random_wallpaper(&day_wallpapers)?;
        }
        sleep(change_wallpaper_every_nth_minutes);
    }
}
