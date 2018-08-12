use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

fn write_to_file(path: &Path, content: &[u8]) -> io::Result<()> {
    let write_content = |mut file: File| -> io::Result<()> {
        file.write_all(content)?;
        Ok(())
    };

    OpenOptions::new()
        .write(true)
        .open(path)
        .and_then(|file| write_content(file))
        .or_else(|_err| write_content(File::create(path)?))
}

fn create_folder(path: &Path) -> io::Result<()> {
    fs::create_dir(path)?;
    Ok(())
}
