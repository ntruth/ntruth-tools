// Input parser module
// TODO: Implement input parsing logic

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, input: &str) -> ParseResult {
        // TODO: Implement parsing
        ParseResult::Unknown(input.to_string())
    }
}

pub enum ParseResult {
    File(String),
    App(String),
    Calculator(String),
    WebSearch { engine: String, query: String },
    AI(String),
    Clipboard(String),
    Unknown(String),
}
