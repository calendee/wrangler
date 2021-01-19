use std::env;
use std::path::PathBuf;
use std::process::Command;

use serde::{Deserialize, Serialize};

use super::ScriptFormat;

const UPLOAD_DIR: &str = "dist";
const WATCH_DIR: &str = "src";

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Builder {
    command: Option<String>,
    #[serde(default = "project_root")]
    pub cwd: PathBuf,
    #[serde(default = "upload_dir")]
    pub upload_dir: PathBuf,
    pub upload_format: ScriptFormat,
    #[serde(default = "watch_dir")]
    pub watch_dir: PathBuf,
}

fn project_root() -> PathBuf {
    env::current_dir().unwrap()
}

fn upload_dir() -> PathBuf {
    project_root().join(UPLOAD_DIR)
}

fn watch_dir() -> PathBuf {
    project_root().join(WATCH_DIR)
}

impl Builder {
    pub fn verify_config(&self) -> Result<(), failure::Error> {
        if self.upload_dir.canonicalize()? == project_root().canonicalize()?
            || self.watch_dir.canonicalize()? == project_root().canonicalize()?
        {
            failure::bail!("Wrangler doesn't support using the project root as the watch directory or upload directory.");
        }
        if !self.upload_dir.is_dir() {
            failure::bail!(format!(
                "A path was provided for upload_dir that is not a directory: {}",
                self.upload_dir.display()
            ));
        }
        if !self.watch_dir.is_dir() {
            failure::bail!(format!(
                "A path was provided for watch_dir that is not a directory: {}",
                self.watch_dir.display()
            ));
        }
        Ok(())
    }

    pub fn build_command(&self) -> Option<(&str, Command)> {
        match &self.command {
            Some(cmd) => {
                let mut c = if cfg!(target_os = "windows") {
                    let args: Vec<&str> = cmd.split_whitespace().collect();
                    let mut c = Command::new("cmd");
                    c.arg("/C");
                    c.args(args.as_slice());
                    c
                } else {
                    let mut c = Command::new("sh");
                    c.arg("-c");
                    c.arg(cmd);
                    c
                };

                c.current_dir(&self.cwd);

                Some((cmd, c))
            }
            None => None,
        }
    }
}
