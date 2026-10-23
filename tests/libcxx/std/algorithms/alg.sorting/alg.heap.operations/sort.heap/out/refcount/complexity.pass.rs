extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct Stats {
    #[offset(0)]
    pub compared: (),
    #[offset(4)]
    pub copied: (),
    #[offset(8)]
    pub moved: (),
}
impl Default for Stats {
    fn default() -> Self {
        {
            Stats {
                compared: 0_(),
                copied: 0_(),
                moved: 0_(),
            }
        }
    }
}
thread_local!(
    pub static stats_0: Value<Stats> = Rc::new(RefCell::new(<Stats>::default()));
);
pub fn operator_lt_1(a: Ptr<MyInt>, b: Ptr<MyInt>) -> () {
    (*stats_0.with(Value::clone).borrow_mut())
        .compared
        .prefix_inc();
    return ({ a.with(|__s| __s.value) } < { b.with(|__s| __s.value) });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign, Default)]
#[byte_size(4)]
pub struct MyInt {
    #[offset(0)]
    pub value: (),
}
impl MyInt {
    pub fn new(mut xval: ()) -> Self {
        Self { value: xval }
    }
    pub fn copy_from(other: Ptr<MyInt>) -> Self {
        let __this: MyInt = Self {
            value: other.with(|__s| __s.value),
        };
        (*stats_0.with(Value::clone).borrow_mut())
            .copied
            .prefix_inc();
        __this
    }
    pub fn move_from(other: Ptr<MyInt>) -> Self {
        let __this: MyInt = Self {
            value: other.with(|__s| __s.value),
        };
        (*stats_0.with(Value::clone).borrow_mut())
            .moved
            .prefix_inc();
        __this
    }
}
impl std::cmp::Ord for MyInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_1(
                Rc::new(RefCell::new(MyInt {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(MyInt {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_1(
                Rc::new(RefCell::new(MyInt {
                    value: other.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(MyInt {
                    value: self.value.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for MyInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MyInt {
    fn eq(&self, other: &Self) -> bool {
        {
            !(operator_lt_1(
                Rc::new(RefCell::new(MyInt {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(MyInt {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            )) && !(operator_lt_1(
                Rc::new(RefCell::new(MyInt {
                    value: other.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(MyInt {
                    value: self.value.clone(),
                }))
                .as_pointer(),
            ))
        }
    }
}
impl std::cmp::Eq for MyInt {}
impl Clone for MyInt {
    fn clone(&self) -> Self {
        let __src: Value<MyInt> = Rc::new(RefCell::new(MyInt {
            value: self.value.clone(),
        }));
        MyInt::copy_from(__src.as_pointer())
    }
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    let mut N: () = (1_() << 20_());
    let mut v: () = unimplemented!();
    unimplemented!();
    let g: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < N) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut logn: () = 10_();
    'loop_: while (logn <= 20_()) {
        let mut n: () = (1_() << logn);
        let first: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let last: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut debug_elements: () = (unimplemented!().read());
        let mut debug_comparisons: () = ((2_() * (debug_elements + 1_())) * debug_elements);
        &(debug_comparisons);
        unimplemented!();
        unimplemented!();
        stats_0.with(|rc| *rc.borrow_mut() = <Stats>::default());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        logn.prefix_inc();
    }
    return 0_();
}
pub trait MyIntImpl {
    fn copy_assign(&self, other: Ptr<MyInt>) -> Ptr<MyInt>;
    fn move_assign(&self, other: Ptr<MyInt>) -> Ptr<MyInt>;
}
impl MyIntImpl for Ptr<MyInt> {
    fn copy_assign(&self, other: Ptr<MyInt>) -> Ptr<MyInt> {
        field!((*self), value).write({ other.with(|__s| __s.value) });
        (*stats_0.with(Value::clone).borrow_mut())
            .copied
            .prefix_inc();
        return (*self).clone();
    }
    fn move_assign(&self, other: Ptr<MyInt>) -> Ptr<MyInt> {
        field!((*self), value).write({ other.with(|__s| __s.value) });
        (*stats_0.with(Value::clone).borrow_mut())
            .moved
            .prefix_inc();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = stats_0.with(|_| ());
}
