mod dirs;
mod sh;

use std::{env, fmt, path::PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
enum WhisperModel {
    #[clap(name = "tiny")]
    Tiny,
    #[clap(name = "tiny.en")]
    TinyEn,
    #[clap(name = "base")]
    Base,
    #[clap(name = "base.en")]
    BaseEn,
    #[clap(name = "small")]
    Small,
    #[clap(name = "small.en")]
    SmallEn,
    #[clap(name = "small.en-tdrz")]
    SmallEnTdrz,
    #[clap(name = "medium")]
    Medium,
    #[clap(name = "medium.en")]
    MediumEn,
    #[clap(name = "large-v1")]
    LargeV1,
    #[clap(name = "large-v2")]
    LargeV2,
    #[clap(name = "large-v2-q5_0")]
    LargeV2Q,
    #[clap(name = "large-v3")]
    LargeV3,
    #[clap(name = "large-v3-q5_0")]
    LargeV3Q,
}

impl fmt::Display for WhisperModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WhisperModel::Tiny => "tiny".to_string(),
                WhisperModel::TinyEn => "tiny.en".to_string(),
                WhisperModel::Base => "base".to_string(),
                WhisperModel::BaseEn => "base.en".to_string(),
                WhisperModel::Small => "small".to_string(),
                WhisperModel::SmallEn => "small.en".to_string(),
                WhisperModel::SmallEnTdrz => "small.en-tdrz".to_string(),
                WhisperModel::Medium => "medium".to_string(),
                WhisperModel::MediumEn => "medium.en".to_string(),
                WhisperModel::LargeV1 => "large-v1".to_string(),
                WhisperModel::LargeV2 => "large-v2".to_string(),
                WhisperModel::LargeV2Q => "large-v2-q5_0".to_string(),
                WhisperModel::LargeV3 => "large-v3".to_string(),
                WhisperModel::LargeV3Q => "large-v3-q5_0".to_string(),
            }
        )
    }
}

#[derive(Clone, Debug, ValueEnum)]
enum Format {
    #[clap(name = "srt")]
    Srt,
    #[clap(name = "txt")]
    Txt,
    #[clap(name = "vtt")]
    Vtt,
    #[clap(name = "lrc")]
    Lrc,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Format::Srt => "-osrt".to_string(),
                Format::Txt => "-otxt".to_string(),
                Format::Vtt => "-ovtt".to_string(),
                Format::Lrc => "-olrc".to_string(),
            }
        )
    }
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Transcribe media file to text using target whisper.cpp model
    Transcribe {
        /// The whisper.cpp model to use
        #[clap(long, default_value = "large-v3-q5_0")]
        model: WhisperModel,
        /// The input media file to transcribe
        #[clap(long)]
        input: PathBuf,
        /// The input media file to transcribe
        #[clap(long, default_value = "srt")]
        format: Format,
    },
    /// Print information about the CLI and the environment
    Info,
}

fn main() -> Result<()> {
    sh::check_dependencies()?;

    let cli = Cli::parse();

    match cli.command {
        Command::Transcribe {
            model,
            input,
            format,
        } => {
            sh::setup(&model)?;
            sh::transcribe(input, env::current_dir()?, &model, &format)?;
        }
        Command::Info => {
            println!("Directories:");
            println!("  whisper.cpp: {}", dirs::repository()?.to_string_lossy());
        }
    }

    Ok(())
}
