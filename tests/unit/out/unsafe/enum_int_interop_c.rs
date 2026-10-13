extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Entry {
    pub name: *const libc::c_char,
    pub color: Color,
    pub opt: Option,
}
pub static mut global_color_0: std::cell::LazyCell<Color> =
    std::cell::LazyCell::new(|| unsafe { Color_GREEN });
pub static mut global_opt_1: std::cell::LazyCell<Option> =
    std::cell::LazyCell::new(|| unsafe { Option_OPT_B });
pub static mut global_tag_2: std::cell::LazyCell<Tag_enum> =
    std::cell::LazyCell::new(|| unsafe { Tag_enum_TAG_TWO });
pub static mut entries_3: std::cell::LazyCell<[Entry; 3]> = std::cell::LazyCell::new(|| unsafe {
    [
        Entry {
            name: (c"first".as_ptr().cast_mut()).cast_const(),
            color: Color_RED,
            opt: Option_OPT_NONE,
        },
        Entry {
            name: (c"second".as_ptr().cast_mut()).cast_const(),
            color: Color_GREEN,
            opt: Option_OPT_A,
        },
        Entry {
            name: (c"third".as_ptr().cast_mut()).cast_const(),
            color: Color_BLUE,
            opt: Option_OPT_C,
        },
    ]
});
pub unsafe fn as_int_4(mut c: Color) -> i32 {
    return (c as i32);
}
pub unsafe fn classify_option_5(mut option: i32) -> i32 {
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
pub unsafe fn make_color_6(mut n: i32) -> Color {
    return ((n) as Color);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: Color = Color_RED;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c as u32) == ((Color_RED as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"c == RED".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        56_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c as u32) == (0_u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"c == 0".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        57_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c as u32) != (1_u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"c != 1".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        58_u32,
                        c"int main()".as_ptr(),
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
        &(::std::mem::size_of::<i32>());
        {
            if ((((x) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"x == 0".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        76_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut y: i32 = (((c as u32).wrapping_add(1_u32)) as i32);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((y) == (1)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"y == 1".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        79_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    c = ((2) as Color);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c as u32) == ((Color_BLUE as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"c == BLUE".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        82_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c as u32) == (2_u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"c == 2".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        83_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    c = (unsafe { make_color_6(1) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c as u32) == ((Color_GREEN as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"c == GREEN".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        86_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut cmp: Color = (((c as u32).wrapping_add(1_u32)) as Color);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((cmp as u32) == ((Color_BLUE as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"cmp == BLUE".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        89_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut o: Option = Option_OPT_A;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((o as u32) == ((Option_OPT_A as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"o == OPT_A".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        92_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((o as u32) == (10_u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"o == 10".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        93_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut oi: i32 = (o as i32);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((oi) == (10)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"oi == 10".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        96_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    o = ((20) as Option);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((o as u32) == ((Option_OPT_B as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"o == OPT_B".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        99_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut rc: i32 = (unsafe { classify_option_5((o as i32)) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((rc) == (2)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"rc == 2".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        102_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    rc = (unsafe { classify_option_5(20) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((rc) == (2)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"rc == 2".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        105_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    rc = (unsafe { classify_option_5((Option_OPT_C as i32)) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((rc) == (3)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"rc == 3".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        108_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut t: Tag_enum = Tag_enum_TAG_ONE;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((t as u32) == (1_u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"t == 1".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        111_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((t as u32) == ((Tag_enum_TAG_ONE as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"t == TAG_ONE".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        112_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut ti: i32 = (t as i32);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((ti) == (1)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"ti == 1".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        115_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    t = ((2) as Tag_enum);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((t as u32) == ((Tag_enum_TAG_TWO as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"t == TAG_TWO".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        118_u32,
                        c"int main()".as_ptr(),
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
        &(::std::mem::size_of::<i32>());
        {
            if ((((extra) == (((0) + (1)) + (2))) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"extra == 0 + 1 + 2".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        130_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*std::cell::LazyCell::force_mut(&mut *&raw mut global_color_0)) as u32)
                == ((Color_GREEN as i32) as u32)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"global_color == GREEN".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        132_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*std::cell::LazyCell::force_mut(&mut *&raw mut global_opt_1)) as u32)
                == ((Option_OPT_B as i32) as u32)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"global_opt == OPT_B".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        133_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*std::cell::LazyCell::force_mut(&mut *&raw mut global_tag_2)) as u32)
                == ((Tag_enum_TAG_TWO as i32) as u32)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"global_tag == TAG_TWO".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        134_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*std::cell::LazyCell::force_mut(&mut *&raw mut entries_3))[(0) as usize].color
                as u32)
                == ((Color_RED as i32) as u32)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"entries[0].color == RED".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        136_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*std::cell::LazyCell::force_mut(&mut *&raw mut entries_3))[(0) as usize].opt
                as u32)
                == ((Option_OPT_NONE as i32) as u32)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"entries[0].opt == OPT_NONE".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        137_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*std::cell::LazyCell::force_mut(&mut *&raw mut entries_3))[(1) as usize].color
                as u32)
                == ((Color_GREEN as i32) as u32)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"entries[1].color == GREEN".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        138_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*std::cell::LazyCell::force_mut(&mut *&raw mut entries_3))[(1) as usize].opt
                as u32)
                == ((Option_OPT_A as i32) as u32)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"entries[1].opt == OPT_A".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        139_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*std::cell::LazyCell::force_mut(&mut *&raw mut entries_3))[(2) as usize].color
                as u32)
                == ((Color_BLUE as i32) as u32)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"entries[2].color == BLUE".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        140_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*std::cell::LazyCell::force_mut(&mut *&raw mut entries_3))[(2) as usize].opt
                as u32)
                == ((Option_OPT_C as i32) as u32)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"entries[2].opt == OPT_C".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        141_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut names: [*const libc::c_char; 3] = [
        (c"red".as_ptr().cast_mut()).cast_const(),
        (c"green".as_ptr().cast_mut()).cast_const(),
        (c"blue".as_ptr().cast_mut()).cast_const(),
    ];
    let mut idx: Color = Color_GREEN;
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*names[(idx) as usize].offset((0) as isize)) as i32) == ('g' as i32)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"names[idx][0] == \'g\'".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        145_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*std::cell::LazyCell::force_mut(&mut *&raw mut entries_3))[(idx) as usize].opt
                as u32)
                == ((Option_OPT_A as i32) as u32)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"entries[idx].opt == OPT_A".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        146_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*names[(*std::cell::LazyCell::force_mut(&mut *&raw mut global_tag_2)) as usize]
                .offset((0) as isize)) as i32)
                == ('b' as i32)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"names[global_tag][0] == \'b\'".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        147_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut pp: *mut *const libc::c_char = (&mut names[(idx) as usize] as *mut *const libc::c_char);
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*(*pp).offset((0) as isize)) as i32) == ('g' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"(*pp)[0] == \'g\'".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        150_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut pe: *mut Entry = (&mut (*std::cell::LazyCell::force_mut(&mut *&raw mut entries_3))
        [(idx) as usize] as *mut Entry);
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*pe).opt as u32) == ((Option_OPT_A as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_7(
                        (c"pe->opt == OPT_A".as_ptr().cast_mut()).cast_const(),
                        (c"enum_int_interop_c.c".as_ptr().cast_mut()).cast_const(),
                        152_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
pub unsafe fn __assert_fail_7(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const global_color_0);
    std::cell::LazyCell::force(&*&raw const global_opt_1);
    std::cell::LazyCell::force(&*&raw const global_tag_2);
    std::cell::LazyCell::force(&*&raw const entries_3);
}
