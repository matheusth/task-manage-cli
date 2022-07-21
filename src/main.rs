pub mod archive;
pub mod issue;
pub mod ui;

use archive::*;
use issue::Activity;
use ui::*;

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
            print_tasks(&activities);
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
