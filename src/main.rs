pub mod issue;

use issue::{Activity, Issue};
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::io::Write;

fn main() {
    let mut activities: std::vec::Vec<Activity> = std::vec::Vec::new();

    loop {
        if !handle_command(&mut activities) {
            break;
        }
    }
    let data = serde_json::to_string(&activities).unwrap();
    println!("{}", data);
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

fn get_input(message: &str) -> std::io::Result<String> {
    let mut line = String::new();

    print!("{}", message);
    std::io::stdout().flush().expect("Failed to flush stdout");
    std::io::stdin().read_line(&mut line)?;

    Ok(String::from(line.trim()))
}
