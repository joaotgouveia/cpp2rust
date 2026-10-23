extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static has_round_v_0: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static has_round_v_1: Value<()> = Rc::new(RefCell::new(false));
);
const _: () = assert!(true, "has_round_v<std::chrono::seconds>");
const _: () = assert!(true, "!has_round_v<int>");
pub fn test_2(df: Ptr<()>, d: Ptr<()>) {
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "(std::is_same<R, ToTimePoint>::value)");;
        unimplemented!();
    }
}
pub fn test_3(df: Ptr<()>, d: Ptr<()>) {
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "(std::is_same<R, ToTimePoint>::value)");;
        unimplemented!();
    }
}
pub fn test_constexpr_4() {
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::chrono::round<ToDuration>(f) == t");;
    }
}
pub fn test_constexpr_5() {
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::chrono::round<ToDuration>(f) == t");;
    }
}
pub fn test_constexpr_6() {
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::chrono::round<ToDuration>(f) == t");;
    }
}
pub fn test_constexpr_7() {
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::chrono::round<ToDuration>(f) == t");;
    }
}
pub fn test_constexpr_8() {
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::chrono::round<ToDuration>(f) == t");;
    }
}
pub fn test_constexpr_9() {
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::chrono::round<ToDuration>(f) == t");;
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
    ({
        let _df: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(_df.as_pointer(), _d.as_pointer())
    });
    ({
        let _df: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(_df.as_pointer(), _d.as_pointer())
    });
    ({
        let _df: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_3(_df.as_pointer(), _d.as_pointer())
    });
    ({
        let _df: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_3(_df.as_pointer(), _d.as_pointer())
    });
    ({ test_constexpr_4() });
    ({ test_constexpr_5() });
    ({ test_constexpr_6() });
    ({ test_constexpr_7() });
    ({ test_constexpr_8() });
    ({ test_constexpr_9() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = has_round_v_0.with(|_| ());
    let _ = has_round_v_1.with(|_| ());
}
