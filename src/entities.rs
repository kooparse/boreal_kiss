use crate::renderer::{LightProbes, LoadedMesh, Text};
use std::fmt::Debug;
use std::iter::Iterator;
use std::mem::*;

#[derive(Debug)]
pub struct Wall;
#[derive(Default, Debug)]
pub struct Player;
#[derive(Debug)]
pub struct Camera;

#[derive(Default)]
pub struct Entities {
    pub walls: Arena<Wall>,
    pub light_probes: Arena<LightProbes>,
    pub text_widgets: Arena<Text>,
    pub meshes: Arena<LoadedMesh>,
    pub player: Player,
}

/// Memory arena.
pub struct Arena<T: Debug> {
    data: Vec<T>,
    handles: Vec<MemoryHandle>,
    // Index of dirty handles.
    free_handles: Vec<usize>,
    version_count: usize,
}

impl<T: Debug> Arena<T> {
    // size_alloc N size in Mb on the heap.
    #[allow(unused)]
    pub fn size_alloc(size: usize) -> Self {
        let element_size = size_of::<T>();
        let size_in_bytes = size * 1000_000;
        let capacity = size_in_bytes / element_size;

        Self {
            data: Vec::with_capacity(capacity),
            handles: vec![],
            free_handles: vec![],
            version_count: 0,
        }
    }

    pub fn alloc(nb_items: usize) -> Self {
        Self {
            data: Vec::with_capacity(nb_items),
            handles: vec![],
            free_handles: vec![],
            version_count: 0,
        }
    }
    // We don't want to re-allocate when the data exceed the
    // vector capacity. We currently prefer to crash.
    pub fn insert(&mut self, value: T) -> MemoryHandle {
        if self.data.capacity() <= self.data.len() {
            panic!("Arena too small to contains the amount of data.")
        }

        self.version_count += 1;

        if self.free_handles.is_empty() {
            self.data.push(value);
            // Create new handle pointed to the datum index.
            let handle =
                MemoryHandle::new(self.data.len() - 1, self.version_count);

            self.handles.push(handle);
            // Cloned occured here.
            handle
        } else {
            // Remove and return the last free handles available.
            let index = self.free_handles.pop().unwrap();

            // Update this handle
            let handle = &mut self.handles[index];
            // Update the version of this dirty handles.
            handle.version = self.version_count;
            handle.is_dirty = false;

            // Replace old mem block by this new block.
            self.data[handle.value] = value;

            *handle
        }
    }

    pub fn get(&self, handle: &MemoryHandle) -> &T {
        if self.is_dirty(handle) {
            panic!("This value was freed.")
        }

        self.data
            .get(handle.value)
            .expect("No block found for this index.")
    }

    pub fn get_mut(&mut self, handle: &MemoryHandle) -> &mut T {
        if self.is_dirty(handle) {
            panic!("This value was freed.")
        }

        self.data
            .get_mut(handle.value)
            .expect("No block found for this index.")
    }

    pub fn remove(&mut self, handle: MemoryHandle) {
        // Verify if handle block exists.
        let index = self
            .handles
            .iter()
            .position(|h| *h == handle)
            .expect("Handle not found.");

        // Set this handle as dirty.
        self.handles[index].is_dirty = true;

        // Push this index to the freed blucket.
        self.free_handles.push(index);
    }

    // Mark all handles as dirty and free.
    pub fn flush(&mut self) {
        let mut free_indexes: Vec<usize> = vec![];

        self.handles
            .iter_mut()
            // If we don't iterate over all the valide handles,
            // we're going to have not uniq values in free handles.
            .filter(|h| !h.is_dirty)
            .enumerate()
            .for_each(|(index, handle)| {
                handle.is_dirty = true;
                free_indexes.push(index);
            });

        self.free_handles.extend(free_indexes);
    }

    fn is_dirty(&self, handle: &MemoryHandle) -> bool {
        self.handles
            .iter()
            .any(|h| h == handle && h.is_dirty == true)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.handles
            .iter()
            .filter(|h| h.is_dirty == false)
            .map(move |h| &self.data[h.value])
    }
}

impl<T: Debug> Default for Arena<T> {
    fn default() -> Self {
        Arena::<T>::alloc(10)
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct MemoryHandle {
    value: usize,
    version: usize,
    is_dirty: bool,
}

impl MemoryHandle {
    fn new(value: usize, version: usize) -> Self {
        Self {
            value,
            version,
            is_dirty: false,
        }
    }
}

impl PartialEq for MemoryHandle {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.version == other.version
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::catch_unwind;

    #[test]
    fn arena_alloc() {
        // Allocate 10Mb.
        let mem_size = 10;
        let arena = Arena::<bool>::size_alloc(mem_size);

        assert_eq!(arena.data.capacity(), mem_size * 1000_000);
    }

    #[test]
    #[should_panic]
    fn exceed_allocated_reserve() {
        // Allocate 10Mb.
        let mut arena = Arena::<bool>::size_alloc(10);
        let mut last_handle = MemoryHandle::default();
        // Push elements of 1 byte each.
        for _ in 0..1000_0000 {
            last_handle = arena.insert(true);
        }

        assert_eq!(last_handle.value, 999_9999);

        // Panic here.
        arena.insert(true);
    }

    #[test]
    fn store_new_block() {
        let mut arena = Arena::<bool>::size_alloc(10);
        let handle_0 = arena.insert(true);
        let handle_1 = arena.insert(true);

        assert_eq!(handle_0.value, 0);
        assert_eq!(handle_1.value, 1);
        assert_ne!(handle_0, handle_1);
    }

    #[test]
    fn remove_block() {
        let mut arena = Arena::<bool>::size_alloc(10);
        let handle_0 = arena.insert(true);
        let handle_1 = arena.insert(true);

        assert_eq!(handle_0.version, 1);
        assert_eq!(handle_1.version, 2);
        arena.remove(handle_0);
        assert_eq!(arena.handles[0].is_dirty, true);

        let handle_2 = arena.insert(false);
        assert_eq!(handle_2.version, 3);

        assert_eq!(arena.data[0], false);
    }

    #[test]
    fn iter_data() {
        let mut arena = Arena::<bool>::size_alloc(10);
        let _ = arena.insert(true);
        let _ = arena.insert(true);

        arena.iter().for_each(|data| {
            assert_eq!(*data, true);
        });

        // arena.iter_mut().for_each(|data| {
        //     *data = false;
        // });

        // assert_eq!(*arena.get(&handle_0), false);
        // assert_eq!(*arena.get(&handle_1), false);
    }

    #[test]
    fn flush() {
        let mut arena = Arena::<bool>::size_alloc(10);
        let handle_0 = arena.insert(true);
        let _ = arena.insert(true);
        let _ = arena.insert(true);

        assert_eq!(*arena.get(&handle_0), true);

        arena.flush();

        let should_panic = catch_unwind(|| arena.get(&handle_0));
        assert_eq!(should_panic.is_err(), true);
    }
}
