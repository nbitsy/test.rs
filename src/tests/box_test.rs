#[test]
fn test_box_base() {
    let value_on_stack = 99;
    let value_in_heap = Box::new(100);

    println!("value_on_stack {:?}, address of value_on_stack: {:?}",
             value_on_stack,
             &value_on_stack as *const i32
    );
    println!("value_in_heap: {:?}, address of value_in_heap: {:?}",
             value_in_heap,
             &value_in_heap as *const Box<i32>);

    println!("&value_in_head - &value_on_stack: {:?}",
             (&value_in_heap as *const Box<i32> as usize) - (&value_on_stack as *const i32 as usize));

    //let take_the_value_in_heap = value_in_heap;
    println!("{:?}", value_in_heap);
}

#[test]
fn test_box_leak() {
    let value_in_heap = Box::new(99);
    println!("value_in_heap: {:?}, &value_in_heap: {:?}, value heap address: {:?}",
             value_in_heap,
             &value_in_heap as *const Box<i32>,
             value_in_heap.as_ref() as *const i32
    );

    println!("value_in_heap: {:?}", value_in_heap);
    let v = Box::leak(value_in_heap);
    *v += 1;
    println!("leak value: {:?}, address in heap: {:?}", v, v as *const i32);
    let box_value = unsafe { Box::from_raw(v) };
    println!("box_value: {:?}", box_value);
}
