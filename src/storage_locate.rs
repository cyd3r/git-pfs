use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::paths::{gitignore_location, get_storage_dir, TRACK_FILENAME, IGNORE_START};

pub fn locate(local_file: String) -> Result<()> {
    let storage_dir = get_storage_dir()?;
    let file = File::open(gitignore_location()?)?;

    let mut is_linked_region = false;
    let mut return_hash = false;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if return_hash {
            let hash = line.trim_start_matches(&['#', ' '][..]);
            match storage_dir.join(hash).into_os_string().into_string() {
                Ok(path) => {
                    println!("{}", path);
                    return Ok(());
                },
                Err(_) => bail!("Path conversion failed"),
            }
        }
        if is_linked_region {
            if line == TRACK_FILENAME {
                bail!("File is not present")
            }
            else if line == local_file {
                return_hash = true;
            }
        }
        else if line.starts_with(IGNORE_START) {
            is_linked_region = true;
        }
    }
    bail!("File is not present");
}
