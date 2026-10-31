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
thread_local!(
    pub static regex_error_internal_0: Value<()> = Rc::new(RefCell::new(((-1_i32) as ())));
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
            write_int_refcount(&__s, (unimplemented!() as i32));
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
            write_int_refcount(&__s, (unimplemented!() as i32));
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
            write_int_refcount(&__s, (unimplemented!() as i32));
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
            write_int_refcount(&__s, (unimplemented!() as i32));
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
            write_int_refcount(&__s, (unimplemented!() as i32));
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
            write_int_refcount(&__s, (unimplemented!() as i32));
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
            write_int_refcount(&__s, (unimplemented!() as i32));
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
            write_int_refcount(&__s, (unimplemented!() as i32));
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
            write_int_refcount(&__s, (unimplemented!() as i32));
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
            write_int_refcount(&__s, (unimplemented!() as i32));
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
            write_int_refcount(&__s, (unimplemented!() as i32));
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
            write_int_refcount(&__s, (unimplemented!() as i32));
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
            write_int_refcount(&__s, (unimplemented!() as i32));
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
}
pub fn regex_sanity_check_3(__res: Ptr<Vec<i8>>, __f: Option<()>, __error: Option<()>) {
    let mut __f: () = __f.unwrap_or_else(|| unimplemented!());
    let mut __error: () = __error.unwrap_or_else(|| regex_error_internal_0.with(|rc| *rc.borrow()));
    let reo: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut n: u32 = unimplemented!();
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
pub fn test01_4() {
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
    let re0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let re1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let re2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ regex_sanity_check_3(sre0.as_pointer(), Some(unimplemented!()), None) });
    ({ regex_sanity_check_3(sre1.as_pointer(), None, None) });
    ({ regex_sanity_check_3(sre2.as_pointer(), None, None) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_4() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = regex_error_internal_0.with(|_| ());
    let _ = regex_error_codes_1.with(|_| ());
}
