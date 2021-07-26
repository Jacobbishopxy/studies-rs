use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

fn main() {
    // Before
    let mut collection = HeteroCollection::default();
    collection.set("name", "Jakob");
    collection.set("language", "Rust");
    collection.set("dominant hand", DominantHand::Right);

    let _name = collection.get::<&'static str>("name");
    let _language = collection.get::<&'static str>("language");
    let _dominant_hand = collection.get::<DominantHand>("dominant hand");

    println!("{:#?}", collection);

    // After
    let mut collection = SingletonCollection::default();
    collection.set(Name("Jakob"));
    collection.set(Language("Rust"));
    collection.set(DominantHand::Right);

    let _name = collection.get::<Name>().0;
    let _language = collection.get::<Language>().0;
    let _dominant_hand = collection.get::<DominantHand>();

    println!("{:#?}", collection);
}

#[derive(Default, Debug)]
struct HeteroCollection {
    data: HashMap<&'static str, Box<dyn Any>>,
}

impl HeteroCollection {
    pub fn get<T: 'static>(&self, key: &'static str) -> Option<&T> {
        let unknown_output: &Box<dyn Any> = self.data.get(key)?;
        unknown_output.downcast_ref()
    }

    pub fn set<T: 'static>(&mut self, key: &'static str, value: T) {
        self.data.insert(key, Box::new(value));
    }
}

#[derive(Default, Debug)]
struct SingletonCollection {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl SingletonCollection {
    pub fn get<T: Any>(&self) -> &T {
        self.data[&TypeId::of::<T>()]
            .downcast_ref()
            .as_ref()
            .unwrap()
    }

    pub fn set<T: Any>(&mut self, value: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
    }
}

// For completeness: Type Definitions
struct Name(&'static str);
struct Language(&'static str);
pub enum DominantHand {
    Left,
    Right,
    Both,
    Neither,
    Unknown,
    Other,
}
