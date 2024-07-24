// region: imports
//- stdlib
use std::{
    fs::read_to_string,
    path::PathBuf
};

//- crates
use markdown_it::{MarkdownIt, Node};
//- local

// endregion imports

pub struct Parser {
    parser: MarkdownIt
}


impl Parser {
    pub fn new() -> Self {
        let mut parser =  markdown_it::MarkdownIt::new();
        markdown_it::plugins::cmark::add(&mut parser);
        markdown_it_front_matter::add(&mut parser);
        markdown_it_tasklist::add(&mut parser);
        Self {
            parser
        }
    }

    pub fn parse(&mut self, path: & PathBuf) -> Node {
        let content: String = read_to_string(path).expect("Could not read markdown file");
        self.parser.parse(content.as_str())
    }

}

// region: Tests
// endregion Tests
