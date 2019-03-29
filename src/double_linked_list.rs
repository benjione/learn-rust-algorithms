
use std::ptr;
// See http://cglab.ca/~abeinges/blah/too-many-lists/book/README.html

pub struct List<'a, T> {
    head: ListLink<'a, T>,
    last: *mut ListNode<'a, T>,
}

type ListLink<'a, T> = Option<Box<ListNode<'a, T>>>;

struct ListNode<'a, T> {
    data: &'a T,
    next: ListLink<'a, T>,
    prev: *mut ListNode<'a, T>
}

impl <'a, T> List<'a, T> {

    pub fn new() -> Self {
        List{
            head: None,
            last: ptr::null_mut()
        }
    }

    pub fn append(&mut self, data: &'a T) -> () {
        let mut new = Box::new(ListNode{
            data: data,
            next: None,
            prev: self.last,
        });
        let new_last: *mut _ = &mut *new;
        if self.last.is_null() {
            self.head = Some(new);
        }
        else{
            unsafe{
                (*self.last).next = Some(new);
            }
        }
        self.last = new_last;
    }

    pub fn to_vector(self) -> Vec<&'a T> {
        let mut ele = self.head;
        let mut ret = Vec::<&'a T>::new();
        loop {
            match ele {
                None => return ret,
                Some(e) => {
                    ret.push(e.data);
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
        list.append(&1);
        list.append(&2);
        list.append(&3);
        let vec = list.to_vector();
        assert_eq!(vec, vec![&1,&2,&3]);
    }

}
