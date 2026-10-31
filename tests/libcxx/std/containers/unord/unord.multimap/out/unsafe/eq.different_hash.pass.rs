extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn hash_identity_0(mut val: u64) -> usize {
    return (val as usize);
}
pub unsafe fn hash_identity_1(mut val: bool) -> usize {
    return (val as usize);
}
pub unsafe fn hash_neg_2(mut val: u64) -> usize {
    return ((u64::MAX as u64).wrapping_sub(val) as usize);
}
pub unsafe fn hash_neg_3(mut val: bool) -> usize {
    return (((true as i32) - (val as i32)) as usize);
}
pub unsafe fn hash_scale_4(mut val: u64) -> usize {
    return (((val) << (1)) as usize);
}
pub unsafe fn hash_scale_5(mut val: bool) -> usize {
    return (((val as i32) << (1)) as usize);
}
pub unsafe fn hash_even_6(mut val: u64) -> usize {
    return (if (((val) & (1_u64)) != 0) { 1 } else { 0 } as usize);
}
pub unsafe fn hash_even_7(mut val: bool) -> usize {
    return (if (((val as i32) & (1)) != 0) { 1 } else { 0 } as usize);
}
pub unsafe fn hash_same_8(mut _a0: u64) -> usize {
    return 1_usize;
}
pub unsafe fn hash_same_9(mut _a0: bool) -> usize {
    return 1_usize;
}
pub unsafe fn hash_same_10(mut _a0: *mut libc::c_char) -> usize {
    return 1_usize;
}
pub unsafe fn hash_identity_11(mut val: *mut libc::c_char) -> usize {
    return ((*val) as usize);
}
pub unsafe fn hash_neg_12(mut val: *mut libc::c_char) -> usize {
    return (((libc::c_char::MAX as i32) - ((*val) as i32)) as usize);
}
pub unsafe fn hash_scale_13(mut val: *mut libc::c_char) -> usize {
    return ((((*val) as i32) << (1)) as usize);
}
pub unsafe fn hash_even_14(mut val: *mut libc::c_char) -> usize {
    return (if ((((*val) as i32) & (1)) != 0) { 1 } else { 0 } as usize);
}
pub unsafe fn populate_15(m: *mut (), mut start: *mut u64, mut end: *mut u64) {
    let mut p1: *mut u64 = start;
    let mut p2: *mut u64 = end.offset(-((1) as isize));
    'loop_: while ((p1) != (end)) {
        unimplemented!();
        {
            p1.prefix_inc();
            p2.prefix_dec()
        };
    }
}
pub unsafe fn populate_16(m: *mut (), mut start: *mut bool, mut end: *mut bool) {
    let mut p1: *mut bool = start;
    let mut p2: *mut bool = end.offset(-((1) as isize));
    'loop_: while ((p1) != (end)) {
        unimplemented!();
        {
            p1.prefix_inc();
            p2.prefix_dec()
        };
    }
}
pub unsafe fn populate_17(
    m: *mut (),
    mut start: *mut *mut libc::c_char,
    mut end: *mut *mut libc::c_char,
) {
    let mut p1: *mut *mut libc::c_char = start;
    let mut p2: *mut *mut libc::c_char = end.offset(-((1) as isize));
    'loop_: while ((p1) != (end)) {
        unimplemented!();
        {
            p1.prefix_inc();
            p2.prefix_dec()
        };
    }
}
pub unsafe fn test_18(vals: *mut [u64; 46]) {
    let mut c1: () = unimplemented!();
    let mut c2: () = unimplemented!();
    let mut c3: () = unimplemented!();
    let mut c4: () = unimplemented!();
    let mut c5: () = unimplemented!();
    (unsafe {
        let _start: *mut u64 = unimplemented!();
        let _end: *mut u64 = (*vals).as_ptr().add((*vals).len());
        populate_15(&mut c1, _start, _end)
    });
    (unsafe {
        let _start: *mut u64 = unimplemented!();
        let _end: *mut u64 = (*vals).as_ptr().add((*vals).len());
        populate_15(&mut c2, _start, _end)
    });
    (unsafe {
        let _start: *mut u64 = unimplemented!();
        let _end: *mut u64 = (*vals).as_ptr().add((*vals).len());
        populate_15(&mut c3, _start, _end)
    });
    (unsafe {
        let _start: *mut u64 = unimplemented!();
        let _end: *mut u64 = (*vals).as_ptr().add((*vals).len());
        populate_15(&mut c4, _start, _end)
    });
    (unsafe {
        let _start: *mut u64 = unimplemented!();
        let _end: *mut u64 = (*vals).as_ptr().add((*vals).len());
        populate_15(&mut c5, _start, _end)
    });
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn test_19(vals: *mut [bool; 2]) {
    let mut c1: () = unimplemented!();
    let mut c2: () = unimplemented!();
    let mut c3: () = unimplemented!();
    let mut c4: () = unimplemented!();
    let mut c5: () = unimplemented!();
    (unsafe {
        let _start: *mut bool = unimplemented!();
        let _end: *mut bool = (*vals).as_ptr().add((*vals).len());
        populate_16(&mut c1, _start, _end)
    });
    (unsafe {
        let _start: *mut bool = unimplemented!();
        let _end: *mut bool = (*vals).as_ptr().add((*vals).len());
        populate_16(&mut c2, _start, _end)
    });
    (unsafe {
        let _start: *mut bool = unimplemented!();
        let _end: *mut bool = (*vals).as_ptr().add((*vals).len());
        populate_16(&mut c3, _start, _end)
    });
    (unsafe {
        let _start: *mut bool = unimplemented!();
        let _end: *mut bool = (*vals).as_ptr().add((*vals).len());
        populate_16(&mut c4, _start, _end)
    });
    (unsafe {
        let _start: *mut bool = unimplemented!();
        let _end: *mut bool = (*vals).as_ptr().add((*vals).len());
        populate_16(&mut c5, _start, _end)
    });
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn test_20(vals: *mut [*mut libc::c_char; 3]) {
    let mut c1: () = unimplemented!();
    let mut c2: () = unimplemented!();
    let mut c3: () = unimplemented!();
    let mut c4: () = unimplemented!();
    let mut c5: () = unimplemented!();
    (unsafe {
        let _start: *mut *mut libc::c_char = unimplemented!();
        let _end: *mut *mut libc::c_char = (*vals).as_ptr().add((*vals).len());
        populate_17(&mut c1, _start, _end)
    });
    (unsafe {
        let _start: *mut *mut libc::c_char = unimplemented!();
        let _end: *mut *mut libc::c_char = (*vals).as_ptr().add((*vals).len());
        populate_17(&mut c2, _start, _end)
    });
    (unsafe {
        let _start: *mut *mut libc::c_char = unimplemented!();
        let _end: *mut *mut libc::c_char = (*vals).as_ptr().add((*vals).len());
        populate_17(&mut c3, _start, _end)
    });
    (unsafe {
        let _start: *mut *mut libc::c_char = unimplemented!();
        let _end: *mut *mut libc::c_char = (*vals).as_ptr().add((*vals).len());
        populate_17(&mut c4, _start, _end)
    });
    (unsafe {
        let _start: *mut *mut libc::c_char = unimplemented!();
        let _end: *mut *mut libc::c_char = (*vals).as_ptr().add((*vals).len());
        populate_17(&mut c5, _start, _end)
    });
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
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
    {
        let mut vals: [usize; 46] = [
            1_usize, 2_usize, 2_usize, 3_usize, 3_usize, 3_usize, 4_usize, 4_usize, 4_usize,
            4_usize, 5_usize, 5_usize, 5_usize, 5_usize, 5_usize, 6_usize, 6_usize, 6_usize,
            6_usize, 6_usize, 6_usize, 7_usize, 7_usize, 7_usize, 7_usize, 7_usize, 7_usize,
            7_usize, 8_usize, 8_usize, 8_usize, 8_usize, 8_usize, 8_usize, 8_usize, 8_usize,
            9_usize, 9_usize, 9_usize, 9_usize, 9_usize, 9_usize, 9_usize, 9_usize, 9_usize,
            10_usize,
        ];
        (unsafe { test_18(&mut *(&raw mut vals).cast::<[u64; 46]>()) });
    }
    {
        let mut vals: [bool; 2] = [true, false];
        (unsafe { test_19(&mut vals) });
    }
    {
        let mut vals: [*mut libc::c_char; 3] = [
            ((c"a").as_ptr() as *mut libc::c_char),
            ((c"b").as_ptr() as *mut libc::c_char),
            ((c"cde").as_ptr() as *mut libc::c_char),
        ];
        (unsafe { test_20(&mut vals) });
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
