use actix::prelude::*;

pub extern crate serde;
pub extern crate serde_json;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ChatMessage(pub String);

#[derive(Clone, Message)]
#[rtype(result = "usize")]
pub struct JoinRoom(pub String, pub Option<String>, pub Recipient<ChatMessage>);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct LeaveRoom(pub String, pub usize);

#[derive(Clone, Message)]
#[rtype(result = "Vec<String>")]
pub struct ListRooms;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendMessage(pub String, pub usize, pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum KucoinWebsocketMsg {
    TickerMsg(WSResp<DefaultMsg>),
    MarginTradeDoneMsg(WSResp<MarginTradeUpdate>),
    Error(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WSResp<T> {
    pub r#type: String,
    pub topic: String,
    pub subject: String,
    pub data: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultMsg {
    pub id: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTradeUpdate {
    pub currency: String,
    pub order_id: String,
    pub daily_int_rate: f32,
    pub term: i32,
    pub size: i32,
    pub lent_size: f32,
    pub side: String,
    pub ts: i64,
}
