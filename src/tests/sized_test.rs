
#![feature(type_name_of_val)]

use std::any::{Any};

#[test]
fn test_sized_base() {
    #[derive(Debug)]
    struct SizedObject<T: Sized> {
        value: T,
    }

    {
        let so = SizedObject { value: 100 };
        println!("{:?} size: {:?}", so, std::mem::size_of_val(&so));
    }

    {
        let so = SizedObject { value: 100.9 };
        println!("{:?} size: {:?}", so, std::mem::size_of_val(&so));
    }
}
