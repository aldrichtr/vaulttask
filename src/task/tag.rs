

use crate::task::id::{Id, IdType};

pub struct Tag {
    id: String,
    title: String,
    color: String,
}

impl Tag {
    pub fn new() -> Self {
        Self {
            id: String::from("0"),
            title: String::from("<none>"),
            color: String::from("#ffffff")
        }
    }
}
