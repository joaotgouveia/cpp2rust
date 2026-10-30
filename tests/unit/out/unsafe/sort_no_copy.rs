extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
pub struct Tracked {
    pub v: (),
}
impl Tracked {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { v: 0_() };
        this
    }
    pub unsafe fn new_2(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const Tracked) -> Self {
        let mut this = Self { v: (*o).v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn move_from(o: *mut Tracked) -> Self {
        let mut this = Self { v: (*o).v };
        this
    }
    pub unsafe fn copy_assign(&mut self, o: *const Tracked) -> *mut Tracked {
        self.v = (*o).v;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        return &mut (*(self as *mut Tracked));
    }
    pub unsafe fn move_assign(&mut self, o: *mut Tracked) -> *mut Tracked {
        self.v = (*o).v;
        return &mut (*(self as *mut Tracked));
    }
}
impl std::cmp::Ord for Tracked {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_1(self as *const Tracked, other as *const Tracked) {
                std::cmp::Ordering::Less
            } else if operator_lt_1(other as *const Tracked, self as *const Tracked) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Tracked {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Tracked {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_1(self as *const Tracked, other as *const Tracked))
                && !(operator_lt_1(other as *const Tracked, self as *const Tracked))
        }
    }
}
impl std::cmp::Eq for Tracked {}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        unsafe { Tracked::copy_from(self as *const Tracked) }
    }
}
impl Default for Tracked {
    fn default() -> Self {
        unsafe { Tracked::new_1() }
    }
}
pub unsafe fn operator_lt_1(a: *const Tracked, b: *const Tracked) -> () {
    return (((*a).v) < ((*b).v));
}
pub unsafe fn greater_2(a: *const Tracked, b: *const Tracked) -> () {
    return (((*a).v) > ((*b).v));
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Bag {
    pub items: (),
}
impl Bag {
    pub unsafe fn move_from(_a0: *mut Bag) -> Self {
        let mut this = Self {
            items: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Bag) -> *mut Bag {
        unimplemented!();
        return &mut (*(self as *mut Bag));
    }
}
impl std::cmp::Ord for Bag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_3(self as *const Bag, other as *const Bag) {
                std::cmp::Ordering::Less
            } else if operator_lt_3(other as *const Bag, self as *const Bag) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Bag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_4(self as *const Bag, other as *const Bag) }
    }
}
impl std::cmp::Eq for Bag {}
pub unsafe fn operator_lt_3(a: *const Bag, b: *const Bag) -> () {
    return ((unimplemented!()) < (unimplemented!()));
}
pub unsafe fn operator_eq_4(a: *const Bag, b: *const Bag) -> () {
    return ((unimplemented!()) == (unimplemented!()));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut arr: [Tracked; 5] = [
        Tracked::new_2({ 3_() }),
        Tracked::new_2({ 1_() }),
        Tracked::new_2({ 4_() }),
        Tracked::new_2({ 1_() }),
        Tracked::new_2({ 5_() }),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut vec_: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut bags: [Bag; 3] = std::array::from_fn::<_, 3, _>(|_| <Bag>::default());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut bag_vec: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut bag_map: () = unimplemented!();
    (*unimplemented!()) = 2_();
    (*unimplemented!()) = 0_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0));
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
}
