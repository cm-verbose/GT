use super::{parser::Parser, reader::Reader, token::Token, tokenizer::Tokenizer};

pub struct Transpiler {}

impl Transpiler {
  pub fn new() -> Self {
    Transpiler {}
  }

  /// Reads a speicified file from its path and transpile its contents
  pub fn transpile_file(&mut self, path: &'static str) {
    let source_text: Result<String, String> = Reader::read(path);

    if let Err(message) = source_text {
      println!("{}", message);
      return;
    } else if let Ok(source_code) = source_text {
      self.transpile(source_code);
    }
  }

  /// Transpiles source code
  pub fn transpile(&mut self, source: String) {
    let mut tokenizer: Tokenizer = Tokenizer::new();
    let tokens: Result<&Vec<Token>, String> = tokenizer.tokenize(source);

    if let Err(message) = tokens {
      println!("{}", message);
      return;
    }
    let mut parser: Parser = Parser::new();
    parser.parse(tokens.unwrap());
  }
}
