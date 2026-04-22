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

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn modified_time(&self) -> Result<Option<std::time::SystemTime>> {
        if !self.path.exists() {
            return Ok(None);
        }
        Ok(Some(fs::metadata(&self.path)?.modified()?))
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

#[cfg(test)]
mod tests {
    use super::JsonStore;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct Fixture {
        value: String,
    }

    #[test]
    fn saves_and_loads_json() {
        let dir = tempfile::tempdir().unwrap();
        let store = JsonStore::new(dir.path().join("data.json"));
        let data = Fixture {
            value: "ok".to_string(),
        };

        store.save(&data).unwrap();
        assert_eq!(store.load::<Fixture>().unwrap(), data);
    }

    #[test]
    fn backs_up_corrupt_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("data.json");
        std::fs::write(&path, "not-json").unwrap();
        let store = JsonStore::new(&path);

        let backup = store.backup_corrupt().unwrap().expect("backup path");

        assert!(!path.exists());
        assert!(backup.exists());
    }
}
