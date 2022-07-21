use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn issue_new_return_a_issue() {
    let title = String::from("title");
    let description = String::from("description");
    let issue = Issue::new(
      title.clone(),
      description.clone(),
      12.4,
      String::from("22/07/2021"),
    );

    assert_eq!(description, issue.description);
    assert_eq!(title, issue.title);
    assert_eq!(12.4, issue.time_spent);
    assert_eq!("22/07/2021", issue.date.as_str());
  }

  #[test]
  fn add_issue_to_activity() {
    let title = String::from("title");
    let description = String::from("description");
    let issue = Issue::new(
      title.clone(),
      description.clone(),
      12.4,
      String::from("22/07/2021"),
    );
    let mut activity = Activity::new(
      String::from("Atendimento ao Usuário"),
      String::from("descrição"),
      40.0,
    );

    activity.add_issue(issue);
    assert_eq!(activity.issues.len(), 1);
  }
  #[test]
  fn remove_issue_from_activity() {
    let title = String::from("title");
    let description = String::from("description");
    let issue = Issue::new(
      title.clone(),
      description.clone(),
      12.4,
      String::from("22/07/2021"),
    );
    let mut activity = Activity::new(
      String::from("Atendimento ao Usuário"),
      String::from("descrição"),
      40.0,
    );
    activity.add_issue(issue);
    activity.remove_issue(0);
    assert_eq!(activity.issues.len(), 0);
  }
}

#[derive(Serialize, Deserialize)]
pub struct Issue {
  pub title: String,
  pub description: String,
  pub time_spent: f32,
  pub date: String,
}

impl Issue {
  pub fn new(title: String, description: String, time_spent: f32, date: String) -> Self {
    Issue {
      title,
      description,
      time_spent,
      date,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Activity {
  pub category: String,
  pub description: String,
  pub planned_time: f32,
  pub issues: std::vec::Vec<Issue>,
}

impl Activity {
  pub fn new(category: String, description: String, planned_time: f32) -> Activity {
    Activity {
      category,
      description,
      planned_time,
      issues: Vec::new(),
    }
  }
  pub fn add_issue(&mut self, issue: Issue) {
    self.issues.push(issue);
  }
  pub fn remove_issue(&mut self, issue_index: usize) {
    self.issues.swap_remove(issue_index);
  }

  pub fn print(&self) {
    println!(
      "{}",
      serde_json::to_string(&self).expect("Could not serialize activity")
    );
  }
}
