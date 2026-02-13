use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum EventCommands {
    /// Manage single events
    Event(EventCmd),

    /// Manage recurring events
    Recurring(RecurringCmd),

    /// Manage recurring occurrences
    Occurrence(OccurrenceCmd),
}

#[derive(Args)]
pub struct EventCmd {
    #[command(subcommand)]
    pub action: EventAction,
}

#[derive(Subcommand)]
pub enum EventAction {
    /// Create a new event
    Create {
        #[arg(short, long)]
        calendar_id: String,

        #[arg(short, long)]
        title: String,

        #[arg(short, long)]
        description: Option<String>,

        /// RFC3339 datetime
        #[arg(long)]
        start: String,

        /// RFC3339 datetime
        #[arg(long)]
        end: String,

        #[arg(long)]
        color: Option<String>,

        #[arg(long)]
        all_day: bool,
    },

    /// Cancel an event
    Cancel {
        #[arg(short, long)]
        id: String,
    },

    /// Restore a cancelled event
    Restore {
        #[arg(short, long)]
        id: String,
    },

    /// Rename an event
    Rename {
        #[arg(short, long)]
        id: String,

        #[arg(short, long)]
        title: String,
    },

    /// Update description
    SetDescription {
        #[arg(short, long)]
        id: String,

        #[arg(short, long)]
        description: Option<String>,
    },

    /// Update time range
    SetTime {
        #[arg(short, long)]
        id: String,

        #[arg(long)]
        start: String,

        #[arg(long)]
        end: String,
    },

    /// Update color
    SetColor {
        #[arg(short, long)]
        id: String,

        #[arg(long)]
        color: String,
    },

    /// List events
    List {
        #[arg(long)]
        calendar_id: Option<String>,

        #[arg(long)]
        include_cancelled: bool,
    },
}

#[derive(Args)]
pub struct RecurringCmd {
    #[command(subcommand)]
    pub action: RecurringAction,
}

#[derive(Subcommand)]
pub enum RecurringAction {
    /// Create recurring event
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
        frequency: String,

        #[arg(long)]
        interval: u32,

        #[arg(long)]
        until: Option<String>,

        #[arg(long)]
        color: Option<String>,

        #[arg(long)]
        all_day: bool,
    },

    /// Cancel recurring event
    Cancel {
        #[arg(short, long)]
        id: String,
    },

    /// Restore recurring event
    Restore {
        #[arg(short, long)]
        id: String,
    },

    /// List recurring events
    List {
        #[arg(long)]
        calendar_id: Option<String>,
    },
}

#[derive(Args)]
pub struct OccurrenceCmd {
    #[command(subcommand)]
    pub action: OccurrenceAction,
}

#[derive(Subcommand)]
pub enum OccurrenceAction {
    /// Cancel one occurrence
    Cancel {
        #[arg(short, long)]
        event_id: String,

        /// Original start time (RFC3339)
        #[arg(long)]
        at: String,
    },

    /// Restore one occurrence
    Restore {
        #[arg(short, long)]
        event_id: String,

        #[arg(long)]
        at: String,
    },

    /// Reschedule one occurrence
    Reschedule {
        #[arg(short, long)]
        event_id: String,

        #[arg(long)]
        at: String,

        #[arg(long)]
        new_start: String,

        #[arg(long)]
        new_end: String,
    },
}

