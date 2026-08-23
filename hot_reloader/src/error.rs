//! The crate's unified error type.
//!
//! Every fallible operation returns [`LiveCodeError`] instead of a bare
//! `String`, so callers can match on the failure kind and the message always
//! carries the context needed to fix it.

use std::fmt;
use std::path::PathBuf;

/// The crate's unified error type.  Every variant carries the context needed
/// to diagnose the failure without hunting through logs.
#[derive(Debug)]
pub enum LiveCodeError {
    /// The [`LiveCodeSessionConfig`](crate::LiveCodeSessionConfig) is invalid
    /// (missing directories, empty build command, ...).
    InvalidConfig(String),
    /// The configured build command (`cargo build ...`) failed.
    BuildFailed {
        /// The build tool's captured stderr (and stdout when non-empty).
        stderr: String,
    },
    /// Copying the freshly built DLL to a unique versioned name failed.
    CopyFailed {
        /// The freshly built DLL that could not be copied.
        from: PathBuf,
        /// The versioned destination path.
        to: PathBuf,
        /// The underlying I/O error.
        source: std::io::Error,
    },
    /// `LoadLibrary` failed for a freshly built or patch DLL.
    LoadFailed {
        /// The library file that failed to load.
        path: PathBuf,
        /// The loader's error message.
        message: String,
    },
    /// `rustc` rejected the generated patch module.
    RustcFailed {
        /// rustc's captured stderr.
        stderr: String,
    },
    /// A required exported symbol is missing from a loaded DLL.
    SymbolMissing {
        /// The symbol name (e.g. `project_set_api`).
        name: String,
    },
    /// A prologue patch could not be applied.
    PatchFailed {
        /// The base function address whose prologue was being patched.
        address: usize,
        /// Why the patch failed.
        message: String,
    },
    /// Reading a source file (or walking the source tree) failed.
    SourceIo {
        /// The file that could not be read.
        path: PathBuf,
        /// The underlying I/O error.
        source: std::io::Error,
    },
}

impl fmt::Display for LiveCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiveCodeError::InvalidConfig(why) => {
                write!(f, "invalid hot_reloader configuration: {why}")
            }
            LiveCodeError::BuildFailed { stderr } => write!(f, "build failed:\n{stderr}"),
            LiveCodeError::CopyFailed { from, to, source } => {
                write!(
                    f,
                    "cannot copy {} to {}: {source}",
                    from.display(),
                    to.display()
                )
            }
            LiveCodeError::LoadFailed { path, message } => {
                write!(f, "LoadLibrary failed for {}: {message}", path.display())
            }
            LiveCodeError::RustcFailed { stderr } => write!(f, "rustc failed:\n{stderr}"),
            LiveCodeError::SymbolMissing { name } => {
                write!(f, "missing exported symbol `{name}`")
            }
            LiveCodeError::PatchFailed { address, message } => {
                write!(f, "prologue patch at {address:#x} failed: {message}")
            }
            LiveCodeError::SourceIo { path, source } => {
                write!(f, "cannot read {}: {source}", path.display())
            }
        }
    }
}

impl std::error::Error for LiveCodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LiveCodeError::CopyFailed { source, .. } | LiveCodeError::SourceIo { source, .. } => {
                Some(source)
            }
            _ => None,
        }
    }
}

/// The crate-wide result alias.
pub type Result<T> = std::result::Result<T, LiveCodeError>;
