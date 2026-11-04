extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn pick_0(x: Option<i32>) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x.unwrap_or_else(|| {
        ({
            FnPtr::<fn() -> i32>::new(|| -> i32 {
                {
                    return 237;
                }
            })
            .call()
        })
    })));
    return (*x.borrow());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    pub j: i32,
    #[offset(4)]
    pub k: i32,
}
impl CppMove for S {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                j: 10,
                k: (({
                    FnPtr::<fn() -> i32>::new(|| -> i32 {
                        {
                            return 3;
                        }
                    })
                    .call()
                }) + 1),
            }
        }
    }
}
thread_local!(
    pub static g_1: Value<i32> = Rc::new(RefCell::new(
        ({
            FnPtr::<fn() -> i32>::new(|| -> i32 {
                {
                    let s: Value<i32> = Rc::new(RefCell::new(0));
                    let mut i: i32 = 1;
                    'loop_: while (i <= 4) {
                        (*s.borrow_mut()) += i;
                        i.postfix_inc();
                    }
                    return (*s.borrow());
                }
            })
            .call()
        }),
    ));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ cpp2rust_assert_fail_2((({ pick_0(None) }) == 237)) });
    ({ cpp2rust_assert_fail_2((({ pick_0(Some(1)) }) == 1)) });
    let mut s: S = <S>::default();
    ({ cpp2rust_assert_fail_2((s.k == 4)) });
    ({ cpp2rust_assert_fail_2((g_1.with(|rc| *rc.borrow()) == 10)) });
    let a: Value<i32> = Rc::new(RefCell::new(2));
    let mut c: i32 = ({
        lambda!(
            {
                let a: Ptr<i32> = a.as_pointer();
            },
            || -> i32 {
                a.with_mut(|__v| __v.postfix_inc());
                return ((a.read()) * 10);
            }
        )
        .call()
    });
    ({ cpp2rust_assert_fail_2((c == 30)) });
    ({ cpp2rust_assert_fail_2(((*a.borrow()) == 3)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = g_1.with(|_| ());
}
