extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static value_0: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_plus_int___two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_plus_int__ {}
thread_local!(
    pub static value_1: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_plus_std_basic_string_char____two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_plus_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
thread_local!(
    pub static value_2: Value<()> = Rc::new(RefCell::new((::std::mem::size_of::<()>() == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_plus_void__ {}
thread_local!(
    pub static value_3: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_minus_int___two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_minus_int__ {}
thread_local!(
    pub static value_4: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_minus_std_basic_string_char____two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_minus_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
thread_local!(
    pub static value_5: Value<()> = Rc::new(RefCell::new((::std::mem::size_of::<()>() == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_minus_void__ {}
thread_local!(
    pub static value_6: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_multiplies_int___two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_multiplies_int__ {}
thread_local!(
    pub static value_7: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_multiplies_std_basic_string_char____two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_multiplies_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
thread_local!(
    pub static value_8: Value<()> = Rc::new(RefCell::new((::std::mem::size_of::<()>() == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_multiplies_void__ {}
thread_local!(
    pub static value_9: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_divides_int___two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_divides_int__ {}
thread_local!(
    pub static value_10: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_divides_std_basic_string_char____two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_divides_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
thread_local!(
    pub static value_11: Value<()> = Rc::new(RefCell::new((::std::mem::size_of::<()>() == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_divides_void__ {}
thread_local!(
    pub static value_12: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_modulus_int___two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_modulus_int__ {}
thread_local!(
    pub static value_13: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_modulus_std_basic_string_char____two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_modulus_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
thread_local!(
    pub static value_14: Value<()> = Rc::new(RefCell::new((::std::mem::size_of::<()>() == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_modulus_void__ {}
thread_local!(
    pub static value_15: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_negate_int___two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_negate_int__ {}
thread_local!(
    pub static value_16: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_negate_std_basic_string_char____two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_negate_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
thread_local!(
    pub static value_17: Value<()> = Rc::new(RefCell::new((::std::mem::size_of::<()>() == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_negate_void__ {}
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    const _: () = assert!(true, "!is_transparent<std::plus<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::plus<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::plus<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::plus<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::minus<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::minus<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::minus<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::minus<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::multiplies<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::multiplies<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::multiplies<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::multiplies<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::divides<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::divides<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::divides<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::divides<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::modulus<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::modulus<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::modulus<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::modulus<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::negate<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::negate<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::negate<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::negate<>>::value");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = value_0.with(|_| ());
    let _ = value_1.with(|_| ());
    let _ = value_2.with(|_| ());
    let _ = value_3.with(|_| ());
    let _ = value_4.with(|_| ());
    let _ = value_5.with(|_| ());
    let _ = value_6.with(|_| ());
    let _ = value_7.with(|_| ());
    let _ = value_8.with(|_| ());
    let _ = value_9.with(|_| ());
    let _ = value_10.with(|_| ());
    let _ = value_11.with(|_| ());
    let _ = value_12.with(|_| ());
    let _ = value_13.with(|_| ());
    let _ = value_14.with(|_| ());
    let _ = value_15.with(|_| ());
    let _ = value_16.with(|_| ());
    let _ = value_17.with(|_| ());
}
