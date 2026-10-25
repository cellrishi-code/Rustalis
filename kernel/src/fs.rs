//! Fixed-capacity in-memory filesystem primitives for early Rustalis bring-up.
//!
//! This is a RAM-backed prototype, not persistent storage. It deliberately avoids
//! allocation so it can be used before the kernel heap and block-device layer exist.

const MAX_PATH_BYTES: usize = 64;
const MAX_FILE_BYTES: usize = 256;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FsError {
    InvalidPath,
    PathTooLong,
    FileTooLarge,
    AlreadyExists,
    NotFound,
    NoFreeSlots,
}

/// A small, allocation-free filesystem suitable for kernel bring-up.
pub struct RamFs<const FILES: usize> {
    entries: [Entry; FILES],
}

#[derive(Clone, Copy)]
struct Entry {
    used: bool,
    path: [u8; MAX_PATH_BYTES],
    path_len: usize,
    data: [u8; MAX_FILE_BYTES],
    data_len: usize,
}

impl Entry {
    const EMPTY: Self = Self {
        used: false,
        path: [0; MAX_PATH_BYTES],
        path_len: 0,
        data: [0; MAX_FILE_BYTES],
        data_len: 0,
    };

    fn path(&self) -> &str {
        // Paths are validated as UTF-8 before they are stored.
        core::str::from_utf8(&self.path[..self.path_len])
            .expect("stored filesystem path must be valid UTF-8")
    }
}

impl<const FILES: usize> RamFs<FILES> {
    pub const fn new() -> Self {
        Self {
            entries: [Entry::EMPTY; FILES],
        }
    }

    /// Create an empty file at an absolute path.
    pub fn create(&mut self, path: &str) -> Result<(), FsError> {
        let path = validate_path(path)?;
        if self.find(path).is_some() {
            return Err(FsError::AlreadyExists);
        }
        let slot = self
            .entries
            .iter()
            .position(|entry| !entry.used)
            .ok_or(FsError::NoFreeSlots)?;
        let entry = &mut self.entries[slot];
        entry.path[..path.len()].copy_from_slice(path.as_bytes());
        entry.path_len = path.len();
        entry.data_len = 0;
        entry.used = true;
        Ok(())
    }

    /// Replace a file's contents. Fails without modifying the file if too large.
    pub fn write(&mut self, path: &str, bytes: &[u8]) -> Result<(), FsError> {
        let path = validate_path(path)?;
        if bytes.len() > MAX_FILE_BYTES {
            return Err(FsError::FileTooLarge);
        }
        let index = self.find(path).ok_or(FsError::NotFound)?;
        let entry = &mut self.entries[index];
        entry.data[..bytes.len()].copy_from_slice(bytes);
        entry.data_len = bytes.len();
        Ok(())
    }

    /// Read a file as bytes; contents need not be UTF-8.
    pub fn read(&self, path: &str) -> Result<&[u8], FsError> {
        let path = validate_path(path)?;
        let index = self.find(path).ok_or(FsError::NotFound)?;
        let entry = &self.entries[index];
        Ok(&entry.data[..entry.data_len])
    }

    /// Remove a file and release its slot.
    pub fn remove(&mut self, path: &str) -> Result<(), FsError> {
        let path = validate_path(path)?;
        let index = self.find(path).ok_or(FsError::NotFound)?;
        self.entries[index] = Entry::EMPTY;
        Ok(())
    }

    /// Return the path of the Nth occupied slot, for simple directory listings.
    pub fn path_at(&self, occupied_index: usize) -> Option<&str> {
        self.entries
            .iter()
            .filter(|entry| entry.used)
            .nth(occupied_index)
            .map(Entry::path)
    }

    pub fn file_count(&self) -> usize {
        self.entries.iter().filter(|entry| entry.used).count()
    }

    fn find(&self, path: &str) -> Option<usize> {
        self.entries
            .iter()
            .position(|entry| entry.used && entry.path() == path)
    }
}

fn validate_path(path: &str) -> Result<&str, FsError> {
    if !path.starts_with('/') || path == "/" || path.ends_with('/') {
        return Err(FsError::InvalidPath);
    }
    if path.len() > MAX_PATH_BYTES {
        return Err(FsError::PathTooLong);
    }
    if path
        .split('/')
        .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(FsError::InvalidPath);
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::{FsError, RamFs};

    #[test]
    fn create_write_read_and_remove() {
        let mut fs = RamFs::<4>::new();
        fs.create("/hello.txt").unwrap();
        fs.write("/hello.txt", b"Rustalis").unwrap();
        assert_eq!(fs.read("/hello.txt").unwrap(), b"Rustalis");
        assert_eq!(fs.file_count(), 1);
        fs.remove("/hello.txt").unwrap();
        assert_eq!(fs.read("/hello.txt"), Err(FsError::NotFound));
        assert_eq!(fs.file_count(), 0);
    }

    #[test]
    fn rejects_traversal_and_duplicate_paths() {
        let mut fs = RamFs::<2>::new();
        assert_eq!(fs.create("/../secret"), Err(FsError::InvalidPath));
        fs.create("/note.txt").unwrap();
        assert_eq!(fs.create("/note.txt"), Err(FsError::AlreadyExists));
    }

    #[test]
    fn reports_capacity_and_size_errors_without_corrupting_data() {
        let mut fs = RamFs::<1>::new();
        fs.create("/a").unwrap();
        assert_eq!(fs.create("/b"), Err(FsError::NoFreeSlots));
        fs.write("/a", b"ok").unwrap();
        assert_eq!(fs.write("/a", &[0; 257]), Err(FsError::FileTooLarge));
        assert_eq!(fs.read("/a").unwrap(), b"ok");
    }
}
