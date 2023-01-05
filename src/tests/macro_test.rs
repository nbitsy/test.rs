use procmacro::{make_hello};

macro_rules! MAX {
    ($x:expr, $y: expr) => { if ($x) > ($y) { ($x) } else { ($y) } }
}

#[test]
fn test_macro_max() {
    println!("Max: {:?}", MAX!(100, 999));
}

#[test]
fn test_mac_proc() {
    make_hello!(world);
    hello_world();
}

//#[derive(TestDerived)]
//struct DeriveTest {}
//
//#[test]
//fn test_mac_proc_derive() {
//    DeriveTest::hello();
//}