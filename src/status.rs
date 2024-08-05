use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumProperty, EnumString};

#[derive(Debug, Clone, EnumString, EnumProperty, EnumIter, Deserialize, Serialize)]
pub enum Status {
    #[strum(props(pretty = "Stud. Uni Köln"))]
    StudentUniKoeln,
    #[strum(props(pretty = "Stud. DSHS Köln"))]
    StudentDSHSKoeln,
    #[strum(props(pretty = "Stud. TH Köln"))]
    StudentTHKoeln,
    #[strum(props(pretty = "Stud. Macromedia Köln"))]
    StudentMacromediaKoeln,
    #[strum(props(pretty = "Stud. KHM Köln"))]
    StudentKunsthochschuleFuerMedien,
    #[strum(props(pretty = "Stud. HMKW Köln"))]
    StudentHochschuleFuerMedienKommunikationUndWirtschaft,
    #[strum(props(pretty = "Stud. HfMT Köln"))]
    StudentHochschuleFuerMusikKoeln,
    #[strum(props(pretty = "Stud. anderer Hochschulen"))]
    StudentAndereHochschulen,
    #[strum(props(pretty = "Beschäft. staatl. Kölner Hochschulen"))]
    BeschaeftigteStaatlicherKoelnerHochschulen,
    #[strum(props(pretty = "Beschäft. UniKlinik Köln"))]
    BeschaeftigteUniKlinikKoeln,
    #[strum(props(pretty = "Beschäft. Kölner Studierendenwerk"))]
    BeschaeftigteKoelnerStudierendenwerk,
    #[strum(props(pretty = "Mitglied von KölnAlumni"))]
    MitgliedKoelnAlumni,
    #[strum(props(pretty = "Azubi Uni Köln"))]
    AzubiUniKoeln,
    #[strum(props(pretty = "Azubi UniKlinik Köln"))]
    AzubiUniKlinik,
    #[strum(props(pretty = "Azubi Kölner Studierendenwerk"))]
    AzubiKoelnerStudierendenwerk,
    #[strum(props(pretty = "Gast"))]
    Gast,
}

impl Status {
    pub const fn as_payload(&self) -> &'static str {
        match self {
            Self::StudentUniKoeln => "S-UNI",
            Self::StudentDSHSKoeln => "S-DSHS",
            Self::StudentTHKoeln => "S-TH",
            Self::StudentMacromediaKoeln => "S-MAC",
            Self::StudentKunsthochschuleFuerMedien => "S-KHSM",
            Self::StudentHochschuleFuerMedienKommunikationUndWirtschaft => "S-HMKW",
            Self::StudentHochschuleFuerMusikKoeln => "S-MH",
            Self::StudentAndereHochschulen => "S-aH",
            Self::BeschaeftigteStaatlicherKoelnerHochschulen => "B-SFH",
            Self::BeschaeftigteUniKlinikKoeln => "B-UK",
            Self::BeschaeftigteKoelnerStudierendenwerk => "B-KStW",
            Self::MitgliedKoelnAlumni => "Alumni",
            Self::AzubiUniKoeln => "A-Uni",
            Self::AzubiUniKlinik => "A-UK",
            Self::AzubiKoelnerStudierendenwerk => "A-KSTW",
            Self::Gast => "Extern",
        }
    }
}
