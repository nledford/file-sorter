use std::{env, fs};
use std::path::Path;
use std::time::UNIX_EPOCH;

use anyhow::Result;
use chrono::NaiveDateTime;
use walkdir::DirEntry;

pub fn build_new_path(entry: &DirEntry) -> Result<String> {
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

fn convert_systemtime_to_naivedatetime(entry: &DirEntry) -> Result<NaiveDateTime> {
    let created_date = entry.metadata()?.created()?;

    let date = created_date.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let date = NaiveDateTime::from_timestamp(date as i64, 0);

    Ok(date)
}

/// Converts a `NaiveDateTime` to a file path
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

/// Extracts a directory if provided, otherwise returns the current working directory
pub fn get_dir(dir: Option<String>) -> Result<String> {
    let result = match dir {
        Some(dir) => dir,
        None => env::current_dir()?
            .into_os_string()
            .into_string()
            .expect("Could not get current working directory"),
    };

    Ok(result)
}

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
