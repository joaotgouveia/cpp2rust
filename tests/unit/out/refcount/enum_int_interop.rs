extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Color = u32;
pub const Color_RED: Color = 0;
pub const Color_GREEN: Color = 1;
pub const Color_BLUE: Color = 2;
pub type Option = u32;
pub const Option_OPT_NONE: Option = 0;
pub const Option_OPT_A: Option = 10;
pub const Option_OPT_B: Option = 20;
pub const Option_OPT_C: Option = 30;
pub type Tag = u32;
pub const Tag_TAG_ZERO: Tag = 0;
pub const Tag_TAG_ONE: Tag = 1;
pub const Tag_TAG_TWO: Tag = 2;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Entry {
    #[offset(0)]
    #[byte_size(8)]
    pub name: Ptr<i8>,
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
    pub static global_tag_2: Value<Tag> = Rc::new(RefCell::new(Tag_TAG_TWO));
);
thread_local!(
    pub static entries_3: Value<Box<[Entry]>> = Rc::new(RefCell::new(Box::new([
        Entry {
            name: Ptr::<i8>::from_string_literal(b"first"),
            color: Color_RED,
            opt: Option_OPT_NONE,
        },
        Entry {
            name: Ptr::<i8>::from_string_literal(b"second"),
            color: Color_GREEN,
            opt: Option_OPT_A,
        },
        Entry {
            name: Ptr::<i8>::from_string_literal(b"third"),
            color: Color_BLUE,
            opt: Option_OPT_C,
        },
    ])));
);
pub fn as_int_4(mut c: Color) -> i32 {
    return (c as i32);
}
pub fn classify_option_5(mut option: i32) -> i32 {
    'switch: {
        match { option } {
            __v if __v == (Option_OPT_NONE as i32) => {
                return -1_i32;
            }
            __v if __v == (Option_OPT_A as i32) => {
                return 1;
            }
            __v if __v == (Option_OPT_B as i32) => {
                return 2;
            }
            __v if __v == (Option_OPT_C as i32) => {
                return 3;
            }
            _ => {
                return 0;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn make_color_6(mut n: i32) -> Color {
    return ((n) as Color);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut c: Color = Color_RED;
    (if ((c as i32) == (Color_RED as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"c == RED"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((c as i32) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"c == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((c as i32) != 1) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"c != 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    if ((c as i32) == (Color_GREEN as i32)) {
        return 1;
    }
    'switch: {
        match { (c as i32) } {
            __v if __v == 0 => {
                break 'switch;
            }
            __v if __v == 1 => {
                return 1;
            }
            __v if __v == 2 => {
                return 2;
            }
            _ => {
                return 99;
            }
        }
    };
    let mut x: i32 = (c as i32);
    (if (x == 0) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"x == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut y: i32 = ((c as i32) + 1);
    (if (y == 1) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"y == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    c = ((2) as Color);
    (if ((c as i32) == (Color_BLUE as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"c == BLUE"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((c as i32) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"c == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    c = ({ make_color_6(1) });
    (if ((c as i32) == (Color_GREEN as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"c == GREEN"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut cmp: Color = (((c as i32) + 1) as Color);
    (if ((cmp as i32) == (Color_BLUE as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"cmp == BLUE"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut o: Option = Option_OPT_A;
    (if ((o as i32) == (Option_OPT_A as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"o == OPT_A"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((o as i32) == 10) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"o == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut oi: i32 = (o as i32);
    (if (oi == 10) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"oi == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    o = ((20) as Option);
    (if ((o as i32) == (Option_OPT_B as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"o == OPT_B"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut rc: i32 = ({ classify_option_5((o as i32)) });
    (if (rc == 2) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"rc == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    rc = ({ classify_option_5(20) });
    (if (rc == 2) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"rc == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    rc = ({ classify_option_5((Option_OPT_C as i32)) });
    (if (rc == 3) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"rc == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut t: Tag = Tag_TAG_ONE;
    (if ((t as i32) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"t == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((t as i32) == (Tag_TAG_ONE as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"t == TAG_ONE"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut ti: i32 = (t as i32);
    (if (ti == 1) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"ti == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    t = ((2) as Tag);
    (if ((t as i32) == (Tag_TAG_TWO as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"t == TAG_TWO"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    'switch: {
        match { (t as i32) } {
            __v if __v == (Tag_TAG_ZERO as i32) => {
                return 90;
            }
            __v if __v == 1 => {
                return 91;
            }
            __v if __v == 2 => {
                break 'switch;
            }
            _ => {}
        }
    };
    let mut extra: i32 = (((Color_RED as i32) + (Color_GREEN as i32)) + (Color_BLUE as i32));
    (if (extra == ((0 + 1) + 2)) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"extra == 0 + 1 + 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((global_color_0.with(|rc| *rc.borrow()) as i32) == (Color_GREEN as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"global_color == GREEN"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((global_opt_1.with(|rc| *rc.borrow()) as i32) == (Option_OPT_B as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"global_opt == OPT_B"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((global_tag_2.with(|rc| *rc.borrow()) as i32) == (Tag_TAG_TWO as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"global_tag == TAG_TWO"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ (*entries_3.with(Value::clone).borrow())[(0) as usize].color } as i32)
        == (Color_RED as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"entries[0].color == RED"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ (*entries_3.with(Value::clone).borrow())[(0) as usize].opt } as i32)
        == (Option_OPT_NONE as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"entries[0].opt == OPT_NONE"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ (*entries_3.with(Value::clone).borrow())[(1) as usize].color } as i32)
        == (Color_GREEN as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"entries[1].color == GREEN"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ (*entries_3.with(Value::clone).borrow())[(1) as usize].opt } as i32)
        == (Option_OPT_A as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"entries[1].opt == OPT_A"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ (*entries_3.with(Value::clone).borrow())[(2) as usize].color } as i32)
        == (Color_BLUE as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"entries[2].color == BLUE"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ (*entries_3.with(Value::clone).borrow())[(2) as usize].opt } as i32)
        == (Option_OPT_C as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"entries[2].opt == OPT_C"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_7(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {
    let _ = global_color_0.with(|_| ());
    let _ = global_opt_1.with(|_| ());
    let _ = global_tag_2.with(|_| ());
    let _ = entries_3.with(|_| ());
}
