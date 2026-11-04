extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_regex_expected_fail {}
impl CppMove for __gnu_test_regex_expected_fail {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static regex_error_internal_0: Value<std_regex_constants_error_type> =
        Rc::new(RefCell::new(((-1_i32) as std_regex_constants_error_type)));
);
thread_local!(
    pub static regex_error_codes_1: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<i8>::from_string_literal(b"error_collate"),
        Ptr::<i8>::from_string_literal(b"error_ctype"),
        Ptr::<i8>::from_string_literal(b"error_escape"),
        Ptr::<i8>::from_string_literal(b"error_backref"),
        Ptr::<i8>::from_string_literal(b"error_brack"),
        Ptr::<i8>::from_string_literal(b"error_paren"),
        Ptr::<i8>::from_string_literal(b"error_brace"),
        Ptr::<i8>::from_string_literal(b"error_badbrace"),
        Ptr::<i8>::from_string_literal(b"error_range"),
        Ptr::<i8>::from_string_literal(b"error_space"),
        Ptr::<i8>::from_string_literal(b"error_badrepeat"),
        Ptr::<i8>::from_string_literal(b"error_complexity"),
        Ptr::<i8>::from_string_literal(b"error_stack"),
    ])));
);
pub fn show_regex_error_codes_2() {
    let mut tab: i8 = ('\t' as i8);
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cout();
                    let _ =
                        Ptr::<i8>::from_string_literal(b"error_collate =   ").with_c_str(|__cs| {
                            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                        });
                    __s
                };
                let _ = __s.write_all(&[tab as u8]);
                __s
            };
            write_int_refcount(&__s, (error_collate_3.with(|rc| *rc.borrow()) as i32));
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cout();
                    let _ =
                        Ptr::<i8>::from_string_literal(b"error_ctype =     ").with_c_str(|__cs| {
                            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                        });
                    __s
                };
                let _ = __s.write_all(&[tab as u8]);
                __s
            };
            write_int_refcount(&__s, (error_ctype_4.with(|rc| *rc.borrow()) as i32));
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cout();
                    let _ =
                        Ptr::<i8>::from_string_literal(b"error_escape =    ").with_c_str(|__cs| {
                            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                        });
                    __s
                };
                let _ = __s.write_all(&[tab as u8]);
                __s
            };
            write_int_refcount(&__s, (error_escape_5.with(|rc| *rc.borrow()) as i32));
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cout();
                    let _ =
                        Ptr::<i8>::from_string_literal(b"error_backref =   ").with_c_str(|__cs| {
                            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                        });
                    __s
                };
                let _ = __s.write_all(&[tab as u8]);
                __s
            };
            write_int_refcount(&__s, (error_backref_6.with(|rc| *rc.borrow()) as i32));
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cout();
                    let _ =
                        Ptr::<i8>::from_string_literal(b"error_brack =     ").with_c_str(|__cs| {
                            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                        });
                    __s
                };
                let _ = __s.write_all(&[tab as u8]);
                __s
            };
            write_int_refcount(&__s, (error_brack_7.with(|rc| *rc.borrow()) as i32));
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cout();
                    let _ =
                        Ptr::<i8>::from_string_literal(b"error_paren =     ").with_c_str(|__cs| {
                            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                        });
                    __s
                };
                let _ = __s.write_all(&[tab as u8]);
                __s
            };
            write_int_refcount(&__s, (error_paren_8.with(|rc| *rc.borrow()) as i32));
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cout();
                    let _ =
                        Ptr::<i8>::from_string_literal(b"error_brace =     ").with_c_str(|__cs| {
                            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                        });
                    __s
                };
                let _ = __s.write_all(&[tab as u8]);
                __s
            };
            write_int_refcount(&__s, (error_brace_9.with(|rc| *rc.borrow()) as i32));
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cout();
                    let _ =
                        Ptr::<i8>::from_string_literal(b"error_badbrace =  ").with_c_str(|__cs| {
                            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                        });
                    __s
                };
                let _ = __s.write_all(&[tab as u8]);
                __s
            };
            write_int_refcount(&__s, (error_badbrace_10.with(|rc| *rc.borrow()) as i32));
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cout();
                    let _ =
                        Ptr::<i8>::from_string_literal(b"error_range =     ").with_c_str(|__cs| {
                            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                        });
                    __s
                };
                let _ = __s.write_all(&[tab as u8]);
                __s
            };
            write_int_refcount(&__s, (error_range_11.with(|rc| *rc.borrow()) as i32));
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cout();
                    let _ =
                        Ptr::<i8>::from_string_literal(b"error_space =     ").with_c_str(|__cs| {
                            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                        });
                    __s
                };
                let _ = __s.write_all(&[tab as u8]);
                __s
            };
            write_int_refcount(&__s, (error_space_12.with(|rc| *rc.borrow()) as i32));
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cout();
                    let _ =
                        Ptr::<i8>::from_string_literal(b"error_badrepeat = ").with_c_str(|__cs| {
                            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                        });
                    __s
                };
                let _ = __s.write_all(&[tab as u8]);
                __s
            };
            write_int_refcount(&__s, (error_badrepeat_13.with(|rc| *rc.borrow()) as i32));
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cout();
                    let _ =
                        Ptr::<i8>::from_string_literal(b"error_complexity =").with_c_str(|__cs| {
                            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                        });
                    __s
                };
                let _ = __s.write_all(&[tab as u8]);
                __s
            };
            write_int_refcount(&__s, (error_complexity_14.with(|rc| *rc.borrow()) as i32));
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cout();
                    let _ =
                        Ptr::<i8>::from_string_literal(b"error_stack =     ").with_c_str(|__cs| {
                            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                        });
                    __s
                };
                let _ = __s.write_all(&[tab as u8]);
                __s
            };
            write_int_refcount(&__s, (error_stack_15.with(|rc| *rc.borrow()) as i32));
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
}
pub fn regex_sanity_check_16(
    __res: Ptr<Vec<i8>>,
    __f: Option<std_regex_constants_syntax_option_type>,
    __error: Option<std_regex_constants_error_type>,
) {
    let mut __f: std_regex_constants_syntax_option_type =
        __f.unwrap_or_else(|| basic_17.with(|rc| *rc.borrow()));
    let mut __error: std_regex_constants_error_type =
        __error.unwrap_or_else(|| regex_error_internal_0.with(|rc| *rc.borrow()));
    let reo: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
        std_basic_regex_char__std_regex_traits_char__::new_1(
            { (__res).reinterpret_cast::<Vec<i8>>() },
            { __f },
        ),
    ));
    let mut n: u32 = ({ (*reo.borrow()).mark_count() });
    {
        let __s = {
            let __s = {
                let __s = libcc2rs::cout();
                let _ =
                    Ptr::<i8>::from_string_literal(b"regex_type::mark_count ").with_c_str(|__cs| {
                        __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                    });
                __s
            };
            write_int_refcount(&__s, n);
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
}
pub fn test01_18() {
    let sre0: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"\\d{3}-\\d{3}-\\d{4}").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let sre1: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes =
            Ptr::<i8>::from_string_literal(b" this\n  and new : forms\n  ").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let sre2: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes =
            Ptr::<i8>::from_string_literal(b"[:num:]{3}-[:num:]{3}-[:num:]{4}").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let re0: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
        std_basic_regex_char__std_regex_traits_char__::new_2(
            { Ptr::<i8>::from_string_literal(b"\\d{3}-\\d{3}-\\d{4}") },
            { ECMAScript_19.with(|rc| *rc.borrow()) },
        ),
    ));
    let re1: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
        std_basic_regex_char__std_regex_traits_char__::new_2(
            { Ptr::<i8>::from_string_literal(b" this\n  and new : forms\n  ") },
            { basic_17.with(|rc| *rc.borrow()) },
        ),
    ));
    let re2: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
        std_basic_regex_char__std_regex_traits_char__::new_2(
            { Ptr::<i8>::from_string_literal(b"[:num:]{3}-[:num:]{3}-[:num:]{4}") },
            { basic_17.with(|rc| *rc.borrow()) },
        ),
    ));
    ({
        regex_sanity_check_16(
            sre0.as_pointer(),
            Some(ECMAScript_19.with(|rc| *rc.borrow())),
            None,
        )
    });
    ({ regex_sanity_check_16(sre1.as_pointer(), None, None) });
    ({ regex_sanity_check_16(sre2.as_pointer(), None, None) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_18() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = regex_error_internal_0.with(|_| ());
    let _ = regex_error_codes_1.with(|_| ());
}
