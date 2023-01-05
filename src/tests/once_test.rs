
use bwserver::util;

#[test]
fn test_once() {
    util::once::add_once(move || {
        println!("hello");
    });
    util::once::add_once(move || {
        println!("world");
    });

    util::once::call_once();
    util::once::call_once();
    util::once::call_once();
    //util::once::call_once_only(move || {
    //    println!("Can here 1?")
    //});
    //util::once::call_once_only(move |x, y| {
    //    println!("Can here 2?")
    //});
}
