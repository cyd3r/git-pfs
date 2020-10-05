use anyhow::Result;
use crate::paths::{gitignore_location, trackfile_location, get_storage_dir, get_git_toplevel, TRACK_FILENAME, IGNORE_START};
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;

pub fn synchronize() -> Result<()> {
    let storage_dir = get_storage_dir()?;
    let git_toplevel_dir = get_git_toplevel()?;

    // TODO: use an ordered hash map
    let mut hashes = BTreeMap::new();
    let mut ignore_lines_before = Vec::new();
    let mut ignore_lines_after = Vec::new();

    // ignore file
    let ignore_file = File::open(gitignore_location()?)?;

    enum IgnoreRegion {
        BeforeLinked,
        Linked,
        AfterLinked,
    }
    let mut region = IgnoreRegion::BeforeLinked;

    let mut last_filename: String = String::new();
    for line in BufReader::new(ignore_file).lines() {
        let line = line?;
        match region {
            IgnoreRegion::BeforeLinked => {
                if line == IGNORE_START {
                    region = IgnoreRegion::Linked;
                }
                ignore_lines_before.push(line);
            },
            IgnoreRegion::Linked => {
                if line == TRACK_FILENAME {
                    region = IgnoreRegion::AfterLinked;
                    ignore_lines_after.push(line);
                }
                else if line.starts_with("# ") {
                    let hash = &line[2..];
                    hashes.insert(last_filename.to_string(), hash.to_string());
                }
                else {
                    last_filename = line.to_string();
                }
            },
            IgnoreRegion::AfterLinked => {
                ignore_lines_after.push(line);
            },
        }
    }

    // track file
    let track_file = File::open(trackfile_location()?)?;
    for filename in BufReader::new(track_file).lines() {
        let filename = filename?;
        let path = git_toplevel_dir.join(&filename);
        if hashes.contains_key(&filename) {
            if !path.exists() {
                hashes.remove(&filename);
            }
        }
        else if path.exists() {
            std::fs::remove_file(path)?;
        }
    }

    for (filename, hash) in hashes.iter() {
        let path = git_toplevel_dir.join(filename);
        if !path.exists() {
            std::fs::copy(storage_dir.join(hash), path)?;
        }
    }

    // write ignore file
    let mut ignore_file = File::create(gitignore_location()?)?;
    for line in ignore_lines_before {
        writeln!(ignore_file, "{}", line)?;
    }
    for (filename, hash) in hashes.iter() {
        writeln!(ignore_file, "{}", filename)?;
        writeln!(ignore_file, "# {}", hash)?;
    }
    for line in ignore_lines_after {
        writeln!(ignore_file, "{}", line)?;
    }
    ignore_file.flush()?;

    // write track file
    let mut track_file = File::create(trackfile_location()?)?;
    for filename in hashes.keys() {
        writeln!(track_file, "{}", filename)?;
    }
    track_file.flush()?;

    Ok(())
}
