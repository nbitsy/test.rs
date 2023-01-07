use std::marker::PhantomData;

#[test]
fn test_phantom_base() {
    #[derive(Debug)]
    struct Object<'a, T> {
        value: T,
        _unused: PhantomData<&'a T>,
    }

    impl<'a, T> Object<'a, T> {
        pub const fn new(v: T) -> Self {
            return Object {
                value: v,
                _unused: PhantomData,
            };
        }
    }

    let o = Object::new(100);
    println!("{:?}  sizeof: {:?}", o, std::mem::size_of_val(&o));

    let v = 99.99;
    let o = Object::new(v);
    println!("{:?}  sizeof: {:?}", o, std::mem::size_of_val(&o));

    struct Slice<'a, T> {
        start: *const T,
        end: *const T,
        _unused: PhantomData<&'a T>,
    }

    trait Container<T>{}
    struct MyContainer<T, C: Container<T>> {
        container: C,
        _unused: PhantomData<T>,
    }

    struct Vec<T> {
        pointer: *const T,
        len: i32,
        cap: i32,
        _unused: PhantomData<T>,
    }
}
