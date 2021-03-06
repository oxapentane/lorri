//! Bootstrap a new lorri project

use crate::ops::error::{ok, ExitError, OpResult};
use slog_scope::info;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

fn create_if_missing(path: &Path, contents: &str, msg: &str) -> Result<(), io::Error> {
    if path.exists() {
        info!("file already exists, skipping"; "path" => path.to_str(), "message" => msg);
        Ok(())
    } else {
        let mut f = File::create(path)?;
        f.write_all(contents.as_bytes())?;
        info!("wrote file"; "path" => path.to_str());
        Ok(())
    }
}

/// See the documentation for lorri::cli::Command::Init for
/// more details
pub fn main(default_shell: &str, default_envrc: &str) -> OpResult {
    create_if_missing(
        Path::new("./shell.nix"),
        default_shell,
        "Make sure shell.nix is of a form that works with nix-shell.",
    )
    .map_err(|e| ExitError::user_error(format!("{}", e)))?;

    create_if_missing(
        Path::new("./.envrc"),
        default_envrc,
        "Please add 'eval \"$(lorri direnv)\"' to .envrc to set up lorri support.",
    )
    .map_err(|e| ExitError::user_error(format!("{}", e)))?;

    info!("done");
    ok()
}
