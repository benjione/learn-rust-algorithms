
use std::ptr;
// See http://cglab.ca/~abeinges/blah/too-many-lists/book/README.html

pub struct List<T> {
    head: ListLink<T>,
    last: *mut ListNode<T>,
}

type ListLink<T> = Option<Box<ListNode<T>>>;

struct ListNode<T> {
    elem: T,
    next: ListLink<T>
}

impl <T> List<T> {
    pub fn new() -> Self {
        List {head: None,
                last: ptr::null_mut()}
    }

    pub fn append(&mut self, data: T) -> () {
        let mut new = Box::new(ListNode{
            elem: data,
            next: None,
        });
        let new_last: *mut _ = &mut *new;
        if self.last.is_null() {
            self.head = Some(new);
        }
        else {
            unsafe{
                (*self.last).next = Some(new);
            }
        }
        self.last = new_last
    }
}
