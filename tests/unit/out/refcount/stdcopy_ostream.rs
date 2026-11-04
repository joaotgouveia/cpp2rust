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
    let str: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"Hello, world!\n") },
                None,
            ),
        ));
    let file: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(
        b"test_stdcopy_ostream.txt\0",
    )));
    {
        let ofs: Value<std_basic_ofstream_char__std_char_traits_char__> = Rc::new(RefCell::new(
            std_basic_ofstream_char__std_char_traits_char__::new_1(
                { (file.as_pointer() as Ptr<i8>) },
                { binary_0.with(|rc| *rc.borrow()) },
            ),
        ));
        ({
            let ___first: __gnu_cxx___normal_iterator_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___   = ( { (*str.borrow()) . begin_63 ( ) } )   ;
            let ___last: __gnu_cxx___normal_iterator_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___   = ( { (*str.borrow()) . end_65 ( ) } )   ;
            copy_1(
                ___first,
                ___last,
                std_ostream_iterator_char__char__std_char_traits_char__::new_1({
                    ofs.as_pointer()
                }),
            )
        });
    }
    ({ unlink_2((file.as_pointer() as Ptr<i8>)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
