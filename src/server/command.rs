pub enum Command {
  Get(String),
  Set(String, String),
  Delete(String),
}

pub fn parse_command(received: &str) -> Option<Command> {
  let parts: Vec<&str> = received.split_whitespace().collect();
  match parts.as_slice() {
      ["GET", key] => Some(Command::Get(key.to_string())),
      ["SET", key, value] => Some(Command::Set(key.to_string(), value.to_string())),
      ["DELETE", key] => Some(Command::Delete(key.to_string())),
      _ => None,
  }
}