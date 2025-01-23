use chrono::{NaiveDate, NaiveTime};
use std::io::{self, Write};

mod schedule; // Import the schedule module
use schedule::{Ride, Schedule}; // Use the Ride and Schedule structures

fn main() {
    let mut schedule = Schedule::new(); // Create a new schedule

    loop {
        // Main menu
        println!("\n--- Ride Scheduler ---");
        println!("1. Add a new ride");
        println!("2. View all rides");
        println!("3. Search for a ride by time");
        println!("4. Exit");

        // Get user input
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => add_ride(&mut schedule),
            "2" => view_rides(&schedule),
            "3" => search_rides(&schedule),
            "4" => break,
            _ => println!("Invalid choice! Please try again."),
        }
    }
}

// Function to add a new ride
fn add_ride(schedule: &mut Schedule) {
    let mut date = String::new();
    let mut time = String::new();
    let mut destination = String::new();

    print!("Enter ride date (YYYY-MM-DD): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut date).unwrap();

    print!("Enter ride time (HH:MM): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut time).unwrap();

    print!("Enter destination: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut destination).unwrap();

    // Check if the input is valid
    if let Ok(date) = NaiveDate::parse_from_str(&date.trim(), "%Y-%m-%d") {
        if let Ok(time) = NaiveTime::parse_from_str(&time.trim(), "%H:%M") {
            schedule.add_ride(Ride {
                date,
                time,
                destination: destination.trim().to_string(),
            });
            println!("Ride added successfully!");
        } else {
            println!("Invalid time format.");
        }
    } else {
        println!("Invalid date format.");
    }
}

// Function to view all scheduled rides
fn view_rides(schedule: &Schedule) {
    println!("\nAll Scheduled Rides:");
    schedule.list_rides();
}

// Function to search for rides near a specific time
fn search_rides(schedule: &Schedule) {
    let mut time = String::new();
    print!("Enter time to search for (HH:MM): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut time).unwrap();

    if let Ok(time) = NaiveTime::parse_from_str(&time.trim(), "%H:%M") {
        println!("\nRides near {}:", time);
        schedule.search_rides(time);
    } else {
        println!("Invalid time format.");
    }
}
