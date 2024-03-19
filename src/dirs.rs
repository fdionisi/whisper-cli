use std::path::PathBuf;

use anyhow::Result;
use directories::ProjectDirs;

fn project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("me", "fdionisi", "whisper-cli")
        .ok_or_else(|| anyhow::anyhow!("cannot determine project directories"))
}

pub fn repository() -> Result<PathBuf> {
    let data = project_dirs()?.data_dir().to_path_buf();

    if !data.exists() {
        std::fs::create_dir_all(&data)?;
    }

    Ok(data.join("whisper.cpp"))
}
