pub struct Reporter;

impl Reporter {
  pub fn wrap_error(message: String) -> String {
    let message: String = format!("[GT] [Error]: {}", message);
    message
  }

  pub fn wrap_warning(message: String) -> String {
    let message: String = format!("[GT] [Warning]: {}", message);
    message
  }
}
