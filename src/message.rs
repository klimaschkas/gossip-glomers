use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body {
    #[serde(rename = "type")]
    pub type_field: String,
    pub msg_id: i64,
    pub in_reply_to: Option<i64>,
    pub echo: String,
}
