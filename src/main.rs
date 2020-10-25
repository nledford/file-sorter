use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

use anyhow::Result;
use chrono::prelude::*;
use clap::Clap;
use walkdir::{DirEntry, WalkDir};

use file_sorter::cli::{Opts, SubCommand};

fn main() -> Result<()> {
    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Sort(sort) => {
            sort_dir(&sort.dir)?;
        }
        SubCommand::Append(append) => {
            append_dates(&append.dir)?;
        }
    }

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

fn append_dates(dir: &str) -> anyhow::Result<()> {
    println!("Gathering files...");

    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            continue;
        }

        let path = entry.path().to_str().unwrap();
        let file_name = entry.file_name().to_str().unwrap();

        // Build date string
        let date = metadata.created().unwrap();
        let date: DateTime<Utc> = date.into();
        let date = date.format("[%Y-%m-%d]").to_string();

        // Build new file name
        let new_file_name = format!("{} {}", date, file_name);
        let after = path.replace(file_name, new_file_name.as_str());

        // if file has already been renamed, skip it
        if path.contains(&date) {
            continue;
        }

        // Assume file has not been renamed

        println!("BEFORE: {}", &path);
        println!("AFTER: {}\n", after);

        std::fs::rename(path, after)?;
    }

    Ok(())
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_to_file_path() {
        let date = Local.ymd(1984, 4, 12).and_hms(0, 0, 0).naive_local();
        let path = "1984/04/12".to_string();

        let test_path = date_to_file_path(&date).unwrap();

        assert_eq!(path, test_path);
    }
}
