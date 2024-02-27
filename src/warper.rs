use std::{
    fs,
    path::{Path, PathBuf},
};

use serde_json::{json, Value};

pub struct Warper {
    warps_dir: PathBuf,
    warps_filename: String,
    current_directory: PathBuf,
    warps_filepath: PathBuf,
}

impl Warper {
    pub fn new(warps_dir: &str, warps_filename: &str) -> Self {
        let home_dir = home::home_dir().expect("Could not locate home dir.");
        let warps_dir = home_dir.join(warps_dir);
        let current_directory = PathBuf::from(".")
            .canonicalize()
            .expect("Could not create warp path.");

        let warps_filepath = warps_dir.join(warps_filename);

        Self {
            warps_dir,
            current_directory,
            warps_filename: warps_filename.to_string(),
            warps_filepath,
        }
    }

    pub fn get_warps(&mut self) -> serde_json::Map<std::string::String, Value> {
        let raw_warps_result =
            fs::read_to_string(Path::join(&self.warps_dir, self.warps_filename.as_str()));

        if raw_warps_result.is_err() {
            self.create_warps_file();
            return self.get_warps();
        }

        let raw_warps = raw_warps_result.unwrap();

        serde_json::from_str(&raw_warps).expect("Could not parse warps.")
    }

    pub fn list_warps(mut self) {
        let warps = self.get_warps();

        for (_, (name, path)) in warps.iter().enumerate() {
            eprintln!("{} : {}", name, path.as_str().unwrap());
        }
    }

    pub fn create_warps_file(&mut self) {
        fs::create_dir(&self.warps_dir).expect("Could not create directory for warps.");
        let warps_path = self.warps_dir.join("warps.json");
        fs::write(warps_path, "{}").expect("Could not write to warps file.");
    }

    pub fn add_warp(mut self, name: String) {
        let mut warps = self.get_warps();
        let warp_path = warps.get(&name);

        if warp_path.is_some() {
            eprintln!("Warp already exists for {}.", warp_path.unwrap());
            return;
        }

        let warp_path = json!(self.current_directory);
        warps.insert(name, warp_path);

        self.set_warps(json!(warps))
            .expect("Warp could not be added.");
    }

    pub fn set_warps(self, warps: Value) -> Result<(), std::io::Error> {
        let warps_file_path = self.warps_dir.join("warps.json");

        fs::write(warps_file_path, warps.to_string())
    }

    pub fn warp(mut self, name: String) {
        let warps = self.get_warps();

        let warp_path = warps.get(&name);

        if warp_path.is_none() {
            eprintln!("Warp not found {}.", name);
            return;
        }

        print!("{}", warp_path.unwrap().as_str().unwrap());
    }

    pub fn remove_warp(mut self, name: String) {
        let mut warps = self.get_warps();

        warps.remove(&name);

        self.set_warps(json!(warps))
            .expect(format!("Could not remove warp {}", name).as_str());
    }
}
