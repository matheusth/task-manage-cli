pub mod archive;
pub mod cli;
pub mod issue;

use archive::*;
use clap::Parser;
use cli::*;
use issue::{Activity, WorkPlan};
use serde_json;

fn main() {
    open_or_create_file("data.json").unwrap();
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
            WorkPlanSubCommands::Close { plan_id } => {
                let mut workplan = workplans.get_mut(plan_id).unwrap();
                workplan.closed = true;
            }
            WorkPlanSubCommands::Show { closed } => {
                for workplan in workplans.iter() {
                    if !workplan.closed {
                        println!("{}", serde_json::to_string(workplan).unwrap());
                    }
                    if workplan.closed && closed {
                        println!("{}", serde_json::to_string(workplan).unwrap())
                    }
                }
            }
        },
        Entity::Activity(action) => {
            match action {
                ActivitySubCommands::Create(actv_args) => {
                    let plan_id = actv_args.workplan_id;
                    let activity = Activity::new(
                        actv_args.activity_type,
                        actv_args.description,
                        actv_args.carga_horaria,
                    );
                    let workplan = workplans.get_mut(plan_id).unwrap();
                    workplan.add_activity(activity);
                }
                ActivitySubCommands::Cancel {
                    workplan_id,
                    activity_id,
                } => {
                    let workplan = workplans.get_mut(workplan_id as usize).unwrap();
                    workplan.cancel_activity(activity_id);
                }
                ActivitySubCommands::Show {
                    workplan_id,
                    canceled,
                } => {
                    let workplan = workplans.get(workplan_id as usize).unwrap();
                    println!(
                        "start date: {}\t\t\tend_date: {}",
                        workplan.start_date, workplan.end_date
                    );
                    println!("---------------------------------------------------------------------------");
                    workplan.activities.iter().for_each(|activity| {
                        if !activity.canceled || canceled {
                            println!(
                                "category: {}\ndescription: {}\nplanned time: {}\ncanceled: {}",
                                activity.category, activity.description, activity.planned_time, activity.canceled
                            );

                            println!("---------------------------------------------------------------------------");
                        }
                    });
                }
            }
        }
    };
    save_to_file(&workplans).unwrap();
}
