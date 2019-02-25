
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

    pub fn to_vector(self) -> Vec<T> {
        let mut ele = self.head;
        let mut ret = Vec::<T>::new();
        loop {
            match ele {
                None => return ret,
                Some(e) => {
                    ret.push(e.elem);
                    ele = e.next;
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::List;
    #[test]
    fn test_new_list() {
        let list = List::<u32>::new();
        match list.head {
            None => {},
            _ => panic!("head should be None!"),
        }
        if !list.last.is_null() {
            panic!("Has to be a null pointer!");
        }
    }

    #[test]
    fn test_append_and_to_vector() {
        let mut list = List::<u32>::new();
        list.append(1);
        list.append(2);
        list.append(3);
        let vec = list.to_vector();
        assert_eq!(vec, vec![1,2,3]);
    }

}
