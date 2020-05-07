#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandLight {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bri: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sat: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xy: Option<(f32, f32)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitiontime: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<String>,
}

impl Default for CommandLight {
    fn default() -> CommandLight {
        CommandLight {
            on: None,
            bri: None,
            hue: None,
            sat: None,
            transitiontime: None,
            ct: None,
            xy: None,
            alert: None,
        }
    }
}

impl CommandLight {
    pub fn on(self) -> CommandLight {
        CommandLight {
            on: Some(true),
            ..self
        }
    }
    pub fn off(self) -> CommandLight {
        CommandLight {
            on: Some(false),
            ..self
        }
    }
    pub fn with_bri(self, b: u8) -> CommandLight {
        CommandLight {
            bri: Some(b),
            ..self
        }
    }
    pub fn with_hue(self, h: u16) -> CommandLight {
        CommandLight {
            hue: Some(h),
            ..self
        }
    }
    pub fn with_sat(self, s: u8) -> CommandLight {
        CommandLight {
            sat: Some(s),
            ..self
        }
    }
    pub fn with_ct(self, c: u16) -> CommandLight {
        CommandLight {
            ct: Some(c),
            ..self
        }
    }
    pub fn with_xy(self, x: f32, y: f32) -> CommandLight {
        CommandLight {
            xy: Some((x, y)),
            ..self
        }
    }
    pub fn alert(self) -> CommandLight {
        CommandLight {
            alert: Some("select".into()),
            ..self
        }
    }
}