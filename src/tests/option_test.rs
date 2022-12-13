use std::collections::LinkedList;

#[test]
fn test_option_base() {
    let o = Some(100);
    println!("{:?}", o);
}

#[test]
fn test_option_iflet() {
    let hello = Some(100);
    let e = if let Some(100) = hello {
        99
    } else {
        100
    };

    println!("{:?}", e);
}

#[test]
fn test_option_1() {
}
