extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct max_align_t {
    #[offset(0)]
    pub __clang_max_align_nonce1: i64,
    #[offset(16)]
    #[byte_size(16)]
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_byte = u8;
thread_local!(
    pub static p_mut_0: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
);
thread_local!(
    pub static p_const_1: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
);
thread_local!(
    pub static cp_2: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
);
thread_local!(
    pub static arr_of_ptr_3: Value<Box<[Ptr<i32>]>> = Rc::new(RefCell::new(
        (0..4)
            .map(|_| Ptr::<i32>::null())
            .collect::<Box<[Ptr<i32>]>>(),
    ));
);
thread_local!(
    pub static pp_4: Value<Ptr<Ptr<i32>>> = Rc::new(RefCell::new(Ptr::<Ptr<i32>>::null()));
);
thread_local!(
    pub static const_arr_of_ptr_5: Value<Box<[Ptr<i32>]>> = Rc::new(RefCell::new(
        (0..3)
            .map(|_| Ptr::<i32>::null())
            .collect::<Box<[Ptr<i32>]>>(),
    ));
);
thread_local!(
    pub static cp_explicit_null_6: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
);
thread_local!(
    pub static p_zero_7: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    (if (*p_mut_0.with(Value::clone).borrow()).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_8(
                Ptr::<i8>::from_string_literal(b"p_mut == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (*p_const_1.with(Value::clone).borrow()).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_8(
                Ptr::<i8>::from_string_literal(b"p_const == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (*cp_2.with(Value::clone).borrow()).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_8(
                Ptr::<i8>::from_string_literal(b"cp == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut i: i32 = 0;
    'loop_: while (i < 4) {
        (if ({
            let __idx = (i) as usize;
            arr_of_ptr_3.with(|rc| rc.borrow()[__idx].clone())
        })
        .is_null()
        {
            &(0);
        } else {
            ({
                __assert_fail_8(
                    Ptr::<i8>::from_string_literal(b"arr_of_ptr[i] == nullptr"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
        i.prefix_inc();
    }
    (if (*pp_4.with(Value::clone).borrow()).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_8(
                Ptr::<i8>::from_string_literal(b"pp == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut i: i32 = 0;
    'loop_: while (i < 3) {
        (if ({
            let __idx = (i) as usize;
            const_arr_of_ptr_5.with(|rc| rc.borrow()[__idx].clone())
        })
        .is_null()
        {
            &(0);
        } else {
            ({
                __assert_fail_8(
                    Ptr::<i8>::from_string_literal(b"const_arr_of_ptr[i] == nullptr"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
        i.prefix_inc();
    }
    (if (*cp_explicit_null_6.with(Value::clone).borrow()).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_8(
                Ptr::<i8>::from_string_literal(b"cp_explicit_null == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (*p_zero_7.with(Value::clone).borrow()).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_8(
                Ptr::<i8>::from_string_literal(b"p_zero == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_8(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {
    let _ = p_mut_0.with(|_| ());
    let _ = p_const_1.with(|_| ());
    let _ = cp_2.with(|_| ());
    let _ = arr_of_ptr_3.with(|_| ());
    let _ = pp_4.with(|_| ());
    let _ = const_arr_of_ptr_5.with(|_| ());
    let _ = cp_explicit_null_6.with(|_| ());
    let _ = p_zero_7.with(|_| ());
}
