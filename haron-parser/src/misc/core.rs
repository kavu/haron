use std::ops::{Index, IndexMut};

pub struct Core<T> where T: Clone {
    index: usize,
    content: Vec<Option<T>>,
}

impl<T> Core<T> where T: Clone {
    pub fn with_capacity(capacity: usize) -> Core<T> {
        let content = vec![None; capacity];

        Core {
            content,
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.content.capacity()
    }
}

impl<T> Index<usize> for Core<T> where T: Clone {
    type Output = Option<T>;

    fn index(&self, idx: usize) -> &Self::Output {
        if idx > self.len() {
            return &None
        }

        &self.content[idx]
    }
}

impl<T> IndexMut<usize> for Core<T> where T: Clone {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.content[idx]
    }
}

impl<T> Iterator for Core<T> where T: Clone {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.content.get(self.index)?;

        if self.index + 1 == self.content.len() {
            self.index = 0;
        } else {
            self.index += 1;
        }

        item.clone()
    }
}
