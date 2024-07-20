// see the graymatter crate for front matter parsing
pub mod parser;
pub mod tag;

use crate::task::tag::Tag;
use chrono::{DateTime, Local};
use std::{
    vec::Vec,
    // time::{SystemTime, UNIX_EPOCH}
};

#[allow(unused)]
pub struct Task {
    id: String,
    title: String,
    desc: String,
    // state
    status: String,
    context: String,

    memberof: String,
    tags: Vec<Tag>,
    // dates
    due: DateTime<Local>,
    start: DateTime<Local>,
    end: DateTime<Local>,
    schedule: DateTime<Local>,
    delay: DateTime<Local>,
}
