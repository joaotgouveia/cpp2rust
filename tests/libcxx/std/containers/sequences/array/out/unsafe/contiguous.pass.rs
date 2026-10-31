extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn assert_contiguous_0(c: *const Vec<f64>) {
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*c).len())) {
        assert!(
            ((*(unimplemented!().offset((i) as isize)))
                == (*(unimplemented!().offset((i) as isize))))
        );
        i.prefix_inc();
    }
}
pub unsafe fn assert_contiguous_1(c: *const Vec<f64>) {
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*c).len())) {
        assert!(
            ((*(unimplemented!().offset((i) as isize)))
                == (*(unimplemented!().offset((i) as isize))))
        );
        i.prefix_inc();
    }
}
pub unsafe fn assert_contiguous_2(c: *const Vec<f64>) {
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*c).len())) {
        assert!(
            ((*(unimplemented!().offset((i) as isize)))
                == (*(unimplemented!().offset((i) as isize))))
        );
        i.prefix_inc();
    }
}
pub unsafe fn assert_contiguous_3(c: *const Vec<f64>) {
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*c).len())) {
        assert!(
            ((*(unimplemented!().offset((i) as isize)))
                == (*(unimplemented!().offset((i) as isize))))
        );
        i.prefix_inc();
    }
}
pub unsafe fn assert_contiguous_4(c: *const Vec<libc::c_char>) {
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*c).len())) {
        assert!(
            (((*(unimplemented!().offset((i) as isize))) as i32)
                == ((*(unimplemented!().offset((i) as isize))) as i32))
        );
        i.prefix_inc();
    }
}
pub unsafe fn assert_contiguous_5(c: *const Vec<libc::c_char>) {
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*c).len())) {
        assert!(
            (((*(unimplemented!().offset((i) as isize))) as i32)
                == ((*(unimplemented!().offset((i) as isize))) as i32))
        );
        i.prefix_inc();
    }
}
pub unsafe fn assert_contiguous_6(c: *const Vec<libc::c_char>) {
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*c).len())) {
        assert!(
            (((*(unimplemented!().offset((i) as isize))) as i32)
                == ((*(unimplemented!().offset((i) as isize))) as i32))
        );
        i.prefix_inc();
    }
}
pub unsafe fn assert_contiguous_7(c: *const Vec<libc::c_char>) {
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*c).len())) {
        assert!(
            (((*(unimplemented!().offset((i) as isize))) as i32)
                == ((*(unimplemented!().offset((i) as isize))) as i32))
        );
        i.prefix_inc();
    }
}
pub unsafe fn tests_8() -> bool {
    (unsafe {
        let mut _c: Vec<f64> = std::array::from_fn::<_, 0, _>(|_| 0_f64).to_vec();
        assert_contiguous_0(&mut _c)
    });
    (unsafe {
        let mut _c: Vec<f64> = std::array::from_fn::<_, 1, _>(|_| 0_f64).to_vec();
        assert_contiguous_1(&mut _c)
    });
    (unsafe {
        let mut _c: Vec<f64> = std::array::from_fn::<_, 2, _>(|_| 0_f64).to_vec();
        assert_contiguous_2(&mut _c)
    });
    (unsafe {
        let mut _c: Vec<f64> = std::array::from_fn::<_, 3, _>(|_| 0_f64).to_vec();
        assert_contiguous_3(&mut _c)
    });
    (unsafe {
        let mut _c: Vec<libc::c_char> =
            std::array::from_fn::<_, 0, _>(|_| (0 as libc::c_char)).to_vec();
        assert_contiguous_4(&mut _c)
    });
    (unsafe {
        let mut _c: Vec<libc::c_char> =
            std::array::from_fn::<_, 1, _>(|_| (0 as libc::c_char)).to_vec();
        assert_contiguous_5(&mut _c)
    });
    (unsafe {
        let mut _c: Vec<libc::c_char> =
            std::array::from_fn::<_, 2, _>(|_| (0 as libc::c_char)).to_vec();
        assert_contiguous_6(&mut _c)
    });
    (unsafe {
        let mut _c: Vec<libc::c_char> =
            std::array::from_fn::<_, 3, _>(|_| (0 as libc::c_char)).to_vec();
        assert_contiguous_7(&mut _c)
    });
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
    (unsafe { tests_8() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
