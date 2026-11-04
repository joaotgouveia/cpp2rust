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
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('\0' as i8),
        ('c' as i8),
        ('d' as i8),
        ('\0' as i8),
    ])));
    ({
        fputs_0(
            (buf.as_pointer() as Ptr<i8>),
            (*stdout_1.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        fputc_2(
            (('|' as i8) as i32),
            (*stdout_1.with(Value::clone).borrow()).clone(),
        )
    });
    ({ puts_3((buf.as_pointer() as Ptr<i8>)) });
    let s: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { (buf.as_pointer() as Ptr<i8>) },
                None,
            ),
        ));
    println!("{}", ({ (*s.borrow()).size() }));
    println!(
        "{} {}",
        ({ strlen_4((buf.as_pointer() as Ptr::<i8>),) }),
        ({ (*s.borrow()).find_last_of_188((buf.as_pointer() as Ptr::<i8>), None,) })
    );
    let mut lit: Ptr<i8> = Ptr::<i8>::from_string_literal(b"xy\0zw");
    let t: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { (lit).clone() },
                None,
            ),
        ));
    println!("{}", ({ (*t.borrow()).size() }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
