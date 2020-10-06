use std::process::Command;
use anyhow::Result;
use std::fs::File;
use std::path::PathBuf;
use std::io::{Write};

pub const TRACK_FILENAME: &str = ".pfstrack";
pub const IGNORE_START: &str = "#>pfs";

pub fn get_git_toplevel() -> Result<PathBuf> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?;
    if !output.status.success() {
        bail!("Calling git failed")
    }

    if let Some(line) = String::from_utf8(output.stdout)?.lines().next() {
        return Ok(PathBuf::from(line))
    }
    bail!("Could not obtain top level directory")
}

pub fn gitignore_location() -> Result<PathBuf> {
    let path = get_git_toplevel()?.join(".gitignore");
    if !path.exists() {
        let mut file = File::create(&path)?;
        writeln!(file, "{}", IGNORE_START)?;
        writeln!(file, "{}", TRACK_FILENAME)?;
        file.flush()?;
    }
    Ok(path)
}
pub fn trackfile_location() -> Result<PathBuf> {
    let path = get_git_toplevel()?.join(TRACK_FILENAME);
    if !path.exists() {
        File::create(&path)?;
    }
    Ok(path)
}

pub fn get_storage_dir() -> Result<PathBuf> {
    let output = Command::new("git")
        .arg("config")
        .arg("pfs.storage")
        .output()?;
    if !output.status.success() {
        bail!("Could not access pfs.storage. Is the storage set? Try:\ngit config pfs.storage /absolute/path/to/storage")
    }

    if let Some(line) = String::from_utf8(output.stdout)?.lines().next() {
        let storage_dir = PathBuf::from(line);
        if !storage_dir.exists() {
            bail!("The storage directory is set but does not exist or cannot be accessed")
        }
        return Ok(storage_dir)
    }
    bail!("Could not obtain storage directory")
}

/// Returns the path of `filename` that should be placed inside the .gitignore
pub fn as_ignore_filename(filename: String) -> Result<String> {
    let git_dir = get_git_toplevel()?;
    let path = PathBuf::from(filename).canonicalize()?;
    let stripped = path.strip_prefix(git_dir)?;
    match PathBuf::from(stripped).into_os_string().into_string() {
        Ok(filename) => Ok(filename),
        Err(_) => {
            bail!("Path conversion failed")
        }
    }
}
