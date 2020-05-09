use serde_json::{json, Value};
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;
extern crate serde;
extern crate serde_json;
#[macro_use]
pub extern crate serde_derive;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
    static ref COUNT: usize = HASHMAP.len();
    static ref NUMBER: u32 = times_two(21);
    pub static ref DISPATCHER: Mutex<Dispatcher> = Mutex::new(Dispatcher::new());
}

fn times_two(n: u32) -> u32 {
    n * 2
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum GreetEvent {
    SayHello,
    SayBye,
}

type HandlerPtr<T> = Box<dyn Fn(&T)>; // Vec<Arc<dyn EventListener>>; //

pub struct EventEmitter<T: Hash + Eq, U> {
    handlers: HashMap<T, Vec<HandlerPtr<U>>>,
}

impl<T: Hash + Eq, U> EventEmitter<T, U> {
    /// Creates a new instance of `EventEmitter`.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Registers function `handler` as a listener for `event`.  There may be
    /// multiple listeners for a single event.
    pub fn on<F>(&mut self, event: T, handler: F)
    where
        F: Fn(&U) + 'static,
    {
        let event_handlers = self.handlers.entry(event).or_insert_with(|| vec![]);
        event_handlers.push(Box::new(handler));
    }

    /// Invokes all listeners of `event`, passing a reference to `payload` as an
    /// argument to each of them.
    pub fn emit(&self, event: T, payload: U) {
        if let Some(handlers) = self.handlers.get(&event) {
            for handler in handlers {
                handler(&payload);
            }
        }
    }
}

pub struct Dispatcher {
    pub subscribers: HashMap<String, Vec<String>>,
}

impl Dispatcher {
    fn new() -> Self {
        Dispatcher {
            subscribers: HashMap::new(),
        }
    }

    pub fn subscribe_to_event(event: String, subscriber: String) {
        let mut dispatcher = DISPATCHER.lock().unwrap();
        let entry = dispatcher.subscribers.entry(event).or_insert(Vec::new());
        entry.push(subscriber);
    }

    pub fn publish_event(event: &str, message: String) {
        let dispatcher = DISPATCHER.lock().unwrap();
        if let Some(subscribers) = dispatcher.subscribers.get(event) {
            for _subscriber in subscribers {
                println!("{}", message);

                //let json = Event::to_json(event.to_string(), message.clone());
                //subscriber.send(json);
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum MsgEvent {
    TickerMsg,
    MarginTradeDoneMsg,
}

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

fn main() {
    let mut emitter = EventEmitter::new();

    emitter.on(GreetEvent::SayHello, |name| {
        println!("Hello, {}!", name);
    });

    emitter.on(GreetEvent::SayHello, |name| {
        println!("Someone said hello to {}.", name);
    });

    emitter.on(GreetEvent::SayBye, |name| {
        println!("Bye, {}, hope to see you again!", name);
    });

    emitter.emit(GreetEvent::SayHello, "Alex");
    emitter.emit(GreetEvent::SayBye, "Alex");

    let mut emitter2 = EventEmitter::new();

    emitter2.on(GreetEvent::SayHello, |name| {
        println!("Someone said hello to {}.", name);
    });

    emitter2.emit(GreetEvent::SayHello, "Alex");

    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
    assert_eq!(map.get_key_value(&2), None);
    map.insert(1, "a");
    map.insert(1, "b");
    map.insert(2, "b");
    map.insert(3, "b");
    for (a, b) in &map {
        println!("{}: \"{}\"", a, b);
    }
    println!("{:?}", &map);
    println!("{:?}", map);
    println!("{}", json!(&map));
    println!("{}", json!(map));

    let xxx1 = "{1:1}";
    let xxx2 = r#xxx1;
    println!("{}", xxx2);

    let serialized = serde_json::to_string(&map).unwrap();
    println!("{}", serialized);

    let p: Value = serde_json::from_str(&serialized).unwrap();
    println!("{}", p);
    println!("{}", p["1"]);

    println!("{}", p[1]);

    println!("The map has {} entries.", *COUNT);
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
    println!(
        "A expensive calculation on a static results in: {}.",
        *NUMBER
    );

    let mut emitter1 = EventEmitter::new();
    emitter1.on(MsgEvent::TickerMsg, |msg| {
        //let msg1 = KucoinWebsocketMsg::TickerMsg(serde_json::from_str(&msg)?);
        //match msg1 {
        //    KucoinWebsocketMsg::TickerMsg(msg1) => println!("{:#?}", msg1),
        //    _ => (),
        //}

        println!("Bye, {}, hope to see you again!", msg);
    });

    emitter1.on(MsgEvent::MarginTradeDoneMsg, |msg| {
        println!("Bye, {}, hope to see you again!", msg);
    });
    emitter1.emit(MsgEvent::TickerMsg, "{\"xxx\":\"xxxx\"}");
}
