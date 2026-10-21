extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct MoveOnly {
    pub v: (),
}
impl MoveOnly {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn move_from(o: *mut MoveOnly) -> Self {
        let mut this = Self { v: (*o).v };
        (*o).v = unimplemented!();
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct ConstMove {
    pub mark: (),
}
impl ConstMove {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            mark: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_1(o: *mut ConstMove) -> Self {
        let mut this = Self {
            mark: (((*o).mark) + (1_())),
        };
        this
    }
    pub unsafe fn new_2(o: *const ConstMove) -> Self {
        let mut this = Self {
            mark: (((*o).mark) + (10_())),
        };
        this
    }
}
impl Default for ConstMove {
    fn default() -> Self {
        unsafe { ConstMove::new() }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct ThrowingMove {
    pub v: (),
    pub copies: (),
    pub moves: (),
}
impl ThrowingMove {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self {
            v: v,
            copies: unimplemented!(),
            moves: unimplemented!(),
        };
        this
    }
    pub unsafe fn copy_from(o: *const ThrowingMove) -> Self {
        let mut this = Self {
            v: (*o).v,
            copies: (((*o).copies) + (1_())),
            moves: (*o).moves,
        };
        this
    }
    pub unsafe fn move_from(o: *mut ThrowingMove) -> Self {
        let mut this = Self {
            v: (*o).v,
            copies: (*o).copies,
            moves: (((*o).moves) + (1_())),
        };
        (*o).v = unimplemented!();
        this
    }
}
impl Clone for ThrowingMove {
    fn clone(&self) -> Self {
        unsafe { ThrowingMove::copy_from(self as *const ThrowingMove) }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct NoexceptMove {
    pub v: (),
    pub copies: (),
    pub moves: (),
}
impl NoexceptMove {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self {
            v: v,
            copies: unimplemented!(),
            moves: unimplemented!(),
        };
        this
    }
    pub unsafe fn copy_from(o: *const NoexceptMove) -> Self {
        let mut this = Self {
            v: (*o).v,
            copies: (((*o).copies) + (1_())),
            moves: (*o).moves,
        };
        this
    }
    pub unsafe fn move_from(o: *mut NoexceptMove) -> Self {
        let mut this = Self {
            v: (*o).v,
            copies: (*o).copies,
            moves: (((*o).moves) + (1_())),
        };
        (*o).v = unimplemented!();
        this
    }
}
impl Clone for NoexceptMove {
    fn clone(&self) -> Self {
        unsafe { NoexceptMove::copy_from(self as *const NoexceptMove) }
    }
}
pub unsafe fn by_value_0(mut m: MoveOnly) -> () {
    return m.v;
}
pub unsafe fn make_1(mut v: ()) -> MoveOnly {
    let mut m: MoveOnly = MoveOnly::new({ v });
    return MoveOnly::move_from({ &mut (*unimplemented!()) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: MoveOnly = MoveOnly::new({ 1_() });
    let mut b: MoveOnly = MoveOnly::move_from({ &mut (*unimplemented!()) });
    unimplemented!();
    unimplemented!();
    let mut c: MoveOnly = MoveOnly::move_from({ &mut (*unimplemented!()) });
    unimplemented!();
    unimplemented!();
    let mut d: MoveOnly = MoveOnly::move_from({ &mut (*unimplemented!()) });
    unimplemented!();
    unimplemented!();
    let mut e: MoveOnly = (unsafe { make_1(5_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut vec_: () = unimplemented!();
    unimplemented!();
    let mut f: MoveOnly = MoveOnly::new({ 8_() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut m: ConstMove = ConstMove::new();
    let mut m1: ConstMove = ConstMove::new_1({ &mut (*unimplemented!()) });
    let cm: ConstMove = ConstMove::new();
    let mut m2: ConstMove = ConstMove::new_2({ &(*unimplemented!()) });
    unimplemented!();
    unimplemented!();
    let mut t: ThrowingMove = ThrowingMove::new({ 1_() });
    let mut t1: ThrowingMove = ThrowingMove::copy_from({ &(*unimplemented!()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut n: NoexceptMove = NoexceptMove::new({ 2_() });
    let mut n1: NoexceptMove = NoexceptMove::move_from({ &mut (*unimplemented!()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut g: MoveOnly = MoveOnly::new({ 3_() });
    let mut g1: MoveOnly = MoveOnly::move_from({ &mut (*unimplemented!()) });
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
