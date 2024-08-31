use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Message from controller towards the game (via server).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[cfg_attr(
    feature = "postcard-schema",
    derive(postcard::experimental::schema::Schema)
)]
pub enum ControllerToSessionCommand {
    Action1,
    Move { x: f32, y: f32 },
}

/// There are 2 teams, red and blue.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(
    feature = "postcard-schema",
    derive(postcard::experimental::schema::Schema)
)]
pub enum Team {
    Blue,
    Red,
}

/// Messages from game towards a controller (via the server).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[cfg_attr(
    feature = "postcard-schema",
    derive(postcard::experimental::schema::Schema)
)]
pub enum GameToControllerEvent {
    Action2,
    Event1,
    IncreaseScore(Team),
}
