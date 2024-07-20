// region: imports
//- stdlib
use std::{
    fs::{read_to_string, OpenOptions}, io::read_to_string, path::PathBuf
};

use chrono::format::parse;
//- crates
use markdown_it::{MarkdownIt, Node};
use markdown_it_front_matter::FrontMatter;
use markdown_it_tasklist::TodoCheckbox;
use markdown_it::plugins::cmark;
//- local

// endregion imports

pub struct Parser {
    plugins: Vec<MarkdownIt>,
    _parser: MarkdownIt
}

impl Parser {
    pub fn new() -> Self {
        Self {
            plugins: vec![
                cmark,
                FrontMatter,
                TodoCheckbox
            ],
            _parser: MarkdownIt::new()
        }
    }

    pub fn add_plugin(&self, md: &MarkdownIt) -> Self {
        self.plugins.push(md);
        self
    }

    pub fn parse(&self, path: & PathBuf) -> Node {
        for plugin in self.plugins {
            plugin::add(self._parser);
        };

        let content: String = fs::read_to_string(path.display());
        self._parser::parse(content.as_str())
    }

}

// region: Tests
// endregion Tests
