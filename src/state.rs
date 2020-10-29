use std::collections::HashMap;

use serde_json::Value;

use crate::user::User;



pub struct State {
    pub users: HashMap<String, User>,
}

impl State {
    pub fn new() -> State {
        State {
            users: HashMap::new(),
        }
    }

    pub fn users_as_json(&self) -> Value {
        let mut map = serde_json::Map::new();
        for (user_id, user) in &self.users {
            map.insert(user_id.to_string(), user.as_json());
        }
        Value::Object(map)
    }

    pub fn as_message(&self, action: &str) -> String {
        let mut map = serde_json::Map::new();
        map.insert("action".to_string(), Value::String(action.to_string()));
        map.insert("users".to_string(), self.users_as_json());
        Value::Object(map).to_string()
    }
}
