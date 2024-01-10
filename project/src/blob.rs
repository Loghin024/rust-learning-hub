//Binary Large OBject
use crate::hex;
use ::blake3::Hash;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    fs::File,
    io::Read,
    path::Path,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Blob(Hash);

impl Ord for Blob {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.as_bytes().cmp(other.0.as_bytes())
    }
}

impl PartialOrd for Blob {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.as_bytes().partial_cmp(other.0.as_bytes())
    }
}

impl From<&Vec<u8>> for Blob {
    fn from(vec: &Vec<u8>) -> Self {
        Blob(blake3::hash(&vec))
    }
}

impl From<&[u8]> for Blob {
    fn from(bytes: &[u8]) -> Self {
        Blob(blake3::hash(&bytes))
    }
}

impl Display for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b: &[u8] = self.0.as_bytes();
        write!(f, "{}", hex::Hex::from(b))
    }
}

impl Serialize for Blob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let binary: &[u8] = self.0.as_bytes();
        hex::Hex::from(binary).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Blob {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let binary: hex::Hex = Deserialize::deserialize(deserializer)?;
        let v: Vec<u8> = binary.into();
        let mut bytes: [u8; 32] = [0; 32];
        for i in 0..32 {
            bytes[i] = v[i];
        }
        Ok(Blob(Hash::from(bytes)))
    }
}

impl Debug for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl TryFrom<File> for Blob {
    type Error = std::io::Error;

    fn try_from(mut f: File) -> Result<Self, Self::Error> {
        let mut vec = Vec::new();
        f.read_to_end(&mut vec)?;
        Ok((&vec).into())
    }
}

impl<'a> TryFrom<&Path> for Blob {
    type Error = std::io::Error;

    fn try_from(p: &Path) -> Result<Self, Self::Error> {
        let f = File::options().read(true).open(p)?;
        Blob::try_from(f)
    }
}
