pub mod issue;
use issue::{Activity, Issue};
use serde_json;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::ErrorKind;
use std::io::Write;

fn main() {
    let mut json_data = String::new();
    let mut file = open_or_create_file("data.json").unwrap();

    file.read_to_string(&mut json_data).unwrap();
    let mut activities: std::vec::Vec<Activity> = serde_json::from_str(json_data.as_str()).unwrap();
    loop {
        if !handle_command(&mut activities) {
            break;
        }
    }
    let data = serde_json::to_string(&activities).unwrap();
    println!("{}", data);
    save_to_file(data).unwrap();
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
        "create issue" => {
            let activities_iter = activities.into_iter();
            for (index, activity) in activities_iter.enumerate() {
                println!(
                    "index: {:<20}categoria: {:<50}CH: {:4.2}",
                    index, activity.category, activity.planned_time
                );
                println!("descrition: {}", activity.description);
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
    let time_spent = get_input("Tempo previsto")?.parse::<f32>()?;
    let date = get_input("Data: ")?;
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

fn open_or_create_file(path: &str) -> std::io::Result<File> {
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(path)
        .unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("data.json")
                    .unwrap_or_else(|error| panic!("error creating file: {:?}", error))
            } else {
                panic!("Could not read from file");
            }
        });
    Ok(file)
}

fn save_to_file(data: String) -> std::io::Result<()> {
    let mut file = open_or_create_file("data.json")?;

    write!(file, "{}", data).unwrap();

    Ok(())
}
