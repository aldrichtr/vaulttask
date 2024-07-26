use serde_derive::{Deserialize, Serialize};



#[derive(Debug,Deserialize, Serialize)]
pub struct Tag {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) color: String,
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

#[cfg(test)]

mod test {
    use super::Tag;
    #[test]
    fn test_new() {
        let t = Tag::new();
        assert_eq!(t.id, String::from("0"));
        assert_eq!(t.title, String::from("<none>"));
        assert_eq!(t.color, String::from("#ffffff"));
    }
}
