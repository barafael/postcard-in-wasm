use protocol::{ControllerToSessionCommand, GameToControllerEvent, SessionToControllerEvent};
use wasm_bindgen::prelude::*;

/// The session is telling us.
#[wasm_bindgen]
pub fn controller_event_from_bytes(event: &[u8]) -> Result<JsValue, serde_wasm_bindgen::Error> {
    let value: SessionToControllerEvent<GameToControllerEvent> = postcard::from_bytes(event)
        .map_err(|e| serde_wasm_bindgen::Error::new(format!("{e:#?}")))?;
    serde_wasm_bindgen::to_value(&value)
}

/// We are telling the session.
#[wasm_bindgen]
pub fn controller_command_to_bytes(command: JsValue) -> Result<JsValue, serde_wasm_bindgen::Error> {
    let command: ControllerToSessionCommand = serde_wasm_bindgen::from_value(command)?;
    let vec = Vec::new();
    let bytes = postcard::to_extend(&command, vec)
        .map_err(|e| serde_wasm_bindgen::Error::new(format!("{e:#?}")))?;
    serde_wasm_bindgen::to_value(&bytes)
}

/// Run with `wasm-pack test --node`
#[cfg(test)]
mod tests {
    use protocol::{ControllerToSessionCommand, GameToControllerEvent, SessionToControllerEvent};
    use wasm_bindgen_test::*;

    use crate::{controller_command_to_bytes, controller_event_from_bytes};

    #[wasm_bindgen_test]
    fn session_to_controller_message_roundtrip() {
        // session encodes message to controller in bytes:
        let message =
            SessionToControllerEvent::GameToControllerEvent(GameToControllerEvent::Event1);
        let vec = Vec::new();
        let bytes = postcard::to_extend(&message, vec).unwrap();
        println!("{bytes:#?}");

        // controller decodes the bytes:
        let js_value = controller_event_from_bytes(&bytes).unwrap();
        println!("{js_value:#?}");
        let received_message: SessionToControllerEvent<GameToControllerEvent> =
            serde_wasm_bindgen::from_value(js_value).unwrap();
        assert_eq!(received_message, message);
    }

    #[wasm_bindgen_test]
    fn controller_to_session_message_roundtrip() {
        wasm_logger::init(wasm_logger::Config::default());
        // controller encodes js value to send to session to bytes:
        let message = ControllerToSessionCommand::Move { x: 3.0, y: 1.0 };
        let js_value = serde_wasm_bindgen::to_value(&message).unwrap();
        let bytes = controller_command_to_bytes(js_value).unwrap();
        let bytes_contained: Vec<u8> = serde_wasm_bindgen::from_value(bytes).unwrap();

        log::info!("{:#?}", bytes_contained);

        // session deserializes the bytes:
        let received_message: ControllerToSessionCommand =
            postcard::from_bytes(&bytes_contained).unwrap();
        assert_eq!(received_message, message);
    }
}
