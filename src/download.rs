use std::{fs::File, path::Path};

use anyhow::Result;

pub fn download_file(url: &str, filepath: &Path) -> Result<File> {
    let mut resp = reqwest::blocking::get(url)?;
    let mut out = File::create(filepath)?;
    std::io::copy(&mut resp, &mut out)?;
    Ok(out)
}