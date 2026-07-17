use std::path::PathBuf;

use eyre::{Context, Result, bail};
use privilege::user::privileged;
use rand::{RngExt, distr::Alphanumeric};

/// Create temp folder and return path
pub fn temp_folder(
    prefix: &str,
    suffix: &str,
    rand_len: usize,
) -> Result<PathBuf> {
    let mut rng = rand::rng();
    let random_string: String = (0..rand_len)
        .map(|_| char::from(rng.sample(Alphanumeric)))
        .collect();
    let name = format!("{prefix}{random_string}{suffix}");
    let tmp = std::env::temp_dir();
    let temp_path = tmp.join(name);
    std::fs::create_dir(temp_path.clone())?;
    log::trace!("created dir {}", temp_path.clone().display());
    Ok(temp_path)
}

/// dst should be directory
pub fn shadow_copy(src: PathBuf, dst: PathBuf) -> Result<()> {
    if !src.exists() {
        bail!("Source file not exists: {}", src.clone().display())
    }
    if !privileged() {
        bail!("No admin rights")
    }
    log::info!(
        "Creating shadow copy to cookies file from {} to {}",
        src.display(),
        dst.display()
    );
    rawcopy_rs_next::rawcopy(
        src.clone().to_str().unwrap(),
        dst.to_str().unwrap(),
    )
    .map_err(|err| eyre::eyre!(Box::new(err)))
    .context(format!(
        "Can't shadow copy from {} to {}",
        src.display(),
        dst.display(),
    ))?;

    Ok(())
}
