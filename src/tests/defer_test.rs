use bwserver::{defer, util};

#[test]
fn test_defer_base() {
    pub struct Defer<F: FnOnce()>(Option<F>);
    impl<F: FnOnce()> Drop for Defer<F> {
        fn drop(&mut self) {
            if let Some(f) = self.0.take() {
                f();
            }
        }
    }

    {
        let _1 = Defer(Some(|| {
            println!("--------------1");
        }));
    }

    let _2 = Defer(Some(|| {
        println!("---------------2");
    }));

    println!("-----------------3");
}

#[test]
fn test_defer_struct() {
    use bwserver::util::defer;

    defer! {
        println!("-------------------------1");
    }

    {
        defer! {
            println!("-------------------------2");
        }
    }

    {
        defer! {
            println!("-------------------------3");
            println!("-------------------------3")
        }
    }

    defer! {
        println!("-------------------------4");
    }

    defer! (println!("ffffffffffffffffffffffff - 5"); println!("HAHAHAH"));
    defer! {}
    defer! ();
}


#[test]
fn test_defer() {
    println!("defer 1");

    defer!(println!("defer 2"););

    {
        defer!(println!("defer 3"););
        defer! {
            println!("defer 4");
        }
    }

    defer!(println!("defer 5"););
    println!("defer 6");

    defer! (println!("xxxxx"););
}
