use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub entity: Entity,
}

#[derive(Debug, Subcommand)]
pub enum Entity {
    #[clap(subcommand)]
    WorkPlan(WorkPlanSubCommands),
}
#[derive(Debug, Subcommand)]
pub enum WorkPlanSubCommands {
    Create(CreateWorkPlanArgs),
    AddActivity(AddActivityArgs),
    CancelActivity { id_workplan: u32, id_activity: u32 },
    Close { id: u32 },
    Show,
}

#[derive(Debug, Args)]
pub struct CreateWorkPlanArgs {
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Args)]
pub struct AddActivityArgs {
    pub workplan_id: u32,
    pub description: String,
    pub activity_type: String,
    pub carga_horaria: String,
}

#[derive(Debug, Args)]
pub struct DeliverArgs {
    pub delivered_at: String,
    pub hours_spent: u32,
    pub description: String,
}
