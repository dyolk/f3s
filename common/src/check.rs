use std::process::Command;
use std::collections::HashMap;

pub fn command_exsists(name: &str) -> bool {
  which::which(name).is_ok()
}
