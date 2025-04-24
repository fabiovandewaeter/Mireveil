use crate::items::item::Item;

pub struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }
}
