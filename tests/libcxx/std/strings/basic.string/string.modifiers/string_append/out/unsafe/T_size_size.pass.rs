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
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_();
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn pointer_to(
        t: *mut (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(mut p: min_pointer_char__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_char_ {}
impl min_allocator_char_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_char__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub unsafe fn test_4(mut s: (), mut sv: (), mut pos: (), mut n: (), mut expected: ()) {
    if ((pos) <= (unimplemented!())) {
        (unsafe { s.append_69_std_string_view_0(&sv, pos, Some(n)) });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_5(mut s: (), mut sv: (), mut pos: (), mut n: (), mut expected: ()) {
    if ((pos) <= (unimplemented!())) {
        (unsafe { s.append_69_std_string_view_0(&sv, pos, n) });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_npos_6(mut s: (), mut sv: (), mut pos: (), mut expected: ()) {
    if ((pos) <= (unimplemented!())) {
        (unsafe { s.append_69_std_string_view_0(&sv, pos, None) });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_string_7() {
    (unsafe {
        test_4(
            unimplemented!(),
            unimplemented!(),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_4(
            unimplemented!(),
            unimplemented!(),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_4(
            unimplemented!(),
            unimplemented!(),
            0_(),
            3_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"123".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_4(
            unimplemented!(),
            unimplemented!(),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"2345".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_4(
            unimplemented!(),
            unimplemented!(),
            3_(),
            15_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"45".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_4(
            unimplemented!(),
            unimplemented!(),
            5_(),
            15_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_4(
            unimplemented!(),
            unimplemented!(),
            6_(),
            15_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"not happening".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_4(
            unimplemented!(),
            unimplemented!(),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_4(
            unimplemented!(),
            unimplemented!(),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"2".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_4(
            unimplemented!(),
            unimplemented!(),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"345".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_4(
            unimplemented!(),
            unimplemented!(),
            12_(),
            13_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"34567890".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_4(
            unimplemented!(),
            unimplemented!(),
            21_(),
            13_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"not happening".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_4(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            unimplemented!(),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_4(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            unimplemented!(),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"1234534".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_4(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            unimplemented!(),
            0_(),
            100_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"123451234567890".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_4(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345678901234567890".as_ptr() },
                None,
            ),
            unimplemented!(),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345678901234567890".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_4(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345678901234567890".as_ptr() },
                None,
            ),
            unimplemented!(),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345678901234567890234".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_4(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345678901234567890".as_ptr() },
                None,
            ),
            unimplemented!(),
            5_(),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"123456789012345678906789012345".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_4(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"1234512345678901234567890".as_ptr() },
                None,
            ),
            unimplemented!(),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"1234512345678901234567890".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_4(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"1234512345678901234567890".as_ptr() },
                None,
            ),
            unimplemented!(),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"1234512345678901234567890234".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_4(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"1234512345678901234567890".as_ptr() },
                None,
            ),
            unimplemented!(),
            5_(),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345123456789012345678906789012345".as_ptr() },
                None,
            ),
        )
    });
}
pub unsafe fn test_string_8() {
    (unsafe {
        test_5(
            unimplemented!(),
            unimplemented!(),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            unimplemented!(),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            unimplemented!(),
            0_(),
            3_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"123".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            unimplemented!(),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"2345".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            unimplemented!(),
            3_(),
            15_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"45".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            unimplemented!(),
            5_(),
            15_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            unimplemented!(),
            6_(),
            15_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"not happening".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            unimplemented!(),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            unimplemented!(),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"2".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            unimplemented!(),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"345".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            unimplemented!(),
            12_(),
            13_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"34567890".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            unimplemented!(),
            21_(),
            13_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"not happening".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_5(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            unimplemented!(),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_5(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            unimplemented!(),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"1234534".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_5(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            unimplemented!(),
            0_(),
            100_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"123451234567890".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_5(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345678901234567890".as_ptr() },
                None,
            ),
            unimplemented!(),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345678901234567890".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_5(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345678901234567890".as_ptr() },
                None,
            ),
            unimplemented!(),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345678901234567890234".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_5(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345678901234567890".as_ptr() },
                None,
            ),
            unimplemented!(),
            5_(),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"123456789012345678906789012345".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_5(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"1234512345678901234567890".as_ptr() },
                None,
            ),
            unimplemented!(),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"1234512345678901234567890".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_5(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"1234512345678901234567890".as_ptr() },
                None,
            ),
            unimplemented!(),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"1234512345678901234567890234".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_5(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"1234512345678901234567890".as_ptr() },
                None,
            ),
            unimplemented!(),
            5_(),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345123456789012345678906789012345".as_ptr() },
                None,
            ),
        )
    });
}
pub unsafe fn test_9() -> () {
    (unsafe { test_string_7() });
    (unsafe { test_string_8() });
    {
        (unsafe { test_npos_6(unimplemented!(), unimplemented!(), 0_(), unimplemented!()) });
        (unsafe { test_npos_6(unimplemented!(), unimplemented!(), 1_(), unimplemented!()) });
        (unsafe {
            test_npos_6(
                unimplemented!(),
                unimplemented!(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_npos_6(
                unimplemented!(),
                unimplemented!(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"2345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_npos_6(
                unimplemented!(),
                unimplemented!(),
                3_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"45".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_npos_6(
                unimplemented!(),
                unimplemented!(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_npos_6(
                unimplemented!(),
                unimplemented!(),
                6_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"not happening".as_ptr() },
                    None,
                ),
            )
        });
    }
    {
        let mut s: () = unimplemented!();
        let mut sv: () = unimplemented!();
        let mut arr: [(); 5] = std::mem::transmute(*b"IJKL\0");
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            s.append_72(
                c"QRST".as_ptr(),
                0_(),
                (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_10)),
            )
        });
        unimplemented!();
        unimplemented!();
        (unsafe { s.append_69_std_string_view_0(&sv, 0_(), None) });
        unimplemented!();
        unimplemented!();
        (unsafe {
            s.append_69_std_string_view_0(
                &sv,
                0_(),
                Some((*std::cell::LazyCell::force_mut(&mut *&raw mut npos_10))),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            s.append_72(
                (arr.as_mut_ptr()).cast_const(),
                0_(),
                (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_10)),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ABCD".as_ptr() },
            None,
        );
        let mut sv: () = (unsafe { s() });
        (unsafe { s.append_67_std_string_view_0(&sv) });
        unimplemented!();
        unimplemented!();
        (unsafe {
            s.append_69_std_string_view_0(
                &sv,
                0_(),
                Some((*std::cell::LazyCell::force_mut(&mut *&raw mut npos_10))),
            )
        });
        unimplemented!();
        unimplemented!();
        (unsafe {
            let ___t: *const () = &sv;
            let ___pos: () = unimplemented!();
            s.append_69_std_string_view_0(___t, ___pos, None)
        });
        unimplemented!();
    }
    {
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_ptr() },
            None,
        );
        let mut sv: () = (unsafe { s() });
        (unsafe { s.append_67_std_string_view_0(&sv) });
        unimplemented!();
        unimplemented!();
        (unsafe {
            s.append_69_std_string_view_0(
                &sv,
                0_(),
                Some((*std::cell::LazyCell::force_mut(&mut *&raw mut npos_10))),
            )
        });
        unimplemented!();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_9() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
