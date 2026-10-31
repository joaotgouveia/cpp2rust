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
pub struct test_macros_detail_is_same_std_less_int___std_less_int__ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std__Flat_map_impl_int__char__std_less_int___std_vector_int__std_allocator_int____std_vector_char__std_allocator_char____true__value_compare__std__Flat_map_impl_int__char__std_less_int___std_vector_int__std_allocator_int____std_vector_char__std_allocator_char____true__value_compare_
{}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_bool__bool_ {}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_function_bool__int__int____std_function_bool__int__int___
{}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std__Flat_map_impl_int__int__std_function_bool__int__int____std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____true__value_compare__std__Flat_map_impl_int__int__std_function_bool__int__int____std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____true__value_compare_
{}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_less_void___std_less_void__ {}
pub type anon_6 = u32;
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std__Flat_map_impl_int__int__std_less_void___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____true__value_compare__std__Flat_map_impl_int__int__std_less_void___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____true__value_compare_
{}
pub unsafe fn test_7() -> bool {
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "!std::is_same_v<M::value_compare, Comp>");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut kc: () = unimplemented!();
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        let mut vc: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!((unsafe { vc((1, ('2' as libc::c_char)), (2, ('1' as libc::c_char)),) }));
        assert!(!(unsafe { vc((2, ('1' as libc::c_char)), (1, ('2' as libc::c_char)),) }));
    }
    {
        let mut comp: () = unimplemented!();
        let mut m: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut kc: () = unimplemented!();
        assert!(!(unimplemented!()));
        assert!((unimplemented!() as bool));
        let mut vc: () = unimplemented!();
        let mut a: (i32, i32) = (1, 2);
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "!noexcept(vc(a, a))");;
        assert!(!(unsafe { vc((1, 2), (2, 1),) }));
        assert!((unsafe { vc((2, 1), (1, 2),) }));
    }
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut kc: () = unimplemented!();
        assert!((unimplemented!() as bool));
        assert!(!(unimplemented!()));
        let mut vc: () = unimplemented!();
        let mut a: (i32, i32) = (1, 2);
        const _: () = assert!(true, "");;
        assert!((unsafe { vc((1, 2), (2, 1),) }));
        assert!(!(unsafe { vc((2, 1), (1, 2),) }));
    }
    {
        let mut comp: () = unimplemented!();
        let mut m: () = unimplemented!();
        let mut vc: () = unimplemented!();
        const _: () = assert!(
            ((::std::mem::size_of::<()>()) >= (::std::mem::size_of::<()>())),
            "sizeof(vc) >= sizeof(Comp)"
        );;
        unimplemented!();
        m = unimplemented!();
        assert!(unimplemented!());
        let mut a: (Vec<i32>, i32) = (Vec::from([2, 1, 4]), 42);
        let mut b: (Vec<i32>, i32) = (Vec::from([1, 2, 3]), 42);
        let mut c: (Vec<i32>, i32) = (Vec::from([0, 3, 2]), 42);
        assert!(
            (unsafe {
                vc(
                    {
                        let __a0 = (a).clone();
                        (
                            <*const Vec<i32> as CppFromUnsafe<Vec<i32>>>::cpp_from(&__a0.0),
                            <*const i32 as CppFromUnsafe<i32>>::cpp_from(&__a0.1),
                        )
                    },
                    {
                        let __a0 = (b).clone();
                        (
                            <*const Vec<i32> as CppFromUnsafe<Vec<i32>>>::cpp_from(&__a0.0),
                            <*const i32 as CppFromUnsafe<i32>>::cpp_from(&__a0.1),
                        )
                    },
                )
            })
        );
        assert!(
            (unsafe {
                vc(
                    {
                        let __a0 = (b).clone();
                        (
                            <*const Vec<i32> as CppFromUnsafe<Vec<i32>>>::cpp_from(&__a0.0),
                            <*const i32 as CppFromUnsafe<i32>>::cpp_from(&__a0.1),
                        )
                    },
                    {
                        let __a0 = (c).clone();
                        (
                            <*const Vec<i32> as CppFromUnsafe<Vec<i32>>>::cpp_from(&__a0.0),
                            <*const i32 as CppFromUnsafe<i32>>::cpp_from(&__a0.1),
                        )
                    },
                )
            })
        );
        assert!(
            !(unsafe {
                vc(
                    {
                        let __a0 = (b).clone();
                        (
                            <*const Vec<i32> as CppFromUnsafe<Vec<i32>>>::cpp_from(&__a0.0),
                            <*const i32 as CppFromUnsafe<i32>>::cpp_from(&__a0.1),
                        )
                    },
                    {
                        let __a0 = (a).clone();
                        (
                            <*const Vec<i32> as CppFromUnsafe<Vec<i32>>>::cpp_from(&__a0.0),
                            <*const i32 as CppFromUnsafe<i32>>::cpp_from(&__a0.1),
                        )
                    },
                )
            })
        );
        assert!(
            !(unsafe {
                vc(
                    {
                        let __a0 = (c).clone();
                        (
                            <*const Vec<i32> as CppFromUnsafe<Vec<i32>>>::cpp_from(&__a0.0),
                            <*const i32 as CppFromUnsafe<i32>>::cpp_from(&__a0.1),
                        )
                    },
                    {
                        let __a0 = (b).clone();
                        (
                            <*const Vec<i32> as CppFromUnsafe<Vec<i32>>>::cpp_from(&__a0.0),
                            <*const i32 as CppFromUnsafe<i32>>::cpp_from(&__a0.1),
                        )
                    },
                )
            })
        );
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
    (unsafe { test_7() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
