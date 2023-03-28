use axum::extract::State;
use shared::{Boom, Pah};

#[macro_use]
extern crate timber;

pub async fn greet() -> &'static str {
    timber!("Greeting the world...");
    let (x, y) = rayon::join(
        || {
            timber!("Calculating x");
            2
        },
        || {
            timber!("Calculating y");
            3
        }
    );
    timber!("x={}, y={}", x, y);
    
    return "Hello world!"
}


pub async fn boom(State(message): State<Boom>) -> String {
    return message.0
}
pub async fn pah(State(message): State<Pah>) -> String {
    return message.0
}