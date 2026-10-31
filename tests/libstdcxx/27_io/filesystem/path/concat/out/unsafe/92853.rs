extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn compare_paths_0(p1: *const Vec<libc::c_char>, p2: *const Vec<libc::c_char>) {
    if !(unimplemented!()) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if !(unimplemented!()) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    let mut d1: isize = unimplemented!();
    let mut d2: isize = unimplemented!();
    if ((d1) != (d2)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if !(unimplemented!()) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
}
pub static mut test_paths_1: std::cell::LazyCell<[Vec<libc::c_char>; 18]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            std::slice::from_raw_parts(c"".as_ptr(), libcc2rs::c_str_len(c"".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"/".as_ptr(), libcc2rs::c_str_len(c"/".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"//".as_ptr(), libcc2rs::c_str_len(c"//".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"/.".as_ptr(), libcc2rs::c_str_len(c"/.".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"/./".as_ptr(), libcc2rs::c_str_len(c"/./".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"/a".as_ptr(), libcc2rs::c_str_len(c"/a".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"/a/".as_ptr(), libcc2rs::c_str_len(c"/a/".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"/a//".as_ptr(), libcc2rs::c_str_len(c"/a//".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(
                c"/a/b/c/d".as_ptr(),
                libcc2rs::c_str_len(c"/a/b/c/d".as_ptr()) + 1,
            )
            .to_vec(),
            std::slice::from_raw_parts(
                c"/a//b".as_ptr(),
                libcc2rs::c_str_len(c"/a//b".as_ptr()) + 1,
            )
            .to_vec(),
            std::slice::from_raw_parts(c"a".as_ptr(), libcc2rs::c_str_len(c"a".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"a/b".as_ptr(), libcc2rs::c_str_len(c"a/b".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"a/b/".as_ptr(), libcc2rs::c_str_len(c"a/b/".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(
                c"a/b/c".as_ptr(),
                libcc2rs::c_str_len(c"a/b/c".as_ptr()) + 1,
            )
            .to_vec(),
            std::slice::from_raw_parts(
                c"a/b/c.d".as_ptr(),
                libcc2rs::c_str_len(c"a/b/c.d".as_ptr()) + 1,
            )
            .to_vec(),
            std::slice::from_raw_parts(
                c"a/b/..".as_ptr(),
                libcc2rs::c_str_len(c"a/b/..".as_ptr()) + 1,
            )
            .to_vec(),
            std::slice::from_raw_parts(
                c"a/b/c.".as_ptr(),
                libcc2rs::c_str_len(c"a/b/c.".as_ptr()) + 1,
            )
            .to_vec(),
            std::slice::from_raw_parts(
                c"a/b/.c".as_ptr(),
                libcc2rs::c_str_len(c"a/b/.c".as_ptr()) + 1,
            )
            .to_vec(),
        ]
    });
pub unsafe fn root_path_2() -> Vec<libc::c_char> {
    return {
        let mut __v: Vec<libc::c_char> = c"/"
            .path_bytes()
            .into_iter()
            .map(|__b| __b as libc::c_char)
            .collect();
        __v.push(0);
        __v
    };
}
pub unsafe fn nonexistent_path_3(mut file: Option<Vec<libc::c_char>>) -> Vec<libc::c_char> {
    let mut file: Vec<libc::c_char> = file.unwrap_or_else(|| unsafe {
        std::slice::from_raw_parts(
            concat!(file!(), "\0").as_ptr() as *const libc::c_char,
            libcc2rs::c_str_len(concat!(file!(), "\0").as_ptr() as *const libc::c_char) + 1,
        )
        .to_vec()
    });
    if ((file.len() - 1) > (3_usize)) && ((unimplemented!()) == (0)) {
        {
            file.pop();
            file.resize(
                ((file.len() - 1).wrapping_sub(3_usize)) as usize,
                (0 as libc::c_char),
            );
            file.push((0 as libc::c_char))
        };
    }
    let mut pos: usize = {
        let __n = file.len().saturating_sub(1);
        let __end = usize::MAX.saturating_add(1).min(__n);
        let __lookup =
            ::std::slice::from_raw_parts(c"/\\".as_ptr(), libcc2rs::c_str_len(c"/\\".as_ptr()));
        match file
            .iter()
            .take(__end)
            .rposition(|__c| __lookup.contains(__c))
        {
            Some(__idx) => __idx,
            None => usize::MAX,
        }
    };
    if ((pos) != (usize::MAX)) {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: Vec<libc::c_char> = unimplemented!();
    unimplemented!();
    let mut fd: i32 = unimplemented!();
    if ((fd) == (-1_i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    libc::unlink(file.as_ptr());
    libc::close(fd);
    unimplemented!();
    return unimplemented!();
}
pub type __gnu_test_scoped_file_adopt_file_t = u32;
pub const __gnu_test_scoped_file_adopt_file_t_adopt_file: __gnu_test_scoped_file_adopt_file_t = 0;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, DestructorUnsafe)]
pub struct __gnu_test_scoped_file {
    pub path: Vec<libc::c_char>,
}
impl __gnu_test_scoped_file {
    pub unsafe fn new_1(p: Option<*const Vec<libc::c_char>>) -> Self {
        let mut __p_default: Option<Vec<libc::c_char>> = None;
        let mut p: *const Vec<libc::c_char> = p.unwrap_or_else(|| unsafe {
            __p_default.insert((unsafe { nonexistent_path_3(None) })) as *const Vec<libc::c_char>
        });
        let mut this = Self {
            path: unimplemented!(),
        };
        ::std::fs::File::create(
            ::std::ffi::CStr::from_ptr((unimplemented!() as *const ()).cast::<libc::c_char>())
                .to_str()
                .unwrap(),
        )
        .unwrap();
        this
    }
    pub unsafe fn new_2(
        mut p: Vec<libc::c_char>,
        mut _a1: __gnu_test_scoped_file_adopt_file_t,
    ) -> Self {
        let mut this = Self {
            path: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        if !(unimplemented!()) {
            unimplemented!();
        }
    }
    pub unsafe fn move_from(_a0: *mut __gnu_test_scoped_file) -> Self {
        let mut this = Self {
            path: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __gnu_test_scoped_file,
    ) -> *mut __gnu_test_scoped_file {
        unimplemented!();
        return &mut (*(self as *mut __gnu_test_scoped_file));
    }
}
impl Default for __gnu_test_scoped_file {
    fn default() -> Self {
        unsafe { __gnu_test_scoped_file::new_1(None) }
    }
}
pub unsafe fn permissions_are_testable_5(mut print_msg: Option<bool>) -> bool {
    let mut print_msg: bool = print_msg.unwrap_or_else(|| unsafe { true });
    let mut testable: bool = false;
    if ((libc::geteuid()) != (0 as ::libc::uid_t)) {
        testable = true;
    }
    if (print_msg) && (!(testable)) {
        libc::puts(c"Skipping tests that depend on filesystem permissions".as_ptr());
    }
    return testable;
}
static mut counter_4: std::cell::LazyCell<u32> =
    std::cell::LazyCell::new(|| unsafe { (unimplemented!() as u32) });
pub unsafe fn test01_6() {
    let mut p1: Vec<libc::c_char> = {
        let mut __v: Vec<libc::c_char> = c"."
            .path_bytes()
            .into_iter()
            .map(|__b| __b as libc::c_char)
            .collect();
        __v.push(0);
        __v
    };
    let mut p2: Vec<libc::c_char> = {
        let mut __v: Vec<libc::c_char> = c"/"
            .path_bytes()
            .into_iter()
            .map(|__b| __b as libc::c_char)
            .collect();
        __v.push(0);
        __v
    };
    unimplemented!();
    let mut p3: Vec<libc::c_char> = unimplemented!();
    (unsafe {
        let mut _p2: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"./"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        compare_paths_0(&p3, &mut _p2)
    });
}
pub unsafe fn test02_7() {
    let mut p1: Vec<libc::c_char> = {
        let mut __v: Vec<libc::c_char> = c"."
            .path_bytes()
            .into_iter()
            .map(|__b| __b as libc::c_char)
            .collect();
        __v.push(0);
        __v
    };
    let mut p2: Vec<libc::c_char> = {
        let mut __v: Vec<libc::c_char> = c"////"
            .path_bytes()
            .into_iter()
            .map(|__b| __b as libc::c_char)
            .collect();
        __v.push(0);
        __v
    };
    unimplemented!();
    let mut p3: Vec<libc::c_char> = unimplemented!();
    (unsafe {
        let mut _p2: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c".////"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        compare_paths_0(&p3, &mut _p2)
    });
}
pub unsafe fn test03_8() {
    let mut p1: Vec<libc::c_char> = {
        let mut __v: Vec<libc::c_char> = c"./"
            .path_bytes()
            .into_iter()
            .map(|__b| __b as libc::c_char)
            .collect();
        __v.push(0);
        __v
    };
    let mut p2: Vec<libc::c_char> = {
        let mut __v: Vec<libc::c_char> = c"/"
            .path_bytes()
            .into_iter()
            .map(|__b| __b as libc::c_char)
            .collect();
        __v.push(0);
        __v
    };
    unimplemented!();
    let mut p3: Vec<libc::c_char> = unimplemented!();
    (unsafe {
        let mut _p2: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c".//"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        compare_paths_0(&p3, &mut _p2)
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test01_6() });
    (unsafe { test02_7() });
    (unsafe { test03_8() });
    0
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_paths_1);
}
