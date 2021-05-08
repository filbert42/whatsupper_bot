#[derive(Clone)]
pub struct Dish {
    name: String,
    description: String,
    composition: Vec<String>,
}

impl Dish {
    pub fn new(name: String, description: String, composition: Vec<String>) -> Self {
        Dish {
            name: name,
            description: description,
            composition: composition,
        }
    }

    pub fn format_to_string(self) -> String {
        format!(
            "Я предлагаю тебе ответадь сегодня:\n{0}\n{1}\nСостав: {2}",
            self.name,
            self.description,
            self.composition.join(", ")
        )
    }
}
