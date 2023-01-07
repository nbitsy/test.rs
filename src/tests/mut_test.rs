use std::cell::{Cell, RefCell};
use std::mem::size_of_val;

#[test]
fn test_mut_base() {
    let mut can_not_mut_value = 100;
    can_not_mut_value = 1;
}

#[test]
fn test_inner_mut() {
    #[derive(Debug)]
    struct InnerMutValue {
        value: i32,
        mut_value: Cell<i32>,
    }

    let imv = InnerMutValue { value: 100, mut_value: Cell::new(200) };
    println!("{:?}", imv);
    //imv.mut_value.set(199);
    imv.mut_value.set(199);
    println!("{:?}", imv);
}

#[test]
fn test_inner_ref_cell() {
    #[derive(Debug)]
    struct InnerValue {
        value: i32,
        mut_value: i32,
        str_value: &'static str,
    }

    #[derive(Debug)]
    struct InnerMutValue {
        value: i32,
        mut_value: RefCell<InnerValue>,
    }

    let imv = InnerMutValue {
        value: 100,
        mut_value: RefCell::new(
            InnerValue { value: 222, mut_value: 333, str_value: "Hello" }),
    };
    println!("sizeof: {:?}", size_of_val(&imv));
    println!("{:?}", imv);
    (*imv.mut_value.borrow_mut()).value = 999;
    println!("{:?}", imv);
}