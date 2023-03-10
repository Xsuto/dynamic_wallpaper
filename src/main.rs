#![feature(let_chains)]
#![allow(unused)]

use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use chrono::prelude::*;
use clap::Parser;
use log::{info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

mod day_info;
mod wallpaper;
use wallpaper::Wallpapers;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    day_wallpapers_path: String,

    #[arg(long)]
    night_wallpapers_path: String,

    #[arg(long)]
    latitude: f64,

    #[arg(long)]
    longitude: f64,

    #[arg(long, default_value_t = 30)]
    change_wallpaper_every_nth_minutes: u64,

    #[arg(long, default_value_t = true)]
    combine_day_and_night_for_day: bool,
}

fn get_day_info_path() -> anyhow::Result<PathBuf> {
    let mut path = PathBuf::new();
    path.push(dirs::home_dir().ok_or(anyhow::Error::msg("Couldn't get home dir"))?);
    path.push(".day_info.json");
    Ok(path)
}

fn main() -> anyhow::Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )?;
    let args = Args::parse();
    let day_info_path = get_day_info_path()?;
    let mut night_wallpapers = Wallpapers::try_from(&args.night_wallpapers_path)?;
    let mut day_wallpapers = if args.combine_day_and_night_for_day {
        Wallpapers::new(&[&args.day_wallpapers_path, &args.night_wallpapers_path])
    } else {
        Wallpapers::from(&args.day_wallpapers_path)
    };

    // Don't want to crash the program just because we could not fetch info about current day
    // We will try to refetch it after some time
    let mut day_info = day_info::get_from_file(&day_info_path);
    loop {
        let Ok(ref mut day_info) = day_info else {
            info!("Trying to get day_info");
            match day_info::fetch(args.latitude, args.longitude) {
                Ok(it ) => {
                    day_info::save_to_file(&day_info_path, &it);
                    day_info = Ok(it);
                }
                Err(_) => {
                    sleep(Duration::from_secs(60 * 5));
                }
            }
            continue;
        };
        let now = Local::now();
        if now.day() != day_info.sunset.day() && let Ok(info) = day_info::fetch(args.latitude, args.longitude) {
            info!("Updating day_info from day {} to {}",day_info.sunset.day(), now.day());
            *day_info = info;
            day_info::save_to_file(&day_info_path, day_info);
        }
        if (now.hour() >= day_info.sunset.hour()) || (now.hour() < day_info.sunrise.hour()) {
            night_wallpapers.set_random_wallpaper(args.change_wallpaper_every_nth_minutes);
        } else {
            day_wallpapers.set_random_wallpaper(args.change_wallpaper_every_nth_minutes);
        }
    }
}
