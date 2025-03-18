use crate::error::Error;

/// Updates the local rules build if necessary
pub fn update() -> Result<(), Error> { Ok(()) }

/// Checks remote if there is a newer version available
pub fn check_update() -> Result<bool, Error> { Ok(true) }

/// Downloads the remote repo
fn download() -> Result<(), Error> { Ok(()) }

/// Unpacks and builds the fetched files
fn build() -> Result<(), Error> { Ok(()) }
