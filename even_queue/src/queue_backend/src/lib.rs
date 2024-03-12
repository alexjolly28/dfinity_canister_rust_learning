mod queue;
use ic_cdk::{export_candid, query, update};
use queue::{EvenFilter, FilteredQueue};
use std::cell::RefCell;
thread_local! {

static QUEUE: RefCell<FilteredQueue<i32, EvenFilter>> =
    RefCell::new(FilteredQueue::new(EvenFilter))
}

#[update]
fn push(item: i32) -> Result<(), String> {
    QUEUE.with(|queue| {
        let mut queue = queue.borrow_mut();
        queue.enqueue(item)
    })
}

#[update]
fn pop() -> Option<i32> {
    QUEUE.with(|queue| {
        let mut queue = queue.borrow_mut();
        queue.dequeue()
    })
}

#[query]
fn get_queue() -> Vec<i32> {
    QUEUE
        .with(|queue| {
            let queue = queue.borrow();
            queue.get()
        })
        .to_vec()
}

export_candid!();
