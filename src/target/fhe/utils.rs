//! Utility functions to write compiler output to FHE

use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

/// Get FHE source directory
pub fn get_fhe_source() -> String {
    let key = "SEAL_SOURCE";
    match env::var(key) {
        Ok(val) => val,
        Err(e) => panic!("Missing env variable: SEAL_SOURCE, {}", e),
    }
}

/// Given Path `path` and String denominator `lang`, return the filename of the path
pub fn get_path(path: &Path, lang: &str, t: &str) -> String {
    let filename = Path::new(&path.iter().last().unwrap())
        .file_stem()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();

    match fs::create_dir_all("scripts/fhe_tests/tests") {
        Err(why) => panic!("couldn't create {}: {}", "scripts/fhe_tests/tests", why),
        Ok(file) => file,
    };

    let name = format!("{}_{}", filename, lang);
    let path = format!("scripts/fhe_tests/tests/{}_{}.txt", name, t);
    match fs::File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path, why),
        Ok(file) => file,
    };
    path
}

/// Write output to temporary file
pub fn write_lines_to_file(path: &str, lines: &[String]) {
    if !Path::new(&path).exists() {
        fs::File::create(&path).expect(&*format!("Failed to create: {}", path));
    }

    let data = lines.join("");

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .expect("Failed to open seal_tmp file");

    file.write_all(data.as_bytes())
        .expect("Failed to write to seal_tmp file");
}
