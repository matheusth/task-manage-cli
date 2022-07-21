use crate::issue::Activity;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::ErrorKind;

pub fn open_or_create_file(path: &str) -> std::io::Result<File> {
  let file = OpenOptions::new()
    .write(true)
    .read(true)
    .open(path)
    .unwrap_or_else(|error| {
      if error.kind() == ErrorKind::NotFound {
        File::create("data.json").unwrap_or_else(|error| panic!("error creating file: {:?}", error))
      } else {
        panic!("Could not read from file");
      }
    });
  Ok(file)
}

pub fn save_to_file(data: String) -> std::io::Result<()> {
  let mut file = open_or_create_file("data.json")?;

  write!(file, "{}", data).unwrap();

  Ok(())
}

pub fn load_from_file() -> Result<std::vec::Vec<Activity>, Box<dyn Error>> {
  let mut json_data = String::new();
  let mut file = open_or_create_file("data.json").unwrap();

  file.read_to_string(&mut json_data)?;
  let activities: std::vec::Vec<Activity> = serde_json::from_str(json_data.as_str())?;
  Ok(activities)
}
