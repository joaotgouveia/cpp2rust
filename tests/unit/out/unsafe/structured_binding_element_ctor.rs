extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moves_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Elem {
    pub v: (),
}
impl Elem {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(other: *const Elem) -> Self {
        let mut this = Self {
            v: (((*other).v) + (100_())),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut Elem) -> Self {
        let mut this = Self {
            v: (((*other).v) + (1000_())),
        };
        (*other).v = -1_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)).prefix_inc();
        this
    }
}
impl Clone for Elem {
    fn clone(&self) -> Self {
        unsafe { Elem::copy_from(self as *const Elem) }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Two {
    pub a: Elem,
    pub b: Elem,
}
impl Two {
    pub unsafe fn move_from(_a0: *mut Two) -> Self {
        let mut this = Self {
            a: Elem::move_from({ &mut (*_a0).a }),
            b: Elem::move_from({ &mut (*_a0).b }),
        };
        this
    }
}
pub unsafe fn make_two_2() -> Two {
    return Two {
        a: Elem::new({ 1_() }),
        b: Elem::new({ 2_() }),
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut t: Two = Two {
        a: Elem::new({ 1_() }),
        b: Elem::new({ 2_() }),
    };
    let mut base_copies: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0));
    let mut base_moves: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1));
    let mut __decomp_3: Two = t.clone();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let __decomp_4: *mut Two = &mut t;
    unimplemented!();
    unimplemented!();
    (*__decomp_4).a.v = 50_();
    unimplemented!();
    let mut __decomp_5: Two = Two::move_from({ &mut t });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut before_copies: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0));
    let mut before_moves: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1));
    let mut __decomp_6: Two = (unsafe { make_two_2() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: std_pair_Elem__Elem_ = {
        let mut __tmp_0: Elem = Elem::new({ 7_() });
        let mut __tmp_1: Elem = Elem::new({ 8_() });
        std_pair_Elem__Elem_::new({ &mut __tmp_0 }, { &mut __tmp_1 })
    };
    before_copies = (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0));
    let mut __decomp_7: std_pair_Elem__Elem_ = p.clone();
    let x: *mut Elem = (unsafe { get_8(&mut __decomp_7) });
    let y: *mut Elem = (unsafe { get_9(&mut __decomp_7) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
    std::cell::LazyCell::force(&*&raw const moves_1);
}
