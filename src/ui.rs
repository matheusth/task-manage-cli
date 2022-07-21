use crate::issue::{Activity, Issue};
use std::error::Error;
use std::io::Write;

pub fn print_tasks(activities: &std::vec::Vec<Activity>) {
  let activities_iter = activities.into_iter();
  for (index, activity) in activities_iter.enumerate() {
    println!(
      "indice: {:<20}categoria: {:<40}Carga Horária: {:4.2}",
      index, activity.category, activity.planned_time
    );
    println!("descrição: {}", activity.description);
    print!("tarefas: ");
    for issue in &activity.issues {
      print!("{}; ", issue.title);
    }
    println!("\n-------------------------------------------------------------------------------------------------------");
  }
}

pub fn create_activity() -> Result<Activity, Box<dyn Error>> {
  let category = get_input("Categoria: ")?;
  let description = get_input("Descrição: ")?;
  let planned_time = get_input("Tempo previsto: ")?.parse::<f32>()?;

  Ok(Activity::new(category, description, planned_time))
}

pub fn select_activity() -> usize {
  return get_input("Índice da atividade: ")
    .unwrap()
    .parse::<usize>()
    .unwrap();
}

pub fn create_issue(activity: &mut Activity) -> Result<(), Box<dyn Error>> {
  let title = get_input("Title:")?;
  let description = get_input("Descrição: ")?;
  let time_spent = get_input("Tempo gasto: ")?.parse::<f32>()?;
  let date = get_input("Data de execução: ")?;
  activity.add_issue(Issue::new(title, description, time_spent, date));

  Ok(())
}

pub fn get_input(message: &str) -> std::io::Result<String> {
  let mut line = String::new();

  print!("{}", message);
  std::io::stdout().flush().expect("Failed to flush stdout");
  std::io::stdin().read_line(&mut line)?;

  Ok(String::from(line.trim()))
}
