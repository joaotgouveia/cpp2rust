extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static a_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let next: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
            {
                return (*(*n_1.with(Value::clone).borrow_mut()).prefix_inc());
            }
        })));
    ({ cpp2rust_assert_fail_2((({ (*next.borrow()).call() }) == 1)) });
    ({ cpp2rust_assert_fail_2((({ (*next.borrow()).call() }) == 2)) });
    let copy: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*next.borrow()).copy_from()));
    ({ cpp2rust_assert_fail_2((({ (*copy.borrow()).call() }) == 3)) });
    ({ cpp2rust_assert_fail_2((({ (*next.borrow()).call() }) == 4)) });
    let fresh: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
            {
                return (*(*n_1.with(Value::clone).borrow_mut()).prefix_inc());
            }
        })));
    ({ cpp2rust_assert_fail_2((({ (*fresh.borrow()).call() }) == 5)) });
    ({ cpp2rust_assert_fail_2((({ (*next.borrow()).call() }) == 6)) });
    let per_type: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |mut x: i32| -> i32 {
            (*calls_3.with(Value::clone).borrow_mut()).postfix_inc();
            return calls_3.with(|rc| *rc.borrow());
        },
        |mut x: f64| -> i32 {
            (*calls_4.with(Value::clone).borrow_mut()).postfix_inc();
            return calls_4.with(|rc| *rc.borrow());
        }
    )));
    ({
        cpp2rust_assert_fail_2((({ (*per_type.borrow()).spec::<fn(i32) -> i32>(0).call(1) }) == 1))
    });
    ({
        cpp2rust_assert_fail_2((({ (*per_type.borrow()).spec::<fn(i32) -> i32>(0).call(2) }) == 2))
    });
    ({
        cpp2rust_assert_fail_2(
            (({ (*per_type.borrow()).spec::<fn(f64) -> i32>(1).call(1_f64) }) == 1),
        )
    });
    ({
        cpp2rust_assert_fail_2((({ (*per_type.borrow()).spec::<fn(i32) -> i32>(0).call(3) }) == 3))
    });
    let read: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::new(
        |mut _a0: i32| -> i32 {
            {
                return a_0.with(|rc| *rc.borrow());
            }
        },
    )));
    ({ cpp2rust_assert_fail_2((({ (*read.borrow()).call(0) }) == 0)) });
    a_0.with(|rc| *rc.borrow_mut() = 1);
    ({ cpp2rust_assert_fail_2((({ (*read.borrow()).call(0) }) == 1)) });
    let local: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
            {
                let mut p: P_5 = P_5 { x: 2, y: 3 };
                return (p.x * p.y);
            }
        })));
    ({ cpp2rust_assert_fail_2((({ (*local.borrow()).call() }) == 6)) });
    return 0;
}
thread_local!(
    static n_1: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    static calls_3: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    static calls_4: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct P_5 {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl CppMove for P_5 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = a_0.with(|_| ());
}
