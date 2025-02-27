use std::{fs, path::Path};

use super::error_reporter::Reporter;

pub struct Reader {}

/// Reads an input file and outputs it's contents
impl Reader {
  pub fn read(path: &'static str) -> Result<String, String> {
    let target: &Path = Path::new(path);

    if !target.exists() {
      let message: String = format!("Specified path \"{}\", does not exist", path);
      return Err(Reporter::wrap_error(message));
    }

    if let Ok(contents) = fs::read_to_string(path) {
      if contents.trim().len() == 0 {
        let message: String = format!("File at \"{}\" is empty", path);
        return Err(Reporter::wrap_warning(message));
      }
      Ok(contents)
    } else {
      let message: String = format!("Failed reading file \"{}\"", path);
      Err(Reporter::wrap_error(message))
    }
  }
}
