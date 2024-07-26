// region: imports
//- stdlib
use std::{fs::read_to_string, path::PathBuf};


//- crates
use chrono::{Local, TimeZone};
use markdown_it::{
    MarkdownIt,
    Node,
    parser::inline::Text,
    plugins::cmark::block::heading::ATXHeading,
};
use markdown_it_front_matter::FrontMatter;
use markdown_it_tasklist::TodoCheckbox;
use serde_derive::Deserialize;
use serde_yml;

use super::task::{
    Task,
    TaskTagTypes::FromTitle
};

//- local

// endregion imports

// region: FileFrontMatter
#[allow(unused)]
#[derive(Debug, Deserialize, PartialEq)]
struct FileFrontMatter {
    id: String,
    title: String,
    desc: String,
    updated: String,
    created: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    status: String,
    #[serde(default)]
    priority: String,
    #[serde(default)]
    owner: String,
}

impl Default for FileFrontMatter {
    fn default() -> Self {
        Self {
            id: String::from(""),
            title: String::from(""),
            desc: String::from(""),
            tags: Vec::new(),
            updated: String::from(""),
            created: String::from(""),
            status: String::from(""),
            priority: String::from(""),
            owner: String::from(""),
        }
    }
}

// endregion FileFrontMatter

// region: CheckboxData
#[derive(Debug)]
struct CheckboxData {
    title: String,
    checked: bool,
}

impl CheckboxData {
    pub fn new(ttl: &str, chk: bool) -> Self {
        Self {
            title: ttl.to_string(),
            checked: chk,
        }
    }
}

// endregion CheckboxData

// region: HeadingData
#[derive(Debug)]
struct HeadingData {
    title: String,
    level: u8,
}

impl HeadingData {
    pub fn new(ttle: &str, lvl: u8) -> Self {
        Self {
            title: ttle.to_string(),
            level: lvl,
        }
    }

    pub fn to_string(&self) -> String {
        let mut h = String::new();
        for _i in 0..self.level {
            h.push('#');
        }
        let st = String::from(format!("{} {}", h, self.title ));
        st.clone()
    }
}
// endregion HeadingData

// region: FileData

#[derive(Debug)]
struct FileData {
    front_matter: FileFrontMatter,
    headings: Vec<HeadingData>,
    check_boxes: Vec<CheckboxData>,
}

impl Default for FileData {
    fn default() -> Self {
        Self {
            front_matter: FileFrontMatter::default(),
            headings: Vec::new(),
            check_boxes: Vec::new(),
        }
    }
}

impl FileData {
    pub fn new() -> Self {
        FileData::default()
    }

    pub fn add_front_matter(&mut self, fm: FileFrontMatter) {
        self.front_matter = fm;
    }

    pub fn add_heading(&mut self, title: &str, level: u8) {
        let hd = HeadingData::new(title, level);
        self.headings.push(hd);
    }

    pub fn add_checkbox(&mut self, title: &str, checked: bool) {
        let cb = CheckboxData::new(title, checked);
        self.check_boxes.push(cb);
    }
}

// endregion FileData

// region: Parser
pub struct Parser {
    parser: MarkdownIt,
}

impl Parser {
    pub fn new() -> Self {
        let mut parser = markdown_it::MarkdownIt::new();
        markdown_it::plugins::cmark::add(&mut parser);
        markdown_it_front_matter::add(&mut parser);
        markdown_it_tasklist::add(&mut parser);
        Self { parser }
    }

    pub fn parse(&mut self, path: &PathBuf) -> Task {
        // Read in the file content
        let content: String = read_to_string(path).expect("Could not read markdown file");
        // Convert the content to a Markdown AST
        let ast: Node = self.parser.parse(content.as_str());
        // Fill in the the data into a FileData
        let mut file_data = FileData::new();
        ast.walk(|node, _depth| {
            if let Some(fm) = node.cast::<FrontMatter>() {
                let data: FileFrontMatter = serde_yml::from_str(fm.content.as_str())
                    .expect("Could not transform data in markdown frontmatter");
                file_data.add_front_matter(data);
            } else if let Some(hd) = node.cast::<ATXHeading>() {
                if let Some(h_title) = node.children[0].cast::<Text>() {
                    let h_title = h_title.content.as_str();
                    file_data.add_heading(h_title, hd.level);
                }
            }
            if let Some(cb) = node.cast::<TodoCheckbox>() {
                if let Some(cb_title) = node.children[0].cast::<Text>() {
                    let cb_title = cb_title.content.as_str();
                    file_data.add_checkbox(cb_title, cb.checked);
                }
            }
        });
        // Create a task from the data
        println!("File Data is {:#?}", file_data);
        let mut task = Task::new();
        task.id = file_data.front_matter.id.trim().to_string();
        task.title = file_data.front_matter.title.trim().to_string();
        task.desc = file_data.front_matter.desc.trim().to_string();
        task.status = file_data.front_matter.status.trim().to_string();
        task.created = Local
            .timestamp_millis_opt(
                file_data
                    .front_matter
                    .created
                    .parse::<i64>()
                    .expect("could not convert 'created' timestamp"),
            )
            .unwrap();
        task.updated = Local
            .timestamp_millis_opt(
                file_data
                    .front_matter
                    .updated
                    .parse::<i64>()
                    .expect("could not convert 'updated' timestamp"),
            )
            .unwrap();
        for tag in file_data.front_matter.tags {
            task.add_tag(FromTitle(tag))
        }
        task
    }
}

// endregion Parser

// region: Tests
// endregion Tests
