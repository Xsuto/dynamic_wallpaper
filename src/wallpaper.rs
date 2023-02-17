pub fn get_wallpapers<T: AsRef<Path>>(path: T) -> anyhow::Result<Vec<PathBuf>> {
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

pub fn set_random_wallpaper(wallpapers: &[PathBuf], repeat: u64) {
    const TRY_TO_CHANGE_WALLPAPER_EVERY_NTH_SEC: u64 = 2;
    let mut rng = rand::thread_rng();
    let wallpaper_path = wallpapers.choose(&mut rng).unwrap();
    let command = format!(
        "tell application \"System Events\" to set picture of every desktop to \"{}\"",
        wallpaper_path.display()
    );
    for _ in 0..(repeat * 60 / TRY_TO_CHANGE_WALLPAPER_EVERY_NTH_SEC) {
        let _ = Command::new("osascript").arg("-e").arg(&command).output();
        sleep(Duration::from_secs(TRY_TO_CHANGE_WALLPAPER_EVERY_NTH_SEC))
    }
}
