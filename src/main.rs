#![feature(let_chains)]
#![allow(unused)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use chrono::prelude::*;
use clap::Parser;
use log::{info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

mod api;
mod wallpaper;


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

fn main() -> anyhow::Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )?;
    let args = Args::parse();
    let mut night_wallpapers = wallpaper::Wallpapers::try_from(&args.night_wallpapers_path)?;
    let mut day_wallpapers = if args.combine_day_and_night_for_day {
        wallpaper::Wallpapers::new(&[&args.day_wallpapers_path,&args.night_wallpapers_path])
    } else {
        wallpaper::Wallpapers::from(&args.day_wallpapers_path)
    };

    // Don't want to crash the program just because we could not fetch info about current day
    // We will try to refetch it after some time
    let mut day_info = api::get_day_info(args.latitude, args.longitude);

    loop {
        let Ok(ref mut day_info) = day_info else {
            info!("Trying to get day_info");
            day_info = api::get_day_info(args.latitude, args.longitude);
            sleep(Duration::from_secs(60 * 5));
            continue;
        };
        let now = Local::now();
        if now.day() != day_info.sunset.day() && let Ok(info) = api::get_day_info(args.latitude,args.longitude) {
            info!("Updating day_info from day {} to {}",day_info.sunset.day(), now.day());
            *day_info = info;
        }
        if (now.hour() >= day_info.sunset.hour()) || (now.hour() < day_info.sunrise.hour()) {
            night_wallpapers.set_random_wallpaper(args.change_wallpaper_every_nth_minutes);
        } else {
            day_wallpapers.set_random_wallpaper(args.change_wallpaper_every_nth_minutes);
        }
    }
}