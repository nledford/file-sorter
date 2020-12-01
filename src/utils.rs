use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use std::{env, fs, time};

use anyhow::Result;
use chrono::prelude::*;
use walkdir::DirEntry;

const SECONDS_IN_DAY: i64 = 86400;

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

pub fn generate_random_dated_folder_path() -> Result<PathBuf> {
    use rand::distributions::{Distribution, Uniform};

    let mut rng = rand::thread_rng();

    let floor = Local.ymd(2010, 1, 1).and_hms(0, 0, 0);

    let ceiling_year = Local::now().year();
    let ceiling = Local.ymd(ceiling_year, 12, 31);

    let range = ceiling.signed_duration_since(floor.date()).num_days() + 1;
    let range = Uniform::from(0..range);

    let duration = time::Duration::from_secs((range.sample(&mut rng) * SECONDS_IN_DAY) as u64);
    let duration = chrono::Duration::from_std(duration)?;

    let date = floor
        .checked_add_signed(duration)
        .expect("Error occurred while attempting to generate random date");

    let year = &date.format("%Y").to_string();
    let month = &date.format("%m").to_string();
    let day = &date.format("%d").to_string();

    let path = Path::new(year);
    let path = path.join(month);
    let path = path.join(day);

    Ok(path)
}
