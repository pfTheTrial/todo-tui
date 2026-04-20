use std::fs;
use std::path::PathBuf;
use color_eyre::Result;
use crate::model::Task;

pub struct JsonStore {
    path: PathBuf,
}

impl JsonStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn load<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        let content = fs::read_to_string(&self.path)?;
        let data: T = serde_json::from_str(&content)?;
        Ok(data)
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    pub fn save(&self, tasks: &[Task]) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(tasks)?;
        let tmp_path = self.path.with_extension("tmp");
        fs::write(&tmp_path, content)?;
        fs::rename(&tmp_path, &self.path)?;
        Ok(())
    }
}
