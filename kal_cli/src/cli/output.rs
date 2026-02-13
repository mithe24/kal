use colored::Colorize;
use chrono::{DateTime, Utc};
use kal_core::domain::{
    Calendar,
    Event,
    RecurringEvent,
};

pub fn print_calendar(calendar: &Calendar) {
    println!("{}", "Calendar Details".bold().blue());
    println!("{}: {}", "ID".bold(), calendar.id());
    println!("{}: {}", "Name".bold(), calendar.name());

    if let Some(desc) = calendar.description() {
        println!("{}: {}", "Description".bold(), desc);
    }

    println!("{}: {}", "Archived".bold(), 
        if *calendar.is_archived() { "Yes".red() } else { "No".green() }
    );
    println!("{}: {}", "Created".bold(), format_datetime(calendar.created_at()));
    println!("{}: {}", "Updated".bold(), format_datetime(calendar.updated_at()));
}

pub fn print_calendar_list(calendars: &[Calendar]) {
    if calendars.is_empty() {
        println!("{}", "No calendars found.".yellow());
        return;
    }

    println!("\n{}", "Calendars".bold().blue());
    println!("{}", "=".repeat(80));

    for calendar in calendars {
        print!("{} ", calendar.id().to_string().cyan());
        print!("{}", calendar.name().bold());

        if *calendar.is_archived() {
            print!(" {}", "[ARCHIVED]".red());
        }

        if let Some(desc) = calendar.description() {
            print!(" - {}", desc.dimmed());
        }

        println!();
    }

    println!("\nTotal: {}", calendars.len());
}

pub fn print_event(event: &Event) {
    println!("{}", "Event Details".bold().blue());
    println!("{}: {}", "ID".bold(), event.id());
    println!("{}: {}", "Calendar".bold(), event.calendar_id());
    println!("{}: {}", "Title".bold(), event.title());

    if let Some(desc) = event.description() {
        println!("{}: {}", "Description".bold(), desc);
    }

    println!("{}: {}", "Start".bold(), format_datetime(
        &event.time_range().starts_at())
    );
    println!("{}: {}", "End".bold(), format_datetime(
        &event.time_range().ends_at())
    );
    println!("{}: {}", "Color".bold(), u8::from(*event.color()));
    println!("{}: {}", "All Day".bold(), *event.is_all_day());
    println!("{}: {}", "Cancelled".bold(), 
        if *event.is_cancelled() { "Yes".red() } else { "No".green() }
    );
    println!("{}: {}", "Created".bold(), format_datetime(event.created_at()));
    println!("{}: {}", "Updated".bold(), format_datetime(event.updated_at()));
}

pub fn print_event_list(events: &[Event]) {
    if events.is_empty() {
        println!("{}", "No events found.".yellow());
        return;
    }

    println!("\n{}", "Events".bold().blue());
    println!("{}", "=".repeat(80));

    for event in events {
        print!("{} ", event.id().to_string().cyan());
        print!("{}", event.title().bold());

        if *event.is_cancelled() {
            print!(" {}", "[CANCELLED]".red());
        }

        println!();
        println!("  {} → {}", 
            format_datetime(&event.time_range().starts_at()).dimmed(),
            format_datetime(&event.time_range().ends_at()).dimmed()
        );

        if let Some(desc) = event.description() {
            println!("  {}", desc.dimmed());
        }

        println!();
    }

    println!("Total: {}", events.len());
}

pub fn print_recurring_event(event: &RecurringEvent) {
    println!("{}", "Recurring Event Details".bold().blue());
    println!("{}: {}", "ID".bold(), event.id());
    println!("{}: {}", "Calendar".bold(), event.calendar_id());
    println!("{}: {}", "Title".bold(), event.title());

    if let Some(desc) = event.description() {
        println!("{}: {}", "Description".bold(), desc);
    }

    println!("{}: {}", "Start".bold(), format_datetime(
        &event.time_range().starts_at())
    );
    println!("{}: {}", "End".bold(), format_datetime(
        &event.time_range().ends_at())
    );
    println!("{}: {}", "Frequency".bold(), event.rule().frequency());
    println!("{}: {}", "Interval".bold(), event.rule().interval());

    if let Some(until) = event.rule().until() {
        println!("{}: {}", "Until".bold(), format_datetime(&until));
    } else {
        println!("{}: {}", "Until".bold(), "Forever");
    }

    println!("{}: {}", "Color".bold(), u8::from(*event.color()));
    println!("{}: {}", "All Day".bold(), *event.is_all_day());
    println!("{}: {}", "Cancelled".bold(), 
        if *event.is_cancelled() { "Yes".red() } else { "No".green() }
    );

    if !event.exceptions().is_empty() {
        println!("\n{}: {}", "Exceptions".bold(), event.exceptions().len());
    }

    println!("{}: {}", "Created".bold(), format_datetime(event.created_at()));
    println!("{}: {}", "Updated".bold(), format_datetime(event.updated_at()));
}

pub fn print_recurring_event_list(events: &[RecurringEvent]) {
    if events.is_empty() {
        println!("{}", "No recurring events found.".yellow());
        return;
    }

    println!("\n{}", "Recurring Events".bold().blue());
    println!("{}", "=".repeat(80));

    for event in events {
        print!("{} ", event.id().to_string().cyan());
        print!("{}", event.title().bold());

        if *event.is_cancelled() {
            print!(" {}", "[CANCELLED]".red());
        }

        println!();
        println!("  {} every {} {}", 
            format_datetime(&event.time_range().starts_at()).dimmed(),
            event.rule().interval(),
            event.rule().frequency().to_string().dimmed()
        );

        if let Some(desc) = event.description() {
            println!("  {}", desc.dimmed());
        }

        println!();
    }

    println!("Total: {}", events.len());
}

pub fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message);
}

pub fn print_warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message);
}

fn format_datetime(dt: &DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

pub fn confirm(prompt: &str) -> bool {
    use std::io::{self, Write};

    print!("{} [y/N]: ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}
