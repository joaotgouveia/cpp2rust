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
pub type Tag_enum = u32;
pub const Tag_enum_TAG_ZERO: Tag_enum = 0;
pub const Tag_enum_TAG_ONE: Tag_enum = 1;
pub const Tag_enum_TAG_TWO: Tag_enum = 2;
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
    pub static global_tag_2: Value<Tag_enum> = Rc::new(RefCell::new(Tag_enum_TAG_TWO));
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
    {
        &(4usize);
        {
            if ((((c as u32) == ((Color_RED as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"c == RED"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        56_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((c as u32) == 0_u32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"c == 0"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        57_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((c as u32) != 1_u32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"c != 1"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        58_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    if ((((c as u32) == ((Color_GREEN as i32) as u32)) as i32) != 0) {
        return 1;
    }
    'switch: {
        match { (c as u32) } {
            __v if __v == (0 as u32) => {
                break 'switch;
            }
            __v if __v == (1 as u32) => {
                return 1;
            }
            __v if __v == (2 as u32) => {
                return 2;
            }
            _ => {
                return 99;
            }
        }
    };
    let mut x: i32 = (c as i32);
    {
        &(4usize);
        {
            if (((x == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"x == 0"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        76_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut y: i32 = (((c as u32).wrapping_add(1_u32)) as i32);
    {
        &(4usize);
        {
            if (((y == 1) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"y == 1"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        79_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    c = ((2) as Color);
    {
        &(4usize);
        {
            if ((((c as u32) == ((Color_BLUE as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"c == BLUE"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        82_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((c as u32) == 2_u32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"c == 2"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        83_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    c = ({ make_color_6(1) });
    {
        &(4usize);
        {
            if ((((c as u32) == ((Color_GREEN as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"c == GREEN"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        86_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut cmp: Color = (((c as u32).wrapping_add(1_u32)) as Color);
    {
        &(4usize);
        {
            if ((((cmp as u32) == ((Color_BLUE as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"cmp == BLUE"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        89_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut o: Option = Option_OPT_A;
    {
        &(4usize);
        {
            if ((((o as u32) == ((Option_OPT_A as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"o == OPT_A"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        92_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((o as u32) == 10_u32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"o == 10"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        93_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut oi: i32 = (o as i32);
    {
        &(4usize);
        {
            if (((oi == 10) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"oi == 10"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        96_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    o = ((20) as Option);
    {
        &(4usize);
        {
            if ((((o as u32) == ((Option_OPT_B as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"o == OPT_B"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        99_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut rc: i32 = ({ classify_option_5((o as i32)) });
    {
        &(4usize);
        {
            if (((rc == 2) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"rc == 2"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        102_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    rc = ({ classify_option_5(20) });
    {
        &(4usize);
        {
            if (((rc == 2) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"rc == 2"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        105_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    rc = ({ classify_option_5((Option_OPT_C as i32)) });
    {
        &(4usize);
        {
            if (((rc == 3) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"rc == 3"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        108_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut t: Tag_enum = Tag_enum_TAG_ONE;
    {
        &(4usize);
        {
            if ((((t as u32) == 1_u32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"t == 1"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        111_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((t as u32) == ((Tag_enum_TAG_ONE as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"t == TAG_ONE"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        112_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut ti: i32 = (t as i32);
    {
        &(4usize);
        {
            if (((ti == 1) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"ti == 1"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        115_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    t = ((2) as Tag_enum);
    {
        &(4usize);
        {
            if ((((t as u32) == ((Tag_enum_TAG_TWO as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"t == TAG_TWO"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        118_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    'switch: {
        match { (t as u32) } {
            __v if __v == ((Tag_enum_TAG_ZERO as i32) as u32) => {
                return 90;
            }
            __v if __v == (1 as u32) => {
                return 91;
            }
            __v if __v == (2 as u32) => {
                break 'switch;
            }
            _ => {}
        }
    };
    let mut extra: i32 = (((Color_RED as i32) + (Color_GREEN as i32)) + (Color_BLUE as i32));
    {
        &(4usize);
        {
            if (((extra == ((0 + 1) + 2)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"extra == 0 + 1 + 2"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        130_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((global_color_0.with(|rc| *rc.borrow()) as u32) == ((Color_GREEN as i32) as u32))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"global_color == GREEN"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        132_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((global_opt_1.with(|rc| *rc.borrow()) as u32) == ((Option_OPT_B as i32) as u32))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"global_opt == OPT_B"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        133_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((global_tag_2.with(|rc| *rc.borrow()) as u32)
                == ((Tag_enum_TAG_TWO as i32) as u32)) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"global_tag == TAG_TWO"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        134_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ (*entries_3.with(Value::clone).borrow())[(0) as usize].color } as u32)
                == ((Color_RED as i32) as u32)) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"entries[0].color == RED"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        136_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ (*entries_3.with(Value::clone).borrow())[(0) as usize].opt } as u32)
                == ((Option_OPT_NONE as i32) as u32)) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"entries[0].opt == OPT_NONE"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        137_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ (*entries_3.with(Value::clone).borrow())[(1) as usize].color } as u32)
                == ((Color_GREEN as i32) as u32)) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"entries[1].color == GREEN"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        138_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ (*entries_3.with(Value::clone).borrow())[(1) as usize].opt } as u32)
                == ((Option_OPT_A as i32) as u32)) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"entries[1].opt == OPT_A"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        139_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ (*entries_3.with(Value::clone).borrow())[(2) as usize].color } as u32)
                == ((Color_BLUE as i32) as u32)) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"entries[2].color == BLUE"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        140_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ (*entries_3.with(Value::clone).borrow())[(2) as usize].opt } as u32)
                == ((Option_OPT_C as i32) as u32)) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"entries[2].opt == OPT_C"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        141_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let names: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<i8>::from_string_literal(b"red"),
        Ptr::<i8>::from_string_literal(b"green"),
        Ptr::<i8>::from_string_literal(b"blue"),
    ])));
    let mut idx: Color = Color_GREEN;
    {
        &(4usize);
        {
            if (((((elem!((*names.borrow())[(idx) as usize], 0).read()) as i32) == ('g' as i32))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"names[idx][0] == \'g\'"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        145_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ (*entries_3.with(Value::clone).borrow())[(idx) as usize].opt } as u32)
                == ((Option_OPT_A as i32) as u32)) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"entries[idx].opt == OPT_A"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        146_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((elem!(
                (*names.borrow())[(global_tag_2.with(|rc| *rc.borrow())) as usize],
                0
            )
            .read()) as i32)
                == ('b' as i32)) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"names[global_tag][0] == \'b\'"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        147_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut pp: Ptr<Ptr<i8>> = ((names.as_pointer() as Ptr<Ptr<i8>>).offset((idx) as isize));
    {
        &(4usize);
        {
            if (((((elem!((pp.read()), 0).read()) as i32) == ('g' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"(*pp)[0] == \'g\'"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        150_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut pe: Ptr<Entry> =
        ((entries_3.with(|v| v.as_pointer()) as Ptr<Entry>).offset((idx) as isize));
    {
        &(4usize);
        {
            if ((((pe.with(|__s| __s.opt) as u32) == ((Option_OPT_A as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_7(
                        Ptr::<i8>::from_string_literal(b"pe->opt == OPT_A"),
                        Ptr::<i8>::from_string_literal(b"enum_int_interop_c.c"),
                        152_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
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
