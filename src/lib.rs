use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn get_warp_dir() -> PathBuf {
    home::home_dir()
        .expect("Could not locate home dir.")
        .join(".wd")
}

pub fn remove_warp(name: String) {
    let mut warps = get_warps();

    warps.remove(&name);

    set_warps(json!(warps)).expect(format!("Could not remove warp {}", name).as_str());
}

pub fn warp(name: String) {
    let warps = get_warps();

    let warp_path = warps.get(&name);

    if warp_path.is_none() {
        eprintln!("Warp not found {}.", name);
        return;
    }

    print!("{}", warp_path.unwrap().as_str().unwrap());
}

pub fn list_warps() {
    let warps = get_warps();

    for (_, (name, path)) in warps.iter().enumerate() {
        eprintln!("{} : {}", name, path.as_str().unwrap());
    }
}

pub fn add_warp(name: String) {
    let mut warps = get_warps();
    let warp_path = warps.get(&name);

    if warp_path.is_some() {
        eprintln!("Warp already exists for {}.", warp_path.unwrap());
        return;
    }

    let warp_path = json!(PathBuf::from(".")
        .canonicalize()
        .expect("Could not create warp path."));
    warps.insert(name, warp_path);

    set_warps(json!(warps)).expect("Warp could not be added.");
}

pub fn create_warps_file() {
    let warp_dir = get_warp_dir();
    fs::create_dir(&warp_dir).expect("Could not create directory for warps.");
    let warps_path = warp_dir.join("warps.json");
    fs::write(warps_path, "{}").expect("Could not write to warps file.");
}

pub fn set_warps(warps: Value) -> Result<(), std::io::Error> {
    let warps_file_path = get_warp_dir().join("warps.json");

    fs::write(warps_file_path, warps.to_string())
}

pub fn get_warps() -> serde_json::Map<std::string::String, Value> {
    let warp_dir = get_warp_dir();
    let raw_warps_result = fs::read_to_string(Path::join(&warp_dir, "warps.json"));

    if raw_warps_result.is_err() {
        create_warps_file();
        return get_warps();
    }

    let raw_warps = raw_warps_result.unwrap();

    serde_json::from_str(&raw_warps).expect("Could not parse warps.")
}
