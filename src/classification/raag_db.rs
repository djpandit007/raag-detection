use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub name: String,
    pub frequency_ratio: f32, // Ratio to Sa (tonic)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhrasePattern {
    pub notes: Vec<Note>,
    pub weight: f32, // Importance of this phrase for the raag
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Raag {
    pub name: String,
    pub aroha: Vec<Note>,           // Ascending scale
    pub avaroha: Vec<Note>,         // Descending scale
    pub vadi: Note,                 // Most important note
    pub samvadi: Note,              // Second most important note
    pub characteristic_phrases: Vec<PhrasePattern>,
    pub thaat: String,              // Parent scale
}

pub struct RaagDatabase {
    raags: Vec<Raag>,
}

impl RaagDatabase {
    pub fn new() -> Self {
        Self {
            raags: Self::initialize_common_raags(),
        }
    }

    pub fn get_raags(&self) -> &[Raag] {
        &self.raags
    }

    pub fn find_raag(&self, name: &str) -> Option<&Raag> {
        self.raags.iter().find(|r| r.name == name)
    }

    fn initialize_common_raags() -> Vec<Raag> {
        vec![
            // Raag Yaman
            Raag {
                name: "Yaman".to_string(),
                aroha: vec![
                    Note { name: "Sa".to_string(), frequency_ratio: 1.0 },
                    Note { name: "Re".to_string(), frequency_ratio: 9.0/8.0 },
                    Note { name: "Ga".to_string(), frequency_ratio: 5.0/4.0 },
                    Note { name: "Ma#".to_string(), frequency_ratio: 45.0/32.0 },
                    Note { name: "Pa".to_string(), frequency_ratio: 3.0/2.0 },
                    Note { name: "Dha".to_string(), frequency_ratio: 27.0/16.0 },
                    Note { name: "Ni".to_string(), frequency_ratio: 15.0/8.0 },
                ],
                avaroha: vec![
                    Note { name: "Sa".to_string(), frequency_ratio: 2.0 },
                    Note { name: "Ni".to_string(), frequency_ratio: 15.0/8.0 },
                    Note { name: "Dha".to_string(), frequency_ratio: 27.0/16.0 },
                    Note { name: "Pa".to_string(), frequency_ratio: 3.0/2.0 },
                    Note { name: "Ma#".to_string(), frequency_ratio: 45.0/32.0 },
                    Note { name: "Ga".to_string(), frequency_ratio: 5.0/4.0 },
                    Note { name: "Re".to_string(), frequency_ratio: 9.0/8.0 },
                    Note { name: "Sa".to_string(), frequency_ratio: 1.0 },
                ],
                vadi: Note { name: "Ga".to_string(), frequency_ratio: 5.0/4.0 },
                samvadi: Note { name: "Ni".to_string(), frequency_ratio: 15.0/8.0 },
                characteristic_phrases: vec![],
                thaat: "Kalyan".to_string(),
            },
            // Add more raags here as needed
        ]
    }
}