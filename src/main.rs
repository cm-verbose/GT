use components::{reader::Reader, tokenizer::Tokenizer, token::Token};

pub mod components;

fn main() {
  let source_text: Result<String, String> = Reader::read("./programs/test.ir");
  if let Err(message) = source_text {
    println!("{message}");
    return;
  }

  let source: String = source_text.unwrap();
  let mut tokenizer: Tokenizer = Tokenizer::new();

  let possible_tokens: Result<&Vec<Token>, String> = tokenizer.tokenize(source);

  if let Err(message) = possible_tokens {
    println!("{}", message);
    return; 
  }
}
