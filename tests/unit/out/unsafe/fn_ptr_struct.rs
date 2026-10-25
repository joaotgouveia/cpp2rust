extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Handler {
    pub tag: (),
    pub cb: Option<unsafe fn(()) -> ()>,
}
pub unsafe fn double_it_0(mut x: ()) -> () {
    return ((x) * (2_()));
}
pub unsafe fn negate_1(mut x: ()) -> () {
    return -x;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {}
impl S {
    pub unsafe fn pick_1(mut x: ()) -> () {
        return ((x) + (1_()));
    }
    pub unsafe fn pick_2(mut x: ()) -> () {
        return ((x) + (2_()));
    }
    pub unsafe fn solo(mut x: ()) -> () {
        return ((x) + (3_()));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct MoveOnly {
    pub data_: (),
}
impl MoveOnly {
    pub unsafe fn new(mut data: ()) -> Self {
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0_();
        this
    }
    pub unsafe fn get(&self) -> () {
        return self.data_;
    }
}
pub unsafe fn make_move_only_2() -> MoveOnly {
    return MoveOnly::new({ 4_() });
}
pub unsafe fn scale_move_only_3(mut x: MoveOnly) -> MoveOnly {
    return MoveOnly::new({ ((unsafe { MoveOnly::get(&x) }) * (10_())) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut p1: Option<unsafe fn(()) -> ()> = (Some(S::pick_1));
    let mut p2: Option<unsafe fn(()) -> ()> = Some(S::solo);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut h3: Handler = Handler {
        tag: 3_(),
        cb: (Some(S::pick_1)),
    };
    unimplemented!();
    let mut h1: Handler = Handler {
        tag: 1_(),
        cb: Some(double_it_0),
    };
    let mut h2: Handler = Handler {
        tag: 2_(),
        cb: Some(negate_1),
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (h1.cb) = Some(negate_1);
    unimplemented!();
    unimplemented!();
    let mut mk: Option<unsafe fn() -> MoveOnly> = Some(make_move_only_2);
    let mut m: MoveOnly = (unsafe { (mk).unwrap()() });
    unimplemented!();
    let mut sc: Option<unsafe fn(MoveOnly) -> MoveOnly> = Some(scale_move_only_3);
    let mut n: MoveOnly = (unsafe { (sc).unwrap()(MoveOnly::move_from({ &mut m })) });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
