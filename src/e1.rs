fn parse_message(msg: Message) -> Result<KucoinWebsocketMsg, APIError> {
    match msg {
        Message::Text(msg) => {
            if msg.contains("\"type\":\"welcome\"") || msg.contains("\"type\":\"ack\"") {
                Ok(KucoinWebsocketMsg::WelcomeMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"type\":\"ping\"") {
                Ok(KucoinWebsocketMsg::PingMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"type\":\"pong\"") {
                Ok(KucoinWebsocketMsg::PongMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"subject\":\"trade.ticker\"") {
                Ok(KucoinWebsocketMsg::TickerMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"topic\":\"/market/ticker:all\"") {
                Ok(KucoinWebsocketMsg::AllTickerMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"subject\":\"trade.snapshot\"") {
                Ok(KucoinWebsocketMsg::SnapshotMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"subject\":\"trade.l2update\"") {
                Ok(KucoinWebsocketMsg::OrderBookMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("/market/match:") {
                Ok(KucoinWebsocketMsg::MatchMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"subject\":\"trade.l3received\"") {
                Ok(KucoinWebsocketMsg::Level3ReceivedMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"subject\":\"trade.l3open\"") {
                Ok(KucoinWebsocketMsg::Level3OpenMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"subject\":\"trade.l3done\"") {
                Ok(KucoinWebsocketMsg::Level3DoneMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"subject\":\"trade.l3match\"") {
                Ok(KucoinWebsocketMsg::Level3MatchMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"subject\":\"trade.l3change\"") {
                Ok(KucoinWebsocketMsg::Level3ChangeMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("/indicator/index:") {
                Ok(KucoinWebsocketMsg::IndexPriceMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("/indicator/markPrice:") {
                Ok(KucoinWebsocketMsg::MarketPriceMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("/margin/fundingBook:") {
                Ok(KucoinWebsocketMsg::OrderBookChangeMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"type\":\"stop\"") || msg.contains("\"type\":\"activate\"") {
                Ok(KucoinWebsocketMsg::StopOrderMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("/account/balance") {
                Ok(KucoinWebsocketMsg::BalancesMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("debt.ratio")  {
                Ok(KucoinWebsocketMsg::DebtRatioMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("position.status") {
                Ok(KucoinWebsocketMsg::PositionChangeMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("order.open") {
                Ok(KucoinWebsocketMsg::MarginTradeOpenMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("order.update") {
                Ok(KucoinWebsocketMsg::MarginTradeUpdateMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("order.done") {
                Ok(KucoinWebsocketMsg::MarginTradeDoneMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("error") {
                Ok(KucoinWebsocketMsg::Error(msg))
            } else {
                serde::export::Err(APIError::Other("No KucoinWebSocketMsg type to parse".to_string()))
            }
        }
        Message::Binary(b) => Ok(KucoinWebsocketMsg::Binary(b)),
        Message::Pong(..) => Ok(KucoinWebsocketMsg::Pong),
        Message::Ping(..) => Ok(KucoinWebsocketMsg::Ping),
        Message::Close(..) => {
            serde::export::Err(APIError::Other("Socket closed error".to_string()))
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum KucoinWebsocketMsg {
    WelcomeMsg(DefaultMsg),
    SubscribeMsg(Subscribe),
    PingMsg(DefaultMsg),
    PongMsg(DefaultMsg),
    Ping,
    Pong,
    Binary(Vec<u8>),
    TickerMsg(WSResp<SymbolTicker>),
    AllTickerMsg(WSResp<SymbolTicker>),
    SnapshotMsg(WSResp<Snapshot>),
    OrderBookMsg(WSResp<Level2>),
    MatchMsg(WSResp<Match>),
    Level3ReceivedMsg(WSResp<Level3Received>),
    Level3OpenMsg(WSResp<Level3Open>),
    Level3MatchMsg(WSResp<Level3Match>),
    Level3DoneMsg(WSResp<Level3Done>),
    Level3ChangeMsg(WSResp<Level3Change>),
    IndexPriceMsg(WSResp<IndexPrice>),
    MarketPriceMsg(WSResp<MarketPrice>),
    OrderBookChangeMsg(WSResp<BookChange>),
    StopOrderMsg(WSResp<StopOrder>),
    BalancesMsg(WSResp<Balances>),
    DebtRatioMsg(WSResp<DebtRatio>),
    PositionChangeMsg(WSResp<PositionChange>),
    MarginTradeOpenMsg(WSResp<MarginTradeOpen>),
    MarginTradeUpdateMsg(WSResp<MarginTradeUpdate>),
    MarginTradeDoneMsg(WSResp<MarginTradeDone>),
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
pub struct Subscribe {
    pub id: String,
    pub r#type: String,
    pub topic: String,
    pub private_channel: bool,
    pub response: bool,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolTicker {
    pub sequence: String,
    pub best_ask: String,
    pub size: String,
    pub best_bid_size: String,
    pub price: String,
    pub best_ask_size: String,
    pub best_bid: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub sequence: i64,
    pub data: SnapshotData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotData {
    pub trading: bool,
    pub symbol: String,
    pub buy: f32,
    pub sell: f32,
    pub sort: i32,
    pub vol_value: f32,
    pub base_currency: String,
    pub market: String,
    pub quote_currency: String,
    pub symbol_code: String,
    pub datetime: i64,
    pub high: Option<f32>,
    pub vol: f32,
    pub low: Option<f32>,
    pub change_price: Option<f32>,
    pub change_rate: f32,
    pub last_traded_price: f32,
    pub board: i32,
    pub mark: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level2 {
    pub sequence_start: i64,
    pub sequence_end: i64,
    pub symbol: String,
    pub changes: Level2Changes,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level2Changes {
    pub asks: Vec<Vec<String>>,
    pub bids: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub size: String,
    pub price: String,
    pub taker_order_id: String,
    pub time: String,
    pub r#type: String,
    pub maker_order_id: String,
    pub trade_id: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level3Received {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub order_id: String,
    pub price: Option<String>,
    pub time: String,
    pub client_oid: Option<String>,
    pub r#type: String,
    pub order_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level3Open {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub size: String,
    pub order_id: String,
    pub price: String,
    pub time: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level3Done {
    pub sequence: String,
    pub symbol: String,
    pub reason: String,
    pub side: String,
    pub order_id: String,
    pub time: String,
    pub r#type: String,
    pub size: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level3Match {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub size: String,
    pub price: String,
    pub taker_order_id: String,
    pub time: String,
    pub r#type: String,
    pub maker_order_id: String,
    pub trade_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level3Change {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub order_id: String,
    pub price: String,
    pub new_size: String,
    pub time: String,
    pub r#type: String,
    pub old_size: String,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexPrice {
    pub symbol: String,
    pub granularity: i32,
    pub timestamp: i64,
    pub value: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketPrice {
    pub symbol: String,
    pub granularity: i32,
    pub timestamp: i64,
    pub value: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookChange {
    pub sequence: i32,
    pub currency: String,
    pub daily_int_rate: f32,
    pub annual_int_rate: f32,
    pub term: i32,
    pub size: f32,
    pub side: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StopOrder {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub order_id: String,
    pub stop_entry: String,
    pub funds: String,
    pub time: String,
    pub r#type: String,
    pub reason: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Balances {
    pub total: String,
    pub available: String,
    pub available_change: String,
    pub currency: String,
    pub hold: String,
    pub hold_change: String,
    pub relation_event: String,
    pub relation_event_id: String,
    pub time: String,
    pub account_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebtRatio {
    pub debt_ratio: f32,
    pub total_debt: String,
    pub debt_list: HashMap<String, String>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionChange {
    pub r#type: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTradeOpen {
    pub currency: String,
    pub order_id: String,
    pub daily_int_rate: f32,
    pub term: i32,
    pub size: i32,
    pub side: String,
    pub ts: i64,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTradeDone {
    pub currency: String,
    pub order_id: String,
    pub reason: String,
    pub side: String,
    pub ts: i64,
}