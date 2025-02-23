mod components;
use components::lexer::Lexer;
use components::reader::Reader;
use components::token::Token;

fn main() {
  let code: String = Reader::read_path("./programs/test.ir").unwrap();
  let mut lexer: Lexer = Lexer::new();

  let tokens: Result<&Vec<Token>, String> = lexer.lex(code);

  match tokens {
    Ok(tokens) => {
      for token in tokens.iter() {
        println!("[{:?}] {:?}", token.token_type, token.literal);
      }
    }

    Err(message) => {
      println!("{}", message);
    }
  }
}
