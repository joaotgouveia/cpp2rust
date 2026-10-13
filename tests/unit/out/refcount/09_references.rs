extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static counter_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn inc_1() {
    counter_0.with(|rc| *rc.borrow_mut() += 1);
}
pub fn dec_2() {
    counter_0.with(|rc| *rc.borrow_mut() -= 1);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let h: Value<i32> = Rc::new(RefCell::new(15));
    let h_ref1: Ptr<i32> = h.as_pointer();
    h_ref1.write(16);
    let mut h_ptr: Ptr<i32> = (h_ref1).clone();
    let h_ref2: Ptr<i32> = (h_ptr).clone();
    h_ref2.write(17);
    (if (({ (h_ref1.read()) } + { (h_ref2.read()) }) == 34) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"h_ref1 + h_ref2 == 34"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let b: Value<i32> = Rc::new(RefCell::new(2));
    let r: Ptr<i32> = if ((*a.borrow()) < (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    };
    r.write(10);
    (if ((*a.borrow()) == 10) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"a == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let cr: Ptr<i32> = if ((*a.borrow()) > (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    };
    (if ((cr.read()) == 10) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"cr == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let x: Value<i32> = Rc::new(RefCell::new(1));
    let y: Value<i32> = Rc::new(RefCell::new(2));
    let cx: Ptr<i32> = if ((*x.borrow()) < (*y.borrow())) {
        (x.as_pointer())
    } else {
        (y.as_pointer())
    };
    (if ((cx.read()) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"cx == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut cp: Ptr<i32> = if ((*a.borrow()) > (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    };
    (if ((cp.read()) == 10) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"*cp == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut mp: Ptr<i32> = if ((*a.borrow()) < (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    };
    mp.write(20);
    (if ((*b.borrow()) == 20) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"b == 20"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    if ((*a.borrow()) < (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    }
    .write(30);
    (if ((*a.borrow()) == 30) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"a == 30"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((*b.borrow()) == 20) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"b == 20"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    {
        if ((*a.borrow()) < (*b.borrow())) {
            (a.as_pointer())
        } else {
            (b.as_pointer())
        }
        .with_mut(|__v| *__v = *__v + 5)
    };
    (if ((*b.borrow()) == 25) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"b == 25"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut ap: Ptr<i32> = (if ((*a.borrow()) > (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    });
    ap.write(40);
    (if ((*a.borrow()) == 40) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"a == 40"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    if ((*a.borrow()) < (*b.borrow())) {
        ({ inc_1() });
    } else {
        ({ dec_2() });
    };
    (if (counter_0.with(|rc| *rc.borrow()) == -1_i32) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"counter == -1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    if ((*a.borrow()) > (*b.borrow())) {
        ({ inc_1() });
    } else {
        ({ dec_2() });
    };
    (if (counter_0.with(|rc| *rc.borrow()) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"counter == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_3(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {
    let _ = counter_0.with(|_| ());
}
