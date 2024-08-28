use crate::{
    angebotsdaten::Angebotsdaten,
    participant::Participant,
    utils::{german_day_name, perform_signup},
};
use chrono::prelude::*;
use color_eyre::Result;
use futures::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SignupRequest {
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

#[derive(Debug)]
pub struct SignupData {
    pub signup_request: SignupRequest,
    pub participant: Participant,
    pub course_id: i32,
    pub date: NaiveDate,
}

pub async fn perform_signups(
    angebotsdaten: &Angebotsdaten,
    participant: &Participant,
    signup_requests: &mut Vec<SignupRequest>,
) -> Result<()> {
    let signup_datas: Vec<SignupData> = signup_requests
        .iter()
        .filter_map(|signup_request| {
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
                    log::info!("Course found: {course:?} for signup request: {signup_request:?}",);
                    Some(SignupData {
                        signup_request: signup_request.clone(),
                        participant: participant.clone(),
                        course_id: course.kursid,
                        date: signup_request.start_time.date(),
                    })
                }
                None => {
                    log::info!(
                        "No booking found for court on {} from {} to {}",
                        signup_request.start_time.date().format("%d.%m.%Y"),
                        signup_request.start_time.format("%H:%M"),
                        signup_request.end_time.format("%H:%M")
                    );
                    None
                }
            }
        })
        .collect();

    // Perform the signups concurrently
    let futures = signup_datas
        .into_iter()
        .map(perform_signup)
        .collect::<Vec<_>>();
    let results = future::join_all(futures).await;

    results.iter().for_each(|result| match result {
        Ok(signup_request) => {
            log::info!("Signup successful");
            signup_requests.retain(|r| r != signup_request);
        }
        Err(e) => log::error!("Signup failed: {:?}", e),
    });

    Ok(())
}
