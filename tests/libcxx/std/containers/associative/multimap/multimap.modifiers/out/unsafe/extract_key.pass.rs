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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_usize;
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_1: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_2: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_3: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (i32, i32),
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(*mut ::libc::c_void , new_1, value)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_2(mut p: *mut i32) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_1(mut _a0: *mut ::libc::c_void) -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.ptr_);
    }
    pub unsafe fn pointer_to(
        t: *mut i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_2({
            unimplemented!()
        });
    }
}
pub unsafe fn operator_eq_4(
    mut _a0: min_allocator_std_pair_const_int__int__,
    mut _a1: min_allocator_std_pair_const_int__int__,
) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__int__ {}
impl std::cmp::PartialEq for min_allocator_std_pair_const_int__int__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_4(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_std_pair_const_int__int__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Counter_base {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, DestructorUnsafe, Default)]
#[cpp_from(i32, new_1, ptr)]
pub struct Counter_int_ {
    data_: i32,
}
impl Counter_int_ {
    pub unsafe fn new_1(data: *const i32) -> Self {
        let mut this = Self { data_: (*data) };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_5)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(rhs: *const Counter_int_) -> Self {
        let mut this = Self {
            data_: (*rhs).data_,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_5)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, rhs: *const Counter_int_) -> *mut Counter_int_ {
        self.data_ = (*rhs).data_;
        return &mut (*(self as *mut Counter_int_));
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_5)).prefix_dec();
    }
    pub unsafe fn operator_eq(&self, x: *const Counter_int_) -> bool {
        return ((self.data_) == ((*x).data_));
    }
    pub unsafe fn operator_lt(&self, x: *const Counter_int_) -> bool {
        return ((self.data_) < ((*x).data_));
    }
}
impl std::cmp::Ord for Counter_int_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Counter_int_::operator_lt(self, other as *const Counter_int_) {
                std::cmp::Ordering::Less
            } else if Counter_int_::operator_lt(other, self as *const Counter_int_) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Counter_int_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Counter_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Counter_int_::operator_eq(self, other as *const Counter_int_) }
    }
}
impl std::cmp::Eq for Counter_int_ {}
impl Clone for Counter_int_ {
    fn clone(&self) -> Self {
        unsafe { Counter_int_::copy_from(self as *const Counter_int_) }
    }
}
pub static mut gConstructed_5: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn test_6(c: *mut (), mut first: *mut i32, mut last: *mut i32) {
    let mut sz: usize = (unimplemented!() as usize);
    assert!(((unimplemented!() as usize) == (sz)));
    let mut copy: *mut i32 = first;
    'loop_: while ((copy) != (last)) {
        let mut t: () = unimplemented!();
        assert!(!(unimplemented!()));
        sz.prefix_dec();
        {
            assert!(((*unimplemented!()) == (*copy)));
            (*unimplemented!()) = (*first);
            assert!(((*unimplemented!()) == (*first)));
        }
        assert!(unimplemented!());
        assert!(((sz) == (unimplemented!() as usize)));
        copy.prefix_inc();
    }
    assert!(((unimplemented!()) == (0_())));
    let mut copy: *mut i32 = first;
    'loop_: while ((copy) != (last)) {
        let mut t: () = unimplemented!();
        assert!(unimplemented!());
        copy.prefix_inc();
    }
}
pub unsafe fn test_7(c: *mut (), mut first: *mut Counter_int_, mut last: *mut Counter_int_) {
    let mut sz: usize = (unimplemented!() as usize);
    assert!(((unimplemented!() as usize) == (sz)));
    let mut copy: *mut Counter_int_ = first;
    'loop_: while ((copy) != (last)) {
        let mut t: () = unimplemented!();
        assert!(!(unimplemented!()));
        sz.prefix_dec();
        {
            assert!(
                (unsafe {
                    let _x: *const Counter_int_ = &(*copy);
                    Counter_int_::operator_eq(&(*unimplemented!()), _x)
                })
            );
            (unsafe {
                let _rhs: *const Counter_int_ = &(*first);
                Counter_int_::copy_assign(&mut (*unimplemented!()), _rhs)
            });
            assert!(
                (unsafe {
                    let _x: *const Counter_int_ = &(*first);
                    Counter_int_::operator_eq(&(*unimplemented!()), _x)
                })
            );
        }
        assert!(unimplemented!());
        assert!(((sz) == (unimplemented!() as usize)));
        copy.prefix_inc();
    }
    assert!(((unimplemented!()) == (0_())));
    let mut copy: *mut Counter_int_ = first;
    'loop_: while ((copy) != (last)) {
        let mut t: () = unimplemented!();
        assert!(unimplemented!());
        copy.prefix_inc();
    }
}
pub unsafe fn test_8(c: *mut (), mut first: *mut i32, mut last: *mut i32) {
    let mut sz: usize = (unimplemented!() as usize);
    assert!(((unimplemented!() as usize) == (sz)));
    let mut copy: *mut i32 = first;
    'loop_: while ((copy) != (last)) {
        let mut t: () = unimplemented!();
        assert!(!(unimplemented!()));
        sz.prefix_dec();
        {
            assert!(((*unimplemented!()) == (*copy)));
            (*unimplemented!()) = (*first);
            assert!(((*unimplemented!()) == (*first)));
        }
        assert!(
            (unsafe {
                let _arg0: min_allocator_std_pair_const_int__int__ = unimplemented!();
                operator_eq_4(_arg0, unimplemented!())
            })
        );
        assert!(((sz) == (unimplemented!() as usize)));
        copy.prefix_inc();
    }
    assert!(((unimplemented!()) == (0_())));
    let mut copy: *mut i32 = first;
    'loop_: while ((copy) != (last)) {
        let mut t: () = unimplemented!();
        assert!(unimplemented!());
        copy.prefix_inc();
    }
}
pub unsafe fn test_9() -> bool {
    {
        let mut m: () = unimplemented!();
        let mut keys: [i32; 6] = [1, 2, 3, 4, 5, 6];
        (unsafe {
            let _first: *mut i32 = unimplemented!();
            let _last: *mut i32 = keys.as_ptr().add(keys.len());
            test_6(&mut m, _first, _last)
        });
    }
    {
        let mut m: () = unimplemented!();
        let mut ptr: *const i32 = unimplemented!();
        let mut res: () = unimplemented!();
        {
            assert!((((unimplemented!()).cast_const()) == (ptr)));
        }
    }
    {
        let mut m: () = unimplemented!();
        let mut res: () = unimplemented!();
        assert!(!(unimplemented!()));
    }
    {
        let mut m: () = unimplemented!();
        {
            let mut keys: [Counter_int_; 6] = [
                {
                    let mut __tmp_0: i32 = 1;
                    Counter_int_::new_1({ &mut __tmp_0 })
                },
                {
                    let mut __tmp_1: i32 = 2;
                    Counter_int_::new_1({ &mut __tmp_1 })
                },
                {
                    let mut __tmp_2: i32 = 3;
                    Counter_int_::new_1({ &mut __tmp_2 })
                },
                {
                    let mut __tmp_3: i32 = 4;
                    Counter_int_::new_1({ &mut __tmp_3 })
                },
                {
                    let mut __tmp_4: i32 = 5;
                    Counter_int_::new_1({ &mut __tmp_4 })
                },
                {
                    let mut __tmp_5: i32 = 6;
                    Counter_int_::new_1({ &mut __tmp_5 })
                },
            ];
            assert!(
                ((*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_5)) == ((12) + (6)))
            );
            (unsafe {
                let _first: *mut Counter_int_ = unimplemented!();
                let _last: *mut Counter_int_ = keys.as_ptr().add(keys.len());
                test_7(&mut m, _first, _last)
            });
        }
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_5)) == (0)));
    }
    {
        let mut m: () = unimplemented!();
        let mut keys: [i32; 6] = [1, 2, 3, 4, 5, 6];
        (unsafe {
            let _first: *mut i32 = unimplemented!();
            let _last: *mut i32 = keys.as_ptr().add(keys.len());
            test_8(&mut m, _first, _last)
        });
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
    (unsafe { test_9() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const gConstructed_5);
}
