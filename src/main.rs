#![allow(non_snake_case)]

extern crate walkdir;

use std::env;
use std::path::{Path, PathBuf};

use clap::Parser;
use colored::Colorize;
use walkdir::{DirEntry, WalkDir};

use crate::argument::Args;

mod argument;

/// starting point of the application.
///
/// get the [starting path](GetStartingPath).
///
/// get [all subtitles in that directory recursively](GetSubtitlesInDirectory).
///
/// **foreach** subtitle found in given directory, call [`MoveSubtitle`] function and pass dawn the subtitle [path](DirEntry)
fn main() {
    let args = Args::parse();

    let currentDir = GetStartingPath(args.path);

    let subtitlesInDir = GetSubtitlesInDirectory(&currentDir, args.extensions);

    println!("{} '{}'", "Current Dir:".green(), currentDir);

    for subtitle in subtitlesInDir {
        MoveSubtitle(subtitle);
    }
}

///
/// this will try to get the **starting path** _(the path that program will start to scanning for subtitles)_.
///  - if it doesnt get any argument, it will use the [**current directory**](env::current_dir()) `(root directory of the application)`
///  - pass an argument like this : `./SubtitleCleaner /path/to/clean/`
fn GetStartingPath(arg: String) -> String {
    return if String::is_empty(&arg) {
        env::current_dir().unwrap().to_str().unwrap().to_string()
    } else {
        arg
    };
}

/// this will [walk](WalkDir) through the given directory(`dir` argument) **recursively**.
/// - fetch all the files with these formats:
/// - - *.vtt
/// - - *.srt
/// **returns** a [`Vec<DirEntry>`] of all found subtitles.
fn GetSubtitlesInDirectory(dir: &str, extensions: Vec<String>) -> Vec<DirEntry> {
    let mut outList: Vec<DirEntry> = Vec::new();

    for file in WalkDir::new(dir)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| f.metadata().unwrap().is_file())
    {
        let fileName = file.file_name().to_str().unwrap();

        for extension in &extensions {
            if fileName.ends_with(extension) {
                outList.push(file);
                break;
            }
        }
    }
    outList
}

/// this will move the given subtitle(from path) to a sub-directory called **subs** in the parent directory.
///  - find the parent dir, consider this file:(`/path/to/my_subtitle1.srt`)
///  - - parent will be `/path/to`
///  - - **IF** it doesnt exists: **create** a directory in parent directory called **subs** : `/path/to/subs`
///  - - **copy** the subtitle to new directory : `/path/to/subs/my_subtitle1.srt`
///  - - **remove** the old file : rm `/path/to/my_subtitle1.srt`
fn MoveSubtitle(subtitle: DirEntry) {
    let subtitlePathBuf = PathBuf::from(subtitle.path());

    let parent = subtitlePathBuf
        .parent()
        .expect("could not find the parent directory.");
    let subtitleDirectoryInParentDir = format!("{}/subs", parent.to_str().unwrap());

    if !Path::exists(Path::new(&subtitleDirectoryInParentDir)) {
        std::fs::create_dir(&subtitleDirectoryInParentDir)
            .expect("could not create directory in parent Dir.");
    }
    let fileName = subtitlePathBuf.file_name().unwrap().to_str().unwrap();

    let subtitleFullPath = subtitlePathBuf.to_str().unwrap();

    let errCopyFile = format!("cant copy the file:[{}]", fileName).red();
    std::fs::copy(
        subtitleFullPath,
        format!("{}/{}", &subtitleDirectoryInParentDir, fileName),
    )
        .expect(&errCopyFile);

    let errRemoveFile = format!("could not remove the subtitle: {}", &subtitleFullPath).red();
    std::fs::remove_file(subtitleFullPath).expect(&errRemoveFile);
}
