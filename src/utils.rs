use crate::signup::{SignupData, SignupRequest};
use chrono::Datelike;
use color_eyre::{eyre::eyre, Result};
use encoding::{all::ISO_8859_1, Encoding};
use form_urlencoded::byte_serialize;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::RequestBuilder;
use scraper::{ElementRef, Html};
use tokio::time::{sleep, Duration};

const SIGNUP_URL: &str = "https://isis.verw.uni-koeln.de/cgi/anmeldung.fcgi";

fn get_success_response_regex() -> Regex {
    #[allow(clippy::expect_used)]
    Regex::new(r"Sie haben sich verbindlich für das Angebot Nr. \d+ angemeldet.")
        .expect("invalid regex")
}

lazy_static! {
    static ref SUCCESS_RESPONSE_REGEX: Regex = get_success_response_regex();
}

pub async fn perform_signup(signup_data: SignupData) -> Result<SignupRequest> {
    let client = reqwest::Client::new();
    let form_url = String::from("https://isis.verw.uni-koeln.de/cgi/anmeldung.fcgi");

    // Step 1: Get the signup page that contains session specific data
    let params = vec![
        ("Kursid".to_string(), signup_data.course_id.to_string()),
        (
            "Termin".to_string(),
            signup_data.date.format("%Y-%m-%d").to_string(),
        ),
        ("submit".to_string(), "weiter zur Buchung".to_string()),
    ];
    let body = request_body_from_params(params)?;

    let request = client.post(&form_url).body(body);
    let response = request_document(request).await?;
    sleep(Duration::from_secs(3)).await;
    // We need a scope here... https://github.com/causal-agent/scraper/issues/75#issuecomment-1076997293
    let body = {
        let document = scraper::Html::parse_document(response.as_str());
        let form = parse_form(&document)?;
        let mut params = params_from_form(form, false)?;
        let participant_params = signup_data.participant.as_payload();
        for (key, value) in participant_params {
            params.push((key, value));
        }
        request_body_from_params(params)?
    };

    // Step 2: Submit the initial form and get the user confirmation page in response
    let mut request = client
        .post(SIGNUP_URL)
        .header("Referer", &form_url)
        .body(body);
    request = add_headers(request);
    let response = request_document(request).await?;
    sleep(Duration::from_secs(3)).await;
    // We need a scope here... https://github.com/causal-agent/scraper/issues/75#issuecomment-1076997293
    let body = {
        let document = scraper::Html::parse_document(response.as_str());
        let form = parse_form(&document)?;
        let mut params = params_from_form(form, true)?;
        // Add the Kontoinh param
        params.push((
            "Kontoinh".into(),
            "nur ändern, falls nicht mit Teilnehmer identisch".into(),
        ));
        // Add this parameter to "confirm" the signup
        params.push(("submit".into(), "verbindliche Buchung".into()));
        request_body_from_params(params)?
    };

    // Step 3: Finalize the signup
    let mut request = client
        .post(SIGNUP_URL)
        .header("Referer", SIGNUP_URL)
        .body(body);
    request = add_headers(request);

    // Error handling
    match request_document(request).await {
        Ok(response) => {
            let html = scraper::Html::parse_document(response.as_str()).html();
            if SUCCESS_RESPONSE_REGEX.is_match(html.as_str())
                || html.contains(
                    "Bitte geben Sie Ihre Emailadresse ein, um Ihre Buchungsbestätigung abzurufen",
                )
            {
                Ok(signup_data.signup_request)
            } else if html.contains("Für die Buchung dieses Angebots")
                && html.contains("müssen Sie vorher eines folgender Angebote gebucht haben")
                && html.contains("Sportticket")
            {
                Err(eyre!("Kein Sportticket oder fehlerhafte Daten."))
            } else if html.contains("Ihre Buchung konnte leider nicht ausgeführt werden")
                && html.contains("da Sie für diesen Kurs bereits angemeldet sind")
            {
                Err(eyre!("Bereits angemeldet."))
            } else {
                log::error!("response: {html}");
                Err(eyre!("Unbekannter Fehler."))
            }
        }
        Err(err) => Err(err.wrap_err("Verbindungsfehler")),
    }
}

pub async fn request_document(builder: RequestBuilder) -> Result<String> {
    let response = builder.send().await.map_err(|err| {
        log::error!("request error: {err}");
        eyre!("Verbindungsfehler: {err}")
    })?;
    if response.status() != 200 {
        log::error!("got response with code {}", response.status());
        return Err(eyre!(
            "Server hat mit Code {} geantwortet",
            response.status()
        ));
    }
    Ok(response.text().await?)
}

pub fn parse_form(document: &Html) -> Result<ElementRef> {
    let form_selector =
        scraper::Selector::parse("form").map_err(|e| eyre!("scraper error: {e}"))?;
    let form_element = document
        .select(&form_selector)
        .next()
        .ok_or_else(|| eyre!("no form found"))?;
    Ok(form_element)
}

pub fn add_headers(request: RequestBuilder) -> RequestBuilder {
    request
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Host", "isis.verw.uni-koeln.de")
        .header("Origin", "https://isis.verw.uni-koeln.de")
}

pub fn params_from_form(
    form: ElementRef<'_>,
    keep_user_params: bool,
) -> Result<Vec<(String, String)>> {
    let inputs_selector =
        scraper::Selector::parse("input").map_err(|e| eyre!("scraper error: {e}"))?;
    let user_params: &[&str] = &[
        "Geschlecht",
        "Vorname",
        "Name",
        "Strasse",
        "Ort",
        "Statusorig",
        "Matnr",
        "Mail",
        "Tel",
        "iban",
        "bic",
    ];
    let params = form
        .select(&inputs_selector)
        .filter_map(|element| {
            let name = element.value().attr("name")?.to_owned();
            let value = element.value().attr("value")?.to_owned();
            Some((name, value))
        })
        .filter(|(name, _)| *name != "reset")
        .filter(|(name, _)| *name != "back")
        .filter(|(name, _)| keep_user_params || !user_params.contains(&name.as_str()))
        .collect::<Vec<_>>();
    Ok(params)
}

pub fn request_body_from_params(mut params: Vec<(String, String)>) -> Result<String> {
    encode_params(&mut params)?;
    Ok(params
        .iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<String>>()
        .join("&"))
}

fn encode_params(params: &mut [(String, String)]) -> Result<()> {
    for (_, value) in params.iter_mut() {
        *value = byte_serialize(
            &ISO_8859_1
                .encode(value, encoding::EncoderTrap::Strict)
                .map_err(|e| eyre!(e))?,
        )
        .collect();
    }
    Ok(())
}

pub fn german_day_name(date: chrono::NaiveDate) -> &'static str {
    match date.weekday() {
        chrono::Weekday::Mon => "Mo",
        chrono::Weekday::Tue => "Di",
        chrono::Weekday::Wed => "Mi",
        chrono::Weekday::Thu => "Do",
        chrono::Weekday::Fri => "Fr",
        chrono::Weekday::Sat => "Sa",
        chrono::Weekday::Sun => "So",
    }
}
