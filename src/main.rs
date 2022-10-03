pub mod archive;
pub mod cli;
pub mod issue;

use archive::*;
use clap::Parser;
use cli::*;
use issue::WorkPlan;

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
    save_to_file(&workplans).unwrap();
}
