use log::debug;
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    msg: String,
}

const MENU: &'static str = "menu";
const SEARCH: &'static str = "buscar";
const IS_CONNECTED: &'static str = "is_connected";

pub fn route(msg: String) -> GenericResponse {
    let response_msg: String;

    let mut parts = msg.splitn(2, ' ');
    let key = parts.next().unwrap_or("");
    let rest = parts.next().unwrap_or("");

    debug!("{}", key);

    match key {
        SEARCH => {
            let data = rustws::mercadolibre::search(rest.to_string());

            response_msg = serde_json::to_string(&data).unwrap();
        }
        IS_CONNECTED => {
            response_msg = "Hola! En que puedo ayudarte hoy?".to_string();
        }

        _ => {
            let data = rustws::mercadolibre::menu();

            response_msg = serde_json::to_string(&data).unwrap();
        }
    }

    GenericResponse { msg: response_msg }
}
