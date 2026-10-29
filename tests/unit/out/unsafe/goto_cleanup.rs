extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn early_0(mut n: ()) -> () {
    let mut ret: () = unimplemented!();
    let mut intentionally_const_var: () = unimplemented!();
    goto_block!({
        '__entry: {
            ret = 0_();
            if ((((n) < (0_())) as ()) != 0) {
                ret = -1_();
                goto!('out);
            }
            ret = 100_();
            intentionally_const_var = 22_();
        }
        'out: {
            return (((ret) + (intentionally_const_var)) - (intentionally_const_var));
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn from_loop_1(mut n: ()) -> () {
    let mut ret: () = unimplemented!();
    goto_block!({
        '__entry: {
            ret = 0_();
            let mut i: () = 0_();
            'loop_: while ((((i) < (n)) as ()) != 0) {
                if ((((i) == (3_())) as ()) != 0) {
                    ret = 7_();
                    goto!('out);
                }
                ret += i;
                i.postfix_inc();
            }
            ret = 999_();
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn from_switch_2(mut n: ()) -> () {
    let mut ret: () = unimplemented!();
    goto_block!({
        '__entry: {
            ret = 0_();
            'switch: {
                match { n } {
                    __v if __v == 1_() => {
                        ret = 10_();
                        goto!('out);
                    }
                    _ => {
                        ret = 20_();
                        break 'switch;
                    }
                }
            };
            ret = 999_();
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct wrapper {
    pub item: *mut (),
}
pub unsafe fn via_pointer_3(mut w: *mut wrapper, mut fail: ()) -> () {
    let mut ret: () = unimplemented!();
    let mut item: *mut () = std::ptr::null_mut();
    goto_block!({
        '__entry: {
            ret = 0_();
            item = (*w).item;
            if (fail != 0) {
                ret = -1_();
                goto!('out);
            }
            ret = (*item);
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn via_arrays_4(mut fail: ()) -> () {
    let mut ret: () = unimplemented!();
    let mut remain: [(); 4] = [unimplemented!(); 4];
    let mut name: [(); 5] = [unimplemented!(); 5];
    goto_block!({
        '__entry: {
            ret = 0_();
            remain = [0_(), unimplemented!(), unimplemented!(), unimplemented!()];
            name = std::mem::transmute(*b"wxyz\0");
            if (fail != 0) {
                ret = -1_();
                goto!('out);
            }
            remain[(1_()) as usize] = 9_();
            ret = ((((remain[(0_()) as usize]) + (remain[(1_()) as usize]))
                + (((name[(0_()) as usize]) == ('w' as ())) as ()))
                + (((name[(4_()) as usize]) == ('\0' as ())) as ()));
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut value: () = 42_();
    let mut w: wrapper = wrapper {
        item: (&mut value as *mut ()),
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
