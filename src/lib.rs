use std::path::Path;

#[cfg(test)]
mod tests;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod platform;

#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod platform;

#[cfg(target_os = "macos")]
#[path = "macos.rs"]
mod platform;

/// Error that might happen during a remove operation.
#[derive(Debug)]
pub enum Error {
    Unknown,

    /// Error while canonicalizing path
    /// `code` contains a raw os error code is accessible.
    CanonicalizePath {
        code: Option<i32>,
    },

    /// Error while performing the remove operation
    /// `code` contains a raw os error code is accessible.
    Remove {
        code: Option<i32>,
    },
}

/// Removes a single file.
/// 
/// # Example
/// 
/// ```
/// extern crate trash;
/// use std::fs::File;
/// use trash::remove;
/// File::create("remove_me").unwrap();
/// trash::remove("remove_me").unwrap();
/// assert!(File::open("remove_me").is_err());
/// ```
pub fn remove<T: AsRef<Path>>(path: T) -> Result<(), Error> {
    platform::remove(path)
}

/// Removes all files specified by the collection of paths provided as an argument.
/// 
/// # Example
/// 
/// ```
/// extern crate trash;
/// use std::fs::File;
/// use trash::remove_all;
/// File::create("remove_me_1").unwrap();
/// File::create("remove_me_2").unwrap();
/// remove_all(&["remove_me_1", "remove_me_2"]).unwrap();
/// assert!(File::open("remove_me_1").is_err());
/// assert!(File::open("remove_me_2").is_err());
/// ```
pub fn remove_all<I, T>(paths: I) -> Result<(), Error>
where
    I: IntoIterator<Item = T>,
    T: AsRef<Path>,
{
    platform::remove_all(paths)
}

/// Returns true if the functions are implemented on the current platform.
pub fn is_implemented() -> bool {
    platform::is_implemented()
}