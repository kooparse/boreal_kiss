use std::cmp::PartialEq;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

pub type IdType = u32;
pub type GenerationId = GeneratedId<IdType>;

#[derive(Default, Debug, Eq, Hash, Copy, Clone)]
pub struct GeneratedId<T> {
    data: T,
    generation: T,
}

impl<T> GeneratedId<T> {
    fn new(data: T, generation: T) -> Self {
        Self { data, generation }
    }

    fn set_gen(&mut self, generation: T) {
        self.generation = generation;
    }
}

impl<T> Deref for GeneratedId<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for GeneratedId<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: PartialEq> PartialEq for GeneratedId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.generation == other.generation
    }
}

/// This ids pool has no ABA problem and recycles
#[derive(Default)]
pub struct IdentifyGeneratorPool {
    cursor_id: GenerationId,
    unused_ids: Vec<GenerationId>,
    generation_counter: IdType,
}

impl IdentifyGeneratorPool {
    pub fn new_id(&mut self) -> GenerationId {
        self.generation_counter += 1;

        if self.unused_ids.is_empty() {
            self.cursor_id =
                GenerationId::new(*self.cursor_id + 1, self.generation_counter);

            self.cursor_id
        } else {
            let mut unused_id = self.unused_ids.pop().unwrap();
            unused_id.set_gen(self.generation_counter);
            unused_id
        }
    }

    pub fn remove_id(&mut self, id: GenerationId) {
        self.unused_ids.push(id);
    }

    pub fn clear(&mut self) {
        self.cursor_id = GenerationId::default();
        self.generation_counter = 0;

        self.unused_ids.clear();
        self.unused_ids.shrink_to_fit();
    }
}

pub struct Storage<T> {
    id_manager: IdentifyGeneratorPool,
    pub items: HashMap<GenerationId, T>,
}

impl<T> Storage<T> {
    pub fn push(&mut self, new_item: T) -> GenerationId {
        let item_id = self.id_manager.new_id();
        self.items.insert(item_id, new_item);

        item_id
    }

    pub fn remove(&mut self, item_id: GenerationId) {
        self.id_manager.remove_id(item_id);
        self.items.remove(&item_id);
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.items.shrink_to_fit();

        self.id_manager.clear();
    }

    pub fn get_mut(&mut self, item_id: GenerationId) -> Option<&mut T> {
        self.items.get_mut(&item_id)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct MyTestType;

    #[test]
    fn empty_storage() {
        let store: Storage<MyTestType> = Storage::default();

        assert_eq!(store.items.len(), 0);
    }

    #[test]
    fn generate_id() {
        let a = MyTestType;

        let mut store: Storage<MyTestType> = Storage::default();
        let id = store.push(a);

        assert_eq!(id, GenerationId::new(1, 1));
    }

    #[test]
    fn compare_id() {
        let a = MyTestType;
        let b = MyTestType;

        let mut store: Storage<MyTestType> = Storage::default();
        let id_a = store.push(a);
        let id_b = store.push(b);

        assert_ne!(id_a, id_b);
    }

    #[test]
    fn get_id() {
        let a = MyTestType;
        let b = MyTestType;

        let mut store: Storage<MyTestType> = Storage::default();
        let id_a = store.push(a);
        let id_b = store.push(b);

        store.remove(id_a);

        let my_test_type = store.get_mut(id_a);
        assert_eq!(my_test_type.is_some(), false);

        let my_test_type = store.get_mut(id_b);
        assert_eq!(my_test_type.is_some(), true);
    }

    #[test]
    fn recycle_id() {
        let a = MyTestType;
        let b = MyTestType;
        let c = MyTestType;

        let mut store: Storage<MyTestType> = Storage::default();
        let id_a = store.push(a);
        let _id_b = store.push(b);

        store.remove(id_a);
        let id_c = store.push(c);

        assert_eq!(id_c, GenerationId::new(1, 3));
    }

    #[test]
    fn clear_storage() {
        let a = MyTestType;
        let b = MyTestType;
        let c = MyTestType;

        let mut store: Storage<MyTestType> = Storage::default();
        let id_a = store.push(a);
        let id_b = store.push(b);
        let id_c = store.push(c);

        store.clear();
        assert_eq!(store.get_mut(id_a).is_none(), true);
        assert_eq!(store.get_mut(id_b).is_none(), true);
        assert_eq!(store.get_mut(id_c).is_none(), true);
        assert_eq!(store.items.len(), 0);
    }
}
