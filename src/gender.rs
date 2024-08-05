use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumProperty, EnumString};

#[derive(Debug, Clone, EnumString, EnumProperty, EnumIter, Deserialize, Serialize)]
pub enum Gender {
    #[strum(props(pretty = "Männlich"))]
    Male,
    #[strum(props(pretty = "Weiblich"))]
    Female,
    #[strum(props(pretty = "Divers"))]
    Diverse,
}

impl Gender {
    pub const fn as_payload(&self) -> &'static str {
        match self {
            Self::Male => "M",
            Self::Female => "W",
            Self::Diverse => "D",
        }
    }
}
