#![allow(dead_code)]

use std::{fs::File, io::Read, path::{PathBuf, Path}};

use binprot::BinProtRead;

fn read_file(path: &Path) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let _ = File::open(path)?.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn read(file: &str) -> std::io::Result<Vec<u8>> {
    let prefix = std::env::var("CARGO_MANIFEST_DIR").map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "CARGO_MANIFEST_DIR variable")
    })?;
    let path = PathBuf::from(prefix).join("tests/files").join(file);
    read_file(&path)
}

pub fn for_all<F>(dir: &str, mut f: F) -> std::io::Result<()> where F: FnMut(&[u8]) {
    let prefix = std::env::var("CARGO_MANIFEST_DIR").map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "CARGO_MANIFEST_DIR variable")
    })?;
    let path = PathBuf::from(prefix).join("tests/files").join(dir);
    let dir = std::fs::read_dir(path)?;
    for file in dir {
        let path = file?.path();
        if path.extension().map_or(false, |ext| ext == "bin") {
            println!("reading {path:?}...");
            f(&read_file(&path)?);
        }
    }
    Ok(())
}

pub fn assert_binprot_read<T>(mut buf: &[u8]) where T: BinProtRead {
    assert!(T::binprot_read(&mut buf).is_ok());
    assert_eq!(buf.len(), 0);
}
