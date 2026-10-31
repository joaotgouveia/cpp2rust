extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn doAssign_0(mut lhs: (), mut rhs: ()) -> bool {
    const _: () = assert!(true, "");;
    unimplemented!();
    return ((unimplemented!()) == (unimplemented!()))
        && ((unimplemented!()) == (unimplemented!()));
}
pub unsafe fn doAssign_1(mut lhs: (), mut rhs: ()) -> bool {
    const _: () = assert!(true, "");;
    unimplemented!();
    return ((unimplemented!()) == (unimplemented!()))
        && ((unimplemented!()) == (unimplemented!()));
}
pub unsafe fn doAssign_2(mut lhs: (), mut rhs: ()) -> bool {
    const _: () = assert!(true, "");;
    unimplemented!();
    return ((unimplemented!()) == (unimplemented!()))
        && ((unimplemented!()) == (unimplemented!()));
}
pub unsafe fn doAssign_3(mut lhs: (), mut rhs: ()) -> bool {
    const _: () = assert!(true, "");;
    unimplemented!();
    return ((unimplemented!()) == (unimplemented!()))
        && ((unimplemented!()) == (unimplemented!()));
}
pub unsafe fn doAssign_4(mut lhs: (), mut rhs: ()) -> bool {
    const _: () = assert!(true, "");;
    unimplemented!();
    return ((unimplemented!()) == (unimplemented!()))
        && ((unimplemented!()) == (unimplemented!()));
}
pub unsafe fn doAssign_5(mut lhs: (), mut rhs: ()) -> bool {
    const _: () = assert!(true, "");;
    unimplemented!();
    return ((unimplemented!()) == (unimplemented!()))
        && ((unimplemented!()) == (unimplemented!()));
}
pub static mut carr1_6: std::cell::LazyCell<[i32; 4]> =
    std::cell::LazyCell::new(|| unsafe { [1, 2, 3, 4] });
pub static mut carr2_7: std::cell::LazyCell<[i32; 3]> =
    std::cell::LazyCell::new(|| unsafe { [3, 4, 5] });
pub static mut carr3_8: std::cell::LazyCell<[i32; 2]> =
    std::cell::LazyCell::new(|| unsafe { [7, 8] });
pub static mut arr_9: std::cell::LazyCell<[i32; 4]> =
    std::cell::LazyCell::new(|| unsafe { [5, 6, 7, 9] });
pub static mut strs_10: std::cell::LazyCell<[Vec<libc::c_char>; 3]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            std::slice::from_raw_parts(c"ABC".as_ptr(), libcc2rs::c_str_len(c"ABC".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"DEF".as_ptr(), libcc2rs::c_str_len(c"DEF".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"GHI".as_ptr(), libcc2rs::c_str_len(c"GHI".as_ptr()) + 1)
                .to_vec(),
        ]
    });

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
    {
        let spans: [(); 13] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        const _: () = assert!(true, "std::size(spans) == 13");;
        const _: () = assert!(true, "doAssign(spans[0], spans[0])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[1])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[1])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[8], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[8], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[8], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[8], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[8], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[9], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[9], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[9], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[9], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[10], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[10], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[10], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[11], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[11], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[12], spans[12])");;
    }
    {
        let spans: [(); 6] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        const _: () = assert!(true, "std::size(spans) == 6");;
        const _: () = assert!(true, "doAssign(spans[0], spans[0])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[1])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[1])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[5])");;
    }
    {
        let mut spans: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (unimplemented!())) {
            let mut j: usize = i;
            'loop_: while ((j) < (unimplemented!())) {
                assert!(
                    (unsafe {
                        let _lhs: () = unimplemented!();
                        let _rhs: () = unimplemented!();
                        doAssign_2(_lhs, _rhs)
                    })
                );
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut spans: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (unimplemented!())) {
            let mut j: usize = i;
            'loop_: while ((j) < (unimplemented!())) {
                assert!(
                    (unsafe {
                        let _lhs: () = unimplemented!();
                        let _rhs: () = unimplemented!();
                        doAssign_3(_lhs, _rhs)
                    })
                );
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut spans: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (unimplemented!())) {
            let mut j: usize = i;
            'loop_: while ((j) < (unimplemented!())) {
                assert!(
                    (unsafe {
                        let _lhs: () = unimplemented!();
                        let _rhs: () = unimplemented!();
                        doAssign_4(_lhs, _rhs)
                    })
                );
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut spans: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (unimplemented!())) {
            let mut j: usize = i;
            'loop_: while ((j) < (unimplemented!())) {
                assert!(
                    (unsafe {
                        let _lhs: () = unimplemented!();
                        let _rhs: () = unimplemented!();
                        doAssign_5(_lhs, _rhs)
                    })
                );
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const carr1_6);
    std::cell::LazyCell::force(&*&raw const carr2_7);
    std::cell::LazyCell::force(&*&raw const carr3_8);
    std::cell::LazyCell::force(&*&raw const arr_9);
    std::cell::LazyCell::force(&*&raw const strs_10);
}
