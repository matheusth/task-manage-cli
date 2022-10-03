pub mod archive;
pub mod cli;
pub mod issue;
pub mod ui;

use archive::*;
use clap::Parser;
use cli::*;
use issue::{Activity, WorkPlan};
use ui::*;

fn main() {
    let mut workplans: Vec<WorkPlan> = Vec::new();
    let args = CliArgs::parse();
    match args.entity {
        Entity::WorkPlan(action) => match action {
            WorkPlanSubCommands::Create(workplan_info) => {
                let workplan = WorkPlan::new(
                    workplans.len().try_into().unwrap(),
                    workplan_info.start_date.as_str(),
                    workplan_info.end_date.as_str(),
                );

                workplans.push(workplan);
            }
            _ => panic!("Not implemented yet"),
        },
    };
    save_to_file(&workplans);
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
