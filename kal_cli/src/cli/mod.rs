use clap::{Parser, Subcommand};

pub mod commands;
pub mod output;

#[derive(Parser)]
#[command(name = "kal")]
#[command(about = "A calendar management application", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Calendar management commands
    Calendar {
        #[command(subcommand)]
        action: commands::calendar::CalendarCommands,
    },

    /// Event management commands
    Event {
        #[command(subcommand)]
        action: EventCommands,
    },

    /// Recurring event management commands
    Recurring {
        #[command(subcommand)]
        action: RecurringCommands,
    },
}

#[derive(Subcommand)]
pub enum EventCommands {
    /// Create a new event
    Create {
        #[arg(short, long)]
        calendar_id: String,

        #[arg(short, long)]
        title: String,

        #[arg(short, long)]
        description: Option<String>,

        #[arg(long)]
        start: String,

        #[arg(long)]
        end: String,

        #[arg(long)]
        color: Option<String>,
    },

    /// Update event title
    UpdateTitle {
        #[arg(short, long)]
        event_id: String,
        
        #[arg(short, long)]
        title: String,
    },

    /// Cancel an event
    Cancel {
        #[arg(short, long)]
        event_id: String,
    },

    /// Delete an event
    Delete {
        #[arg(short, long)]
        event_id: String,
    },

    /// Restore a cancelled event
    Restore {
        #[arg(short, long)]
        event_id: String,
    },
}

#[derive(Subcommand)]
pub enum RecurringCommands {
    /// Create a recurring event
    Create {
        #[arg(short, long)]
        calendar_id: String,

        #[arg(short, long)]
        title: String,

        #[arg(short, long)]
        pattern: String, // "daily", "weekly", "monthly", yearly
    },

    /// Cancel a recurring event
    Cancel {
        #[arg(short, long)]
        event_id: String,
    },

    /// Cancel a single occurrence
    CancelOccurrence {
        #[arg(short, long)]
        event_id: String,

        #[arg(short, long)]
        date: String,
    },
}
