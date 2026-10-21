extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct S {
    pub v: (),
    pub n: [(); 2],
}
impl S {
    pub unsafe fn new(mut x: ()) -> Self {
        let mut this = Self {
            v: unimplemented!(),
            n: [x, ((x) + (1_()))],
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut S) -> Self {
        let mut this = Self {
            v: unimplemented!(),
            n: std::array::from_fn::<_, 2, _>(|__i: usize| (*_a0).n[(__i) as usize]),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut S) -> *mut S {
        unimplemented!();
        unimplemented!();
        return &mut (*(self as *mut S));
    }
}
pub unsafe fn sum_0(s: *const S) -> () {
    return (((unimplemented!()) + ((*s).n[(unimplemented!()) as usize]))
        + ((*s).n[(1_()) as usize]));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S::new({ 2_() });
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn shuffle_1(mut x: ()) -> () {
    let mut a: S = S::new({ x });
    let mut b: S = S::move_from({ &mut (*unimplemented!()) });
    unimplemented!();
    let mut c: S = S::new({ 1_() });
    (unsafe { S::move_assign(&mut c, &mut (*unimplemented!())) });
    unimplemented!();
    return (unsafe { sum_0(&c) });
}
pub unsafe fn __cpp2rust_init_globals() {}
