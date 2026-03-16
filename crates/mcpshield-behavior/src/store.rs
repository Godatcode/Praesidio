use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::profile::ToolProfile;

/// Persistent storage for behavioral profiles
pub struct ProfileStore {
    profiles: HashMap<String, ToolProfile>,
    dir: PathBuf,
}

impl ProfileStore {
    pub fn new(dir: &Path) -> Self {
        let profiles = Self::load_all(dir);
        Self {
            profiles,
            dir: dir.to_path_buf(),
        }
    }

    fn profile_key(server: &str, tool: &str) -> String {
        format!("{}::{}", server, tool)
    }

    /// Get or create a profile for a tool
    pub fn get_or_create(&mut self, server: &str, tool: &str) -> &mut ToolProfile {
        let key = Self::profile_key(server, tool);
        self.profiles
            .entry(key)
            .or_insert_with(|| ToolProfile::new(server, tool))
    }

    /// Get a profile (read-only)
    pub fn get(&self, server: &str, tool: &str) -> Option<&ToolProfile> {
        let key = Self::profile_key(server, tool);
        self.profiles.get(&key)
    }

    /// Save all profiles to disk
    pub fn save_all(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.dir)?;
        let path = self.dir.join("profiles.json");
        let json = serde_json::to_string_pretty(&self.profiles)?;
        std::fs::write(path, json)
    }

    /// Load all profiles from disk
    fn load_all(dir: &Path) -> HashMap<String, ToolProfile> {
        let path = dir.join("profiles.json");
        if !path.exists() {
            return HashMap::new();
        }
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    /// List all profiles
    pub fn list(&self) -> Vec<&ToolProfile> {
        self.profiles.values().collect()
    }

    /// Number of profiles
    pub fn count(&self) -> usize {
        self.profiles.len()
    }
}
