extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let vec_: Value<Vec<u8>> = Rc::new(RefCell::new(Vec::from([195_u8, 167_u8])));
    let mut i: i32 = 27;
    let mut str: Vec<i8> = {
        let mut __bytes = Ptr::<i8>::from_string_literal(b"bar.").to_c_bytes();
        __bytes.push(0);
        __bytes
    };
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = {
                        let __s = {
                            let __s = {
                                let __s = {
                                    let __s = {
                                        let __s = {
                                            let __s = {
                                                let __s = {
                                                    let __s = {
                                                        let __s = {
                                                            let __s = {
                                                                let __s = {
                                                                    let __s = {
                                                                        let __s = libcc2rs::cerr();
                                                                        write_int_refcount(&__s, i);
                                                                        __s
                                                                    };
                                                                    let _ = Ptr::<i8 >::from_string_literal(b" a" ) .with_c_str(|__cs| __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>()));
                                                                    __s
                                                                };
                                                                let _ = __s.write_all(&[(vec_
                                                                    .decay()
                                                                    .offset(0_usize as isize)
                                                                    .read())]);
                                                                __s
                                                            };
                                                            let _ = __s.write_all(&[(vec_
                                                                .decay()
                                                                .offset(1_usize as isize)
                                                                .read())]);
                                                            __s
                                                        };
                                                        let _ = __s.write_all(&[('o' as i8) as u8]);
                                                        __s
                                                    };
                                                    let _ = __s.write_all(
                                                        &str[..str.len().saturating_sub(1)]
                                                            .iter()
                                                            .map(|&__c| __c as u8)
                                                            .collect::<Vec<u8>>(),
                                                    );
                                                    __s
                                                };
                                                libcc2rs::endl_refcount.call(__s)
                                            };
                                            libcc2rs::hex_refcount.call(__s)
                                        };
                                        let _ = Ptr::<i8>::from_string_literal(b"0x").with_c_str(
                                            |__cs| {
                                                __s.write_all(
                                                    &__cs
                                                        .iter()
                                                        .map(|&__c| __c as u8)
                                                        .collect::<Vec<u8>>(),
                                                )
                                            },
                                        );
                                        __s
                                    };
                                    write_int_refcount(&__s, 27);
                                    __s
                                };
                                let _ = Ptr::<i8>::from_string_literal(b" a\xc3\xa7ordas?")
                                    .with_c_str(|__cs| {
                                        __s.write_all(
                                            &__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>(),
                                        )
                                    });
                                __s
                            };
                            let _ = __s.write_all(&[('\n' as i8) as u8]);
                            __s
                        };
                        let _ = Ptr::<i8>::from_string_literal(b"Sim, 0x").with_c_str(|__cs| {
                            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                        });
                        __s
                    };
                    write_int_refcount(&__s, i);
                    __s
                };
                let _ = __s.write_all(&[('.' as i8) as u8]);
                __s
            };
            libcc2rs::endl_refcount.call(__s)
        };
        libcc2rs::dec_refcount.call(__s)
    };
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = {
                        let __s = {
                            let __s = {
                                let __s = {
                                    let __s = {
                                        let __s = {
                                            let __s = {
                                                let __s = {
                                                    let __s = {
                                                        let __s = {
                                                            let __s = libcc2rs::cerr();
                                                            let _ =
                                                                __s.write_all(&[('H' as i8) as u8]);
                                                            __s
                                                        };
                                                        let _ = __s.write_all(&[('e' as i8) as u8]);
                                                        __s
                                                    };
                                                    let _ = __s.write_all(&[('l' as i8) as u8]);
                                                    __s
                                                };
                                                let _ = __s.write_all(&[('l' as i8) as u8]);
                                                __s
                                            };
                                            let _ = __s.write_all(&[('o' as i8) as u8]);
                                            __s
                                        };
                                        let _ = __s.write_all(&[(',' as i8) as u8]);
                                        __s
                                    };
                                    let _ = __s.write_all(&[(' ' as i8) as u8]);
                                    __s
                                };
                                let _ = __s.write_all(&[('W' as i8) as u8]);
                                __s
                            };
                            let _ = __s.write_all(&[('o' as i8) as u8]);
                            __s
                        };
                        let _ = __s.write_all(&[('r' as i8) as u8]);
                        __s
                    };
                    let _ = __s.write_all(&[('l' as i8) as u8]);
                    __s
                };
                let _ = __s.write_all(&[('d' as i8) as u8]);
                __s
            };
            let _ = __s.write_all(&[('!' as i8) as u8]);
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cerr();
                    let _ = __s.write_all(&[(vec_.decay().offset(0_usize as isize).read())]);
                    __s
                };
                let _ = __s.write_all(&[('\n' as i8) as u8]);
                __s
            };
            let _ = __s.write_all(&[(vec_.decay().offset(1_usize as isize).read())]);
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    return 0;
}
pub fn __cpp2rust_init_globals() {}
