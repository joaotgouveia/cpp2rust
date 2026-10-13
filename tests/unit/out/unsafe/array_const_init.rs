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
pub struct S {
    pub head: i32,
    pub tail: [i32; 3],
    pub buf: [libc::c_char; 4],
}
pub static mut s_0: std::cell::LazyCell<S> = std::cell::LazyCell::new(|| unsafe {
    S {
        head: 5,
        tail: [0; 3],
        buf: [0; 4],
    }
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*std::cell::LazyCell::force_mut(&mut *&raw mut s_0)).head) == (5)) as i32) != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"s.head == 5".as_ptr().cast_mut()).cast_const(),
                        (c"array_const_init.c".as_ptr().cast_mut()).cast_const(),
                        12_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut i: i32 = 0;
    'loop_: while ((((i) < (3)) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*std::cell::LazyCell::force_mut(&mut *&raw mut s_0)).tail[(i) as usize])
                    == (0)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_1(
                            (c"s.tail[i] == 0".as_ptr().cast_mut()).cast_const(),
                            (c"array_const_init.c".as_ptr().cast_mut()).cast_const(),
                            14_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
        i.postfix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((((i) < (4)) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*std::cell::LazyCell::force_mut(&mut *&raw mut s_0)).buf[(i) as usize]
                    as i32)
                    == (0)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_1(
                            (c"s.buf[i] == 0".as_ptr().cast_mut()).cast_const(),
                            (c"array_const_init.c".as_ptr().cast_mut()).cast_const(),
                            17_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
        i.postfix_inc();
    }
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
    std::cell::LazyCell::force(&*&raw const s_0);
}
