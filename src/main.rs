use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

use anyhow::Result;
use chrono::prelude::*;
use clap::Clap;
use walkdir::{DirEntry, WalkDir};

use image_sorter::app::Opts;

fn main() -> Result<()> {
    let opts = Opts::parse();

    let dir: String = opts.dir;
    println!("DIR:{}", &dir);

    sort_dir(&dir)?;

    Ok(())
}

fn sort_dir(dir: &str) -> Result<()> {
    let mut files = Vec::new();

    let walker = WalkDir::new(dir);
    for entry in walker {
        let entry = entry?;
        let metadata = entry.metadata()?;

        // Skip directory entries
        if metadata.is_dir() {
            continue;
        }

        files.push(entry);
    }

    for entry in files {
        // Unwrapping because the path should never be `None`
        let current_path = entry.path().to_str().unwrap().to_string();
        let new_path = build_new_path(&entry)?;

        println!("CURRENT PATH: {}", &current_path);
        println!("NEW PATH:     {}", new_path);

        fs::rename(current_path, new_path)?;

        println!()
    }

    println!("Done!");

    Ok(())
}

fn convert_systemtime_to_naivedatetime(entry: &DirEntry) -> Result<NaiveDateTime> {
    let created_date = entry.metadata()?.created()?;

    let date = created_date.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let date = NaiveDateTime::from_timestamp(date as i64, 0);

    Ok(date)
}

fn build_new_path(entry: &DirEntry) -> Result<String> {
    let date = convert_systemtime_to_naivedatetime(entry)?;
    let date_path = date_to_file_path(&date)?;

    let new_path = entry
        .path()
        .to_str()
        .unwrap()
        .replace(entry.path().file_name().unwrap().to_str().unwrap(), "");
    let new_path = Path::new(&new_path).join(date_path);

    fs::create_dir_all(&new_path)?;

    let new_path = new_path.join(entry.path().file_name().unwrap().to_str().unwrap());
    let new_path = new_path.to_str().unwrap().to_string();

    Ok(new_path)
}

fn date_to_file_path(date: &NaiveDateTime) -> Result<String> {
    let year = date.format("%Y").to_string();
    let month = date.format("%m").to_string();
    let day = date.format("%d").to_string();

    let file_path = Path::new(year.as_str())
        .join(month)
        .join(day)
        .to_str()
        .unwrap()
        .to_string();

    Ok(file_path)
}
