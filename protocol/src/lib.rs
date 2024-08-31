//! # Some Protocol
//!
//! Types defining a game/server and server/controller interface.

use std::collections::{BTreeMap, BTreeSet};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod game;
pub use game::*;

/// Messages from server session towards game.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(
    feature = "postcard-schema",
    derive(postcard::experimental::schema::Schema)
)]
pub enum SessionToGameEvent<ControllerToSessionCommand> {
    /// The session initially sends the randomly chosen ID of a game.
    SetId(String),
    NewPlayer(u16),
    PlayerLeft(u16),
    ControllerCommand {
        id: u16,

        /// The controller command.
        command: ControllerToSessionCommand,
    },
}

/// Messages from server to a connected controller.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[cfg_attr(
    feature = "postcard-schema",
    derive(postcard::experimental::schema::Schema)
)]
pub enum SessionToControllerEvent<GameToControllerEvent> {
    /// Push interval for the controller to use (in milliseconds).
    SetPushInterval(u32),

    /// Forwarded event from game towards the connected controller.
    GameToControllerEvent(GameToControllerEvent),
}

/// Messages from game towards server session.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(
    feature = "postcard-schema",
    derive(postcard::experimental::schema::Schema)
)]
pub struct GameToSessionMessage<GameToControllerEvent> {
    /// This message concerns the controller@id.
    pub id: u16,

    /// The game event to be forwarded by the server to the controller.
    pub event: GameToControllerEvent,
}

/// Game state statistics.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Statistics {
    /// Tree of sessions and controllers.
    pub tree: BTreeMap<String, BTreeSet<u16>>,
}

#[cfg(test)]
mod test {
    use super::*;
    use schemars::schema_for;

    /// Convenience enum for all (server-)incoming messages.
    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    #[cfg_attr(
        feature = "postcard-schema",
        derive(postcard::experimental::schema::Schema)
    )]
    enum IncomingProtocol {
        ControllerToSessionCommand(ControllerToSessionCommand),
        GameToSessionMessage(GameToSessionMessage<GameToControllerEvent>),
    }

    /// Convenience enum for all (server-)outgoing messages.
    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    #[cfg_attr(
        feature = "postcard-schema",
        derive(postcard::experimental::schema::Schema)
    )]
    enum OutgoingProtocol {
        SessionToGameEvent(SessionToGameEvent<ControllerToSessionCommand>),
        SessionToControllerEvent(SessionToControllerEvent<GameToControllerEvent>),
    }

    fn print_json_for<T>(elem: T)
    where
        T: Serialize + std::fmt::Debug,
    {
        let type_name = std::any::type_name::<T>();
        println!("{type_name}: {elem:?}");
        println!("{}", serde_json::to_string_pretty(&elem).unwrap());
        println!();
    }

    fn print_json_schema_for<T: JsonSchema>() {
        let type_name = std::any::type_name::<T>();
        let schema = schema_for!(T);
        println!(
            "Schema of {type_name}:\n{}",
            serde_json::to_string_pretty(&schema).unwrap()
        );
        println!();
    }

    #[test]
    fn json_examples() {
        print_json_for(ControllerToSessionCommand::Move { x: 1.0, y: 2.4 });
        print_json_for(ControllerToSessionCommand::Action1);
        print_json_for(SessionToGameEvent::<ControllerToSessionCommand>::SetId(
            "Party Island".to_string(),
        ));
        print_json_for(GameToSessionMessage {
            id: 89,
            event: GameToControllerEvent::Event1,
        });
        print_json_for(SessionToControllerEvent::GameToControllerEvent(
            GameToControllerEvent::IncreaseScore(Team::Red),
        ));
    }

    #[test]
    fn json_schemas() {
        print_json_schema_for::<IncomingProtocol>();
        print_json_schema_for::<OutgoingProtocol>();
    }

    #[test_case::test_case(SessionToControllerEvent::GameToControllerEvent(
        GameToControllerEvent::Event1
    ); "Event 1")]
    #[test_case::test_case(SessionToControllerEvent::GameToControllerEvent(
        GameToControllerEvent::IncreaseScore(Team::Red)
    ); "Increase Red score")]
    fn controller_event_to_bytes(event: SessionToControllerEvent<GameToControllerEvent>) {
        let vec = Vec::new();
        let bytes = postcard::to_extend(&event, vec).unwrap();
        println!("{bytes:#?}");
    }

    #[test_case::test_case(&[1, 0, 0, 64, 64, 0, 0, 128, 63] => ControllerToSessionCommand::Move {x: 3.0, y: 1.0}; "Move slow")]
    #[test_case::test_case(&[0] => ControllerToSessionCommand::Action1; "Action1")]
    fn controller_command_from_bytes(bytes: &[u8]) -> ControllerToSessionCommand {
        postcard::from_bytes(bytes).unwrap()
    }

    #[cfg(feature = "postcard-schema")]
    #[test]
    fn print_postcard_schema_for() {
        use postcard::experimental::schema::Schema;

        println!("{:#?}", IncomingProtocol::SCHEMA);
        println!("{:#?}", OutgoingProtocol::SCHEMA);
        println!("{:#?}", <(bool, &[f32])>::SCHEMA);
    }
}
