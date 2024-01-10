use crate::blob::Blob;

use super::Objects;

use ::std::collections::{btree_map::Entry, BTreeMap};
use std::convert::Infallible;

pub struct InMemoryBlobStore {
    blobs: BTreeMap<Blob, Vec<u8>>,
}

impl InMemoryBlobStore {
    pub fn new() -> Self {
        Self {
            blobs: BTreeMap::new(),
        }
    }
}

impl Objects for InMemoryBlobStore {
    type Error = Infallible;

    fn exists(&self, id: Blob) -> Result<bool, Self::Error> {
        Ok(self.blobs.contains_key(&id))
    }

    fn get(&self, id: Blob) -> Result<Option<Vec<u8>>, Self::Error> {
        match self.blobs.get(&id) {
            Some(v) => Ok(Some(v.clone())),
            None => Ok(None),
        }
    }

    fn push(&mut self, object: &[u8]) -> Result<Blob, Self::Error> {
        let blob = object.into();
        match self.blobs.entry(blob) {
            Entry::Vacant(v) => {
                v.insert(object.into());
                Ok(blob)
            }
            Entry::Occupied(_o) => Ok(blob),
        }
    }
}
