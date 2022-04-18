use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use VERSION;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Meta {
    #[serde(skip)]
    pub path: PathBuf,
    pub name: String,
    pub seed: String,
    pub version: String,
    pub time: SystemTime,
    pub current_tick: u128,
}

impl Meta {
    pub fn new<S>(name: S, seed: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            path: Default::default(),
            name: name.into(),
            seed: seed.into(),
            version: VERSION.to_string(),
            time: SystemTime::now(),
            current_tick: 0,
        }
    }

    pub fn with_path(mut self, path: &Path) -> Self {
        self.path = path.into();
        self
    }
}

impl Eq for Meta {}

impl PartialEq<Self> for Meta {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl PartialOrd<Self> for Meta {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Meta {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}
