use std::{
    fs,
    io,
    path::{Component, Path, PathBuf},
};

#[derive(Debug)]
pub enum FilesystemError {
    InvalidPath,
    OutsideProject,
    Io(io::Error),
}

impl From<io::Error> for FilesystemError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

pub struct ProjectFilesystem {
    root: PathBuf,
}

impl ProjectFilesystem {
    pub fn new(root: &Path) -> io::Result<Self> {
        let root = fs::canonicalize(root)?;
        if !root.is_dir() {
            return Err(io::Error::new(io::ErrorKind::NotADirectory, "project root is not a directory"));
        }
        Ok(Self { root })
    }

    pub fn list(&self) -> Result<Vec<FileEntry>, FilesystemError> {
        let mut entries = Vec::new();
        self.list_directory(&self.root, &mut entries)?;
        entries.sort_by(|left, right| left.path.cmp(&right.path));
        Ok(entries)
    }

    pub fn read_file(&self, path: &str) -> Result<String, FilesystemError> {
        Ok(fs::read_to_string(self.resolve_existing(path)?)?)
    }

    pub fn write_file(&self, path: &str, content: &str) -> Result<(), FilesystemError> {
        let relative = validate_relative_path(path)?;
        let target = self.root.join(&relative);
        let parent = target.parent().ok_or(FilesystemError::InvalidPath)?;
        let canonical_parent = fs::canonicalize(parent)?;
        if !canonical_parent.starts_with(&self.root) {
            return Err(FilesystemError::OutsideProject);
        }

        let file_name = target.file_name().ok_or(FilesystemError::InvalidPath)?;
        let resolved_target = if target.exists() {
            let resolved = fs::canonicalize(&target)?;
            if !resolved.starts_with(&self.root) {
                return Err(FilesystemError::OutsideProject);
            }
            resolved
        } else {
            canonical_parent.join(file_name)
        };
        fs::write(resolved_target, content)?;
        Ok(())
    }

    fn list_directory(&self, directory: &Path, entries: &mut Vec<FileEntry>) -> Result<(), FilesystemError> {
        for item in fs::read_dir(directory)? {
            let item = item?;
            let file_type = item.file_type()?;
            let path = item.path();
            let relative = path.strip_prefix(&self.root).map_err(|_| FilesystemError::OutsideProject)?;
            let relative_path = relative.to_string_lossy().replace('\\', "/");

            if file_type.is_dir() {
                entries.push(FileEntry { name: item.file_name().to_string_lossy().into_owned(), path: relative_path, entry_type: EntryType::Directory });
                self.list_directory(&path, entries)?;
            } else if file_type.is_file() {
                entries.push(FileEntry { name: item.file_name().to_string_lossy().into_owned(), path: relative_path, entry_type: EntryType::File });
            }
        }
        Ok(())
    }

    fn resolve_existing(&self, path: &str) -> Result<PathBuf, FilesystemError> {
        let relative = validate_relative_path(path)?;
        let resolved = fs::canonicalize(self.root.join(relative))?;
        if !resolved.starts_with(&self.root) {
            return Err(FilesystemError::OutsideProject);
        }
        if !resolved.is_file() {
            return Err(FilesystemError::Io(io::Error::new(io::ErrorKind::InvalidInput, "path is not a file")));
        }
        Ok(resolved)
    }
}

#[derive(Debug)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub entry_type: EntryType,
}

#[derive(Debug)]
pub enum EntryType {
    File,
    Directory,
}

fn validate_relative_path(path: &str) -> Result<PathBuf, FilesystemError> {
    let path = Path::new(path);
    if path.is_absolute() || path.as_os_str().is_empty() {
        return Err(FilesystemError::InvalidPath);
    }
    for component in path.components() {
        if matches!(component, Component::ParentDir | Component::RootDir | Component::Prefix(_)) {
            return Err(FilesystemError::InvalidPath);
        }
    }
    Ok(path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_filesystem() -> (ProjectFilesystem, tempfile::TempDir) {
        let temp_dir = tempfile::tempdir().unwrap();
        let root = temp_dir.path();
        fs::create_dir_all(root.join("chapters")).unwrap();
        fs::write(root.join("main.tex"), "main").unwrap();
        (ProjectFilesystem::new(root).unwrap(), temp_dir)
    }

    #[test]
    fn lists_files_reads_and_writes() {
        let (filesystem, temp_dir) = test_filesystem();
        let entries = filesystem.list().unwrap();
        assert!(entries.iter().any(|entry| entry.path == "main.tex"));
        assert!(entries.iter().any(|entry| entry.path == "chapters"));
        assert_eq!(filesystem.read_file("main.tex").unwrap(), "main");
        filesystem.write_file("main.tex", "updated").unwrap();
        assert_eq!(fs::read_to_string(temp_dir.path().join("main.tex")).unwrap(), "updated");
    }

    #[test]
    fn rejects_traversal_and_absolute_paths() {
        let (filesystem, _temp_dir) = test_filesystem();
        assert!(matches!(filesystem.read_file("../outside.tex"), Err(FilesystemError::InvalidPath)));
        assert!(matches!(filesystem.write_file("/tmp/outside.tex", "x"), Err(FilesystemError::InvalidPath)));
    }
}