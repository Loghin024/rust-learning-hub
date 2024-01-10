use crate::blob::Blob;

pub mod directory;

pub mod memory;

pub trait Objects {
        type Error;
        //check if blob already exists in /objects
        fn exists(&self, id:Blob) -> Result<bool, Self::Error>;

        //get blob data from /objects
        fn get(&self, id:Blob) -> Result<Option<Vec<u8>>, Self::Error>;

        //push blob into /objects
        fn push(&mut self, object: &[u8]) -> Result<Blob, Self::Error>;
}