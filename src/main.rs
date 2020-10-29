use std::sync::{Arc, Mutex};

use rand::Rng;
use mio_extras::timer::Timeout;

use serde_json::Value;

use ws::util::Token;
use ws::{listen, CloseCode, Error, Frame, Handler, Handshake, Message, Sender};


mod state;
mod user;

use state::State;
use user::User;


fn get_rand_id() -> String {
    let mut rng = rand::thread_rng();
    std::iter::repeat(())
        .map(|()| rng.sample(rand::distributions::Alphanumeric))
        .take(22)
        .collect()
}

// Server WebSocket handler
struct Server {
    out: Sender,
    userid: String,
    state: Arc<Mutex<State>>,
}

fn main() {
    env_logger::init();

    let state = Arc::new(Mutex::new(
        State::new(),
    ));

    listen("127.0.0.1:2010", |out| {
        Server {
            out,
            userid: get_rand_id(),
            state: Arc::clone(&state),
        }
    }).unwrap();
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        println!("new user: {}", self.userid);
        let (user_msg, state_msg) = {
            let mut state = self.state.lock().unwrap();
            state.users.insert(self.userid.clone(), User::new_rand());
            let user = &state.users[&self.userid];
            (
                user.as_message("user_update", &self.userid),
                state.as_message("state_update"),
            )
        };
        let _ = self.out.send(
            serde_json::json!({
                "action": "self",
                "id": self.userid,
            }).to_string()
        );
        let _ = self.out.broadcast(user_msg);
        let _ = self.out.send(state_msg);
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        let txt = match &msg {
            Message::Text(txt) => txt,
            _ => return Ok(()),
        };
        let json: Value = match serde_json::from_str(txt) {
            Ok(json) => json,
            _ => return Ok(()),
        };
        if let Value::String(action) = &json["action"] {
            println!("{}", msg);
            match action.as_str() {
                "user_update" => {
                    let mut state = self.state.lock().unwrap();
                    let mut user = state.users.get_mut(&self.userid).unwrap();

                    if let Value::String(name) = &json["name"] {
                        println!("user {} name: {}", self.userid, name);
                        user.name = name.to_string();
                    }
                    if let Value::String(emoji) = &json["emoji"] {
                        println!("user {} emoji: {}", self.userid, emoji);
                        user.emoji = emoji.to_string();
                    }
                    if let (Value::Number(lat), Value::Number(lon)) = (&json["lat"], &json["lon"]) {
                        if let (Some(lat), Some(lon)) = (lat.as_f64(), lon.as_f64()) {
                            println!("user {} pos: {:?}", self.userid, (lat, lon));
                            user.lat = lat;
                            user.lon = lon;
                        }
                    }

                    let _ = self.out.broadcast(user.as_message("user_update", &self.userid));
                },
                "refresh" => {
                    let state_msg = {
                        let state = self.state.lock().unwrap();
                        state.as_message("state_update")
                    };
                    let _ = self.out.send(state_msg);
                },
                "shout" => {
                    if let Value::String(text) = &json["text"] {
                        let _ = self.out.broadcast(
                            serde_json::json!({
                                "action": "shout",
                                "id": self.userid,
                                "text": text,
                            }).to_string()
                        );
                    }
                },
                _ => {},
            }
        }
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing for ({:?}) {}", code, reason);
        let users = {
            let mut s = self.state.lock().unwrap();
            s.users.remove(&self.userid);
            s.users.len()
        };
        println!("users {}", users);

        let _ = self.out.broadcast(
            serde_json::json!({
                "action": "user_leave",
                "id": self.userid,
            }).to_string()
        );

        //self.out.shutdown().unwrap();
    }

    fn on_error(&mut self, err: Error) {
        // Shutdown on any error
        println!("Shutting down server for error: {}", err);
        self.out.shutdown().unwrap();
    }

    fn on_timeout(&mut self, _event: Token) -> ws::Result<()> {
        Ok(())
    }

    fn on_new_timeout(&mut self, _event: Token, _timeout: Timeout) -> ws::Result<()> {
        Ok(())
    }

    fn on_frame(&mut self, frame: Frame) -> ws::Result<Option<Frame>> {
        // Run default frame validation
        DefaultHandler.on_frame(frame)
    }
}

// For accessing the default handler implementation
struct DefaultHandler;

impl Handler for DefaultHandler {}
