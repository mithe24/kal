use clap::Subcommand;

#[derive(Subcommand)]
pub enum CalendarCommands {
    /// Create a new calendar
    Create {
        #[arg(short = 'n', long)]
        name: String,

        #[arg(short = 'd', long)]
        description: Option<String>,
    },

    /// Delete a calendar
    Delete {
        #[arg(short, long)]
        id: String,
    },

    /// Rename a calendar
    Rename {
        #[arg(short, long)]
        id: String,

        #[arg(short = 'n', long)]
        name: String,
    },

    /// Archive a calendar
    Archive {
        #[arg(short, long)]
        id: String,
    },

    /// Unarchive a calendar
    Unarchive {
        #[arg(short, long)]
        id: String,
    },

    /// Update calendar description
    SetDescription {
        #[arg(short, long)]
        id: String,

        /// Use empty string to clear
        #[arg(short = 'd', long)]
        description: Option<String>,
    },

    /// List all calendars
    List {
        /// Include archived calendars
        #[arg(long, default_value_t = false)]
        include_archived: bool,
    },
}
