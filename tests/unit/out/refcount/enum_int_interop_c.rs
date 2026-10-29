extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Color = ();
pub const Color_RED: Color = 0;
pub const Color_GREEN: Color = 1;
pub const Color_BLUE: Color = 2;
pub type Option = ();
pub const Option_OPT_NONE: Option = 0;
pub const Option_OPT_A: Option = 10;
pub const Option_OPT_B: Option = 20;
pub const Option_OPT_C: Option = 30;
pub type Tag_enum = ();
pub const Tag_enum_TAG_ZERO: Tag_enum = 0;
pub const Tag_enum_TAG_ONE: Tag_enum = 1;
pub const Tag_enum_TAG_TWO: Tag_enum = 2;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Entry {
    #[offset(0)]
    #[byte_size(8)]
    pub name: Ptr<()>,
    #[offset(8)]
    pub color: Color,
    #[offset(12)]
    pub opt: Option,
}
thread_local!(
    pub static global_color_0: Value<Color> = Rc::new(RefCell::new(Color_GREEN));
);
thread_local!(
    pub static global_opt_1: Value<Option> = Rc::new(RefCell::new(Option_OPT_B));
);
thread_local!(
    pub static global_tag_2: Value<Tag_enum> = Rc::new(RefCell::new(Tag_enum_TAG_TWO));
);
thread_local!(
    pub static entries_3: Value<Box<[Entry]>> = Rc::new(RefCell::new(Box::new([
        Entry {
            name: Ptr::<()>::from_string_literal(b"first"),
            color: Color_RED,
            opt: Option_OPT_NONE,
        },
        Entry {
            name: Ptr::<()>::from_string_literal(b"second"),
            color: Color_GREEN,
            opt: Option_OPT_A,
        },
        Entry {
            name: Ptr::<()>::from_string_literal(b"third"),
            color: Color_BLUE,
            opt: Option_OPT_C,
        },
    ])));
);
pub fn as_int_4(mut c: Color) -> () {
    return (c as ());
}
pub fn classify_option_5(mut option: ()) -> () {
    'switch: {
        match { option } {
            __v if __v == (Option_OPT_NONE as i32) => {
                return -1_();
            }
            __v if __v == (Option_OPT_A as i32) => {
                return 1_();
            }
            __v if __v == (Option_OPT_B as i32) => {
                return 2_();
            }
            __v if __v == (Option_OPT_C as i32) => {
                return 3_();
            }
            _ => {
                return 0_();
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn make_color_6(mut n: ()) -> Color {
    return ((n) as Color);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut c: Color = Color_RED;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    if ((((c as ()) == (Color_GREEN as i32)) as ()) != 0) {
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
    let mut y: () = (c as ()).wrapping_add(1_());
    unimplemented!();
    c = ((2_()) as Color);
    unimplemented!();
    unimplemented!();
    c = ({ make_color_6(1_()) });
    unimplemented!();
    let mut cmp: Color = (((c as ()).wrapping_add(1_())) as Color);
    unimplemented!();
    let mut o: Option = Option_OPT_A;
    unimplemented!();
    unimplemented!();
    let mut oi: () = (o as ());
    unimplemented!();
    o = ((20_()) as Option);
    unimplemented!();
    let mut rc: () = ({ classify_option_5((o as ())) });
    unimplemented!();
    rc = ({ classify_option_5(20_()) });
    unimplemented!();
    rc = ({ classify_option_5((Option_OPT_C as i32)) });
    unimplemented!();
    let mut t: Tag_enum = Tag_enum_TAG_ONE;
    unimplemented!();
    unimplemented!();
    let mut ti: () = (t as ());
    unimplemented!();
    t = ((2_()) as Tag_enum);
    unimplemented!();
    'switch: {
        match { (t as ()) } {
            __v if __v == (Tag_enum_TAG_ZERO as i32) => {
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
    let mut extra: () = (((Color_RED as i32) + (Color_GREEN as i32)) + (Color_BLUE as i32));
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
    let names: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::from_string_literal(b"red"),
        Ptr::<()>::from_string_literal(b"green"),
        Ptr::<()>::from_string_literal(b"blue"),
    ])));
    let mut idx: Color = Color_GREEN;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut pp: Ptr<Ptr<()>> = ((names.as_pointer() as Ptr<Ptr<()>>).offset((idx) as isize));
    unimplemented!();
    let mut pe: Ptr<Entry> =
        ((entries_3.with(|v| v.as_pointer()) as Ptr<Entry>).offset((idx) as isize));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = global_color_0.with(|_| ());
    let _ = global_opt_1.with(|_| ());
    let _ = global_tag_2.with(|_| ());
    let _ = entries_3.with(|_| ());
}
