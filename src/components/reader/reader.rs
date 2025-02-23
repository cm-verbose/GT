use std::fs;

pub struct Reader {
  //
}

impl Reader {
  pub fn read() -> String {
    let contents: String = fs::read_to_string("./programs/test.ir").expect("Not found");
    return contents;
  }
}
