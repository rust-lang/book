use anyhow::Result;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

#[derive(Default)]
pub struct LogFiles {
  locks: HashMap<PathBuf, Mutex<()>>,
}

impl LogFiles {
  pub fn new(paths: Vec<&str>) -> Self {
    let locks = paths
      .into_iter()
      .map(|s| (PathBuf::from(s), Mutex::new(())))
      .collect::<HashMap<_, _>>();
    LogFiles { locks }
  }

  pub fn append(&self, path: impl AsRef<Path>, contents: &str) -> Result<()> {
    let path = path.as_ref();
    let _lock = self.locks[path].lock().unwrap();
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    f.write_all(contents.as_bytes())?;
    f.write_all(b"\n")?;
    f.sync_all()?;
    Ok(())
  }
}
