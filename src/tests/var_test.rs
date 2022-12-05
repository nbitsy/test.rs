
pub fn test_var() {
    let x = 5;
    println!("{:?}", x);
    let x = 9;
    println!("{:?}", x);

    // 这个无法赋值，因为在 Rust 里变量默认是不可变的
    //x = 199
}

pub fn test_mut_var() {
    let mut x = 5;
    println!("{:?}", x);
    x = 9;
    println!("{:?}", x);
}

#[test]
pub fn test_all() {
    test_var();
    test_mut_var()
}