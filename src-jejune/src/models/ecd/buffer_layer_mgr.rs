use std::{
    collections::{HashMap, VecDeque, hash_map::Keys},
    error::Error,
    fs::File,
    io::prelude::*,
    option::Option,
    path::Path,
    result::Result,
};

trait HandlerInterface {
    fn create() -> Self;
    fn read(filename: String);
    fn update(layer: String); // layer is an event-message result
    fn destroy() -> Result<(), Box<dyn Error>>;
}

struct BridgedThroughputDirector(String);

struct Uuid(String);

struct Queues<K, V>(HashMap<K, V>);

impl<K, V> Queues<K, V> {
    pub fn keys(&self) -> Keys<'_, K, V> {
        return self.0.keys();
    }
}

struct BufferLayerShort;

struct TableEditorShort; // short: as in meaning ,,temporary''

impl BufferLayerShort {
    pub fn new() -> Self {
        return Self;
    }
}

impl HandlerInterface for BufferLayerShort {
    fn create() -> Self {
        return Self::new();
    }

    fn read(filename: String) {
        let path = Path::new(&filename);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        filename = String::new();
        match file.read_to_string(&mut &filename) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("{} contains:\n{}", display, filename),
        }
    }

    fn update(layer: String) {}

    fn destroy() -> Result<(), Box<dyn Error>> {
        return Ok(());
    }
}

impl TableEditorShort {
    pub fn new() -> Self {
        return Self;
    }
}

impl HandlerInterface for TableEditorShort {
    fn create() -> Self {
        return Self::new();
    }

    fn read(filename: String) {}

    fn update(layer: String) {}

    fn destroy() -> Result<(), Box<dyn Error>> {
        return Ok(());
    }
}
