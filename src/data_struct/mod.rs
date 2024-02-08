use leptos::Serializable;
use serde::{Deserialize, Serialize};

use crate::utils::MDRENDERER;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EssayAmb {
    pub title: String,
    pub date: String,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub excerpt: String,
    priority: u32
}
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EssayContent {
    content: String,
}

impl EssayContent {
    pub fn from(content:&str) -> Self {
        Self {
            content: MDRENDERER.render(&content)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Essay {
    pub amb: EssayAmb,
    pub content: EssayContent
}
impl Essay {
    pub fn from(amb: &str ,content: &str) -> Self {
        Self {
            amb: serde_yaml::from_str(amb).unwrap(),
            content: EssayContent::from(content)
        }
    }
}

// impl Serializable for Essay {
//     fn de(bytes: &str) -> Result<Self, leptos::SerializationError> {
        
//     }
//     fn ser(&self) -> Result<String, leptos::SerializationError> {
        
//     }
// }