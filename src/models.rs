use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt::Display;

#[derive(Serialize, Debug)]
pub struct Greeting {
    pub(crate) name: String,
    pub(crate) greeting: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub(crate) message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonData {
    // #[serde(with = "BigArray")]
    pub names: JsonValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub name: String,
}

impl Display for JsonData {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let v = "w name p".replace("w", "{").replace("p", "}").replace(
            "name",
            &r#""names":value"#.replace("value", &self.names.to_string()),
        );
        write!(fmt, "{}", v)
    }
}
