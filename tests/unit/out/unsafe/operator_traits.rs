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
pub struct Lt {
    pub v: (),
}
impl Lt {
    pub unsafe fn operator_lt(&self, o: *const Lt) -> () {
        return ((self.v) < ((*o).v));
    }
}
impl std::cmp::Ord for Lt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Lt::operator_lt(self, other as *const Lt) {
                std::cmp::Ordering::Less
            } else if Lt::operator_lt(other, self as *const Lt) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Lt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Lt {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(Lt::operator_lt(self, other as *const Lt))
                && !(Lt::operator_lt(other, self as *const Lt))
        }
    }
}
impl std::cmp::Eq for Lt {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Eq {
    pub v: (),
}
impl Eq {
    pub unsafe fn operator_eq(&self, o: *const Eq) -> () {
        return ((self.v) == ((*o).v));
    }
}
impl std::cmp::PartialEq for Eq {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Eq::operator_eq(self, other as *const Eq) }
    }
}
impl std::cmp::Eq for Eq {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Cmp {
    pub v: (),
}
impl Cmp {
    pub unsafe fn operator_cmp(&self, o: *const Cmp) -> std_strong_ordering {
        return std::cmp::Ord::cmp(&(self.v), &((*o).v));
    }
    pub unsafe fn operator_eq(&self, o: *const Cmp) -> () {
        return ((self.v) == ((*o).v));
    }
}
impl std::cmp::Ord for Cmp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { Cmp::operator_cmp(self, other as *const Cmp) }
    }
}
impl std::cmp::PartialOrd for Cmp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Cmp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Cmp::operator_eq(self, other as *const Cmp) }
    }
}
impl std::cmp::Eq for Cmp {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Free {
    pub v: (),
}
impl std::cmp::Ord for Free {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_0(self as *const Free, other as *const Free) {
                std::cmp::Ordering::Less
            } else if operator_lt_0(other as *const Free, self as *const Free) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Free {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Free {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const Free, other as *const Free) }
    }
}
impl std::cmp::Eq for Free {}
pub unsafe fn operator_lt_0(a: *const Free, b: *const Free) -> () {
    return (((*a).v) < ((*b).v));
}
pub unsafe fn operator_eq_1(a: *const Free, b: *const Free) -> () {
    return (((*a).v) == ((*b).v));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Wrapped_int_ {
    pub v: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut w: Wrapped_int_ = Wrapped_int_ { v: 2_() };
    unimplemented!();
    let mut lts: std_vector_Lt__std_allocator_Lt__ =
        std_vector_Lt__std_allocator_Lt__::new_1({ unimplemented!() }, None);
    (unsafe {
        let ___first: __gnu_cxx___normal_iterator_Lt_ptr__std_vector_Lt__std_allocator_Lt___ =
            (unsafe { lts.begin_26() });
        let ___last: __gnu_cxx___normal_iterator_Lt_ptr__std_vector_Lt__std_allocator_Lt___ =
            (unsafe { lts.end_28() });
        sort_2(___first, ___last)
    });
    unimplemented!();
    let mut eqs: std_vector_Eq__std_allocator_Eq__ =
        std_vector_Eq__std_allocator_Eq__::new_1({ unimplemented!() }, None);
    let mut two: Eq = Eq { v: 2_() };
    let mut nine: Eq = Eq { v: 9_() };
    unimplemented!();
    unimplemented!();
    let mut cmps: std_vector_Cmp__std_allocator_Cmp__ =
        std_vector_Cmp__std_allocator_Cmp__::new_1({ unimplemented!() }, None);
    (unsafe {
        let ___first: __gnu_cxx___normal_iterator_Cmp_ptr__std_vector_Cmp__std_allocator_Cmp___ =
            (unsafe { cmps.begin_26() });
        let ___last: __gnu_cxx___normal_iterator_Cmp_ptr__std_vector_Cmp__std_allocator_Cmp___ =
            (unsafe { cmps.end_28() });
        sort_3(___first, ___last)
    });
    unimplemented!();
    let mut three: Cmp = Cmp { v: 3_() };
    unimplemented!();
    let mut frees: std_vector_Free__std_allocator_Free__ =
        std_vector_Free__std_allocator_Free__::new_1({ unimplemented!() }, None);
    (unsafe {
        let ___first: __gnu_cxx___normal_iterator_Free_ptr__std_vector_Free__std_allocator_Free___ =
            (unsafe { frees.begin_26() });
        let ___last: __gnu_cxx___normal_iterator_Free_ptr__std_vector_Free__std_allocator_Free___ =
            (unsafe { frees.end_28() });
        sort_4(___first, ___last)
    });
    unimplemented!();
    let mut ftwo: Free = Free { v: 2_() };
    unimplemented!();
    let mut m: std_map_Lt__int__std_less_Lt___std_allocator_std_pair_const_Lt__int___ =
        <std_map_Lt__int__std_less_Lt___std_allocator_std_pair_const_Lt__int___>::default();
    m[(Lt { v: 2_() }) as usize] = 20_();
    m[(Lt { v: 1_() }) as usize] = 10_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
