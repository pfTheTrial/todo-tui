use color_eyre::Result;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

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

    pub fn backup_corrupt(&self) -> Result<Option<PathBuf>> {
        if !self.path.exists() {
            return Ok(None);
        }
        let backup = self.path.with_extension(format!(
            "corrupt-{}.json",
            chrono::Utc::now().format("%Y%m%d%H%M%S")
        ));
        fs::rename(&self.path, &backup)?;
        Ok(Some(backup))
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    pub fn save<T: serde::Serialize>(&self, data: &T) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(data)?;
        let tmp_path = tmp_path_for(&self.path);
        fs::write(&tmp_path, content)?;
        fs::rename(&tmp_path, &self.path)?;
        Ok(())
    }
}

fn tmp_path_for(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("store.json");
    path.with_file_name(format!("{}.{}.tmp", file_name, Uuid::new_v4()))
}
