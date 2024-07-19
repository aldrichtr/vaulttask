use nanoid::nanoid;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

const ID_LEN_LONG: usize = 23;
const ID_LEN_SHORT: usize = 12;
const ID_ALPHABET: [char; 36] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
    'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];

#[derive(Debug, Serialize, Deserialize)]
pub enum IdType {
    Nanoid,
    Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Id;

// TODO: Need to pull in the current configuration before building the id
//       or have a function for each type
impl Id {
    pub fn new(tpe: IdType) -> String {
        match tpe {
            IdType::Nanoid => {
                nanoid!(ID_LEN_LONG, ID_ALPHABET.as_ref())
            },
            IdType::Uuid => {
                Uuid::new_v4().braced().to_string()
            }
        }
    }
}

#[cfg(test)]

mod tests {
    mod task {
        use crate::task::id::*;


        #[test]
        fn test_nanoid_type() {
            let test_id = Id::new(IdType::Nanoid);
            assert_eq!(test_id.len(), ID_LEN_LONG)
        }
    }
}
