use std::{fs, path::Path};

pub struct Reader {
  //
}

impl Reader {
  /// Read a path to obtain a String
  pub fn read_path(path: &'static str) -> Result<String, &'static str> {
    if !Path::new(path).exists() {
      return Err("[Err]: Specified path does not exist");
    }

    if let Ok(contents) = fs::read_to_string(path) {
      Ok(contents)
    } else {
      Err("[Err]: Failed reading file")
    }
  }
}
