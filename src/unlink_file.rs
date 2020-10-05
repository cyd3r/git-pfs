use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use crate::paths::{gitignore_location, trackfile_location, as_ignore_filename, TRACK_FILENAME, IGNORE_START};

fn rm_gitignore(filename: String) -> Result<()> {
    let file = File::open(gitignore_location()?)?;

    let mut write_lines = Vec::new();
    let mut is_linked_region = false;
    let mut skip_line = false;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if skip_line {
            skip_line = false;
            continue;
        }
        if is_linked_region {
            if line == TRACK_FILENAME {
                is_linked_region = false;
            }
            else if line == filename {
                skip_line = true;
                continue;
            }
        }
        else if line.starts_with(IGNORE_START) {
            is_linked_region = true;
        }
        write_lines.push(line);
    }

    // write back
    let mut file = File::create(gitignore_location()?)?;
    for line in write_lines {
        writeln!(file, "{}", line)?;
    }
    file.flush()?;
    Ok(())
}
fn rm_tracked(filename: String) -> Result<()> {
    let file = File::open(trackfile_location()?)?;

    let mut write_lines = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line?;
        if line != filename {
            write_lines.push(line);
        }
    }

    // write back
    let mut file = File::create(trackfile_location()?)?;
    for line in write_lines {
        writeln!(file, "{}", line)?;
    }
    file.flush()?;
    Ok(())
}

pub fn unlink_file(filename: String) -> Result<()> {
    let ignore_filename = as_ignore_filename(filename)?;
    rm_gitignore(ignore_filename.to_string())?;
    rm_tracked(ignore_filename)?;
    println!("File unlinked from the storage. Don't forget to commit your .gitignore");
    Ok(())
}
