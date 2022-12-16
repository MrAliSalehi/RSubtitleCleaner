#![allow(non_snake_case)]
extern crate walkdir;

use std::env;
use std::path::{Path, PathBuf};

use colored::Colorize;
use walkdir::{DirEntry, WalkDir};

fn main() {
    let currentDir = GetStartingPath();

    let mut subtitlesInDir = Vec::new();
    GetSubtitlesInDirectory(&currentDir, &mut subtitlesInDir);

    println!("{} '{}'", "Current Dir:".green(), currentDir);

    for subtitle in subtitlesInDir {
        MoveSubtitle(subtitle);
    }
}

fn GetStartingPath() -> String {
    let arg = env::args().nth(1);

    let currentDir;
    if let Some(..) = arg {
        currentDir = env::current_dir().unwrap().to_str().unwrap().to_string();
    } else {
        currentDir = arg.unwrap();
    }
    currentDir
}

fn GetSubtitlesInDirectory(dir: &str, outList: &mut Vec<DirEntry>) {
    for file in WalkDir::new(dir)
        .into_iter().filter_map(|f| f.ok()).filter(|f| f.metadata().unwrap().is_file()) {
        let fileName = file.file_name().to_str().unwrap();
        if fileName.ends_with(".vtt") || fileName.ends_with(".srt") {
            outList.push(file);
        }
    }
}

fn MoveSubtitle(subtitle: DirEntry) {
    let subtitlePathBuf = PathBuf::from(subtitle.path());

    let parent = subtitlePathBuf.parent().expect("could not find the parent directory.");
    let subtitleDirectoryInParentDir = format!("{}/subs", parent.to_str().unwrap());

    if !Path::exists(Path::new(&subtitleDirectoryInParentDir)) {
        std::fs::create_dir(&subtitleDirectoryInParentDir).expect("could not create directory in parent Dir.");
    }
    let fileName = subtitlePathBuf.file_name().unwrap().to_str().unwrap();

    let subtitleFullPath = subtitlePathBuf.to_str().unwrap();

    let errCopyFile = format!("cant copy the file:[{}]", fileName).red();
    std::fs::copy(subtitleFullPath, format!("{}/{}", &subtitleDirectoryInParentDir, fileName))
        .expect(&errCopyFile);

    let errRemoveFile = format!("could not remove the subtitle: {}", &subtitleFullPath).red();
    std::fs::remove_file(subtitleFullPath)
        .expect(&errRemoveFile);
}



