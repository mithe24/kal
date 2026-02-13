use kal_cli::cli::commands::event::EventCommands;

use kal_core::application::services::EventService;


pub fn handle_event(
    cmd: EventCommands,
    service: EventService,
) -> Result<(), Box<dyn std::error::Error>> {

    match cmd {

        // ----------------------------
        // Create
        // ----------------------------
        EventCommands::Create {
            calendar_id,
            title,
            description,
            start,
            end,
            color,
            all_day,
        } => {

            service.create(
                calendar_id,
                title,
                description,
                start,
                end,
                color,
                all_day,
            )?;

            println!("Event created");
        }


        // ----------------------------
        // Cancel
        // ----------------------------
        EventCommands::Cancel { id } => {
            service.cancel(id)?;

            println!("Event cancelled");
        }


        // ----------------------------
        // Restore
        // ----------------------------
        EventCommands::Restore { id } => {
            service.restore(id)?;

            println!("Event restored");
        }


        // ----------------------------
        // Rename
        // ----------------------------
        EventCommands::Rename { id, title } => {
            service.rename(id, title)?;

            println!("Event renamed");
        }


        // ----------------------------
        // Set Description
        // ----------------------------
        EventCommands::SetDescription { id, description } => {
            service.set_description(id, description)?;

            println!("Description updated");
        }


        // ----------------------------
        // Set Time
        // ----------------------------
        EventCommands::SetTime { id, start, end } => {
            service.set_time(id, start, end)?;

            println!("Time updated");
        }


        // ----------------------------
        // List
        // ----------------------------
        EventCommands::List {
            calendar_id,
            include_cancelled,
        } => {

            let events =
                service.list(calendar_id, include_cancelled)?;

            for e in events {
                println!(
                    "{} | {} | {} → {}",
                    e.id(),
                    e.title(),
                    e.time_range().starts_at(),
                    e.time_range().ends_at(),
                );
            }
        }
    }

    Ok(())
}
