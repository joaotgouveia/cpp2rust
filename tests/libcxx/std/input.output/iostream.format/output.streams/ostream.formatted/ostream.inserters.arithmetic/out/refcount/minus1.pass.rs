extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_octal_0(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::oct_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as u16));
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_0,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_0.upgrade().deref())
                        [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_octal_1(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::oct_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as i16));
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_1,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_1.upgrade().deref())
                        [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_octal_2(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::oct_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as u32));
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_2,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_2.upgrade().deref())
                        [..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_octal_3(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::oct_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as i32));
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_3,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_3.upgrade().deref())
                        [..(*__tmp_3.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_octal_4(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::oct_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as u64));
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_4,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_4.upgrade().deref())
                        [..(*__tmp_4.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_octal_5(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::oct_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as i64));
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_5,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_5.upgrade().deref())
                        [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_octal_6(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::oct_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as u64));
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_6,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_6.upgrade().deref())
                        [..(*__tmp_6.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_octal_7(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::oct_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as i64));
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_7,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_7.upgrade().deref())
                        [..(*__tmp_7.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_dec_8(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::dec_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as u16));
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_8,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_8.upgrade().deref())
                        [..(*__tmp_8.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_dec_9(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::dec_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as i16));
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_9,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_9.upgrade().deref())
                        [..(*__tmp_9.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_dec_10(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::dec_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as u32));
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_10,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_10.upgrade().deref())
                        [..(*__tmp_10.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_dec_11(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::dec_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as i32));
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_11,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_11.upgrade().deref())
                        [..(*__tmp_11.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_dec_12(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::dec_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as u64));
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_12,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_12.upgrade().deref())
                        [..(*__tmp_12.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_dec_13(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::dec_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as i64));
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_13,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_13.upgrade().deref())
                        [..(*__tmp_13.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_dec_14(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::dec_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as u64));
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_14,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_14.upgrade().deref())
                        [..(*__tmp_14.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_dec_15(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::dec_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as i64));
        __s
    };
    assert!(
        match (Rc::new(RefCell::new(({ (*ss.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_15,) => {
                expected.with_c_str(|__s| {
                    (*__tmp_15.upgrade().deref())
                        [..(*__tmp_15.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test_hex_16(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::hex_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as u16));
        __s
    };
    let str: Value<Vec<i8>> = Rc::new(RefCell::new(({ (*ss.borrow()).str_16() })));
    let mut i: usize = 0_usize;
    'loop_: while (i < ((*str.borrow()).len() - 1)) {
        let __rhs = (match ({ ((str.decay().offset(i as isize).read()) as i32) },) {
            (mut __tmp_16,) => {
                if (0..=255).contains(&__tmp_16) {
                    (__tmp_16 as u8).to_ascii_uppercase() as i32
                } else {
                    __tmp_16
                }
            }
        } as i8);
        str.decay().offset(i as isize).write(__rhs);
        i.prefix_inc();
    }
    assert!(
        ({
            expected.with_c_str(|__s| {
                (*str.borrow())[..(*str.borrow()).len().saturating_sub(1)] == *__s
            })
        })
    );
}
pub fn test_hex_17(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::hex_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as i16));
        __s
    };
    let str: Value<Vec<i8>> = Rc::new(RefCell::new(({ (*ss.borrow()).str_16() })));
    let mut i: usize = 0_usize;
    'loop_: while (i < ((*str.borrow()).len() - 1)) {
        let __rhs = (match ({ ((str.decay().offset(i as isize).read()) as i32) },) {
            (mut __tmp_17,) => {
                if (0..=255).contains(&__tmp_17) {
                    (__tmp_17 as u8).to_ascii_uppercase() as i32
                } else {
                    __tmp_17
                }
            }
        } as i8);
        str.decay().offset(i as isize).write(__rhs);
        i.prefix_inc();
    }
    assert!(
        ({
            expected.with_c_str(|__s| {
                (*str.borrow())[..(*str.borrow()).len().saturating_sub(1)] == *__s
            })
        })
    );
}
pub fn test_hex_18(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::hex_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as u32));
        __s
    };
    let str: Value<Vec<i8>> = Rc::new(RefCell::new(({ (*ss.borrow()).str_16() })));
    let mut i: usize = 0_usize;
    'loop_: while (i < ((*str.borrow()).len() - 1)) {
        let __rhs = (match ({ ((str.decay().offset(i as isize).read()) as i32) },) {
            (mut __tmp_18,) => {
                if (0..=255).contains(&__tmp_18) {
                    (__tmp_18 as u8).to_ascii_uppercase() as i32
                } else {
                    __tmp_18
                }
            }
        } as i8);
        str.decay().offset(i as isize).write(__rhs);
        i.prefix_inc();
    }
    assert!(
        ({
            expected.with_c_str(|__s| {
                (*str.borrow())[..(*str.borrow()).len().saturating_sub(1)] == *__s
            })
        })
    );
}
pub fn test_hex_19(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::hex_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as i32));
        __s
    };
    let str: Value<Vec<i8>> = Rc::new(RefCell::new(({ (*ss.borrow()).str_16() })));
    let mut i: usize = 0_usize;
    'loop_: while (i < ((*str.borrow()).len() - 1)) {
        let __rhs = (match ({ ((str.decay().offset(i as isize).read()) as i32) },) {
            (mut __tmp_19,) => {
                if (0..=255).contains(&__tmp_19) {
                    (__tmp_19 as u8).to_ascii_uppercase() as i32
                } else {
                    __tmp_19
                }
            }
        } as i8);
        str.decay().offset(i as isize).write(__rhs);
        i.prefix_inc();
    }
    assert!(
        ({
            expected.with_c_str(|__s| {
                (*str.borrow())[..(*str.borrow()).len().saturating_sub(1)] == *__s
            })
        })
    );
}
pub fn test_hex_20(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::hex_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as u64));
        __s
    };
    let str: Value<Vec<i8>> = Rc::new(RefCell::new(({ (*ss.borrow()).str_16() })));
    let mut i: usize = 0_usize;
    'loop_: while (i < ((*str.borrow()).len() - 1)) {
        let __rhs = (match ({ ((str.decay().offset(i as isize).read()) as i32) },) {
            (mut __tmp_20,) => {
                if (0..=255).contains(&__tmp_20) {
                    (__tmp_20 as u8).to_ascii_uppercase() as i32
                } else {
                    __tmp_20
                }
            }
        } as i8);
        str.decay().offset(i as isize).write(__rhs);
        i.prefix_inc();
    }
    assert!(
        ({
            expected.with_c_str(|__s| {
                (*str.borrow())[..(*str.borrow()).len().saturating_sub(1)] == *__s
            })
        })
    );
}
pub fn test_hex_21(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::hex_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as i64));
        __s
    };
    let str: Value<Vec<i8>> = Rc::new(RefCell::new(({ (*ss.borrow()).str_16() })));
    let mut i: usize = 0_usize;
    'loop_: while (i < ((*str.borrow()).len() - 1)) {
        let __rhs = (match ({ ((str.decay().offset(i as isize).read()) as i32) },) {
            (mut __tmp_21,) => {
                if (0..=255).contains(&__tmp_21) {
                    (__tmp_21 as u8).to_ascii_uppercase() as i32
                } else {
                    __tmp_21
                }
            }
        } as i8);
        str.decay().offset(i as isize).write(__rhs);
        i.prefix_inc();
    }
    assert!(
        ({
            expected.with_c_str(|__s| {
                (*str.borrow())[..(*str.borrow()).len().saturating_sub(1)] == *__s
            })
        })
    );
}
pub fn test_hex_22(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::hex_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as u64));
        __s
    };
    let str: Value<Vec<i8>> = Rc::new(RefCell::new(({ (*ss.borrow()).str_16() })));
    let mut i: usize = 0_usize;
    'loop_: while (i < ((*str.borrow()).len() - 1)) {
        let __rhs = (match ({ ((str.decay().offset(i as isize).read()) as i32) },) {
            (mut __tmp_22,) => {
                if (0..=255).contains(&__tmp_22) {
                    (__tmp_22 as u8).to_ascii_uppercase() as i32
                } else {
                    __tmp_22
                }
            }
        } as i8);
        str.decay().offset(i as isize).write(__rhs);
        i.prefix_inc();
    }
    assert!(
        ({
            expected.with_c_str(|__s| {
                (*str.borrow())[..(*str.borrow()).len().saturating_sub(1)] == *__s
            })
        })
    );
}
pub fn test_hex_23(mut expected: Ptr<i8>) {
    let ss: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    {
        let __s = {
            let __s = (ss.as_pointer() as ::std::fs::File);
            libcc2rs::hex_refcount.call(__s)
        };
        write_int_refcount(&__s, (-1_i32 as i64));
        __s
    };
    let str: Value<Vec<i8>> = Rc::new(RefCell::new(({ (*ss.borrow()).str_16() })));
    let mut i: usize = 0_usize;
    'loop_: while (i < ((*str.borrow()).len() - 1)) {
        let __rhs = (match ({ ((str.decay().offset(i as isize).read()) as i32) },) {
            (mut __tmp_23,) => {
                if (0..=255).contains(&__tmp_23) {
                    (__tmp_23 as u8).to_ascii_uppercase() as i32
                } else {
                    __tmp_23
                }
            }
        } as i8);
        str.decay().offset(i as isize).write(__rhs);
        i.prefix_inc();
    }
    assert!(
        ({
            expected.with_c_str(|__s| {
                (*str.borrow())[..(*str.borrow()).len().saturating_sub(1)] == *__s
            })
        })
    );
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_octal_0(Ptr::<i8>::from_string_literal(b"177777")) });
    ({ test_octal_1(Ptr::<i8>::from_string_literal(b"177777")) });
    ({ test_octal_2(Ptr::<i8>::from_string_literal(b"37777777777")) });
    ({ test_octal_3(Ptr::<i8>::from_string_literal(b"37777777777")) });
    ({ test_octal_4(Ptr::<i8>::from_string_literal(b"1777777777777777777777")) });
    ({ test_octal_5(Ptr::<i8>::from_string_literal(b"1777777777777777777777")) });
    ({ test_octal_4(Ptr::<i8>::from_string_literal(b"1777777777777777777777")) });
    let mut long_is_64: bool = value_24.with(|rc| *rc.borrow());
    let mut long_long_is_64: bool = value_24.with(|rc| *rc.borrow());
    {
        ({ test_octal_4(Ptr::<i8>::from_string_literal(b"1777777777777777777777")) });
        ({ test_octal_5(Ptr::<i8>::from_string_literal(b"1777777777777777777777")) });
    }
    {
        ({ test_octal_6(Ptr::<i8>::from_string_literal(b"1777777777777777777777")) });
        ({ test_octal_7(Ptr::<i8>::from_string_literal(b"1777777777777777777777")) });
    }
    ({ test_dec_8(Ptr::<i8>::from_string_literal(b"65535")) });
    ({ test_dec_9(Ptr::<i8>::from_string_literal(b"-1")) });
    ({ test_dec_10(Ptr::<i8>::from_string_literal(b"4294967295")) });
    ({ test_dec_11(Ptr::<i8>::from_string_literal(b"-1")) });
    ({ test_dec_12(Ptr::<i8>::from_string_literal(b"18446744073709551615")) });
    ({ test_dec_13(Ptr::<i8>::from_string_literal(b"-1")) });
    {
        ({ test_dec_12(Ptr::<i8>::from_string_literal(b"18446744073709551615")) });
        ({ test_dec_13(Ptr::<i8>::from_string_literal(b"-1")) });
    }
    {
        ({ test_dec_14(Ptr::<i8>::from_string_literal(b"18446744073709551615")) });
        ({ test_dec_15(Ptr::<i8>::from_string_literal(b"-1")) });
    }
    ({ test_hex_16(Ptr::<i8>::from_string_literal(b"FFFF")) });
    ({ test_hex_17(Ptr::<i8>::from_string_literal(b"FFFF")) });
    ({ test_hex_18(Ptr::<i8>::from_string_literal(b"FFFFFFFF")) });
    ({ test_hex_19(Ptr::<i8>::from_string_literal(b"FFFFFFFF")) });
    ({ test_hex_20(Ptr::<i8>::from_string_literal(b"FFFFFFFFFFFFFFFF")) });
    ({ test_hex_21(Ptr::<i8>::from_string_literal(b"FFFFFFFFFFFFFFFF")) });
    {
        ({ test_hex_20(Ptr::<i8>::from_string_literal(b"FFFFFFFFFFFFFFFF")) });
        ({ test_hex_21(Ptr::<i8>::from_string_literal(b"FFFFFFFFFFFFFFFF")) });
    }
    {
        ({ test_hex_22(Ptr::<i8>::from_string_literal(b"FFFFFFFFFFFFFFFF")) });
        ({ test_hex_23(Ptr::<i8>::from_string_literal(b"FFFFFFFFFFFFFFFF")) });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
