use clap::Parser;

/// Clean your craps **specially subtitles**
#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// starting path
    #[arg(short, long, default_value_t = std::env::current_dir().unwrap().display().to_string())]
    pub path: String,

    /// extensions to scan for
    #[arg(short, long, num_args = 1.., default_values = ["srt", "vtt"])]
    pub extensions: Vec<String>,
}
