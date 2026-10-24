extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct Moveable {
    int_: (),
    double_: (),
}
impl Moveable {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            int_: 0_(),
            double_: 0_(),
        };
        this
    }
    pub unsafe fn new_2(mut i: (), mut d: ()) -> Self {
        let mut this = Self {
            int_: i,
            double_: d,
        };
        this
    }
    pub unsafe fn move_from(x: *mut Moveable) -> Self {
        let mut this = Self {
            int_: (*x).int_,
            double_: (*x).double_,
        };
        (*x).int_ = -1_();
        (*x).double_ = -1_();
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut Moveable) -> *mut Moveable {
        self.int_ = (*x).int_;
        (*x).int_ = -1_();
        self.double_ = (*x).double_;
        (*x).double_ = -1_();
        return &mut (*(self as *mut Moveable));
    }
    pub unsafe fn operator_eq(&self, x: *const Moveable) -> () {
        return ((self.int_) == ((*x).int_)) && ((self.double_) == ((*x).double_));
    }
    pub unsafe fn operator_lt(&self, x: *const Moveable) -> () {
        return ((self.int_) < ((*x).int_))
            || (((self.int_) == ((*x).int_)) && ((self.double_) < ((*x).double_)));
    }
    pub unsafe fn hash(&self) -> () {
        return (unimplemented!()).wrapping_add(unimplemented!());
    }
    pub unsafe fn get(&self) -> () {
        return self.int_;
    }
    pub unsafe fn moved(&self) -> () {
        return ((self.int_) == (-1_()));
    }
}
impl std::cmp::Ord for Moveable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Moveable::operator_lt(self, other as *const Moveable) {
                std::cmp::Ordering::Less
            } else if Moveable::operator_lt(other, self as *const Moveable) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Moveable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Moveable {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Moveable::operator_eq(self, other as *const Moveable) }
    }
}
impl std::cmp::Eq for Moveable {}
impl Default for Moveable {
    fn default() -> Self {
        unsafe { Moveable::new_1() }
    }
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        let mut m: () = unimplemented!();
        let mut r: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (20_())) {
            unimplemented!();
            i += 2_();
        }
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (20_())) {
            let mut mv: Moveable = Moveable::new_2({ ((i) + (1_())) }, { ((i) + (1_())) });
            r = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            i += 2_();
        }
        let mut mv1: Moveable = Moveable::new_2({ 5_() }, { 5_f64 });
        r = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut mv2: Moveable = Moveable::new_2({ 9_() }, { 9_f64 });
        r = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut mv3: Moveable = Moveable::new_2({ -1_() }, { 5_f64 });
        r = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        let mut r: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (20_())) {
            unimplemented!();
            i += 2_();
        }
        unimplemented!();
        let mut mvkey1: Moveable = Moveable::new_2({ 2_() }, { 2_f64 });
        let mut mv1: Moveable = Moveable::new_2({ 4_() }, { 4_f64 });
        r = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut mvkey2: Moveable = Moveable::new_2({ 3_() }, { 3_f64 });
        let mut mv2: Moveable = Moveable::new_2({ 5_() }, { 5_f64 });
        r = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        let mut r: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (20_())) {
            unimplemented!();
            i += 2_();
        }
        unimplemented!();
        let mut it: () = unimplemented!();
        let mut mv1: Moveable = Moveable::new_2({ 3_() }, { 3_f64 });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut mv2: Moveable = Moveable::new_2({ 5_() }, { 5_f64 });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        let mut r: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (20_())) {
            unimplemented!();
            i += 2_();
        }
        unimplemented!();
        let mut it: () = unimplemented!();
        let mut mvkey1: Moveable = Moveable::new_2({ 2_() }, { 2_f64 });
        let mut mv1: Moveable = Moveable::new_2({ 4_() }, { 4_f64 });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut mvkey2: Moveable = Moveable::new_2({ 3_() }, { 3_f64 });
        let mut mv2: Moveable = Moveable::new_2({ 5_() }, { 5_f64 });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
