use human::character::Character;
use std::fs::{create_dir, remove_file, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use world::{World, WorldMeta};
use CARGO_VERSION;

#[derive(Debug, Clone)]
pub struct SaveFile {
    pub path: PathBuf,
    pub version: String,
    pub time: SystemTime,
    pub meta: WorldMeta,
    pub avatar_data: String,
}

#[derive(Debug)]
pub enum CreateFileError {
    SystemError(String),
    FileExists,
}

impl SaveFile {
    pub fn new(name: &str, seed: &str) -> Self {
        let name = name
            .trim()
            .replace("\n", "")
            .replace("/", "")
            .replace("\\", "");
        let file_name = name.replace(" ", "_");
        let path: PathBuf = ["save", (file_name + ".save").as_str()].iter().collect();
        SaveFile {
            path,
            version: CARGO_VERSION.to_string(),
            time: SystemTime::now(),
            meta: WorldMeta {
                name,
                seed: seed.to_string(),
            },
            avatar_data: String::new(),
        }
    }

    pub fn load(path: PathBuf) -> Option<Self> {
        let file = File::open(&path).ok()?;
        let mut lines = BufReader::new(&file).lines();
        let name = lines.next()?.ok()?;
        if name.is_empty() {
            return None;
        }
        let seed = lines.next()?.ok()?;
        if seed.is_empty() {
            return None;
        }
        let version = lines.next()?.ok()?;
        if version.is_empty() {
            return None;
        }
        let time = lines.next()?.ok()?.parse::<u64>().ok()?;
        let time = SystemTime::UNIX_EPOCH + Duration::new(time, 0);
        let avatar_data = if let Some(line) = lines.next() {
            line.ok()?
        } else {
            String::new()
        };
        Some(SaveFile {
            path,
            version,
            time,
            meta: WorldMeta { name, seed },
            avatar_data,
        })
    }

    pub fn create(&mut self) -> Result<(), CreateFileError> {
        create(&self.path, &self.meta)
    }

    pub fn load_avatar(&self) -> Character {
        serde_json::from_str(self.avatar_data.as_str()).unwrap()
    }
}

pub fn savefiles() -> Vec<SaveFile> {
    let path = Path::new("save");
    let mut files = Vec::new();
    if path.exists() {
        for p in path.read_dir().unwrap() {
            if let Some(s) = SaveFile::load(p.unwrap().path()) {
                files.push(s);
            }
        }
    }
    files.sort_by(|s1, s2| s2.time.cmp(&s1.time));
    files
}

pub fn delete(path: &Path) {
    if path.exists() {
        remove_file(path).ok();
    }
}

pub fn create(path: &Path, meta: &WorldMeta) -> Result<(), CreateFileError> {
    let dir = Path::new("save");
    if !dir.exists() {
        create_dir(dir).map_err(|e| CreateFileError::SystemError(e.to_string()))?;
    }
    if path.is_file() {
        Err(CreateFileError::FileExists)
    } else {
        let time = SystemTime::now();
        let mut file =
            File::create(&path).map_err(|e| CreateFileError::SystemError(e.to_string()))?;
        let data = format!(
            "{}\n{}\n{}\n{}",
            meta.name,
            meta.seed,
            CARGO_VERSION,
            time.duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|e| CreateFileError::SystemError(e.to_string()))?
                .as_secs(),
        );
        file.write_all(data.as_bytes())
            .map_err(|e| CreateFileError::SystemError(e.to_string()))?;
        Ok(())
    }
}

pub fn save(path: &Path, world: &World) -> Result<(), String> {
    let dir = Path::new("save");
    if !dir.exists() {
        create_dir(dir).map_err(|e| e.to_string())?;
    }
    let time = SystemTime::now();
    let mut file = File::create(path).map_err(|e| e.to_string())?;
    let data = format!(
        "{}\n{}\n{}\n{}\n{}",
        world.meta.name,
        world.meta.seed,
        CARGO_VERSION,
        time.duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs(),
        serde_json::to_string(&world.avatar).map_err(|e| e.to_string())?
    );
    file.write_all(data.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}