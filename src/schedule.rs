use chrono::{NaiveDate, NaiveTime};
use std::collections::HashMap;

// Structure to represent a single ride
pub struct Ride {
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub destination: String,
}

// Structure to hold the schedule of rides
pub struct Schedule {
    rides: HashMap<String, Vec<Ride>>, // Key is the date, value is a list of rides
}

impl Schedule {
    // Create a new, empty schedule
    pub fn new() -> Self {
        Schedule {
            rides: HashMap::new(),
        }
    }

    // Add a new ride to the schedule
    pub fn add_ride(&mut self, ride: Ride) {
        let key = ride.date.to_string();
        self.rides.entry(key).or_insert(vec![]).push(ride);
    }

    // List all rides in the schedule
    pub fn list_rides(&self) {
        for (date, rides) in &self.rides {
            println!("\nDate: {}", date);
            for ride in rides {
                println!(
                    "- Time: {}, Destination: {}",
                    ride.time, ride.destination
                );
            }
        }
    }

    // Search for rides within 30 minutes of a given time
    pub fn search_rides(&self, time: NaiveTime) {
        for (_date, rides) in &self.rides {
            for ride in rides.iter().filter(|r| (r.time - time).num_minutes().abs() <= 30) {
                println!(
                    "- Date: {}, Time: {}, Destination: {}",
                    ride.date, ride.time, ride.destination
                );
            }
        }
    }
}
