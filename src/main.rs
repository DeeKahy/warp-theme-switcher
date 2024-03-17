

use std::fs;
use std::path::{Path, PathBuf};
use rand::Rng;
use rand::seq::SliceRandom;
use std::os::unix::fs as unix_fs;

fn main() {
    let dir_path = Path::new("/Users/lennartdiegokahn/.warp/themes/custom");
    let background_path = dir_path.join("background.png");

    // Step 1: Delete the existing 'background.png' if it exists
    if background_path.exists() {
        if let Err(e) = fs::remove_file(&background_path) {
            println!("Error deleting existing background.png: {}", e);
            return;
        }
    }

    // Step 2: Select a random PNG file from the directory
    let png_files = match fs::read_dir(dir_path) {
        Ok(entries) => {
            entries.filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.is_file() && p.extension() == Some("png".as_ref()))
                .collect::<Vec<PathBuf>>()
        }
        Err(e) => {
            println!("Failed to read directory: {}", e);
            return;
        }
    };

    if png_files.is_empty() {
        println!("No PNG files found in the directory.");
        return;
    }

    let mut rng = rand::thread_rng();
    let random_file = match png_files.choose(&mut rng) {
        Some(file) => file,
        None => {
            println!("Random selection failed.");
            return;
        }
    };

    // Step 3: Create a symbolic link to the randomly selected PNG file
    if let Err(e) = unix_fs::symlink(random_file, &background_path) {
        println!("Failed to create symbolic link: {}", e);
    }
}
