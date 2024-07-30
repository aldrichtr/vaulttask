use std::vec::Vec;

use crate::data::{
    id::{Id, IdType},
    tag::Tag,
    parser::FileData
};
use chrono::{DateTime, Local, TimeZone};

#[derive(Debug)]
pub struct Task {
    pub id : String,
    pub title : String,
    pub desc : String,
    // state
    pub status : String,
    pub context : String,

    pub memberof : String,
    tags : Vec<Tag>,
    // dates
    pub created : DateTime<Local>,
    pub updated : DateTime<Local>,
    // --
    pub due : DateTime<Local>,
    pub start : DateTime<Local>,
    pub end : DateTime<Local>,
    pub schedule : DateTime<Local>,
    pub delay : DateTime<Local>,
}

pub enum TaskTagTypes {
    AsTag(Tag),
    FromTitle(String),
    FromFields(String, String),
}

impl Task {
    pub fn new() -> Self {
        Self {
            id : String::from(""),
            title : String::from(""),
            desc : String::from(""),
            // state
            status : String::from(""),
            context : String::from(""),

            memberof : String::from(""),
            tags : Vec::new(),
            // dates
            created : Local.timestamp_millis_opt(0).unwrap(),
            updated : Local.timestamp_millis_opt(0).unwrap(),
            due : Local.timestamp_millis_opt(0).unwrap(),
            start : Local.timestamp_millis_opt(0).unwrap(),
            end : Local.timestamp_millis_opt(0).unwrap(),
            schedule : Local.timestamp_millis_opt(0).unwrap(),
            delay : Local.timestamp_millis_opt(0).unwrap(),
        }
    }

    // TODO: need to create a list of the tags in the vaults, and use
    //       them when creating a tag from title.
    pub fn add_tag(&mut self, ctor : TaskTagTypes) {
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
            TaskTagTypes::FromFields(title, color) => {
                let mut tag = Tag::new();
                tag.title = title;
                tag.color = color
            }
        }
    }
}

impl From<FileData> for Task {
    fn from(fd: FileData) -> Self {
        // TODO: Should this be `new` or `default`?
        let mut task = Task::new();

        task.id = fd.front_matter.id.trim().to_string();
        task.title = fd.front_matter.title.trim().to_string();
        task.desc = fd.front_matter.desc.trim().to_string();
        task.status = fd.front_matter.status.trim().to_string();
        task.created = Local
            .timestamp_millis_opt(
                fd
                    .front_matter
                    .created
                    .parse::<i64>()
                    .expect("could not convert 'created' timestamp"),
            )
            .unwrap();
        task.updated = Local
            .timestamp_millis_opt(
                fd
                    .front_matter
                    .updated
                    .parse::<i64>()
                    .expect("could not convert 'updated' timestamp"),
            )
            .unwrap();
        for tag in &fd.front_matter.tags {
            task.add_tag(TaskTagTypes::FromTitle(tag.to_string()))
        }
        task
    }
}
