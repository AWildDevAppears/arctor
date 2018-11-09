extern crate flate2;
extern crate tar;

use std::path::Path;
use std::fs::File;
use std::io::Error;



pub fn extract(path: &Path) -> Result<(), Error> {
    let tar_gz = File::open(path)?;
    let tar = flate2::read::GzDecoder::new(tar_gz);

    let mut archive = tar::Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}