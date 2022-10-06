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

/// Commands to manage workplans
#[derive(Debug, Subcommand)]
pub enum WorkPlanSubCommands {
    /// Create a Work plan
    Create(CreateWorkPlanArgs),
    /// Add an activity to a work plan
    AddActivity(AddActivityArgs),
    /// Close a work plan who already been delivered
    Close { plan_id: usize },
    /// Show work plans
    Show {
        /// Show closed workplans
        #[arg(short, long, default_value_t = false)]
        closed: bool,
    },
}

#[derive(Debug, Args)]
pub struct CreateWorkPlanArgs {
    /// Start of the execution of the work plan
    pub start_date: String,
    /// Last to finish the execution of the work plan
    pub end_date: String,
}

#[derive(Debug, Args)]
pub struct AddActivityArgs {
    /// Id of the WorkPlan you want to add the activity
    pub workplan_id: usize,
    /// Description of the activity
    pub description: String,
    /// Type or Category of the activity
    pub activity_type: String,
    /// Time you plan to spend in the activity
    pub carga_horaria: f32,
}

#[derive(Debug, Args)]
pub struct DeliverArgs {
    pub delivered_at: String,
    pub hours_spent: u32,
    pub description: String,
}
