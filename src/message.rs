use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body {
    #[serde(rename = "type")]
    pub type_field: String,
    pub msg_id: Option<i64>,
    pub in_reply_to: Option<i64>,
    pub echo: Option<String>,
}
