#[test]
fn test_string_base() {
    let mut s = String::from("Hello");
    s.push_str(", World");
    println!("{:?}", s);
}

#[test]
fn test_string_byte_array() {
    let s = String::from("Hello");
    println!("{:?}", &s as *const String);
    let a = s.as_bytes();
    println!("{:?} {:?}", &a[0] as *const u8, a);
    let a = s.as_ptr();
    println!("{:?} {:?}", a as *const u8, a);
}