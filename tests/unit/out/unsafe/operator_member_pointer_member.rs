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
pub struct Inner {
    pub x: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Table {}
impl Table {
    pub unsafe fn operator_index(mut i: i32) -> *mut i32 {
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut Table_table_0))[(i) as usize];
    }
}
pub static mut Table_table_0: std::cell::LazyCell<[i32; 3]> =
    std::cell::LazyCell::new(|| unsafe { [7, 8, 9] });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub data: [i32; 3],
    pub inner: Inner,
}
impl S {
    pub unsafe fn operator_index_1(&mut self, mut i: i32) -> *mut i32 {
        return &mut self.data[(i) as usize];
    }
    pub unsafe fn operator_index_2(&self, mut i: i32) -> *const i32 {
        return &self.data[(i) as usize];
    }
    pub unsafe fn operator_deref(&mut self) -> *mut Inner {
        return &mut self.inner;
    }
    pub unsafe fn operator_arrow(&mut self) -> *mut Inner {
        return (&mut self.inner as *mut Inner);
    }
    pub unsafe fn operator_addr(&mut self) -> *mut i32 {
        return (&mut self.data[(0) as usize] as *mut i32);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S {
        data: [1, 2, 3],
        inner: Inner { x: 9 },
    };
    (if ((*(unsafe { S::operator_index_1(&mut s, 1) })) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s[1] == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*(unsafe { S::operator_index_1(&mut s, 1) })) = 20;
    (if ((*(unsafe { S::operator_index_1(&mut s, 1) })) == (20)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s[1] == 20".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let cs: *const S = &s;
    (if ((*(unsafe { S::operator_index_2(&(*cs), 2) })) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"cs[2] == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { S::operator_deref(&mut s) })).x) == (9)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"(*s).x == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*(unsafe { S::operator_deref(&mut s) })).x = 10;
    (if (((*(unsafe { S::operator_arrow(&mut s) })).x) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s->x == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*(unsafe { S::operator_arrow(&mut s) })).x = 11;
    (if ((s.inner.x) == (11)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.inner.x == 11".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut p: *mut i32 = (unsafe { S::operator_addr(&mut s) });
    (if ((*p) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"*p == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*p) = 5;
    (if ((s.data[(0) as usize]) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.data[0] == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut t: Table = <Table>::default();
    (if ((*(unsafe { Table::operator_index(1) })) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"t[1] == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*(unsafe { Table::operator_index(1) })) = 80;
    (if (((*std::cell::LazyCell::force_mut(&mut *&raw mut Table_table_0))[(1) as usize]) == (80)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"Table::table[1] == 80".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_1(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const Table_table_0);
}
