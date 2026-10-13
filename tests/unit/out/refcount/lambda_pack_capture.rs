extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_0() -> i32 {
    return 0;
}
pub fn sum_1(mut t: i32, mut u_1: i32, mut u_2: i32) -> i32 {
    return (t + ({ sum_2(u_1, u_2) }));
}
pub fn sum_2(mut t: i32, mut u: i32) -> i32 {
    return (t + ({ sum_3(u) }));
}
pub fn sum_3(mut t: i32) -> i32 {
    return (t + ({ sum_0() }));
}
pub fn by_value_4() -> i32 {
    let f: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
            {
                return ({ sum_0() });
            }
        })));
    ();
    return ({ (*f.borrow()).call() }).clone();
}
pub fn by_value_6(t_0: i32, t_1: i32, t_2: i32) -> i32 {
    let t_0: Value<i32> = Rc::new(RefCell::new(t_0));
    let t_1: Value<i32> = Rc::new(RefCell::new(t_1));
    let t_2: Value<i32> = Rc::new(RefCell::new(t_2));
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let t_0: Value<i32> = Rc::new(RefCell::new((*t_0.borrow())));
            let t_1: Value<i32> = Rc::new(RefCell::new((*t_1.borrow())));
            let t_2: Value<i32> = Rc::new(RefCell::new((*t_2.borrow())));
        },
        || -> i32 {
            return ({ sum_1((*t_0.borrow()), (*t_1.borrow()), (*t_2.borrow())) });
        }
    )));
    {
        (*t_0.borrow_mut()) = 0;
        {
            (*t_1.borrow_mut()) = 0;
            (*t_2.borrow_mut()) = 0
        }
    };
    return ({ (*f.borrow()).call() }).clone();
}
pub fn by_ref_8(t_0: i32, t_1: i32, t_2: i32) -> i32 {
    let t_0: Value<i32> = Rc::new(RefCell::new(t_0));
    let t_1: Value<i32> = Rc::new(RefCell::new(t_1));
    let t_2: Value<i32> = Rc::new(RefCell::new(t_2));
    let f: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
        {
            let t_0: Ptr<i32> = t_0.as_pointer();
            let t_1: Ptr<i32> = t_1.as_pointer();
            let t_2: Ptr<i32> = t_2.as_pointer();
        },
        || {
            {
                {
                    t_0.with_mut(|__v| *__v = *__v * 2)
                };
                {
                    {
                        t_1.with_mut(|__v| *__v = *__v * 2)
                    };
                    { t_2.with_mut(|__v| *__v = *__v * 2) }
                }
            };
        }
    )));
    ({ (*f.borrow()).call() });
    return ({ sum_1((*t_0.borrow()), (*t_1.borrow()), (*t_2.borrow())) });
}
pub fn init_pack_10(mut t_0: i32, mut t_1: i32, mut t_2: i32) -> i32 {
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let xs_0: Value<i32> = Rc::new(RefCell::new((t_0 + 1)));
            let xs_1: Value<i32> = Rc::new(RefCell::new((t_1 + 1)));
            let xs_2: Value<i32> = Rc::new(RefCell::new((t_2 + 1)));
        },
        || -> i32 {
            return ({ sum_1((*xs_0.borrow()), (*xs_1.borrow()), (*xs_2.borrow())) });
        }
    )));
    return ({ (*f.borrow()).call() }).clone();
}
pub fn implicit_12(t_0: i32, t_1: i32) -> i32 {
    let t_0: Value<i32> = Rc::new(RefCell::new(t_0));
    let t_1: Value<i32> = Rc::new(RefCell::new(t_1));
    return ({
        lambda!(
            {
                let t_0: Value<i32> = Rc::new(RefCell::new((*t_0.borrow())));
                let t_1: Value<i32> = Rc::new(RefCell::new((*t_1.borrow())));
            },
            || -> i32 {
                return ({ sum_2((*t_0.borrow()), (*t_1.borrow())) });
            }
        )
        .call()
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    (if (({ by_value_4() }) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"by_value() == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ by_value_6(1, 2, 3) }) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"by_value(1, 2, 3) == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ by_ref_8(1, 2, 3) }) == 12) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"by_ref(1, 2, 3) == 12"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ init_pack_10(1, 2, 3) }) == 9) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"init_pack(1, 2, 3) == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ implicit_12(4, 5) }) == 9) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"implicit(4, 5) == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_13(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
