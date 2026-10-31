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
pub struct soccc_allocator_char_ {
    soccc_count: *mut i32,
    self_soccc_count: i32,
}
impl soccc_allocator_char_ {
    pub unsafe fn new(mut soccc_count_: *mut i32, mut self_coccc_count_: Option<i32>) -> Self {
        let mut self_coccc_count_: i32 = self_coccc_count_.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self {
            soccc_count: soccc_count_,
            self_soccc_count: self_coccc_count_,
        };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut libc::c_char {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut libc::c_char, mut s: usize) {
        unimplemented!();
    }
    pub unsafe fn select_on_container_copy_construction(&self) -> soccc_allocator_char_ {
        (*self.soccc_count) += 1;
        return soccc_allocator_char_::new({ self.soccc_count }, {
            Some(((self.self_soccc_count) + (1)))
        });
    }
    pub unsafe fn get_soccc(&self) -> *mut i32 {
        return self.soccc_count;
    }
    pub unsafe fn get_self_soccc(&self) -> i32 {
        return self.self_soccc_count;
    }
    pub unsafe fn operator_eq_8_char(&self, that: *const soccc_allocator_char_) -> bool {
        return ((self.soccc_count) == (unsafe { soccc_allocator_char_::get_soccc(&(*that)) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct soccc_allocator_wchar_t_ {
    soccc_count: *mut i32,
    self_soccc_count: i32,
}
impl soccc_allocator_wchar_t_ {
    pub unsafe fn new(mut soccc_count_: *mut i32, mut self_coccc_count_: Option<i32>) -> Self {
        let mut self_coccc_count_: i32 = self_coccc_count_.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self {
            soccc_count: soccc_count_,
            self_soccc_count: self_coccc_count_,
        };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut i32 {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut s: usize) {
        unimplemented!();
    }
    pub unsafe fn select_on_container_copy_construction(&self) -> soccc_allocator_wchar_t_ {
        (*self.soccc_count) += 1;
        return soccc_allocator_wchar_t_::new({ self.soccc_count }, {
            Some(((self.self_soccc_count) + (1)))
        });
    }
    pub unsafe fn get_soccc(&self) -> *mut i32 {
        return self.soccc_count;
    }
    pub unsafe fn get_self_soccc(&self) -> i32 {
        return self.self_soccc_count;
    }
    pub unsafe fn operator_eq_8_wchar_t(&self, that: *const soccc_allocator_wchar_t_) -> bool {
        return ((self.soccc_count) == (unsafe { soccc_allocator_wchar_t_::get_soccc(&(*that)) }));
    }
}
pub unsafe fn test_0() -> bool {
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<libc::c_char> = unimplemented!();
        let mut rhs: Vec<libc::c_char> = unimplemented!();
        let mut r: Vec<libc::c_char> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_char_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_lhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_char_::get_self_soccc(&unimplemented!(),) }) == (1)));
        assert!(((soccc_lhs) == (1)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<libc::c_char> = unimplemented!();
        let mut rhs: Vec<libc::c_char> = unimplemented!();
        let mut r: Vec<libc::c_char> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_char_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_rhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_char_::get_self_soccc(&unimplemented!(),) }) == (0)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<libc::c_char> = unimplemented!();
        let mut rhs: Vec<libc::c_char> = unimplemented!();
        let mut r: Vec<libc::c_char> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_char_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_lhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_char_::get_self_soccc(&unimplemented!(),) }) == (0)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<libc::c_char> = unimplemented!();
        let mut rhs: Vec<libc::c_char> = unimplemented!();
        let mut r: Vec<libc::c_char> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_char_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_lhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_char_::get_self_soccc(&unimplemented!(),) }) == (0)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<libc::c_char> = unimplemented!();
        let mut rhs: Vec<libc::c_char> = unimplemented!();
        let mut r: Vec<libc::c_char> = {
            let mut __tmp2 = lhs.clone();
            __tmp2.pop();
            __tmp2.extend_from_slice(::std::slice::from_raw_parts(
                (rhs.as_mut_ptr()).cast_const(),
                libcc2rs::c_str_len((rhs.as_mut_ptr()).cast_const()),
            ));
            __tmp2.push((0 as libc::c_char));
            __tmp2
        };
        assert!(
            ((unsafe { soccc_allocator_char_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_lhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_char_::get_self_soccc(&unimplemented!(),) }) == (1)));
        assert!(((soccc_lhs) == (1)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<libc::c_char> = unimplemented!();
        let mut rhs: Vec<libc::c_char> = unimplemented!();
        let mut r: Vec<libc::c_char> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_char_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_lhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_char_::get_self_soccc(&unimplemented!(),) }) == (1)));
        assert!(((soccc_lhs) == (1)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<libc::c_char> = unimplemented!();
        let mut rhs: Vec<libc::c_char> = unimplemented!();
        let mut r: Vec<libc::c_char> = {
            let mut __tmp2 = lhs.clone();
            __tmp2.pop();
            __tmp2.extend_from_slice(::std::slice::from_raw_parts(
                (rhs.as_mut_ptr()).cast_const(),
                libcc2rs::c_str_len((rhs.as_mut_ptr()).cast_const()),
            ));
            __tmp2.push((0 as libc::c_char));
            __tmp2
        };
        assert!(
            ((unsafe { soccc_allocator_char_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_lhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_char_::get_self_soccc(&unimplemented!(),) }) == (0)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<libc::c_char> = unimplemented!();
        let mut rhs: Vec<libc::c_char> = unimplemented!();
        let mut r: Vec<libc::c_char> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_char_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_lhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_char_::get_self_soccc(&unimplemented!(),) }) == (0)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<libc::c_char> = unimplemented!();
        let mut rhs: Vec<libc::c_char> = unimplemented!();
        let mut r: Vec<libc::c_char> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_char_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_rhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_char_::get_self_soccc(&unimplemented!(),) }) == (1)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (1)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<libc::c_char> = unimplemented!();
        let mut rhs: Vec<libc::c_char> = unimplemented!();
        let mut r: Vec<libc::c_char> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_char_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_rhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_char_::get_self_soccc(&unimplemented!(),) }) == (1)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (1)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<libc::c_char> = unimplemented!();
        let mut rhs: Vec<libc::c_char> = unimplemented!();
        let mut r: Vec<libc::c_char> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_char_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_rhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_char_::get_self_soccc(&unimplemented!(),) }) == (0)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<libc::c_char> = unimplemented!();
        let mut rhs: Vec<libc::c_char> = unimplemented!();
        let mut r: Vec<libc::c_char> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_char_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_rhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_char_::get_self_soccc(&unimplemented!(),) }) == (0)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (0)));
    }
    return true;
}
pub unsafe fn test_1() -> bool {
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_wchar_t_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_lhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_wchar_t_::get_self_soccc(&unimplemented!(),) }) == (1)));
        assert!(((soccc_lhs) == (1)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_wchar_t_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_rhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_wchar_t_::get_self_soccc(&unimplemented!(),) }) == (0)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_wchar_t_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_lhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_wchar_t_::get_self_soccc(&unimplemented!(),) }) == (0)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_wchar_t_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_lhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_wchar_t_::get_self_soccc(&unimplemented!(),) }) == (0)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = {
            let mut __tmp2 = lhs.clone();
            __tmp2.pop();
            __tmp2.extend_from_slice(::std::slice::from_raw_parts(
                (rhs.as_mut_ptr()).cast_const(),
                libcc2rs::c_str_len((rhs.as_mut_ptr()).cast_const()),
            ));
            __tmp2.push(0_i32);
            __tmp2
        };
        assert!(
            ((unsafe { soccc_allocator_wchar_t_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_lhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_wchar_t_::get_self_soccc(&unimplemented!(),) }) == (1)));
        assert!(((soccc_lhs) == (1)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_wchar_t_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_lhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_wchar_t_::get_self_soccc(&unimplemented!(),) }) == (1)));
        assert!(((soccc_lhs) == (1)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = {
            let mut __tmp2 = lhs.clone();
            __tmp2.pop();
            __tmp2.extend_from_slice(::std::slice::from_raw_parts(
                (rhs.as_mut_ptr()).cast_const(),
                libcc2rs::c_str_len((rhs.as_mut_ptr()).cast_const()),
            ));
            __tmp2.push(0_i32);
            __tmp2
        };
        assert!(
            ((unsafe { soccc_allocator_wchar_t_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_lhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_wchar_t_::get_self_soccc(&unimplemented!(),) }) == (0)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_wchar_t_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_lhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_wchar_t_::get_self_soccc(&unimplemented!(),) }) == (0)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_wchar_t_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_rhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_wchar_t_::get_self_soccc(&unimplemented!(),) }) == (1)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (1)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_wchar_t_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_rhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_wchar_t_::get_self_soccc(&unimplemented!(),) }) == (1)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (1)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_wchar_t_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_rhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_wchar_t_::get_self_soccc(&unimplemented!(),) }) == (0)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (0)));
    }
    {
        let mut soccc_lhs: i32 = 0;
        let mut soccc_rhs: i32 = 0;
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            ((unsafe { soccc_allocator_wchar_t_::get_soccc(&unimplemented!(),) })
                == (&mut soccc_rhs as *mut i32))
        );
        assert!(((unsafe { soccc_allocator_wchar_t_::get_self_soccc(&unimplemented!(),) }) == (0)));
        assert!(((soccc_lhs) == (0)));
        assert!(((soccc_rhs) == (0)));
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
    (unsafe { test_1() });
    const _: () = assert!(true, "test<char>()");;
    const _: () = assert!(true, "test<wchar_t>()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
