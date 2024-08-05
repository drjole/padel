use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Angebotsdaten {
    pub angebote: Angebote,
    pub zeitraum: Zeitraum,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Angebote {
    pub angebot: Vec<Angebot>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Zeitraum {
    pub bezeichnung: String,
    pub dauer: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Angebot {
    pub aboid: i32,
    pub angebotsname: String,
    // pub bereich: String,
    pub buchung: i32,
    pub details: String,
    pub frei: i32,
    pub kursid: i32,
    // pub kursleiter: String,
    pub kursnr: i32,
    // pub preis: Vec<f32>,
    pub raum: Vec<String>,
    pub tag: Vec<String>,
    pub uhrzeit: Vec<String>,
    pub zeitraum: String,
}

impl Angebot {
    pub fn clean(&mut self) {
        let raum = self
            .raum
            .clone()
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect();
        let tag = self
            .tag
            .clone()
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect();
        let uhrzeit = self
            .uhrzeit
            .clone()
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect();

        self.raum = raum;
        self.tag = tag;
        self.uhrzeit = uhrzeit;
    }
}
