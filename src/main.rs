pub mod archive;
pub mod issue;
pub mod ui;

use archive::*;
use issue::Activity;
use ui::*;

fn main() {
    let mut activities: std::vec::Vec<Activity> = load_from_file().unwrap_or(std::vec::Vec::new());
    loop {
        if !handle_command(&mut activities) {
            break;
        }
    }
    save_to_file(&activities).unwrap();
}

fn handle_command(activities: &mut std::vec::Vec<Activity>) -> bool {
    let option = get_input("Commando >> ").expect("Failed to read command");
    match option.as_str() {
        "criar atividade" => match create_activity() {
            Ok(activity) => {
                activities.push(activity);
                true
            }
            _ => false,
        },
        "remover atividade" => {
            print_tasks(&activities);
            activities.remove(select_activity());
            true
        }
        "listar atividades" => {
            print_tasks(&activities);
            true
        }
        "exportar html" => match export_to_html(&activities[select_activity()]) {
            Ok(_) => true,
            _ => false,
        },
        "add tarefa" => {
            print_tasks(&activities);
            match create_issue(&mut activities[select_activity()]) {
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
teste2
