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
pub struct constexpr_char_traits_char_ {}
impl constexpr_char_traits_char_ {
    pub unsafe fn assign_1(c1: *mut (), c2: *const ()) {
        (*c1) = (*c2);
    }
    pub unsafe fn eq(mut c1: (), mut c2: ()) -> () {
        return ((c1) == (c2));
    }
    pub unsafe fn lt(mut c1: (), mut c2: ()) -> () {
        return ((c1) < (c2));
    }
    pub unsafe fn compare(mut s1: *const (), mut s2: *const (), mut n: ()) -> () {
        'loop_: while (n != 0) {
            if (unsafe {
                let _c1: () = (*s1);
                let _c2: () = (*s2);
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if (unsafe {
                let _c1: () = (*s2);
                let _c2: () = (*s1);
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return 1_();
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0_();
    }
    pub unsafe fn length(mut s: *const ()) -> () {
        let mut len: () = 0_();
        'loop_: while !(unsafe {
            let _c1: () = (*s);
            let _c2: () = 0_();
            constexpr_char_traits_char_::eq(_c1, _c2)
        }) {
            len.prefix_inc();
            s.prefix_inc();
        }
        return len;
    }
    pub unsafe fn copy(mut s1: *mut (), mut s2: *const (), mut n: ()) -> *mut () {
        {
            unimplemented!();
        }
        let mut r: *mut () = s1;
        'loop_: while (n != 0) {
            (unsafe {
                let _c1: *mut () = &mut (*s1);
                let _c2: *const () = &(*s2);
                constexpr_char_traits_char_::assign_1(_c1, _c2)
            });
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return r;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MultiStringType {
    pub w_: *const (),
    pub wn_: (),
    pub u8_: *const (),
    pub u8n_: (),
    pub u16_: *const (),
    pub u16n_: (),
    pub u32_: *const (),
    pub u32n_: (),
    pub s_: *const (),
    pub sn_: (),
}
impl MultiStringType {
    pub unsafe fn new(
        mut w: *const (),
        mut wn: (),
        mut u8: *const (),
        mut u8n: (),
        mut u16: *const (),
        mut u16n: (),
        mut u32: *const (),
        mut u32n: (),
        mut s: *const (),
        mut sn: (),
    ) -> Self {
        let mut this = Self {
            w_: w,
            wn_: wn,
            u8_: u8,
            u8n_: u8n,
            u16_: u16,
            u16n_: u16n,
            u32_: u32,
            u32n_: u32n,
            s_: s,
            sn_: sn,
        };
        this
    }
    pub unsafe fn as_ptr_2(&self, mut _a0: *const ()) -> *const () {
        return self.s_;
    }
    pub unsafe fn as_ptr_3(&self, mut _a0: *const ()) -> *const () {
        return self.w_;
    }
    pub unsafe fn as_ptr_4(&self, mut _a0: *const ()) -> *const () {
        return self.u8_;
    }
    pub unsafe fn as_ptr_5(&self, mut _a0: *const ()) -> *const () {
        return self.u16_;
    }
    pub unsafe fn as_ptr_6(&self, mut _a0: *const ()) -> *const () {
        return self.u32_;
    }
    pub unsafe fn length_7(&self, mut _a0: *const ()) -> () {
        return self.sn_;
    }
    pub unsafe fn length_8(&self, mut _a0: *const ()) -> () {
        return self.wn_;
    }
    pub unsafe fn length_9(&self, mut _a0: *const ()) -> () {
        return self.u8n_;
    }
    pub unsafe fn length_10(&self, mut _a0: *const ()) -> () {
        return self.u16n_;
    }
    pub unsafe fn length_11(&self, mut _a0: *const ()) -> () {
        return self.u32n_;
    }
    pub unsafe fn to_pconst_12(&self) -> *const () {
        return self.s_;
    }
    pub unsafe fn to_pconst_13(&self) -> *const () {
        return self.w_;
    }
    pub unsafe fn to_pconst_14(&self) -> *const () {
        return self.u8_;
    }
    pub unsafe fn to_pconst_15(&self) -> *const () {
        return self.u16_;
    }
    pub unsafe fn to_pconst_16(&self) -> *const () {
        return self.u32_;
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleTo_std_string_view_ {
    pub t_: (),
}
impl ConvertibleTo_std_string_view_ {
    pub unsafe fn new(mut t: ()) -> Self {
        let mut this = Self {
            t_: unimplemented!(),
        };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleTo_std_wstring_view_ {
    pub t_: (),
}
impl ConvertibleTo_std_wstring_view_ {
    pub unsafe fn new(mut t: ()) -> Self {
        let mut this = Self {
            t_: unimplemented!(),
        };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleTo_std_u16string_view_ {
    pub t_: (),
}
impl ConvertibleTo_std_u16string_view_ {
    pub unsafe fn new(mut t: ()) -> Self {
        let mut this = Self {
            t_: unimplemented!(),
        };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleTo_std_u32string_view_ {
    pub t_: (),
}
impl ConvertibleTo_std_u32string_view_ {
    pub unsafe fn new(mut t: ()) -> Self {
        let mut this = Self {
            t_: unimplemented!(),
        };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleTo_std_basic_string_view_char__constexpr_char_traits_char___ {
    pub t_: (),
}
impl ConvertibleTo_std_basic_string_view_char__constexpr_char_traits_char___ {
    pub unsafe fn new(mut t: ()) -> Self {
        let mut this = Self {
            t_: unimplemented!(),
        };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleTo_std_u8string_view_ {
    pub t_: (),
}
impl ConvertibleTo_std_u8string_view_ {
    pub unsafe fn new(mut t: ()) -> Self {
        let mut this = Self {
            t_: unimplemented!(),
        };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub unsafe fn test_0() -> () {
    let mut v: [(); 4] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut i: () = 0_();
    'loop_: while ((i) < (4_())) {
        let mut j: () = 0_();
        'loop_: while ((j) < (4_())) {
            let mut expected: () = ((i) < (j));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                unimplemented!();
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut abc: () = unimplemented!();
    let mut abc0def: () = unimplemented!();
    let mut abcdef: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub unsafe fn test_1() -> () {
    let mut v: [(); 4] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut i: () = 0_();
    'loop_: while ((i) < (4_())) {
        let mut j: () = 0_();
        'loop_: while ((j) < (4_())) {
            let mut expected: () = ((i) < (j));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                unimplemented!();
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut abc: () = unimplemented!();
    let mut abc0def: () = unimplemented!();
    let mut abcdef: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub unsafe fn test_2() -> () {
    let mut v: [(); 4] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut i: () = 0_();
    'loop_: while ((i) < (4_())) {
        let mut j: () = 0_();
        'loop_: while ((j) < (4_())) {
            let mut expected: () = ((i) < (j));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                unimplemented!();
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut abc: () = unimplemented!();
    let mut abc0def: () = unimplemented!();
    let mut abcdef: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub unsafe fn test_3() -> () {
    let mut v: [(); 4] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut i: () = 0_();
    'loop_: while ((i) < (4_())) {
        let mut j: () = 0_();
        'loop_: while ((j) < (4_())) {
            let mut expected: () = ((i) < (j));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                unimplemented!();
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut abc: () = unimplemented!();
    let mut abc0def: () = unimplemented!();
    let mut abcdef: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub unsafe fn test_4() -> () {
    let mut v: [(); 4] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut i: () = 0_();
    'loop_: while ((i) < (4_())) {
        let mut j: () = 0_();
        'loop_: while ((j) < (4_())) {
            let mut expected: () = ((i) < (j));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                unimplemented!();
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut abc: () = unimplemented!();
    let mut abc0def: () = unimplemented!();
    let mut abcdef: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub unsafe fn test_5() -> () {
    let mut v: [(); 4] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut i: () = 0_();
    'loop_: while ((i) < (4_())) {
        let mut j: () = 0_();
        'loop_: while ((j) < (4_())) {
            let mut expected: () = ((i) < (j));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                unimplemented!();
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut abc: () = unimplemented!();
    let mut abc0def: () = unimplemented!();
    let mut abcdef: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
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
    (unsafe { test_0() });
    (unsafe { test_1() });
    (unsafe { test_2() });
    (unsafe { test_3() });
    const _: () = assert!(true, "test<std::string_view>()");;
    const _: () = assert!(true, "test<std::wstring_view>()");;
    const _: () = assert!(true, "test<std::u16string_view>()");;
    const _: () = assert!(true, "test<std::u32string_view>()");;
    (unsafe { test_4() });
    const _: () = assert!(
        true,
        "test<std::basic_string_view<char, constexpr_char_traits<char>>>()"
    );;
    (unsafe { test_5() });
    const _: () = assert!(true, "test<std::u8string_view>()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
