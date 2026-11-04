extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    const _: () = assert!(true, "");;
    let h: Value<std_hash_std_basic_string_char__std_char_traits_char___std_allocator_char___> =
        Rc::new(RefCell::new(
            <std_hash_std_basic_string_char__std_char_traits_char___std_allocator_char___>::default(
            ),
        ));
    let g1: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567890").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let g2: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567891").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let s1: Value<Vec<i8>> = Rc::new(RefCell::new({
        use std::io::Read;
        let mut __bytes: Vec<u8> = Vec::new();
        let mut __f = &RandomAccessIterator::Vec(g1.decay());
        __f.read_to_end(&mut __bytes)
            .expect("couldn't read the file");
        __bytes.push(0);
        CChar::from_byte_vec(__bytes)
    }));
    let s2: Value<Vec<i8>> = Rc::new(RefCell::new({
        use std::io::Read;
        let mut __bytes: Vec<u8> = Vec::new();
        let mut __f = &RandomAccessIterator::Vec(g2.decay());
        __f.read_to_end(&mut __bytes)
            .expect("couldn't read the file");
        __bytes.push(0);
        CChar::from_byte_vec(__bytes)
    }));
    assert!(
        ({
            (({
                ( (*h.borrow_mut()) as std___str_hash_base_char__std_allocator_char___std_basic_string_char__std_char_traits_char___std_allocator_char___ ) ( s1 .as_pointer()  , )
            }) != ({
                ( (*h.borrow_mut()) as std___str_hash_base_char__std_allocator_char___std_basic_string_char__std_char_traits_char___std_allocator_char___ ) ( s2 .as_pointer()  , )
            }))
        })
    );
}
pub fn test_1() {
    const _: () = assert!(true, "");;
    let h : Value<std_hash_std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t___ > = Rc::new(RefCell::new(<std_hash_std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t___  >::default() )) ;
    let g1: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567890").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let g2: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567891").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let s1: Value<Vec<u8>> = Rc::new(RefCell::new({
        use std::io::Read;
        let mut __bytes: Vec<u8> = Vec::new();
        let mut __f = &RandomAccessIterator::Vec(g1.decay());
        __f.read_to_end(&mut __bytes)
            .expect("couldn't read the file");
        __bytes.push(0);
        CChar::from_byte_vec(__bytes)
    }));
    let s2: Value<Vec<u8>> = Rc::new(RefCell::new({
        use std::io::Read;
        let mut __bytes: Vec<u8> = Vec::new();
        let mut __f = &RandomAccessIterator::Vec(g2.decay());
        __f.read_to_end(&mut __bytes)
            .expect("couldn't read the file");
        __bytes.push(0);
        CChar::from_byte_vec(__bytes)
    }));
    assert!(
        ({
            (({
                ( (*h.borrow_mut()) as std___str_hash_base_char8_t__std_allocator_char8_t___std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t___ ) ( s1 .as_pointer()  , )
            }) != ({
                ( (*h.borrow_mut()) as std___str_hash_base_char8_t__std_allocator_char8_t___std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t___ ) ( s2 .as_pointer()  , )
            }))
        })
    );
}
pub fn test_2() {
    const _: () = assert!(true, "");;
    let h : Value<std_hash_std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t___ > = Rc::new(RefCell::new(<std_hash_std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t___  >::default() )) ;
    let g1: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567890").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let g2: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567891").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let s1: Value<Vec<u16>> = Rc::new(RefCell::new({
        use std::io::Read;
        let mut __bytes: Vec<u8> = Vec::new();
        let mut __f = &RandomAccessIterator::Vec(g1.decay());
        __f.read_to_end(&mut __bytes)
            .expect("couldn't read the file");
        __bytes.push(0);
        CChar::from_byte_vec(__bytes)
    }));
    let s2: Value<Vec<u16>> = Rc::new(RefCell::new({
        use std::io::Read;
        let mut __bytes: Vec<u8> = Vec::new();
        let mut __f = &RandomAccessIterator::Vec(g2.decay());
        __f.read_to_end(&mut __bytes)
            .expect("couldn't read the file");
        __bytes.push(0);
        CChar::from_byte_vec(__bytes)
    }));
    assert!(
        ({
            (({
                ( (*h.borrow_mut()) as std___str_hash_base_char16_t__std_allocator_char16_t___std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t___ ) ( s1 .as_pointer()  , )
            }) != ({
                ( (*h.borrow_mut()) as std___str_hash_base_char16_t__std_allocator_char16_t___std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t___ ) ( s2 .as_pointer()  , )
            }))
        })
    );
}
pub fn test_3() {
    const _: () = assert!(true, "");;
    let h : Value<std_hash_std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t___ > = Rc::new(RefCell::new(<std_hash_std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t___  >::default() )) ;
    let g1: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567890").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let g2: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567891").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let s1: Value<Vec<u32>> = Rc::new(RefCell::new({
        use std::io::Read;
        let mut __bytes: Vec<u8> = Vec::new();
        let mut __f = &RandomAccessIterator::Vec(g1.decay());
        __f.read_to_end(&mut __bytes)
            .expect("couldn't read the file");
        __bytes.push(0);
        CChar::from_byte_vec(__bytes)
    }));
    let s2: Value<Vec<u32>> = Rc::new(RefCell::new({
        use std::io::Read;
        let mut __bytes: Vec<u8> = Vec::new();
        let mut __f = &RandomAccessIterator::Vec(g2.decay());
        __f.read_to_end(&mut __bytes)
            .expect("couldn't read the file");
        __bytes.push(0);
        CChar::from_byte_vec(__bytes)
    }));
    assert!(
        ({
            (({
                ( (*h.borrow_mut()) as std___str_hash_base_char32_t__std_allocator_char32_t___std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t___ ) ( s1 .as_pointer()  , )
            }) != ({
                ( (*h.borrow_mut()) as std___str_hash_base_char32_t__std_allocator_char32_t___std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t___ ) ( s2 .as_pointer()  , )
            }))
        })
    );
}
pub fn test_4() {
    const _: () = assert!(true, "");;
    let h : Value<std_hash_std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ > = Rc::new(RefCell::new(<std_hash_std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___  >::default() )) ;
    let g1: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567890").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let g2: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567891").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let s1: Value<Vec<i32>> = Rc::new(RefCell::new({
        use std::io::Read;
        let mut __bytes: Vec<u8> = Vec::new();
        let mut __f = &RandomAccessIterator::Vec(g1.decay());
        __f.read_to_end(&mut __bytes)
            .expect("couldn't read the file");
        __bytes.push(0);
        CChar::from_byte_vec(__bytes)
    }));
    let s2: Value<Vec<i32>> = Rc::new(RefCell::new({
        use std::io::Read;
        let mut __bytes: Vec<u8> = Vec::new();
        let mut __f = &RandomAccessIterator::Vec(g2.decay());
        __f.read_to_end(&mut __bytes)
            .expect("couldn't read the file");
        __bytes.push(0);
        CChar::from_byte_vec(__bytes)
    }));
    assert!(
        ({
            (({
                ( (*h.borrow_mut()) as std___str_hash_base_wchar_t__std_allocator_wchar_t___std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ ) ( s1 .as_pointer()  , )
            }) != ({
                ( (*h.borrow_mut()) as std___str_hash_base_wchar_t__std_allocator_wchar_t___std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ ) ( s2 .as_pointer()  , )
            }))
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
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    ({ test_4() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
