pub mod archive;
pub mod cli;
pub mod issue;

use archive::*;
use clap::Parser;
use cli::*;
use issue::{Activity, WorkPlan};

fn main() {
    let mut workplans: Vec<WorkPlan> = load_from_file().unwrap();
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
            WorkPlanSubCommands::AddActivity(actv_args) => {
                let plan_id = actv_args.workplan_id;
                let activity = Activity::new(
                    actv_args.activity_type,
                    actv_args.description,
                    actv_args.carga_horaria,
                );
                let workplan = workplans.get_mut(plan_id).unwrap();
                workplan.add_activity(activity);
            }
            _ => panic!("Not implemented yet"),
        },
    };
    save_to_file(&workplans).unwrap();
}
