#[macro_use]
extern crate error_chain;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

error_chain! {
    types  { HueError, HueErrorKind, HueResultExt, HueResult; }
    foreign_links {
        Reqwest(reqwest::Error);
        SerdeJson(serde_json::Error);
    }

    errors {
        ProtocolError(msg: String)
        BridgeError(code: usize, msg: String)
    }
}

mod discover;
pub mod bridge;
