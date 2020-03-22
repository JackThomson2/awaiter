use dashmap::DashMap;
use std::{fs::{self, File}};
use std::io::Read;

use colored::Colorize;

use mime_guess::from_path;

#[derive(Clone, Debug)]
pub struct FoundFile {
    pub data: Vec<u8>,
    pub mime: String
}

/// Used to store a cache of directory locations and their bytes
pub type MappedFiles = DashMap<String, FoundFile>;

/// This will recursively search a directory and store the file
/// contents into the dashmap
fn recursively_search(path: &str, relative_len: usize, map: &mut MappedFiles) {
    let directory = fs::read_dir(path);
    if directory.is_err() {return;}

    let directory = directory.unwrap();

    for entry in directory {
        if entry.is_err() {continue};

        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = fs::metadata(&path).unwrap();

        let path_str = path.to_str().unwrap();

        if metadata.is_dir() {
            recursively_search(path_str, relative_len, map);
            continue;
        }

        let f = File::open(path_str);
        if f.is_err() {continue;}

        let mut f = f.unwrap();
        let mut buffer = Vec::new();
        let read_res = f.read_to_end(&mut buffer);
        if read_res.is_err() {
            println!("Error reading {}", path_str);
            continue;
        }
        let key = &path_str[relative_len..path_str.len()];
        let mime = from_path(key).first_raw().unwrap_or("text/html");

        map.insert(key.to_string(), FoundFile {data: buffer, mime: mime.to_string()});
    }
}

/// This starts the recursive search to find the files from a path
pub fn get_mapped_files(relative: &str) -> MappedFiles {
    let mut mapping = MappedFiles::new();

    println!(
        "[{}] {:8} Loading file from '{}'",
        "*".blue(),
        "Cache".cyan().bold(),
        relative.green().underline()
    );

    recursively_search(relative, relative.len(), &mut mapping);

    let len_str = format!("{}", mapping.len());

    println!(
        "[{}] {:8} Read {} files into our cache",
        "*".blue(),
        "Cache".cyan().bold(),
        len_str.green().bold()
    );

    mapping
}