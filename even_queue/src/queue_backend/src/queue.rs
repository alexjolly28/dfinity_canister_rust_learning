pub trait Filter<T> {
    fn filter(&self, item: T) -> bool;
}

pub struct EvenFilter;

impl Filter<i32> for EvenFilter {
    fn filter(&self, item: i32) -> bool {
        if item % 2 == 0 {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct FilteredQueue<T, F>
where
    F: Filter<T>,
{
    pub items: Vec<T>,
    filter: F,
}

impl<T, F> FilteredQueue<T, F>
where
    T: Copy,
    F: Filter<T>,
{
    pub fn new(filter: F) -> Self {
        FilteredQueue {
            items: Vec::new(),
            filter,
        }
    }

    pub fn enqueue(&mut self, item: T) -> Result<(), String> {
        if self.filter.filter(item) {
            self.items.push(item);
            Ok(())
        } else {
            Err("It is an odd".to_string())
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if !self.is_empty() {
            Some(self.items.remove(0))
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // pub fn len(&self) -> usize {
    //     self.items.len()
    // }

    pub fn get(&self) -> Vec<T> {
        let items = &self.items;
        items.to_vec()
    }
}
