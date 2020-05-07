use std::collections::HashMap;
use std::str::FromStr;

use reqwest;
use serde_json::Value;

use crate::*;
use crate::commandlight::CommandLight;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct LightState {
    pub on: bool,
    pub bri: Option<u8>,
    pub hue: Option<u16>,
    pub sat: Option<u8>,
    pub ct: Option<u16>,
    pub xy: Option<(f32, f32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Light {
    pub name: String,
    pub modelid: String,
    pub swversion: String,
    pub uniqueid: String,
    pub state: LightState,
}

#[derive(Debug, Clone)]
pub struct IdentifiedLight {
    pub id: usize,
    pub light: Light,
}

#[derive(Debug)]
pub struct Bridge {
    address: String,
    username: Option<String>,
    client: reqwest::blocking::Client,
}

impl Bridge {
    pub fn discover_bridge_by_meethue() -> Option<Bridge> {
        discover::discover_bridgeaddress_by_meethue().ok().map(|i| Bridge {
            address: i,
            ..Bridge::default()
        })
    }

    pub fn with_user(self, username: String) -> Bridge {
        Bridge {
            username: Some(username),
            ..self
        }
    }

    pub fn register_user(&self, devicetype: &str) -> HueResult<String> {
        #[derive(Serialize, Deserialize)]
        struct PostApi {
            devicetype: String,
        }
        #[derive(Serialize, Deserialize)]
        struct Success {
            success: Username,
        }
        #[derive(Serialize, Deserialize)]
        struct Username {
            username: String,
        }
        let obtain = PostApi {
            devicetype: devicetype.to_string(),
        };
        let url = format!("http://{}/api", self.address);
        let success: Success =
            self.parse(self.client.post(&url[..]).json(&obtain).send()?.json()?)?;
        Ok(success.success.username)
    }

    pub fn get_all_lights(&self) -> HueResult<Vec<IdentifiedLight>> {
        let url = format!(
            "http://{}/api/{}/lights",
            self.address,
            self.username.clone().unwrap()
        );
        let resp: HashMap<String, Light> = self.parse(self.client.get(&url[..]).send()?.json()?)?;
        let mut lights = vec![];
        for (k, v) in resp {
            let id: usize = usize::from_str(&k).or(Err(HueErrorKind::ProtocolError(
                "Light id should be a number".to_string(),
            )))?;
            lights.push(IdentifiedLight { id: id, light: v });
        }
        lights.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(lights)
    }

    pub fn get_light_by_name(&self, light_name: String) -> Option<IdentifiedLight> {
        let coll = self.get_all_lights().unwrap().into_iter();
        let mut selected_light: Option<IdentifiedLight> = None;

        for identifiedlight in coll {
            if identifiedlight.light.name == light_name {
                selected_light = Some(identifiedlight);
                break;
            }
        }

        selected_light
    }

    pub fn set_light_state(&self, light: usize, command: &CommandLight) -> HueResult<Value> {
        let url = format!(
            "http://{}/api/{}/lights/{}/state",
            self.address,
            self.username.clone().unwrap(),
            light
        );
        let body = ::serde_json::to_vec(command)?;
        let resp = self
            .client
            .put(&url[..])
            .body(::reqwest::blocking::Body::from(body))
            .send()?
            .json()?;
        self.parse(resp)
    }

    fn parse<T: ::serde::de::DeserializeOwned>(&self, value: Value) -> HueResult<T> {
        use serde_json::*;
        if !value.is_array() {
            return Ok(from_value(value)?);
        }
        let mut objects: Vec<Value> = from_value(value)?;
        if objects.len() == 0 {
            Err(HueErrorKind::ProtocolError(
                "expected non-empty array".to_string(),
            ))?
        }
        let value = objects.remove(0);
        {
            let object = value.as_object().ok_or(HueErrorKind::ProtocolError(
                "expected first item to be an object".to_string(),
            ))?;
            if let Some(e) = object.get("error").and_then(|o| o.as_object()) {
                let code: u64 = e.get("type").and_then(|s| s.as_u64()).unwrap_or(0);
                let desc = e
                    .get("description")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                Err(HueErrorKind::BridgeError(code as usize, desc))?
            }
        }
        Ok(from_value(value)?)
    }
}

impl Default for Bridge {
    fn default() -> Self {
        Self {
            address: "".into(),
            username: None,
            client: reqwest::blocking::Client::new()
        }
    }
}
