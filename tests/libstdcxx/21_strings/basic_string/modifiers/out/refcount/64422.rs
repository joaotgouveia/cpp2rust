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
    let x: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_string_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    ({
        let ___p: __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___   = { let __tmp_0 : Value<__gnu_cxx___normal_iterator_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ > = Rc::new(RefCell::new(( { (*x.borrow()) . begin_63 ( ) } )  )); __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ :: new_1 ( { __tmp_0.as_pointer() } , )  }  ;
        (*x.borrow()).insert_123(___p, 10_u64, ('x' as i8))
    });
    let test: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"test\0")));
    ({
        let ___i1: __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___   = { let __tmp_1 : Value<__gnu_cxx___normal_iterator_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ > = Rc::new(RefCell::new(( { (*x.borrow()) . begin_63 ( ) } )  )); __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ :: new_1 ( { __tmp_1.as_pointer() } , )  }  ;
        let ___i2: __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___   = { let __tmp_2 : Value<__gnu_cxx___normal_iterator_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ > = Rc::new(RefCell::new(( { (*x.borrow()) . end_65 ( ) } )  )); __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ :: new_1 ( { __tmp_2.as_pointer() } , )  }  ;
        (*x.borrow()).replace_145(___i1, ___i2, (test.as_pointer() as Ptr<i8>), 4_u64)
    });
    ({
        let ___i1: __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___   = { let __tmp_3 : Value<__gnu_cxx___normal_iterator_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ > = Rc::new(RefCell::new(( { (*x.borrow()) . begin_63 ( ) } )  )); __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ :: new_1 ( { __tmp_3.as_pointer() } , )  }  ;
        let ___i2: __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___   = { let __tmp_4 : Value<__gnu_cxx___normal_iterator_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ > = Rc::new(RefCell::new(( { (*x.borrow()) . end_65 ( ) } )  )); __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ :: new_1 ( { __tmp_4.as_pointer() } , )  }  ;
        (*x.borrow()).replace_146(___i1, ___i2, (test.as_pointer() as Ptr<i8>))
    });
    let best: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"best\0")));
    ({
        let ___i1: __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___   = { let __tmp_5 : Value<__gnu_cxx___normal_iterator_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ > = Rc::new(RefCell::new(( { (*x.borrow()) . begin_63 ( ) } )  )); __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ :: new_1 ( { __tmp_5.as_pointer() } , )  }  ;
        let ___i2: __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___   = { let __tmp_6 : Value<__gnu_cxx___normal_iterator_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ > = Rc::new(RefCell::new(( { (*x.borrow()) . end_65 ( ) } )  )); __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ :: new_1 ( { __tmp_6.as_pointer() } , )  }  ;
        let ___k1: Ptr<i8> = (best.as_pointer() as Ptr<i8>);
        let ___k2: Ptr<i8> = (best.as_pointer() as Ptr<i8>).offset((4) as isize);
        (*x.borrow()).replace_149(___i1, ___i2, ___k1, ___k2)
    });
    ({
        let ___i1: __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___   = { let __tmp_7 : Value<__gnu_cxx___normal_iterator_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ > = Rc::new(RefCell::new(( { (*x.borrow()) . begin_63 ( ) } )  )); __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ :: new_1 ( { __tmp_7.as_pointer() } , )  }  ;
        let ___i2: __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___   = { let __tmp_8 : Value<__gnu_cxx___normal_iterator_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ > = Rc::new(RefCell::new(( { (*x.borrow()) . end_65 ( ) } )  )); __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ :: new_1 ( { __tmp_8.as_pointer() } , )  }  ;
        let ___str: Ptr<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
            x.as_pointer();
        (*x.borrow()).replace_144(___i1, ___i2, ___str)
    });
    ({
        let ___first: __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___   = { let __tmp_9 : Value<__gnu_cxx___normal_iterator_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ > = Rc::new(RefCell::new(( { (*x.borrow()) . begin_63 ( ) } )  )); __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ :: new_1 ( { __tmp_9.as_pointer() } , )  }  ;
        let ___last: __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___   = { let __tmp_10 : Value<__gnu_cxx___normal_iterator_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ > = Rc::new(RefCell::new(( { (*x.borrow()) . end_65 ( ) } )  )); __gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ :: new_1 ( { __tmp_10.as_pointer() } , )  }  ;
        (*x.borrow()).erase_137(___first, ___last)
    });
    let w: Value<std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__> =
        Rc::new(RefCell::new(
            <std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__>::default(
            ),
        ));
    ({
        let ___p: __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___   = { let __tmp_11 : Value<__gnu_cxx___normal_iterator_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ > = Rc::new(RefCell::new(( { (*w.borrow()) . begin_63 ( ) } )  )); __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ :: new_1 ( { __tmp_11.as_pointer() } , )  }  ;
        (*w.borrow()).insert_123(___p, 10_u64, (120 as i32))
    });
    let west: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
        119 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32,
    ])));
    ({
        let ___i1: __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___   = { let __tmp_12 : Value<__gnu_cxx___normal_iterator_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ > = Rc::new(RefCell::new(( { (*w.borrow()) . begin_63 ( ) } )  )); __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ :: new_1 ( { __tmp_12.as_pointer() } , )  }  ;
        let ___i2: __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___   = { let __tmp_13 : Value<__gnu_cxx___normal_iterator_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ > = Rc::new(RefCell::new(( { (*w.borrow()) . end_65 ( ) } )  )); __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ :: new_1 ( { __tmp_13.as_pointer() } , )  }  ;
        (*w.borrow()).replace_145(___i1, ___i2, (west.as_pointer() as Ptr<i32>), 4_u64)
    });
    ({
        let ___i1: __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___   = { let __tmp_14 : Value<__gnu_cxx___normal_iterator_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ > = Rc::new(RefCell::new(( { (*w.borrow()) . begin_63 ( ) } )  )); __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ :: new_1 ( { __tmp_14.as_pointer() } , )  }  ;
        let ___i2: __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___   = { let __tmp_15 : Value<__gnu_cxx___normal_iterator_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ > = Rc::new(RefCell::new(( { (*w.borrow()) . end_65 ( ) } )  )); __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ :: new_1 ( { __tmp_15.as_pointer() } , )  }  ;
        (*w.borrow()).replace_146(___i1, ___i2, (west.as_pointer() as Ptr<i32>))
    });
    let rest: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
        114 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32,
    ])));
    ({
        let ___i1: __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___   = { let __tmp_16 : Value<__gnu_cxx___normal_iterator_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ > = Rc::new(RefCell::new(( { (*w.borrow()) . begin_63 ( ) } )  )); __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ :: new_1 ( { __tmp_16.as_pointer() } , )  }  ;
        let ___i2: __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___   = { let __tmp_17 : Value<__gnu_cxx___normal_iterator_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ > = Rc::new(RefCell::new(( { (*w.borrow()) . end_65 ( ) } )  )); __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ :: new_1 ( { __tmp_17.as_pointer() } , )  }  ;
        let ___k1: Ptr<i32> = (rest.as_pointer() as Ptr<i32>);
        let ___k2: Ptr<i32> = (rest.as_pointer() as Ptr<i32>).offset((4) as isize);
        (*w.borrow()).replace_149(___i1, ___i2, ___k1, ___k2)
    });
    ({
        let ___i1: __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___   = { let __tmp_18 : Value<__gnu_cxx___normal_iterator_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ > = Rc::new(RefCell::new(( { (*w.borrow()) . begin_63 ( ) } )  )); __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ :: new_1 ( { __tmp_18.as_pointer() } , )  }  ;
        let ___i2: __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___   = { let __tmp_19 : Value<__gnu_cxx___normal_iterator_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ > = Rc::new(RefCell::new(( { (*w.borrow()) . end_65 ( ) } )  )); __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ :: new_1 ( { __tmp_19.as_pointer() } , )  }  ;
        let ___str: Ptr<
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
        > = w.as_pointer();
        (*w.borrow()).replace_144(___i1, ___i2, ___str)
    });
    ({
        let ___first: __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___   = { let __tmp_20 : Value<__gnu_cxx___normal_iterator_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ > = Rc::new(RefCell::new(( { (*w.borrow()) . begin_63 ( ) } )  )); __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ :: new_1 ( { __tmp_20.as_pointer() } , )  }  ;
        let ___last: __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___   = { let __tmp_21 : Value<__gnu_cxx___normal_iterator_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ > = Rc::new(RefCell::new(( { (*w.borrow()) . end_65 ( ) } )  )); __gnu_cxx___normal_iterator_const_wchar_t_ptr__std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ :: new_1 ( { __tmp_21.as_pointer() } , )  }  ;
        (*w.borrow()).erase_137(___first, ___last)
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
