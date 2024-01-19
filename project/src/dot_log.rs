use derive_more::From;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fs::{create_dir, create_dir_all, read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{
    blob::Blob,
    commit::Commit,
    directory::{Directory, Ignores},
    objects::{directory::DirectoryObjects, Objects},
};

#[derive(Debug, From)]
pub enum Error {
    #[from]
    IO(std::io::Error),
    #[from]
    Serde(serde_json::Error),
    MissingObject(Blob),
}

pub struct DotLog {
    root: PathBuf,
}

impl DotLog {
    pub fn init(root: PathBuf) -> Result<Self, Error> {
        if root.exists() {
            println!("A repository already exists here!");
            return Ok(DotLog { root });
        }

        create_dir_all(&root)?;

        //set default branch (master)
        let mut file = File::options()
            .create(true)
            .write(true)
            .open(root.join("branch"))?;
        file.write_all("master".as_bytes())?;

        // Create the branches directory
        create_dir(root.join("branches"))?;

        //initial commit
        let mut objects = DirectoryObjects::new(root.clone())?;
        let blob_dir = Directory::default();
        let blob_dir = objects.insert_json(&blob_dir)?;
        let commit = Commit {
            directory: blob_dir,
            message: String::from("first commit"),
            previous: BTreeSet::new(),
        };

        let commit_id = objects.insert_json(&commit)?;
        write_json(&commit_id, &root.join("branches").join("master"))?;
        let ignores = Ignores::default();
        write_json(&ignores, &root.join("ignores"))?;

        Ok(DotLog { root })
    }

    pub fn is_log_repo(path: PathBuf) -> Option<Self> {
        if path.exists() {
            Some(DotLog { root: path })
        } else {
            None
        }
    }

    pub fn get_branch(&self) -> Result<String, Error> {
        Ok(read_to_string(self.root.join("branch"))?)
    }

    pub fn set_branch(&self, new_branch: &str) -> Result<(), Error> {
        let mut file = File::options()
            .write(true)
            .truncate(true)
            .open(self.root.join("branch"))?;
        file.write_all(new_branch.as_bytes())?;
        Ok(())
    }

    pub fn get_objects(&self) -> Result<DirectoryObjects, Error> {
        Ok(DirectoryObjects::new(self.root.clone())?)
    }

    pub fn get_branch_commit_hash(&self, branch: &str) -> Result<Blob, Error> {
        read_json(&self.root.join("branches").join(branch))
    }

    pub fn set_branch_commit_hash(&self, branch: &str, blob: Blob) -> Result<(), Error> {
        write_json(&blob, &self.root.join("branches").join(branch))
    }

    pub fn branch_exists(&self, branch: &str) -> bool {
        self.root.join("branches").join(branch).exists()
    }

    pub fn create_branch(&self, new_branch: &str) -> Result<(), Error> {
        if !self.branch_exists(new_branch) {
            let commit_hash = self.get_branch_commit_hash(&self.get_branch()?)?;
            return write_json(&commit_hash, &self.root.join("branches").join(new_branch));
        }
        Ok(())
    }

    pub fn ignores(&self) -> Result<Ignores, Error> {
        read_json(&self.root.join("ignores"))
    }
}

//writing and reading json files from /objects
pub trait JSON {
    fn insert_json<A: Serialize>(&mut self, thing: &A) -> Result<Blob, Error>;
    fn read_json<A: for<'de> Deserialize<'de>>(&mut self, object_id: Blob) -> Result<A, Error>;
}

impl JSON for DirectoryObjects {
    fn insert_json<A: Serialize>(&mut self, thing: &A) -> Result<Blob, Error> {
        Ok(self.push(&serde_json::to_vec_pretty(thing)?)?)
    }

    fn read_json<A: for<'de> Deserialize<'de>>(&mut self, object_id: Blob) -> Result<A, Error> {
        match self.get(object_id)? {
            None => Err(Error::MissingObject(object_id)),
            Some(obj) => Ok(serde_json::from_slice(&obj)?),
        }
    }
}

fn read_json<A: for<'de> Deserialize<'de>>(path: &Path) -> Result<A, Error> {
    Ok(serde_json::from_reader(
        File::options().read(true).open(path)?,
    )?)
}

fn write_json<A: Serialize>(thing: &A, path: &Path) -> Result<(), Error> {
    Ok(serde_json::to_writer_pretty(
        File::options().write(true).create(true).open(path)?,
        thing,
    )?)
}
