use serde_json::{json, Value};
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

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
}
