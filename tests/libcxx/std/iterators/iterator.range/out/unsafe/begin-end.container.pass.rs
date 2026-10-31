extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_reverse_iterator_int_ptr___std_reverse_iterator_int_ptr__
{}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_reverse_iterator_const_int_ptr___std_reverse_iterator_const_int_ptr__
{}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std__List_iterator_int___std__List_iterator_int__ {}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std__List_const_iterator_int___std__List_const_iterator_int__
{}
pub type anon_6 = u32;
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_reverse_iterator_std__List_iterator_int____std_reverse_iterator_std__List_iterator_int___
{}
pub type anon_7 = u32;
pub const anon_7_value: anon_7 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_reverse_iterator_std__List_const_iterator_int____std_reverse_iterator_std__List_const_iterator_int___
{}
pub type anon_8 = u32;
pub const anon_8_value: anon_8 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same___gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int_______gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int____
{}
pub type anon_9 = u32;
pub const anon_9_value: anon_9 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int_______gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int____
{}
pub type anon_10 = u32;
pub const anon_10_value: anon_10 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_reverse_iterator___gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int______std_reverse_iterator___gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int_____
{}
pub type anon_11 = u32;
pub const anon_11_value: anon_11 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_reverse_iterator___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int______std_reverse_iterator___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int_____
{}
pub unsafe fn test_12() -> bool {
    let mut c: Vec<i32> = Vec::from([1, 2, 3]);
    let cc: *const Vec<i32> = &c;
    {
        const _: () = assert!(true, "");;
        assert!(((unimplemented!()) == (unimplemented!())));
        const _: () = assert!(true, "");;
        assert!(((unimplemented!()) == (unimplemented!())));
    }
    {
        const _: () = assert!(true, "");;
        assert!(((unimplemented!()) == (unimplemented!())));
        const _: () = assert!(true, "");;
        assert!(((unimplemented!()) == (unimplemented!())));
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(
            true,
            "noexcept(std::cbegin(cc)) == noexcept(std::begin(cc))"
        );;
        assert!(((unimplemented!()) == (unimplemented!())));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cend(cc)) == noexcept(std::end(cc))");;
        assert!(((unimplemented!()) == (unimplemented!())));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cbegin(c)) == noexcept(std::begin(cc))");;
        assert!(((unimplemented!()) == (unimplemented!())));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cend(c)) == noexcept(std::end(cc))");;
        assert!(((unimplemented!()) == (unimplemented!())));
    }
    {
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
    }
    {
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
    }
    {
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
    }
    return true;
}
pub unsafe fn test_13() -> bool {
    let mut c: () = unimplemented!();
    let cc: *const () = &c;
    {
        const _: () = assert!(true, "");;
        assert!(unimplemented!() == unimplemented!());
        const _: () = assert!(true, "");;
        assert!(unimplemented!() == unimplemented!());
    }
    {
        const _: () = assert!(true, "");;
        assert!(unimplemented!() == unimplemented!());
        const _: () = assert!(true, "");;
        assert!(unimplemented!() == unimplemented!());
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(
            true,
            "noexcept(std::cbegin(cc)) == noexcept(std::begin(cc))"
        );;
        assert!(unimplemented!() == unimplemented!());
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cend(cc)) == noexcept(std::end(cc))");;
        assert!(unimplemented!() == unimplemented!());
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cbegin(c)) == noexcept(std::begin(cc))");;
        assert!(unimplemented!() == unimplemented!());
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cend(c)) == noexcept(std::end(cc))");;
        assert!(unimplemented!() == unimplemented!());
    }
    {
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
    }
    {
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
    }
    {
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
    }
    return true;
}
pub unsafe fn test_14() -> bool {
    let mut c: Vec<i32> = Vec::from([1, 2, 3]);
    let cc: *const Vec<i32> = &c;
    {
        const _: () = assert!(true, "");;
        assert!(unimplemented!() == c.begin());
        const _: () = assert!(true, "");;
        assert!(unimplemented!() == c.end());
    }
    {
        const _: () = assert!(true, "");;
        assert!(unimplemented!() == (*cc).begin());
        const _: () = assert!(true, "");;
        assert!(unimplemented!() == (*cc).end());
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(
            true,
            "noexcept(std::cbegin(cc)) == noexcept(std::begin(cc))"
        );;
        assert!(unimplemented!() == unimplemented!());
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cend(cc)) == noexcept(std::end(cc))");;
        assert!(unimplemented!() == unimplemented!());
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cbegin(c)) == noexcept(std::begin(cc))");;
        assert!(unimplemented!() == unimplemented!());
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cend(c)) == noexcept(std::end(cc))");;
        assert!(unimplemented!() == unimplemented!());
    }
    {
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
    }
    {
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
    }
    {
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
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
    (unsafe { test_12() });
    (unsafe { test_13() });
    (unsafe { test_14() });
    const _: () = assert!(true, "test<std::array<int, 3>>()");;
    {
        let mut a: [i32; 3] = [1, 2, 3];
        let ca: *const [i32; 3] = &a;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let b: *const i32 = unimplemented!();
        assert!(((b) == ((*std::cell::LazyCell::force_mut(&mut *&raw mut a_15)).as_ptr())));
        let e: *const i32 = unimplemented!();
        assert!(
            ((e) == ((*std::cell::LazyCell::force_mut(&mut *&raw mut a_15))
                .as_ptr()
                .offset((3) as isize)))
        );
    }
    return 0;
}
static mut a_15: std::cell::LazyCell<[i32; 3]> = std::cell::LazyCell::new(|| unsafe { [1, 2, 3] });
pub unsafe fn __cpp2rust_init_globals() {}
