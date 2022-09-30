use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct CliArgs {}

#[derive(Debug, Subcommand)]
enum Entity {
    #[clap(subcommand)]
    WorkPlan(WorkPlanSubCommands),
}
#[derive(Debug, Subcommand)]
enum WorkPlanSubCommands {
    Create(CreateWorkPlanArgs),
    AddActivity(AddActivityArgs),
    CancelActivity { id_workplan: u32, id_activity: u32 },
    Close { id: u32 },
    Show,
}

#[derive(Debug, Args)]
struct CreateWorkPlanArgs {
    start_date: String,
    end_date: String,
}

#[derive(Debug, Args)]
struct AddActivityArgs {
    workplan_id: u32,
    description: String,
    activity_type: String,
    carga_horaria: String,
}

#[derive(Debug, Args)]
struct DeliverArgs {
    delivered_at: String,
    hours_spent: u32,
    description: String,
}
