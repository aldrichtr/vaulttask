use std::vec::Vec;

use chrono::{
    DateTime,
    Local,
    TimeZone
};
use crate::data::{
    tag::Tag,
    id::{Id, IdType}
};


#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub desc: String,
    // state
    pub status: String,
    pub context: String,

    pub memberof: String,
    tags: Vec<Tag>,
    // dates
    pub created: DateTime<Local>,
    pub updated: DateTime<Local>,
    // --
    pub due: DateTime<Local>,
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub schedule: DateTime<Local>,
    pub delay: DateTime<Local>,
}


pub enum TaskTagTypes {
    AsTag(Tag),
    FromTitle(String),
    FromFields(String, String)
}

impl Task {
    pub fn new() -> Self {
        Self {
            id: String::from(""),
            title: String::from(""),
            desc: String::from(""),
    // state
            status: String::from(""),
            context: String::from(""),

            memberof: String::from(""),
            tags: Vec::new(),
    // dates
            created: Local.timestamp_millis_opt(0).unwrap(),
            updated: Local.timestamp_millis_opt(0).unwrap(),
            due: Local.timestamp_millis_opt(0).unwrap(),
            start: Local.timestamp_millis_opt(0).unwrap(),
            end: Local.timestamp_millis_opt(0).unwrap(),
            schedule: Local.timestamp_millis_opt(0).unwrap(),
            delay: Local.timestamp_millis_opt(0).unwrap()
        }
    }

    // TODO: need to create a list of the tags in the vaults, and use
    //       them when creating a tag from title.
    pub fn add_tag(&mut self, ctor: TaskTagTypes) {
        match ctor {
            TaskTagTypes::AsTag(tag) => {
                self.tags.push(tag);
            }
            TaskTagTypes::FromTitle(title) => {
                let mut tag = Tag::new();
                // TODO: Add new id command somewhere so that we can use like
                //       tag.id = id::new()
                tag.title = title;
                self.tags.push(tag);
            }
            TaskTagTypes::FromFields(title,color ) => {
                let mut tag = Tag::new();
                tag.title = title;
                tag.color = color
            }
        }
    }
}
