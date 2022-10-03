use crate::issue::{Activity, WorkPlan};
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
                File::create(path)
                    .unwrap_or_else(|error| panic!("error creating file: {:?}", error))
            } else {
                panic!("Could not read from file");
            }
        });
    Ok(file)
}

pub fn save_to_file(activities: &std::vec::Vec<WorkPlan>) -> std::io::Result<()> {
    let mut file = open_or_create_file("data.json")?;
    file.set_len(0)?;
    let data = serde_json::to_string(activities)?;
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

pub fn generate_html(activity: &Activity) -> String {
    let mut html_output = String::from(
        r#"
  <table border="1" style="max-width: 23cm;">
  <tr>
  <th>titulo</th>
  <th>descrição</th>
  <th>Tempo gasto</th>
  <th>data</td>
  </tr>
  "#,
    );
    for issue in &activity.issues {
        let line = format!(
            r#"
        <tr>
          <td>{}</td>
          <td>{}</td>
          <td>{}</td>
          <td>{}</td>
        </tr>
    "#,
            issue.title, issue.description, issue.time_spent, issue.date
        );
        html_output.push_str(line.as_str());
    }
    html_output.push_str("</table>");

    return html_output;
}
pub fn export_to_html(activity: &Activity) -> std::io::Result<()> {
    let file_name = format!("{}.html", activity.description);
    let mut html_file = open_or_create_file(file_name.as_str())?;
    let html = generate_html(activity);
    write!(&mut html_file, "{}", html)?;
    Ok(())
}
