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
    int_: i32,
    double_: f64,
}
impl Moveable {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            int_: 0,
            double_: 0_f64,
        };
        this
    }
    pub unsafe fn new_2(mut i: i32, mut d: f64) -> Self {
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
        (*x).int_ = -1_i32;
        (*x).double_ = (-1_i32 as f64);
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut Moveable) -> *mut Moveable {
        self.int_ = (*x).int_;
        (*x).int_ = -1_i32;
        self.double_ = (*x).double_;
        (*x).double_ = (-1_i32 as f64);
        return &mut (*(self as *mut Moveable));
    }
    pub unsafe fn operator_eq(&self, x: *const Moveable) -> bool {
        return ((self.int_) == ((*x).int_)) && ((self.double_) == ((*x).double_));
    }
    pub unsafe fn operator_lt(&self, x: *const Moveable) -> bool {
        return ((self.int_) < ((*x).int_))
            || (((self.int_) == ((*x).int_)) && ((self.double_) < ((*x).double_)));
    }
    pub unsafe fn get(&self) -> i32 {
        return self.int_;
    }
    pub unsafe fn moved(&self) -> bool {
        return ((self.int_) == (-1_i32));
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
pub unsafe fn test_0() -> bool {
    {
        let mut m: UnsafeMap<i32, (i32, Moveable)> = UnsafeMap::new();
        let mut r: (UnsafeBidirectionalIterator<(i32, Moveable)>, bool) = unimplemented!();
        let mut i: i32 = 0;
        'loop_: while ((i) < (20)) {
            unimplemented!();
            i += 2;
        }
        assert!(((m.len()) == (10_usize)));
        let mut mv1: Moveable = Moveable::new_2({ 3 }, { 3_f64 });
        let mut i: i32 = 0;
        'loop_: while ((i) < (20)) {
            r = unimplemented!();
            assert!(((m.len()) == (10_usize)));
            assert!(!(r.1));
            assert!(!(unsafe { Moveable::moved(&mv1,) }));
            assert!((((*r.0.iter_deref()).0) == (i)));
            i += 2;
        }
        r = unimplemented!();
        assert!(((m.len()) == (11_usize)));
        assert!((r.1 as bool));
        assert!((unsafe { Moveable::moved(&mv1,) }));
        assert!((((*r.0.iter_deref()).0) == (-1_i32)));
        assert!(((unsafe { Moveable::get(&(*r.0.iter_deref()).1,) }) == (3)));
        let mut mv2: Moveable = Moveable::new_2({ 5 }, { 3_f64 });
        r = unimplemented!();
        assert!(((m.len()) == (12_usize)));
        assert!((r.1 as bool));
        assert!((unsafe { Moveable::moved(&mv2,) }));
        assert!((((*r.0.iter_deref()).0) == (5)));
        assert!(((unsafe { Moveable::get(&(*r.0.iter_deref()).1,) }) == (5)));
        let mut mv3: Moveable = Moveable::new_2({ -1_i32 }, { 3_f64 });
        r = unimplemented!();
        assert!(((m.len()) == (13_usize)));
        assert!((r.1 as bool));
        assert!((unsafe { Moveable::moved(&mv3,) }));
        assert!((((*r.0.iter_deref()).0) == (117)));
        assert!(((unsafe { Moveable::get(&(*r.0.iter_deref()).1,) }) == (-1_i32)));
    }
    {
        let mut m: UnsafeMap<Moveable, (Moveable, Moveable)> = UnsafeMap::new();
        let mut r: (UnsafeBidirectionalIterator<(Moveable, Moveable)>, bool) = unimplemented!();
        let mut i: i32 = 0;
        'loop_: while ((i) < (20)) {
            unimplemented!();
            i += 2;
        }
        assert!(((m.len()) == (10_usize)));
        let mut mvkey1: Moveable = Moveable::new_2({ 2 }, { 2_f64 });
        let mut mv1: Moveable = Moveable::new_2({ 4 }, { 4_f64 });
        r = unimplemented!();
        assert!(((m.len()) == (10_usize)));
        assert!(!(r.1));
        assert!(!(unsafe { Moveable::moved(&mv1,) }));
        assert!(!(unsafe { Moveable::moved(&mvkey1,) }));
        assert!((unsafe { Moveable::operator_eq(&(*r.0.iter_deref()).0, &mvkey1,) }));
        let mut mvkey2: Moveable = Moveable::new_2({ 3 }, { 3_f64 });
        r = unimplemented!();
        assert!(((m.len()) == (11_usize)));
        assert!((r.1 as bool));
        assert!((unsafe { Moveable::moved(&mv1,) }));
        assert!((unsafe { Moveable::moved(&mvkey2,) }));
        assert!(((unsafe { Moveable::get(&(*r.0.iter_deref()).0,) }) == (3)));
        assert!(((unsafe { Moveable::get(&(*r.0.iter_deref()).1,) }) == (4)));
    }
    {
        let mut m: UnsafeMap<i32, (i32, Moveable)> = UnsafeMap::new();
        let mut r: UnsafeBidirectionalIterator<(i32, Moveable)> =
            UnsafeBidirectionalIterator::default();
        let mut i: i32 = 0;
        'loop_: while ((i) < (20)) {
            unimplemented!();
            i += 2;
        }
        assert!(((m.len()) == (10_usize)));
        let mut it: UnsafeBidirectionalIterator<(i32, Moveable)> =
            <UnsafeBidirectionalIterator<(i32, Moveable)>>::from(m.find(&2));
        let mut mv1: Moveable = Moveable::new_2({ 3 }, { 3_f64 });
        let mut i: i32 = 0;
        'loop_: while ((i) < (20)) {
            r = unimplemented!();
            assert!(((m.len()) == (10_usize)));
            assert!(!(unsafe { Moveable::moved(&mv1,) }));
            assert!((((*r.iter_deref()).0) == (i)));
            assert!(((unsafe { Moveable::get(&(*r.iter_deref()).1,) }) == (i)));
            i += 2;
        }
        r = unimplemented!();
        assert!(((m.len()) == (11_usize)));
        assert!((unsafe { Moveable::moved(&mv1,) }));
        assert!((((*r.iter_deref()).0) == (3)));
        assert!(((unsafe { Moveable::get(&(*r.iter_deref()).1,) }) == (3)));
    }
    {
        let mut m: UnsafeMap<Moveable, (Moveable, Moveable)> = UnsafeMap::new();
        let mut r: UnsafeBidirectionalIterator<(Moveable, Moveable)> =
            UnsafeBidirectionalIterator::default();
        let mut i: i32 = 0;
        'loop_: while ((i) < (20)) {
            unimplemented!();
            i += 2;
        }
        assert!(((m.len()) == (10_usize)));
        let mut it: UnsafeBidirectionalIterator<(Moveable, Moveable)> =
            libcc2rs::next_unsafe(unimplemented!(), 1_isize);
        let mut mvkey1: Moveable = Moveable::new_2({ 2 }, { 2_f64 });
        let mut mv1: Moveable = Moveable::new_2({ 4 }, { 4_f64 });
        r = unimplemented!();
        assert!(((m.len()) == (10_usize)));
        assert!(!(unsafe { Moveable::moved(&mv1,) }));
        assert!(!(unsafe { Moveable::moved(&mvkey1,) }));
        assert!((unsafe { Moveable::operator_eq(&(*r.iter_deref()).0, &mvkey1,) }));
        let mut mvkey2: Moveable = Moveable::new_2({ 3 }, { 3_f64 });
        r = unimplemented!();
        assert!(((m.len()) == (11_usize)));
        assert!((unsafe { Moveable::moved(&mv1,) }));
        assert!((unsafe { Moveable::moved(&mvkey2,) }));
        assert!(((unsafe { Moveable::get(&(*r.iter_deref()).0,) }) == (3)));
        assert!(((unsafe { Moveable::get(&(*r.iter_deref()).1,) }) == (4)));
    }
    return true;
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
