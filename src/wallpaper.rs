use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use rand::seq::SliceRandom;

pub struct Wallpapers {
    wallpapers_dirs: Vec<PathBuf>,
    current_wallpaper: Option<PathBuf>,
}

impl<T> From<T> for Wallpapers
where
    T: Into<PathBuf> + Clone,
{
    fn from(path: T) -> Self {
        Self::new(&[path])
    }
}
impl Wallpapers {
    pub fn new<T: Into<PathBuf> + Clone>(paths: &[T]) -> Self {
        let paths = Vec::from(paths);
        let wallpapers_dirs = paths
            .into_iter()
            .map(|it| it.into())
            .collect::<Vec<PathBuf>>();
        Self {
            wallpapers_dirs,
            current_wallpaper: None
        }
    }
    pub fn set_random_wallpaper(&mut self, repeat: u64) -> anyhow::Result<()> {
        const TRY_TO_CHANGE_WALLPAPER_EVERY_NTH_SEC: u64 = 2;
        let wallpapers = self
            .wallpapers_dirs
            .iter()
            .filter_map(|it| get_wallpapers(it).ok())
            .flatten()
            .filter_map(|it| match &self.current_wallpaper {
                None => Some(it),
                Some(current_wallpaper) => {
                    if current_wallpaper == &it {
                        None
                    } else {
                        Some(it)
                    }
                }
            })
            .collect::<Vec<PathBuf>>();
        let mut rng = rand::thread_rng();

        let wallpaper_path = wallpapers.choose(&mut rng).unwrap();
        self.current_wallpaper = Some(wallpaper_path.clone());
        let command = format!(
            "tell application \"System Events\" to set picture of every desktop to \"{}\"",
            wallpaper_path.display()
        );

        for _ in 0..(repeat * 60 / TRY_TO_CHANGE_WALLPAPER_EVERY_NTH_SEC) {
            let _ = Command::new("osascript").arg("-e").arg(&command).output();
            sleep(Duration::from_secs(TRY_TO_CHANGE_WALLPAPER_EVERY_NTH_SEC))
        }
        Ok(())
    }
}

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
