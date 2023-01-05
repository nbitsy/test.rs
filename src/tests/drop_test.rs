use std::mem;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct MyObject {
    name: String,
}

impl MyObject {
    pub fn new(n: &str) -> MyObject {
        return MyObject { name: String::from(n) };
    }
}

impl Drop for MyObject {
    fn drop(&mut self) {
        println!("Drop @{:?} {:?}", self as *const MyObject, self);
    }
}

#[test]
fn test_drop_base() {
    let o1 = MyObject::new("o1");
    println!("{:?}", o1);
    {
        let o2 = MyObject::new("o2");
        println!("{:?}", o2);
        {
            let o3 = MyObject::new("o3");
            println!("{:?}", o3);
        }
    }
}

#[test]
fn test_drop_advance() {
    let o1 = MyObject::new("o1");
    {
        let o2 = MyObject::new("o2");
        println!("{:?}", o2);
        drop(o1);
    }
}
