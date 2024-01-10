use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::blob::Blob;

//Commit of a version.
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    //Commit message.
    pub message: String,
    // Blob of the directory structure.
    pub directory: Blob,
    //link to previous blob
    pub previous: BTreeSet<Blob>,
}
