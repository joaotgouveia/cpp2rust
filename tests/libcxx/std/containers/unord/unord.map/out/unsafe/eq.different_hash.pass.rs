extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn hash_identity_0(mut val: ()) -> () {
    return val;
}
pub unsafe fn hash_identity_1(mut val: ()) -> () {
    return val;
}
pub unsafe fn hash_neg_2(mut val: ()) -> () {
    return (unimplemented!() as ()).wrapping_sub(val);
}
pub unsafe fn hash_neg_3(mut val: ()) -> () {
    return ((unimplemented!()) - (val));
}
pub unsafe fn hash_scale_4(mut val: ()) -> () {
    return (((val) << (1_())) as ());
}
pub unsafe fn hash_scale_5(mut val: ()) -> () {
    return ((val) << (1_()));
}
pub unsafe fn hash_even_6(mut val: ()) -> () {
    return if (((val) & (1_())) != 0) { 1_() } else { 0_() };
}
pub unsafe fn hash_even_7(mut val: ()) -> () {
    return if (((val) & (1_())) != 0) { 1_() } else { 0_() };
}
pub unsafe fn hash_same_8(mut _a0: ()) -> () {
    return 1_();
}
pub unsafe fn hash_same_9(mut _a0: ()) -> () {
    return 1_();
}
pub unsafe fn hash_same_10(mut _a0: *mut ()) -> () {
    return 1_();
}
pub unsafe fn hash_identity_11(mut val: *mut ()) -> () {
    return (*val);
}
pub unsafe fn hash_neg_12(mut val: *mut ()) -> () {
    return ((unimplemented!()) - (*val));
}
pub unsafe fn hash_scale_13(mut val: *mut ()) -> () {
    return ((*val) << (1_()));
}
pub unsafe fn hash_even_14(mut val: *mut ()) -> () {
    return if (((*val) & (1_())) != 0) { 1_() } else { 0_() };
}
pub unsafe fn populate_15(m: *mut (), mut start: *mut (), mut end: *mut ()) {
    let mut p1: *mut () = start;
    let mut p2: *mut () = end.offset(-((1_()) as isize));
    'loop_: while ((p1) != (end)) {
        unimplemented!();
        {
            p1.prefix_inc();
            p2.prefix_dec()
        };
    }
}
pub unsafe fn populate_16(m: *mut (), mut start: *mut (), mut end: *mut ()) {
    let mut p1: *mut () = start;
    let mut p2: *mut () = end.offset(-((1_()) as isize));
    'loop_: while ((p1) != (end)) {
        unimplemented!();
        {
            p1.prefix_inc();
            p2.prefix_dec()
        };
    }
}
pub unsafe fn populate_17(m: *mut (), mut start: *mut *mut (), mut end: *mut *mut ()) {
    let mut p1: *mut *mut () = start;
    let mut p2: *mut *mut () = end.offset(-((1_()) as isize));
    'loop_: while ((p1) != (end)) {
        unimplemented!();
        {
            p1.prefix_inc();
            p2.prefix_dec()
        };
    }
}
pub unsafe fn test_18(vals: *mut [(); 46]) {
    let mut c1: () = unimplemented!();
    let mut c2: () = unimplemented!();
    let mut c3: () = unimplemented!();
    let mut c4: () = unimplemented!();
    let mut c5: () = unimplemented!();
    (unsafe {
        let _start: *mut () = unimplemented!();
        let _end: *mut () = unimplemented!();
        populate_15(&mut c1, _start, _end)
    });
    (unsafe {
        let _start: *mut () = unimplemented!();
        let _end: *mut () = unimplemented!();
        populate_15(&mut c2, _start, _end)
    });
    (unsafe {
        let _start: *mut () = unimplemented!();
        let _end: *mut () = unimplemented!();
        populate_15(&mut c3, _start, _end)
    });
    (unsafe {
        let _start: *mut () = unimplemented!();
        let _end: *mut () = unimplemented!();
        populate_15(&mut c4, _start, _end)
    });
    (unsafe {
        let _start: *mut () = unimplemented!();
        let _end: *mut () = unimplemented!();
        populate_15(&mut c5, _start, _end)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_19(vals: *mut [(); 2]) {
    let mut c1: () = unimplemented!();
    let mut c2: () = unimplemented!();
    let mut c3: () = unimplemented!();
    let mut c4: () = unimplemented!();
    let mut c5: () = unimplemented!();
    (unsafe {
        let _start: *mut () = unimplemented!();
        let _end: *mut () = unimplemented!();
        populate_16(&mut c1, _start, _end)
    });
    (unsafe {
        let _start: *mut () = unimplemented!();
        let _end: *mut () = unimplemented!();
        populate_16(&mut c2, _start, _end)
    });
    (unsafe {
        let _start: *mut () = unimplemented!();
        let _end: *mut () = unimplemented!();
        populate_16(&mut c3, _start, _end)
    });
    (unsafe {
        let _start: *mut () = unimplemented!();
        let _end: *mut () = unimplemented!();
        populate_16(&mut c4, _start, _end)
    });
    (unsafe {
        let _start: *mut () = unimplemented!();
        let _end: *mut () = unimplemented!();
        populate_16(&mut c5, _start, _end)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_20(vals: *mut [*mut (); 3]) {
    let mut c1: () = unimplemented!();
    let mut c2: () = unimplemented!();
    let mut c3: () = unimplemented!();
    let mut c4: () = unimplemented!();
    let mut c5: () = unimplemented!();
    (unsafe {
        let _start: *mut *mut () = unimplemented!();
        let _end: *mut *mut () = unimplemented!();
        populate_17(&mut c1, _start, _end)
    });
    (unsafe {
        let _start: *mut *mut () = unimplemented!();
        let _end: *mut *mut () = unimplemented!();
        populate_17(&mut c2, _start, _end)
    });
    (unsafe {
        let _start: *mut *mut () = unimplemented!();
        let _end: *mut *mut () = unimplemented!();
        populate_17(&mut c3, _start, _end)
    });
    (unsafe {
        let _start: *mut *mut () = unimplemented!();
        let _end: *mut *mut () = unimplemented!();
        populate_17(&mut c4, _start, _end)
    });
    (unsafe {
        let _start: *mut *mut () = unimplemented!();
        let _end: *mut *mut () = unimplemented!();
        populate_17(&mut c5, _start, _end)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    {
        let mut vals: [(); 46] = [
            1_(),
            2_(),
            2_(),
            3_(),
            3_(),
            3_(),
            4_(),
            4_(),
            4_(),
            4_(),
            5_(),
            5_(),
            5_(),
            5_(),
            5_(),
            6_(),
            6_(),
            6_(),
            6_(),
            6_(),
            6_(),
            7_(),
            7_(),
            7_(),
            7_(),
            7_(),
            7_(),
            7_(),
            8_(),
            8_(),
            8_(),
            8_(),
            8_(),
            8_(),
            8_(),
            8_(),
            9_(),
            9_(),
            9_(),
            9_(),
            9_(),
            9_(),
            9_(),
            9_(),
            9_(),
            10_(),
        ];
        (unsafe { test_18(&mut vals) });
    }
    {
        let mut vals: [(); 2] = [true, false];
        (unsafe { test_19(&mut vals) });
    }
    {
        let mut vals: [*mut (); 3] = [
            ((c"a").as_ptr() as *mut ()),
            ((c"b").as_ptr() as *mut ()),
            ((c"cde").as_ptr() as *mut ()),
        ];
        (unsafe { test_20(&mut vals) });
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
