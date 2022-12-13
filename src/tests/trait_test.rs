use std::any::{Any, type_name};

#[derive(Debug)]
struct MyCell<T: ?Sized> {
    value: T
}

#[test]
fn test_trait() {
    let c = MyCell{value: 100};
    println!("{}", type_name::<i32>());
    println!("{:?}", c.type_id());
    //println!("{:?}, {:?}", c, type_name_of_val(&c));
}

#[derive(Debug)]
struct YaCell<T> {
    value: T
}

#[test]
fn test_trait_1() {
    let c = MyCell{value: 90};
    let c1 = MyCell{value: YaCell{value: 99}, };
    println!("{:?}", c1);
    let c2 = MyCell{value: YaCell{value: &[1,2,3]}, };
    println!("{:?}", c2);

    let v = &vec![1,2,3];
    let c3 = MyCell{value: YaCell{value: v, }};
    println!("{:?}", c3);

    let vv = Vec::from([1,2,3]);
    let vvv = vv.as_slice();
    println!("{:?}", vvv);
    let c4 = MyCell{value: YaCell{value: vvv, }};
    println!("{:?}", c4);
}
