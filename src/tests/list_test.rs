use std::borrow::{Borrow, BorrowMut};
use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::rc::Rc;
use crate::tests::list_test::EList::{Nil, Value};

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

#[derive(Debug)]
enum EList<T> {
    Value(T, Box<EList<T>>),
    Nil,
}

#[test]
fn test_elist() {
    let l = Value(1, Box::new(Value(2, Box::new(Value(3, Box::new(Nil))))));
    println!("{:?}", l);
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    size: usize,
}

#[derive(Debug)]
struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    value: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        return Self { head: None, tail: None, size: 0 };
    }

    fn push_back_node(&mut self, mut node: Box<Node<T>>) {
    }

    fn push_front_node(&mut self, mut node: Box<Node<T>>) {}

    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        None
    }

    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        None
    }
}

#[test]
fn test_real_list() {
    let mut l = LinkedList::<i32>::new();
    println!("{:?}", l);
}