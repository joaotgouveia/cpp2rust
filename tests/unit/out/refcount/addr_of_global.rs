extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner {
    #[offset(0)]
    pub value: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<Inner>,
}
thread_local!(
    pub static alpha_0: Value<Inner> = Rc::new(RefCell::new(Inner { value: 1 }));
);
thread_local!(
    pub static beta_1: Value<Inner> = Rc::new(RefCell::new(Inner { value: 2 }));
);
thread_local!(
    pub static shared_2: Value<Inner> = Rc::new(RefCell::new(Inner { value: 42 }));
);
thread_local!(
    pub static items_3: Value<Box<[Ptr<Inner>]>> = Rc::new(RefCell::new(Box::new([
        (alpha_0.with(|v| v.as_pointer())),
        (beta_1.with(|v| v.as_pointer())),
    ])));
);
thread_local!(
    pub static obj_4: Value<Outer> = Rc::new(RefCell::new(Outer {
        p: (shared_2.with(|v| v.as_pointer())),
    }));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    (if (({
        let __idx = (0) as usize;
        items_3.with(|rc| rc.borrow()[__idx].clone())
    })
    .with(|__s| __s.value)
        == 1)
    {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"items[0]->value == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let __idx = (1) as usize;
        items_3.with(|rc| rc.borrow()[__idx].clone())
    })
    .with(|__s| __s.value)
        == 2)
    {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"items[1]->value == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*obj_4.with(Value::clone).borrow()).p.clone() }.with(|__s| __s.value) == 42) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"obj.p->value == 42"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let __idx = (0) as usize;
        cache_6.with(|rc| rc.borrow()[__idx].clone())
    })
    .with(|__s| __s.value)
        == 1)
    {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"cache[0]->value == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let __idx = (1) as usize;
        cache_6.with(|rc| rc.borrow()[__idx].clone())
    })
    .with(|__s| __s.value)
        == 2)
    {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"cache[1]->value == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
thread_local!(
    static cache_6: Value<Box<[Ptr<Inner>]>> = Rc::new(RefCell::new(Box::new([
        (alpha_0.with(|v| v.as_pointer())),
        (beta_1.with(|v| v.as_pointer())),
    ])));
);
pub fn __assert_fail_5(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {
    let _ = alpha_0.with(|_| ());
    let _ = beta_1.with(|_| ());
    let _ = shared_2.with(|_| ());
    let _ = items_3.with(|_| ());
    let _ = obj_4.with(|_| ());
}
