use rand::seq::IteratorRandom;
use rand::Rng;
use serde_json::Value;


const EMOJIS: [&str; 49] = ["🌞", "🍓", "🍒", "🍑", "🍐", "🍏", "🍎", "🍍", "🍌", "🍋", "🍊", "🍉", "🍇", "🍆", "🍄", "🌜", "🌛", "🌟", "🌠", "🌺", "🌻", "🌼", "🌽", "🌷", "😃", "😀", "😁", "😂", "😅", "😆", "😇", "😉", "😊", "😋", "😌", "😍", "😎", "😏", "😗", "😘", "😙", "😚", "😛", "😜", "😝", "😬", "🙋", "🐙", "🐴",];


#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    pub emoji: String,
    pub lat: f64,
    pub lon: f64,
}

impl User {
    pub fn new_rand() -> User {
        let mut rng = rand::thread_rng();
        User {
            name: format!("USER{:04}", rng.gen::<u32>() % 1000),
            emoji: EMOJIS.iter().choose(&mut rng).unwrap().to_string(),
            lat: 52.398 + rng.gen::<f64>() * 0.05,
            lon: 13.043 + rng.gen::<f64>() * 0.05,
        }
    }

    pub fn as_json(&self) -> Value {
        serde_json::json!({
            "name": self.name,
            "emoji": self.emoji,
            "lat": self.lat,
            "lon": self.lon,
        })
    }

    pub fn as_message(&self, action: &str, id: &str) -> String {
        let mut j = self.as_json();
        if let Value::Object(ref mut obj) = j {
            obj.insert("action".to_string(), Value::String(action.to_string()));
            obj.insert("id".to_string(), Value::String(id.to_string()));
        };
        j.to_string()
    }
}
