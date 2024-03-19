use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::anyhow;
use tempfile::tempdir;

use crate::WhisperModel;

fn build_whisper(model: &WhisperModel) -> anyhow::Result<()> {
    let whisper_dir_path = crate::dirs::repository()?;
    let whisper_dir_path = whisper_dir_path.to_string_lossy();

    Command::new("bash")
        .args([
            format!("{}/models/download-ggml-model.sh", &whisper_dir_path).as_str(),
            &model.to_string(),
        ])
        .output()
        .expect("failed to execute bash ./models/download-ggml-model.sh base.en");

    Command::new("make")
        .arg(format!("--directory={}", &whisper_dir_path))
        .output()
        .expect("failed to execute make");

    Command::new("mv")
        .args([
            format!("{}/main", &whisper_dir_path),
            format!("{}/main-{}", &whisper_dir_path, model),
        ])
        .output()
        .expect("failed to execute mv");

    Ok(())
}

pub fn check_dependencies() -> anyhow::Result<()> {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map_err(|_| anyhow!("ffmpeg not found. use brew install ffmpeg"))?;

    Ok(())
}

fn ensure_repository() -> anyhow::Result<()> {
    let repository = crate::dirs::repository()?;

    if !repository.join(".git").exists() {
        std::fs::create_dir_all(&repository)?;
        Command::new("git")
            .arg("clone")
            .arg("git@github.com:ggerganov/whisper.cpp.git")
            .arg(&repository)
            .output()?;
    }

    Ok(())
}

fn media_to_wav(media: PathBuf, temp_dir: &Path) -> anyhow::Result<PathBuf> {
    let wav_path = temp_dir
        .join(media.file_name().ok_or(anyhow!("not a file"))?)
        .with_extension("wav");

    Command::new("ffmpeg")
        .args([
            "-i",
            &media.to_string_lossy(),
            "-ar",
            "16000",
            "-f",
            "wav",
            &wav_path.to_string_lossy(),
        ])
        .output()
        .expect("failed to execute ffmpeg");

    Ok(wav_path)
}

pub fn setup(model: &WhisperModel) -> anyhow::Result<()> {
    ensure_repository()?;
    build_whisper(model)?;

    Ok(())
}

pub fn transcribe(
    input_file: PathBuf,
    output_dir: PathBuf,
    model: &WhisperModel,
) -> anyhow::Result<()> {
    let temp_dir = tempdir()?;

    wav_to_text(
        media_to_wav(input_file, temp_dir.path())?,
        output_dir,
        model,
    )?;

    Ok(())
}

fn wav_to_text(wav: PathBuf, out: PathBuf, model: &WhisperModel) -> anyhow::Result<()> {
    let whisper_dir_path = crate::dirs::repository()?;
    let whisper_dir_path = whisper_dir_path.to_string_lossy();

    let out = out.join(wav.file_name().ok_or(anyhow!("not a file"))?);

    Command::new(format!("{}/main-{}", whisper_dir_path, &model))
        .args([
            "-m",
            format!("{whisper_dir_path}/models/ggml-{model}.bin").as_str(),
            "-f",
            &wav.to_string_lossy(),
            "-otxt",
            "-of",
            &out.to_string_lossy(),
        ])
        .output()
        .expect("failed to execute whisper");

    Ok(())
}
