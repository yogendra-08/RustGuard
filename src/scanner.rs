use sha2::{Digest, Sha256};
use std::fs;
use std::io;
use std::path::Path;

pub fn scan_directory(dir: &Path) -> io::Result<Vec<(String, String)>> {
    let mut results = Vec::new();
    scan_recursive(dir, dir, &mut results)?;
    results.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(results)
}

fn scan_recursive(
    root: &Path,
    current: &Path,
    results: &mut Vec<(String, String)>,
) -> io::Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;

        if file_type.is_symlink() {
            continue;
        }

        if file_type.is_dir() {
            scan_recursive(root, &path, results)?;
        } else if file_type.is_file() {
            let bytes = fs::read(&path)?;
            let digest = Sha256::digest(&bytes);
            let hash = digest
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<String>();

            let relative_path = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");

            results.push((relative_path, hash));
        }
    }

    Ok(())
}