use std::marker::PhantomData;
use std::mem::*;
use std::fmt::Debug;

/// Memory arena. 
pub struct Arena<T: Debug> {
    data: Vec<Option<MemoryBlock<T>>>,
    free_bucket: Vec<MemoryHandle<T>>,
    version_count: usize,
}

impl<T: Debug> Arena<T> {
    // Alloc N size in Mb on the heap.
    pub fn alloc(size: usize) -> Self {
        let element_size = size_of::<T>();
        let size_in_bytes = size * 1000_000;
        let capacity = size_in_bytes / element_size;

        Self {
            data: Vec::with_capacity(capacity),
            free_bucket: vec![],
            version_count: 0,
        }
    }
    // We don't want to re-allocate when the data exceed the
    // vector capacity. We currently prefer to crash.
    pub fn insert(&mut self, value: T) -> MemoryHandle<T> {
        if self.data.capacity() <= self.data.len() {
            panic!("Arena too small to contains the amount of data.")
        }

        self.version_count += 1;
        let new_mem_block =
            Some(MemoryBlock::new(value, self.version_count));

        if self.free_bucket.is_empty() {
            let new_index =
                MemoryHandle::new(self.data.len(), self.version_count);
            self.data.push(new_mem_block);
            return new_index;
        } else {
            let mut index_unused = self.free_bucket.pop().unwrap();
            index_unused.version = self.version_count;
            // Replace old mem block by this new block.
            self.data[index_unused.value] = new_mem_block;

            index_unused
        }
    }

    pub fn get(&self, index: &MemoryHandle<T>) -> &MemoryBlock<T> {
        let mem_block = self
            .data
            .get(index.value)
            .expect("No block found for this index.");

        if let Some(value) = mem_block {
            if value.version == index.version {
                return value;
            } else {
                panic!("Generations doesn't match.")
            }

        } 

        panic!("Value was freed.")
    }

    pub fn get_mut(&mut self, index: &MemoryHandle<T>) -> &mut MemoryBlock<T> {
        let mem_block = self
            .data
            .get_mut(index.value)
            .expect("No block found for this index.");

        if let Some(value) = mem_block {
            if value.version == index.version {
                return value;
            } else {
                panic!("Generations doesn't match.")
            }

        } 

        panic!("Value was freed.")
    }

    pub fn remove(&mut self, index: MemoryHandle<T>) {
        // Verify if the block exists.
        self.get(&index);
        // Set the block to None.
        self.data[index.value] = None;
        // Push this index to the freed blucket.
        self.free_bucket.push(index);
    }
}

// TODO: Remove the version, 'cause it will fuckup
// the cache I guess.
pub struct MemoryBlock<T: Debug> {
    value: T,
    version: usize,
}

impl<T: Debug> MemoryBlock<T> {
    fn new(value: T, version: usize) -> Self {
        Self { value, version }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct MemoryHandle<T: Debug> {
    value: usize,
    version: usize,
    _phantom_type: PhantomData<T>,
}

impl<T: Debug> MemoryHandle<T> {
    fn new(value: usize, version: usize) -> Self {
        Self { value, version, _phantom_type: PhantomData }
    }
}


impl<T: Debug> PartialEq for MemoryHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.version == other.version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arena_alloc() {
        // Allocate 10Mb.
        let mem_size = 10;
        let arena = Arena::<bool>::alloc(mem_size);

        assert_eq!(arena.data.capacity(), mem_size * 1000_000);
    }

    #[test]
    #[should_panic]
    fn exceed_allocated_reserve() {
        // Allocate 10Mb.
        let mut arena = Arena::<bool>::alloc(10);
        let mut last_index = MemoryHandle::default();
        // Push elements of 1 byte each.
        for _ in 0..1000_0000 {
            last_index = arena.insert(true);
        }

        assert_eq!(last_index.value, 999_9999);

        // Panic here.
        arena.insert(true);
    }

    #[test]
    fn store_new_block() {
        let mut arena = Arena::<bool>::alloc(10);
        let index_0 = arena.insert(true);
        let index_1 = arena.insert(true);

        assert_eq!(index_0.value, 0);
        assert_eq!(index_1.value, 1);
        assert_ne!(index_0, index_1);
    }

    #[test]
    fn remove_block() {
        let mut arena = Arena::<bool>::alloc(10);
        let index_0 = arena.insert(true);
        let index_1 = arena.insert(true);

        assert_eq!(index_0.version, 1);
        assert_eq!(index_1.version, 2);
        arena.remove(index_0);
        assert_eq!(arena.data[0].is_none(), true);

        let index_2 = arena.insert(false);
        assert_eq!(index_2.version, 3);

        assert_eq!(arena.data[0].as_ref().unwrap().value, false);
    }
}
