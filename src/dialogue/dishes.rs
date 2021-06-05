use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Clone, Serialize, Deserialize)]
pub struct Dish {
    pub name: String,
    pub description: String,
    pub composition: Vec<String>,
}

impl Dish {
    pub fn new(name: &str, description: &str, composition: Vec<&str>) -> Self {
        Dish {
            name: name.to_string(),
            description: description.to_string(),
            composition: composition
                .iter()
                .map(|&s| s.to_string())
                .collect::<Vec<String>>(),
        }
    }

    pub fn format_to_string(&self) -> String {
        format!(
            "{0}\n{1}\nСостав: {2}",
            self.name,
            self.description,
            self.composition.join(", ")
        )
    }
}
