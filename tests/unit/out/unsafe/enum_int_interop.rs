extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Color = ();
pub const Color_RED: Color = 0;
pub const Color_GREEN: Color = 1;
pub const Color_BLUE: Color = 2;
pub type Option = ();
pub const Option_OPT_NONE: Option = 0;
pub const Option_OPT_A: Option = 10;
pub const Option_OPT_B: Option = 20;
pub const Option_OPT_C: Option = 30;
pub type Tag = ();
pub const Tag_TAG_ZERO: Tag = 0;
pub const Tag_TAG_ONE: Tag = 1;
pub const Tag_TAG_TWO: Tag = 2;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Entry {
    pub name: *const (),
    pub color: Color,
    pub opt: Option,
}
pub static mut global_color_0: std::cell::LazyCell<Color> =
    std::cell::LazyCell::new(|| unsafe { Color_GREEN });
pub static mut global_opt_1: std::cell::LazyCell<Option> =
    std::cell::LazyCell::new(|| unsafe { Option_OPT_B });
pub static mut global_tag_2: std::cell::LazyCell<Tag> =
    std::cell::LazyCell::new(|| unsafe { Tag_TAG_TWO });
pub static mut entries_3: std::cell::LazyCell<[Entry; 3]> = std::cell::LazyCell::new(|| unsafe {
    [
        Entry {
            name: c"first".as_ptr(),
            color: Color_RED,
            opt: Option_OPT_NONE,
        },
        Entry {
            name: c"second".as_ptr(),
            color: Color_GREEN,
            opt: Option_OPT_A,
        },
        Entry {
            name: c"third".as_ptr(),
            color: Color_BLUE,
            opt: Option_OPT_C,
        },
    ]
});
pub unsafe fn as_int_4(mut c: Color) -> () {
    return (c as ());
}
pub unsafe fn classify_option_5(mut option: ()) -> () {
    'switch: {
        match { option } {
            __v if __v == (Option_OPT_NONE as ()) => {
                return -1_();
            }
            __v if __v == (Option_OPT_A as ()) => {
                return 1_();
            }
            __v if __v == (Option_OPT_B as ()) => {
                return 2_();
            }
            __v if __v == (Option_OPT_C as ()) => {
                return 3_();
            }
            _ => {
                return 0_();
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn make_color_6(mut n: ()) -> Color {
    return ((n) as Color);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut c: Color = Color_RED;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    if ((c as ()) == (Color_GREEN as ())) {
        return 1_();
    }
    'switch: {
        match { (c as ()) } {
            __v if __v == 0_() => {
                break 'switch;
            }
            __v if __v == 1_() => {
                return 1_();
            }
            __v if __v == 2_() => {
                return 2_();
            }
            _ => {
                return 99_();
            }
        }
    };
    let mut x: () = (c as ());
    unimplemented!();
    let mut y: () = ((c as ()) + (1_()));
    unimplemented!();
    c = ((2_()) as Color);
    unimplemented!();
    unimplemented!();
    c = (unsafe { make_color_6(1_()) });
    unimplemented!();
    let mut cmp: Color = (((c as ()) + (1_())) as Color);
    unimplemented!();
    let mut o: Option = Option_OPT_A;
    unimplemented!();
    unimplemented!();
    let mut oi: () = (o as ());
    unimplemented!();
    o = ((20_()) as Option);
    unimplemented!();
    let mut rc: () = (unsafe { classify_option_5((o as ())) });
    unimplemented!();
    rc = (unsafe { classify_option_5(20_()) });
    unimplemented!();
    rc = (unsafe { classify_option_5((Option_OPT_C as ())) });
    unimplemented!();
    let mut t: Tag = Tag_TAG_ONE;
    unimplemented!();
    unimplemented!();
    let mut ti: () = (t as ());
    unimplemented!();
    t = ((2_()) as Tag);
    unimplemented!();
    'switch: {
        match { (t as ()) } {
            __v if __v == (Tag_TAG_ZERO as ()) => {
                return 90_();
            }
            __v if __v == 1_() => {
                return 91_();
            }
            __v if __v == 2_() => {
                break 'switch;
            }
            _ => {}
        }
    };
    let mut extra: () = (((Color_RED as ()) + (Color_GREEN as ())) + (Color_BLUE as ()));
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
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const global_color_0);
    std::cell::LazyCell::force(&*&raw const global_opt_1);
    std::cell::LazyCell::force(&*&raw const global_tag_2);
    std::cell::LazyCell::force(&*&raw const entries_3);
}
