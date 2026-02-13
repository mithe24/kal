use crate::cli::commands::calendar::CalendarCommands;

use kal_core::application::{commands::calendars::CreateCalendarCommand};

pub fn handle_calendar(
    cmd: CalendarCommands,
    service: CalendarService,
) -> Result<(), Box<dyn std::error::Error>> {

    match cmd {
        // ----------------------------
        // Create
        // ----------------------------
        CalendarCommands::Create { name, description } => {

            println!("Calendar created");
        }

        // ----------------------------
        // Delete
        // ----------------------------
        CalendarCommands::Delete { id } => {
            service.delete(id)?;

            println!("Calendar deleted");
        }

        // ----------------------------
        // Rename
        // ----------------------------
        CalendarCommands::Rename { id, name } => {
            service.rename(id, name)?;

            println!("Calendar renamed");
        }

        // ----------------------------
        // Archive
        // ----------------------------
        CalendarCommands::Archive { id } => {
            service.archive(id)?;

            println!("Calendar archived");
        }

        // ----------------------------
        // Unarchive
        // ----------------------------
        CalendarCommands::Unarchive { id } => {
            service.unarchive(id)?;

            println!("Calendar restored");
        }

        // ----------------------------
        // Description
        // ----------------------------
        CalendarCommands::SetDescription { id, description } => {
            service.set_description(id, description)?;

            println!("Description updated");
        }

        // ----------------------------
        // List
        // ----------------------------
        CalendarCommands::List { include_archived } => {
            let calendars =
                service.list(include_archived)?;

            for cal in calendars {
                println!(
                    "{} | {} | archived={}",
                    cal.id(),
                    cal.name(),
                    cal.is_archived(),
                );
            }
        }
    }

    Ok(())
}
