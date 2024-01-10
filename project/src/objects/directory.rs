use std::{
    fs::{create_dir, File},
    io::{ErrorKind, Read, Write},
    path::PathBuf,
};

use super::Objects;
use crate::blob::Blob;

#[derive(Debug, Clone)]
pub struct DirectoryObjects {
    root: PathBuf,
}

impl DirectoryObjects {
    pub fn new(root: PathBuf) -> Result<Self, std::io::Error> {
        if !root.join("objects").exists() {
            create_dir(&root.join("objects"))?;
        }
        Ok(Self { root })
    }
}

impl Objects for DirectoryObjects {
    type Error = std::io::Error;

    fn exists(&self, id: Blob) -> Result<bool, Self::Error> {
        let blob_hash = format!("{}", id);
        let blob_folder_name = &blob_hash[0..2];
        let blob_filename = &blob_hash[2..];
        let path_to_blob = self
            .root
            .join("objects")
            .join(format!("{}", blob_folder_name))
            .join(format!("{}", blob_filename));
        if path_to_blob.exists() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn get(&self, id: Blob) -> Result<Option<Vec<u8>>, Self::Error> {
        let blob_hash = format!("{}", id);
        let blob_folder_name = &blob_hash[0..2];
        let blob_filename = &blob_hash[2..];
        let path_to_blob = self
            .root
            .join("objects")
            .join(format!("{}", blob_folder_name))
            .join(format!("{}", blob_filename));
        match std::fs::File::options().read(true).open(path_to_blob) {
            Ok(mut file) => {
                let mut v = Vec::new();
                file.read_to_end(&mut v)?;
                return Ok(Some(v));
            }
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    return Ok(None);
                } else {
                    return Err(err);
                }
            }
        }
    }

    fn push(&mut self, object: &[u8]) -> Result<Blob, Self::Error> {
        let blob = object.into();
        let blob_hash = format!("{}", blob);
        let blob_folder_name = &blob_hash[0..2];
        let blob_filename = &blob_hash[2..];
        let path_to_blob_folder = self
            .root
            .join("objects")
            .join(format!("{}", blob_folder_name));
        let path_to_blob_file = path_to_blob_folder.join(format!("{}", blob_filename));

        if path_to_blob_file.exists() {
            Ok(blob)
        } else {
            if !path_to_blob_folder.exists() {
                create_dir(path_to_blob_folder)?;
            }
            let mut file = File::options()
                .create(true)
                .read(true)
                .write(true)
                .open(path_to_blob_file)?;
            file.write(object)?;
            Ok(blob)
        }
    }
}
