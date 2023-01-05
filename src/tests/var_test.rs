use std::collections::LinkedList;

#[test]
fn test_list() {
    let l = LinkedList::<i32>::new();
}


/// 不可变变量和不可变变量的隐藏
pub fn test_var() {
    let x = 5;
    println!("{:?}", x);
    let x = 9;
    println!("{:?}", x);

    // 这个无法赋值，因为在 Rust 里变量默认是不可变的
    //x = 199
}

/// 可变变量
pub fn test_mut_var() {
    let mut x = 5;
    println!("{:?}", x);
    x = 9;
    println!("{:?}", x);
}

#[test]
pub fn test_cast_var() {
    let x = 5;
    let mut xx = x;
    println!("{:?} {:?}", &x as *const i32, x);
    println!("{:?} {}", &xx as *const i32, xx);
    xx = 999;
    println!("{:?} {}", &xx as *const i32, xx);

    // 所谓的不可变，都只是字面意思，正常情况下是不可变，但实际上需要修改它是可以做到的
    let px = (&x as *const i32) as *mut i32;
    unsafe { *px = 98; }
    println!("{:?} {}", px, x);
}

const CONST_VALUE_STR: &'static str = "Hello World";
const CONST_VALUE_INT32: i32 = 9999;
const CONST_VALUE_F64: f64 = 99999.;

#[test]
pub fn test_const() {
    println!("{:?}", CONST_VALUE_STR);
    println!("{:?}", CONST_VALUE_INT32);
    println!("{:?}", CONST_VALUE_F64);
}

#[test]
pub fn test_all() {
    test_var();
    test_mut_var();
    test_const();
}