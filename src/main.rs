mod components;

use components::lexer::Lexer;
use components::reader::Reader;
use components::token::Token;

fn main() {
  let code: String = Reader::read();

  let mut lexer: Lexer = Lexer::ini();
  let tokens: &Vec<Token> = lexer.lex(code);

  for token in tokens.iter() {
    println!("[{:#?}] : {:?}", token.token_type, token.literal);
  }
}
