use std::cell::Cell;

#[test]
fn test_lifetime_base() {
    let mut long_life_value = 199;
    {
        println!("long_life_value: {:?}", long_life_value);
        let short_life_value = 123;

        unsafe {
            std::ptr::write(&long_life_value as *const i32 as *mut i32, short_life_value);
        }

        println!("long_life_value: {:?}", long_life_value);
    }
}