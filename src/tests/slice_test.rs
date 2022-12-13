#[test]
fn test_str_slice() {
    let s = String::from("Hello World!");
    let ss = s.as_bytes();
    let s1 = &ss[0..2];
    println!("{:?}", String::from_utf8_lossy(s1));
}

fn first_world(s: &str) -> &str {
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}

#[test]
fn test_str() {
    let w = "Hello World!";
    let w = first_world(w);
    println!("{:?}", w);
}
