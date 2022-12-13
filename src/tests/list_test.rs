use std::borrow::{Borrow, BorrowMut};
use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct List<T> {
    next: Option<Box<List<T>>>,
    value: T,
}

impl<T: Display + Debug + Copy> List<T> {
    fn is_empty(&self) -> bool {
        if let Some(v) = &self.next {
            return false;
        }

        return true;
    }

    fn add(&mut self, v: T) {
        let n = Box::new(List::<T> { next: self.next.take(), value: v });
        self.next = Some(n);
        println!("added {:?} len: {:?}", v, self.count());
    }

    fn pop(&mut self) -> Option<T> {
        let mut r: Option<T> = None;
        if let Some(v) = self.next.as_mut() {
            r = Some(v.value);
            self.next = v.next.take();
        }
        r
    }

    fn clear(&mut self) {
        let mut r = self.next.as_mut();
        while let Some(v) = r.take() {
            v.next = None;
        }

        self.next = None;
    }

    fn count(&self) -> i32 {
        let mut c = 0;
        let mut r = self.next.as_ref();
        loop {
            if let Some(v) = r {
                r = v.next.as_ref();
                c = c + 1;
                continue;
            }
            return c;
        }
    }

    fn print(&self) {
        println!("=================Begin===================");
        let mut r = self.next.as_ref();
        loop {
            if let Some(v) = r {
                println!("value: {:?}", v.value);
                r = v.next.as_ref();
            } else {
                println!("=================End=====================");
                return;
            }
        }
    }
}

#[test]
fn test_list_base() {
    let n = Option::<Box<List<i32>>>::None;
    let mut l = Box::new(List::<i32> { next: None, value: 0 });
    println!("empty: {:?}", l.is_empty());
    l.add(100);
    println!("empty: {:?}", l.is_empty());
    l.add(999);
    l.add(87);

    l.print();

    //l.clear();

    while let Some(v) = l.pop() {
        println!("popd {:?} len: {:?}", v, l.count());
    }
}
