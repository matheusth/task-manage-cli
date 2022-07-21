pub mod archive;
pub mod issue;

use archive::*;
use issue::{Activity, Issue};
use std::error::Error;
use std::io::Write;

fn main() {
    let mut activities: std::vec::Vec<Activity> = load_from_file().unwrap_or_else(|error| {
        panic!("Faield on loading file!\nError: {}", error);
    });
    loop {
        if !handle_command(&mut activities) {
            break;
        }
    }
    save_to_file(&activities).unwrap();
}

fn handle_command(activities: &mut std::vec::Vec<Activity>) -> bool {
    let opcao = get_input("Commando >> ").expect("Failed to read command");
    match opcao.as_str() {
        "criar atividade" => match create_activity() {
            Ok(activity) => {
                activities.push(activity);
                true
            }
            _ => false,
        },
        "add tarefa" => {
            let activities_iter = activities.into_iter();
            for (index, activity) in activities_iter.enumerate() {
                println!(
                    "indice: {:<20}categoria: {:<50}Carga Horária: {:4.2}",
                    index, activity.category, activity.planned_time
                );
                println!("descrição: {}", activity.description);
                println!(
                    "=========================================================================="
                );
            }
            let index = get_input("Inidice da atividade: ")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            match create_issue(&mut activities[index]) {
                Ok(_) => true,
                _ => false,
            }
        }
        "salvar" => match save_to_file(&activities) {
            Ok(_) => true,
            _ => false,
        },
        "sair" => false,
        _ => true,
    }
}
fn create_activity() -> Result<Activity, Box<dyn Error>> {
    let category = get_input("Categoria: ")?;
    let description = get_input("Descrição: ")?;
    let planned_time = get_input("Tempo previsto: ")?.parse::<f32>()?;

    Ok(Activity::new(category, description, planned_time))
}
fn create_issue(activity: &mut Activity) -> Result<(), Box<dyn Error>> {
    let title = get_input("Title:")?;
    let description = get_input("Descrição: ")?;
    let time_spent = get_input("Tempo gasto: ")?.parse::<f32>()?;
    let date = get_input("Data de execução: ")?;
    activity.add_issue(Issue::new(title, description, time_spent, date));

    Ok(())
}

fn get_input(message: &str) -> std::io::Result<String> {
    let mut line = String::new();

    print!("{}", message);
    std::io::stdout().flush().expect("Failed to flush stdout");
    std::io::stdin().read_line(&mut line)?;

    Ok(String::from(line.trim()))
}
