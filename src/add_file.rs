use anyhow::Result;
use crate::paths::{get_storage_dir, gitignore_location, trackfile_location, as_ignore_filename, TRACK_FILENAME, IGNORE_START};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn add_gitignore(filename: &String, hash: String) -> Result<()> {
    let file = File::open(gitignore_location()?)?;
    let mut new_lines: Vec<String> = Vec::new();

    let mut is_linked_region = false;
    let mut skip_line = false;
    let mut inserted = false;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if skip_line {
            skip_line = false;
            continue;
        }
        if is_linked_region {
            if line == TRACK_FILENAME {
                is_linked_region = false;
                if !inserted {
                    // the file does not exist here yet, add a new entry
                    new_lines.push(filename.to_string());
                    new_lines.push(format!("# {}", hash));
                    inserted = true;
                }
            }
            else if line == filename.to_string() {
                skip_line = true;
                new_lines.push(filename.to_string());
                new_lines.push(format!("# {}", hash));
                inserted = true;
                continue;
            }
        }
        else if line.starts_with(IGNORE_START) {
            is_linked_region = true;
        }
        new_lines.push(line);
    }

    // does the PFS region exist?
    if !inserted {
        new_lines.push(IGNORE_START.to_string());
        new_lines.push(filename.to_string());
        new_lines.push(format!("# {}", hash));
        new_lines.push(TRACK_FILENAME.to_string());
        println!("Created the PFS section in the .gitignore for you. Do not edit any lines between '{}' and '{}'", IGNORE_START, TRACK_FILENAME);
    }

    let mut file = File::create(gitignore_location()?)?;
    for line in new_lines {
        writeln!(file, "{}", line)?;
    }
    file.flush()?;
    println!("File added to the storage. Don't forget to commit your .gitignore");
    Ok(())
}

fn add_tracker(filename: String) -> Result<()> {
    // check if the file already exists on the tracker
    let file = File::open(trackfile_location()?)?;
    let mut write_lines = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line?;
        if line == filename {
            return Ok(())
        }
        write_lines.push(line);
    }

    let mut file = File::create(trackfile_location()?)?;
    for line in write_lines {
        writeln!(file, "{}", line)?;
    }
    writeln!(file, "{}", filename)?;
    file.flush()?;
    Ok(())
}

pub fn add_file(filename: String) -> Result<()> {
    let storage_dir = get_storage_dir()?;

    // generate a hash of the file
    let mut file = File::open(&filename)?;
    let mut sha = Sha256::new();
    std::io::copy(&mut file, &mut sha)?;
    let hex = format!("{:x}", sha.finalize());

    // copy the file to the storage
    let target = storage_dir.join(&hex);
    std::fs::copy(&filename, target)?;

    let ignore_filename = as_ignore_filename(filename)?;
    // write it to .gitignore
    add_gitignore(&ignore_filename, hex)?;
    // write it to the local track file
    add_tracker(ignore_filename)?;
    Ok(())
}
