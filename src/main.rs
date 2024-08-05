extern crate pretty_env_logger;

mod angebotsdaten;
mod gender;
mod participant;
mod status;
mod utils;

use crate::participant::Participant;
use crate::utils::german_day_name;
use angebotsdaten::Angebotsdaten;
use chrono::prelude::*;
use chrono::NaiveDate;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use utils::perform_signup;

const COURSES_URL: &str = "https://unisport.koeln/e65/e35801/e35916/e35928/publicXMLData";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct SignupRequest {
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
}

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

    let xml_data = reqwest::get(COURSES_URL).await?.text().await?;
    let mut angebotsdaten: Angebotsdaten = serde_xml_rs::from_str(&xml_data)?;

    angebotsdaten
        .angebote
        .angebot
        .retain(|angebot| angebot.angebotsname == "Padel" && angebot.details == "Platzmiete");

    angebotsdaten
        .angebote
        .angebot
        .iter_mut()
        .for_each(|angebot| angebot.clean());

    // Parse the participant from the JSON file
    let participant_json = std::fs::read_to_string("data/participant.json")?;
    let participant: Participant = serde_json::from_str(&participant_json)?;

    // Parse the signup requests from the JSON file
    let signup_requests = std::fs::read_to_string("data/signups.json")?;
    let mut signup_requests: Vec<SignupRequest> = serde_json::from_str(&signup_requests)?;

    for signup_request in signup_requests.clone().iter() {
        let start_of_week = NaiveDate::from_isoywd_opt(
            signup_request.start_time.year(),
            signup_request.start_time.iso_week().week(),
            chrono::Weekday::Mon,
        )
        .expect("a valid date");

        let end_of_week = NaiveDate::from_isoywd_opt(
            signup_request.end_time.year(),
            signup_request.end_time.iso_week().week(),
            chrono::Weekday::Sun,
        )
        .expect("a valid date");

        let course = angebotsdaten.angebote.angebot.iter().find(|angebot| {
            angebot.frei != 0
                && angebot.zeitraum
                    == format!(
                        "{}-{}",
                        start_of_week.format("%d.%m."),
                        end_of_week.format("%d.%m.%y")
                    )
                && angebot.uhrzeit.contains(&format!(
                    "{}-{}",
                    signup_request.start_time.format("%H:%M"),
                    signup_request.end_time.format("%H:%M")
                ))
                && angebot
                    .tag
                    .contains(&german_day_name(signup_request.start_time.date()).to_string())
        });

        match course {
            Some(course) => {
                log::info!(
                    "Course found: {:?} for signup request: {:?}",
                    course,
                    signup_request
                );
                // perform_signup(
                //     &participant,
                //     course.kursid,
                //     signup_request.start_time.date(),
                // )
                // .await?;
                // Remove the signup request as we have successfully signed up
                signup_requests.retain(|r| r != signup_request);
            }
            None => {
                log::info!("No course found for signup request: {:?}", signup_request);
            }
        };
    }

    // Write the remaining signup requests back to signups.json
    let signup_requests = serde_json::to_string_pretty(&signup_requests)?;
    std::fs::write("data/signups.json", signup_requests)?;

    Ok(())
}
