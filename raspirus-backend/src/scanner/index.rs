use std::path::{Path, PathBuf};

use log::{trace, warn};

type Error = crate::Error;

/// Holds all the paths of files, which are contained in a root path
#[derive(Default)]
pub struct Index {
    /// All paths indexes
    pub paths: Vec<PathBuf>,
    /// Total size in bytes
    pub total_size: usize,
}

impl Index {
    /// Creates a new struct which contains every child file of root
    pub fn new(root: PathBuf) -> Result<Self, Error> {
        let mut indexed = Self::default();
        if root.is_dir() {
            indexed.index_folder(&root)?;
        }

        if root.is_file() {
            indexed.index_file(&root)?;
        }

        Ok(indexed)
    }

    /// Tries to add a file path to the path list
    fn index_file(&mut self, root: &Path) -> Result<(), Error> {
        // checks if file exists
        if !root.exists() {
            Err(Error::IndexFileNotFound(root.display().to_string()))?
        }

        // checks if permissions suffice, and if file is is_symlink
        let metadata = root.metadata().map_err(|error| Error::IndexPermission {
            path: root.display().to_string(),
            error,
        })?;

        if metadata.is_symlink() {
            Err(Error::IndexSymlink(root.display().to_string()))?
        }

        self.total_size += metadata.len() as usize;
        self.paths.push(root.to_path_buf());
        Ok(())
    }

    /// Tries to add all of a folders subfolders / files to the list of paths
    fn index_folder(&mut self, root: &Path) -> Result<(), Error> {
        // go through all children of a folder
        for entry in
            std::fs::read_dir(root).map_err(|_| Error::IndexEntries(root.display().to_string()))?
        {
            // if we can fetch entry, save its path, otherwise log and skip
            let root = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    warn!("Failed to get entry: {err:?}; Skipping...");
                    continue;
                }
            }
            .path();

            // if entry is file, index it, otherwise index it as folder, or, if neither applies,
            // skip
            if root.is_file() {
                match self.index_file(&root) {
                    Ok(_) => trace!("Indexed {}", root.display()),
                    Err(err) => warn!("{err}; Skipping..."),
                }
            } else if !root.is_symlink() {
                match self.index_folder(&root) {
                    Ok(_) => trace!("Indexed {}", root.display()),
                    Err(err) => warn!("{err}; Skipping..."),
                }
            } else {
                warn!("{} is a symlink; Skipping...", root.display());
            }
        }
        Ok(())
    }
}
