use std::collections::HashMap;

pub type GeneratedId = u32;

/// TODO: Add generation id, to remove the ABA problem here.
#[derive(Default)]
pub struct IdentifyGeneratorPool {
    cursor_id: GeneratedId,
    unused_ids: Vec<GeneratedId>,
}

impl IdentifyGeneratorPool {
    pub fn new_id(&mut self) -> GeneratedId {
        if self.unused_ids.is_empty() {
            self.cursor_id += 1;
            self.cursor_id
        } else {
            self.unused_ids.pop().unwrap()
        }

    }

    pub fn remove_id(&mut self, id: GeneratedId) {
        self.unused_ids.push(id);
    }

    pub fn clear(&mut self) {
        self.cursor_id = 0;
        self.unused_ids.clear();
        self.unused_ids.shrink_to_fit();
    }
}


pub struct Storage<T> {
    id_manager: IdentifyGeneratorPool,
    pub items: HashMap<GeneratedId, T>,
}

impl<T> Storage<T> {
    pub fn push(&mut self, new_item: T) -> GeneratedId {
        let item_id = self.id_manager.new_id();
        self.items.insert(item_id, new_item);

        item_id
    }

    pub fn remove(&mut self, item_id: &GeneratedId) {
        self.id_manager.remove_id(*item_id);
        self.items.remove(&item_id);
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.items.shrink_to_fit();

        self.id_manager.clear();

    }

    pub fn get_mut(&mut self, item_id: &GeneratedId) -> Option<&mut T>{
        self.items.get_mut(item_id)
    }
}

impl<T> Default for Storage<T> {
    fn default() -> Self {
        Self {
            id_manager: IdentifyGeneratorPool::default(),
            items: HashMap::new(),
        }
    }
}

// TODO: Should add tests there.
