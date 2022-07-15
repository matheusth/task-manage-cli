#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn issue_new_return_a_issue() {
    let title = String::from("title");
    let description = String::from("description");
    let issue = Issue::new(&title, &description, 12.4, String::from("22/07/2021"));

    assert_eq!(description, issue.description);
    assert_eq!(title, issue.title);
    assert_eq!(12.4, issue.time_spent);
    assert_eq!("22/07/2021", issue.date.as_str());
  }
}

pub struct Issue {
  title: String,
  description: String,
  time_spent: f32,
  date: String,
}

impl Issue {
  pub fn new(title: &String, description: &String, time_spent: f32, date: String) -> Self {
    Issue {
      title: title.clone(),
      description: description.clone(),
      time_spent,
      date,
    }
  }
}
