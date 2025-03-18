use std::path::{Path, PathBuf};

use log::{trace, warn};

struct Index(Vec<PathBuf>);

impl Index {
    fn new(root: PathBuf) -> Result<Self, String> {
        let mut indexed = Self(Vec::new());
        if root.is_dir() {
            indexed.index_folder(&root)?;
        }

        if root.is_file() {
            indexed.index_file(&root)?;
        }

        Ok(indexed)
    }

    /// Tries to add a file path to the path list
    fn index_file(&mut self, root: &Path) -> Result<(), String> {
        // checks if file exists
        if !root.exists() {
            Err(format!("File {} does not exist", root.display()))?
        }

        // checks if permissions suffice, and if file is symlink
        if root
            .metadata()
            .map_err(|err| format!("Failed to get metadata: {err:?}"))?
            .is_symlink()
        {
            Err(format!("File {} is a symlink", root.display()))?
        }

        self.0.push(root.to_path_buf());
        Ok(())
    }

    /// Tries to add all of a folders subfolders / files to the list of paths
    fn index_folder(&mut self, root: &Path) -> Result<(), String> {
        // go through all children of a folder
        for entry in std::fs::read_dir(root).map_err(|err| {
            format!(
                "Failed to get folder entries for {}: {err:?}",
                root.display()
            )
        })? {
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

#[derive(Debug)]
pub struct Scanner();

impl Scanner {
    pub fn new(root: PathBuf) -> Result<Self, String> {
        let indexed = Index::new(root)?;
        dbg!(indexed.0.len());
        Ok(Self {})
    }
}
