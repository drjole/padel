extern crate pretty_env_logger;

mod angebotsdaten;
mod gender;
mod participant;
mod signup;
mod status;
mod utils;

use std::time::Duration;

use crate::angebotsdaten::Angebotsdaten;
use crate::participant::Participant;
use crate::signup::{perform_signups, SignupRequest};
use chrono::{prelude::*, TimeDelta};
use color_eyre::Result;

const COURSES_URL: &str = "https://unisport.koeln/e65/e35801/e35916/e35928/publicXMLData";

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().map_or_else(
        |_| println!("did not initialize dotenv"),
        |path| {
            println!(
                "initialized dotenv from: {}",
                path.to_str().unwrap_or("unknown")
            );
        },
    );

    pretty_env_logger::init_timed();

    let now = Local::now().naive_local();

    // Parse the participant from the JSON file
    let participant_json = std::fs::read_to_string("data/participant.json")?;
    let participant: Participant = serde_json::from_str(&participant_json)?;

    // Parse the signup requests from the JSON file
    let signup_requests_json = std::fs::read_to_string("data/signups.json")?;
    let mut signup_requests: Vec<SignupRequest> = serde_json::from_str(&signup_requests_json)?;

    // Remove signup requests from the past
    signup_requests.retain(|signup_request| signup_request.start_time.date() >= now.date());

    // Remove signup requests that are less than 36 hours in the future
    // (24 hours cancellation period + 12 hours buffer)
    signup_requests.retain(|signup_request| signup_request.start_time - now > TimeDelta::hours(36));

    if signup_requests.is_empty() {
        log::info!("No signups to perform");
        return Ok(());
    }

    // Print the remaining signup requests
    for signup_request in signup_requests.iter() {
        log::info!(
            "Trying to signup for court on {} from {} to {}",
            signup_request.start_time.date().format("%d.%m.%Y"),
            signup_request.start_time.format("%H:%M"),
            signup_request.end_time.format("%H:%M")
        );
    }

    // Query the courses
    let xml_data = reqwest::get(COURSES_URL).await?.text().await?;
    let mut angebotsdaten: Angebotsdaten = serde_xml_rs::from_str(&xml_data)?;

    // Filter for Padel Platzmiete
    angebotsdaten
        .angebote
        .angebot
        .retain(|angebot| angebot.angebotsname == "Padel" && angebot.details == "Platzmiete");

    // Clean up the data
    angebotsdaten
        .angebote
        .angebot
        .iter_mut()
        .for_each(|angebot| angebot.clean());

    // Perform the signups
    perform_signups(&angebotsdaten, &participant, &mut signup_requests).await?;

    // Write the remaining signup requests back to signups.json
    let signup_requests = serde_json::to_string_pretty(&signup_requests)?;
    std::fs::write("data/signups.json", signup_requests)?;

    Ok(())
}
