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
pub struct type_list_char__signed_char__short__int__long__long_long____int128_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct type_list_char__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct type_list_float__double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct type_concat_type_list_char__signed_char__short__int__long__long_long____int128___type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}pub fn concat_0 ( mut _a0 : type_list_char__signed_char__short__int__long__long_long____int128_  , mut _a1 : type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_  , ) -> type_list_char__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_{
    return <type_list_char__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_  >::default()   ;
}
thread_local!(
    pub static all_signed_1: Value<
        type_list_char__signed_char__short__int__long__long_long____int128_,
    > = Rc::new(RefCell::new(
        <type_list_char__signed_char__short__int__long__long_long____int128_>::default(),
    ));
);
thread_local!(
    pub static all_unsigned_2 : Value<type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ > = Rc::new(RefCell::new(<type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_  >::default() )) ;
);
thread_local!(
    pub static integrals_3 : Value<type_list_char__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ > = Rc::new(RefCell::new(( { concat_0 ( ((*all_signed_1.with(Value::clone).borrow()) ).clone()  , ((*all_unsigned_2.with(Value::clone).borrow()) ).clone()  , ) } )  )) ;
);
thread_local!(
    pub static all_floats_4: Value<type_list_float__double_> =
        Rc::new(RefCell::new(<type_list_float__double_>::default()));
);
pub fn run_5(mut _a0: type_list_float__double_) {
    let mut ls: [(); 2] = [
        {
            ({
                test_basics_float_Impl::operator_call(
                    &Rc::new(RefCell::new(<test_basics_float_>::default())).as_pointer(),
                )
            });
            0_()
        },
        {
            ({
                test_basics_double_Impl::operator_call(
                    &Rc::new(RefCell::new(<test_basics_double_>::default())).as_pointer(),
                )
            });
            0_()
        },
    ];
    &(ls);
}
pub fn run_6(mut _a0: type_list_float__double_) {
    let mut ls: [(); 2] = [
        {
            ({
                test_scientific_float_Impl::operator_call(
                    &Rc::new(RefCell::new(<test_scientific_float_>::default())).as_pointer(),
                )
            });
            0_()
        },
        {
            ({
                test_scientific_double_Impl::operator_call(
                    &Rc::new(RefCell::new(<test_scientific_double_>::default())).as_pointer(),
                )
            });
            0_()
        },
    ];
    &(ls);
}
pub fn run_7(mut _a0: type_list_float__double_) {
    let mut ls: [(); 2] = [
        {
            ({
                test_fixed_float_Impl::operator_call(
                    &Rc::new(RefCell::new(<test_fixed_float_>::default())).as_pointer(),
                )
            });
            0_()
        },
        {
            ({
                test_fixed_double_Impl::operator_call(
                    &Rc::new(RefCell::new(<test_fixed_double_>::default())).as_pointer(),
                )
            });
            0_()
        },
    ];
    &(ls);
}
pub fn run_8(mut _a0: type_list_float__double_) {
    let mut ls: [(); 2] = [
        {
            ({
                test_general_float_Impl::operator_call(
                    &Rc::new(RefCell::new(<test_general_float_>::default())).as_pointer(),
                )
            });
            0_()
        },
        {
            ({
                test_general_double_Impl::operator_call(
                    &Rc::new(RefCell::new(<test_general_double_>::default())).as_pointer(),
                )
            });
            0_()
        },
    ];
    &(ls);
}
pub fn run_9(mut _a0: type_list_float__double_) {
    let mut ls: [(); 2] = [
        {
            ({
                test_hex_float_Impl::operator_call(
                    &Rc::new(RefCell::new(<test_hex_float_>::default())).as_pointer(),
                )
            });
            0_()
        },
        {
            ({
                test_hex_double_Impl::operator_call(
                    &Rc::new(RefCell::new(<test_hex_double_>::default())).as_pointer(),
                )
            });
            0_()
        },
    ];
    &(ls);
}
pub fn test_infinity_10(mut fmt: ()) {
    let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-InFiNiTyXXX");
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_infinity_11(mut fmt: ()) {
    let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-InFiNiTyXXX");
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_nan_12(mut fmt: ()) {
    {
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-NaN(1_A)XXX");
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"NaN()");
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!().write(('a' as ()));
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        };
        'loop_: for c in IterRange::new(
            (b"!@#$%^&*(-=+[]{}|\\;:\'\",./<>?~` \t\x0b\r\n" as Ptr<()>),
            (b"!@#$%^&*(-=+[]{}|\\;:\'\",./<>?~` \t\x0b\r\n" as Ptr<()>).offset((36_()) as isize),
        ) {
            let mut c: () = (c.read());
            let mut value: () = 0.25_f64;
            unimplemented!().write(c);
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_nan_13(mut fmt: ()) {
    {
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-NaN(1_A)XXX");
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"NaN()");
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!().write(('a' as ()));
        {
            let mut value: () = 0.25_f64;
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        };
        'loop_: for c in IterRange::new(
            (b"!@#$%^&*(-=+[]{}|\\;:\'\",./<>?~` \t\x0b\r\n" as Ptr<()>),
            (b"!@#$%^&*(-=+[]{}|\\;:\'\",./<>?~` \t\x0b\r\n" as Ptr<()>).offset((36_()) as isize),
        ) {
            let mut c: () = (c.read());
            let mut value: () = 0.25_f64;
            unimplemented!().write(c);
            let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_fmt_independent_14(mut fmt: ()) {
    ({ test_infinity_10(fmt) });
    ({ test_nan_12(fmt) });
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-");
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b".");
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-.");
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"+0.25");
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_fmt_independent_15(mut fmt: ()) {
    ({ test_infinity_11(fmt) });
    ({ test_nan_13(fmt) });
    {
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-");
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b".");
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-.");
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"+0.25");
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_basics_float_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_basics_double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_fixed_float_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_fixed_double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_scientific_float_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_scientific_double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_general_float_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_general_double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_hex_float_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_hex_double_ {}
pub fn test_random_errors_16() {
    {
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"4.219902180869891e-2788");
        let mut last: Ptr<()> = s
            .offset(({ strlen_17((s).clone()) }) as isize)
            .offset(-((1_()) as isize));
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"7.411412e-39U");
        let mut last: Ptr<()> = s
            .offset(({ strlen_17((s).clone()) }) as isize)
            .offset(-((1_()) as isize));
        let mut value: () = 0.25_f64;
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ run_5((*all_floats_4.with(Value::clone).borrow()).clone()) });
    ({ run_6((*all_floats_4.with(Value::clone).borrow()).clone()) });
    ({ run_7((*all_floats_4.with(Value::clone).borrow()).clone()) });
    ({ run_8((*all_floats_4.with(Value::clone).borrow()).clone()) });
    ({ run_9((*all_floats_4.with(Value::clone).borrow()).clone()) });
    ({ test_random_errors_16() });
    return 0_();
}
pub trait test_basics_double_Impl {
    fn operator_call(&self);
}
impl test_basics_double_Impl for Ptr<test_basics_double_> {
    fn operator_call(&self) {
        'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut fmt: () = (fmt.read());
            ({ test_fmt_independent_15(fmt) });
        }
    }
}
pub trait test_basics_float_Impl {
    fn operator_call(&self);
}
impl test_basics_float_Impl for Ptr<test_basics_float_> {
    fn operator_call(&self) {
        'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut fmt: () = (fmt.read());
            ({ test_fmt_independent_14(fmt) });
        }
    }
}
pub trait test_fixed_double_Impl {
    fn operator_call(&self);
}
impl test_fixed_double_Impl for Ptr<test_fixed_double_> {
    fn operator_call(&self) {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut x: () = 0.25_f64;
        {
            let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            'loop_: for c in IterRange::new( ( b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n"  as Ptr::<()> )  , ( b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n"  as Ptr::<()> ) . offset ( ( 87_() ) as isize )  ) { let mut c : ()  = ( c .read() )   ;
  ;
 ;
 unimplemented!()  .write( c  ) ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"001x");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b".5");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-.5");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25.78");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5E10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e-10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e-");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e +1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+ 1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e -1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e- 1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e0e12");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e++12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e+-12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e-+12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e--12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e0x12");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"20040229");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"123.456");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"123.456e3");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> =
                    Ptr::<()>::from_string_literal(b"0.111111111111111111111111111111111111111111");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(
                    b"111111111111.111111111111111111111111111111111111111111",
                );
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-0.25");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub trait test_fixed_float_Impl {
    fn operator_call(&self);
}
impl test_fixed_float_Impl for Ptr<test_fixed_float_> {
    fn operator_call(&self) {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut x: () = 0.25_f64;
        {
            let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            'loop_: for c in IterRange::new( ( b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n"  as Ptr::<()> )  , ( b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n"  as Ptr::<()> ) . offset ( ( 87_() ) as isize )  ) { let mut c : ()  = ( c .read() )   ;
  ;
 ;
 unimplemented!()  .write( c  ) ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"001x");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b".5");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-.5");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25.78");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5E10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e-10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e-");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e +1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+ 1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e -1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e- 1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e0e12");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e++12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e+-12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e-+12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e--12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e0x12");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"20040229");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"123.456");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"123.456e3");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> =
                    Ptr::<()>::from_string_literal(b"0.111111111111111111111111111111111111111111");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(
                    b"111111111111.111111111111111111111111111111111111111111",
                );
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-0.25");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub trait test_general_double_Impl {
    fn operator_call(&self);
}
impl test_general_double_Impl for Ptr<test_general_double_> {
    fn operator_call(&self) {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut x: () = 0.25_f64;
        {
            let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            'loop_: for c in IterRange::new( ( b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n"  as Ptr::<()> )  , ( b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n"  as Ptr::<()> ) . offset ( ( 87_() ) as isize )  ) { let mut c : ()  = ( c .read() )   ;
  ;
 ;
 unimplemented!()  .write( c  ) ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"001x");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b".5e0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-.5e0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b".5");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-.5");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25.78");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5E10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e-10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e-");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e +1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+ 1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e -1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e- 1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e++12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e+-12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e-+12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e--12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e0x12");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e0e12");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"20040229");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"123.456");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"123.456e3");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> =
                    Ptr::<()>::from_string_literal(b"0.111111111111111111111111111111111111111111");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(
                    b"111111111111.111111111111111111111111111111111111111111",
                );
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-0.25");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"1e9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"-1e9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"1e-9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"-1e-9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub trait test_general_float_Impl {
    fn operator_call(&self);
}
impl test_general_float_Impl for Ptr<test_general_float_> {
    fn operator_call(&self) {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut x: () = 0.25_f64;
        {
            let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            'loop_: for c in IterRange::new( ( b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n"  as Ptr::<()> )  , ( b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n"  as Ptr::<()> ) . offset ( ( 87_() ) as isize )  ) { let mut c : ()  = ( c .read() )   ;
  ;
 ;
 unimplemented!()  .write( c  ) ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"001x");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b".5e0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-.5e0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b".5");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-.5");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25.78");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5E10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e-10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e-");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e +1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+ 1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e -1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e- 1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e++12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e+-12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e-+12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e--12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e0x12");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e0e12");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"20040229");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"123.456");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"123.456e3");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> =
                    Ptr::<()>::from_string_literal(b"0.111111111111111111111111111111111111111111");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(
                    b"111111111111.111111111111111111111111111111111111111111",
                );
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-0.25");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"1e9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"-1e9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"1e-9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"-1e-9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub trait test_hex_double_Impl {
    fn operator_call(&self);
}
impl test_hex_double_Impl for Ptr<test_hex_double_> {
    fn operator_call(&self) {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut x: () = 0.25_f64;
        {
            let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            'loop_: for c in IterRange::new( ( b"ghijklmnopqrstuvwxyzGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n"  as Ptr::<()> )  , ( b"ghijklmnopqrstuvwxyzGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n"  as Ptr::<()> ) . offset ( ( 75_() ) as isize )  ) { let mut c : ()  = ( c .read() )   ;
  ;
 ;
 unimplemented!()  .write( c  ) ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"001x");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b".5p0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-.5p0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b".5");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-.5");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25.78");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5P10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p+10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p-10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p+");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p-");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p +1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p+ 1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p -1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p- 1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25p++12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25p+-12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25p-+12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25p--12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25p0x12");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25p0p12");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"131CA25");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"123.456");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"123.456p3");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> =
                    Ptr::<()>::from_string_literal(b"0.111111111111111111111111111111111111111111");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(
                    b"111111111111.111111111111111111111111111111111111111111",
                );
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-0.25");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"1p9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"-1p9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"1p-9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"-1p-9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub trait test_hex_float_Impl {
    fn operator_call(&self);
}
impl test_hex_float_Impl for Ptr<test_hex_float_> {
    fn operator_call(&self) {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut x: () = 0.25_f64;
        {
            let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            'loop_: for c in IterRange::new( ( b"ghijklmnopqrstuvwxyzGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n"  as Ptr::<()> )  , ( b"ghijklmnopqrstuvwxyzGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n"  as Ptr::<()> ) . offset ( ( 75_() ) as isize )  ) { let mut c : ()  = ( c .read() )   ;
  ;
 ;
 unimplemented!()  .write( c  ) ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"001x");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b".5p0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-.5p0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b".5");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-.5");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25.78");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5P10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p+10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p-10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p+");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p-");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p +1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p+ 1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p -1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5p- 1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25p++12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25p+-12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25p-+12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25p--12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25p0x12");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25p0p12");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"131CA25");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"123.456");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"123.456p3");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> =
                    Ptr::<()>::from_string_literal(b"0.111111111111111111111111111111111111111111");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(
                    b"111111111111.111111111111111111111111111111111111111111",
                );
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-0.25");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"1p9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"-1p9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"1p-9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"-1p-9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub trait test_scientific_double_Impl {
    fn operator_call(&self);
}
impl test_scientific_double_Impl for Ptr<test_scientific_double_> {
    fn operator_call(&self) {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut x: () = 0.25_f64;
        {
            let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            'loop_: for c in IterRange::new( ( b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n"  as Ptr::<()> )  , ( b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n"  as Ptr::<()> ) . offset ( ( 87_() ) as isize )  ) { let mut c : ()  = ( c .read() )   ;
  ;
 ;
 unimplemented!()  .write( c  ) ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.23");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.23e");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e-");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e +1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+ 1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e -1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e- 1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e++12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e+-12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e-+12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e--12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"001e0x");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e0.78");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5E10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e-10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e0x12");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e0e12");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"20040229e0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"123.456e3");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(
                    b"0.111111111111111111111111111111111111111111e0",
                );
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(
                    b"111111111111.111111111111111111111111111111111111111111e0",
                );
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-0.25e0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"1e9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"-1e9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"1e-9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"-1e-9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub trait test_scientific_float_Impl {
    fn operator_call(&self);
}
impl test_scientific_float_Impl for Ptr<test_scientific_float_> {
    fn operator_call(&self) {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut x: () = 0.25_f64;
        {
            let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            'loop_: for c in IterRange::new( ( b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n"  as Ptr::<()> )  , ( b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n"  as Ptr::<()> ) . offset ( ( 87_() ) as isize )  ) { let mut c : ()  = ( c .read() )   ;
  ;
 ;
 unimplemented!()  .write( c  ) ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.23");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.23e");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e-");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e +1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+ 1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e -1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e- 1");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e++12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e+-12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e-+12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e--12");
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"001e0x");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e0.78");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5E10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e+10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.5e-10");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e0x12");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"1.25e0e12");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"20040229e0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"123.456e3");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(
                    b"0.111111111111111111111111111111111111111111e0",
                );
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: Ptr<()> = Ptr::<()>::from_string_literal(
                    b"111111111111.111111111111111111111111111111111111111111e0",
                );
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"-0.25e0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"1e9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"-1e9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"1e-9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: Ptr<()> =
                Ptr::<()>::from_string_literal(b"-1e-9999999999999999999999999999999999999999");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = all_signed_1.with(|_| ());
    let _ = all_unsigned_2.with(|_| ());
    let _ = integrals_3.with(|_| ());
    let _ = all_floats_4.with(|_| ());
}
