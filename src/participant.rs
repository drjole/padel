use crate::{gender::Gender, status::Status};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Participant {
    pub given_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<Gender>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<Status>,
    pub status_info: Option<String>,
    pub iban: Option<String>,
    pub bic: Option<String>,
}

impl Participant {
    pub fn as_payload(&self) -> Vec<(String, String)> {
        vec![
            (
                "Geschlecht".into(),
                self.gender
                    .clone()
                    .map_or(String::new(), |g| g.as_payload().to_string()),
            ),
            (
                "Vorname".into(),
                self.given_name.clone().unwrap_or_default(),
            ),
            ("Name".into(), self.last_name.clone().unwrap_or_default()),
            ("Strasse".into(), self.street.clone().unwrap_or_default()),
            ("Ort".into(), self.city.clone().unwrap_or_default()),
            (
                "Statusorig".into(),
                self.status
                    .clone()
                    .map_or(String::new(), |s| s.as_payload().to_string()),
            ),
            ("Matnr".into(), self.status_info.clone().unwrap_or_default()),
            (
                "Institut".into(),
                self.status_info.clone().unwrap_or_default(),
            ),
            ("Mail".into(), self.email.clone().unwrap_or_default()),
            ("Tel".into(), self.phone.clone().unwrap_or_default()),
            ("iban".into(), self.iban.clone().unwrap_or_default()),
            ("bic".into(), self.bic.clone().unwrap_or_default()),
        ]
    }
}
