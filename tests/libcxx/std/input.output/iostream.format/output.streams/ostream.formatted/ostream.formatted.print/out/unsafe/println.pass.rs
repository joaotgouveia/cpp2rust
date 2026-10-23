extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_eprintf_0(
    mut fmt: *const (),
    args_1: *const *const (),
    args_2: *const *const (),
    args_3: *const *const (),
    args_4: *const (),
    args_5: *const *const (),
) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args_1),
            (*args_2),
            (*args_3),
            (*args_4),
            (*args_5),
        )
    });
}
pub unsafe fn test_eprintf_1(
    mut fmt: *const (),
    args_1: *const *const (),
    args_2: *const *const (),
    args_3: *const (),
) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args_1),
            (*args_2),
            (*args_3),
        )
    });
}
pub unsafe fn test_eprintf_2(mut fmt: *const (), args: *const *const ()) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args),
        )
    });
}
pub unsafe fn test_log_3(
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
    mut message: *const (),
) {
    let mut msg: *const () = if !(condition).is_null() {
        c"Assertion failure: ".as_ptr()
    } else {
        c"Unconditional failure:".as_ptr()
    };
    (unsafe {
        test_eprintf_0(
            c"%s%s %s %d\n%s".as_ptr(),
            &msg,
            &condition,
            &file,
            &line,
            &message,
        )
    });
}
pub unsafe fn test_log_4(
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_5(
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_require_6(
    mut condition: (),
    mut condition_str: *const (),
    mut file: *const (),
    mut line: (),
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const () = condition_str;
        let _file: *const () = file;
        let _line: () = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_4(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_require_7(
    mut condition: (),
    mut condition_str: *const (),
    mut file: *const (),
    mut line: (),
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const () = condition_str;
        let _file: *const () = file;
        let _line: () = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_5(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_is_high_surrogate_8(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (56319_()));
}
pub unsafe fn test_is_low_surrogate_9(mut value: ()) -> () {
    return ((value) >= (56320_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_surrogate_10(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_code_point_11(mut value: ()) -> () {
    return ((value) <= (1114111_()));
}
pub unsafe fn test_is_scalar_value_12(mut value: ()) -> () {
    return (unsafe { test_is_code_point_11(value) })
        && (!(unsafe { test_is_surrogate_10(value) }));
}
pub static mut test_replacement_character_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { (65533 as ()) });
pub unsafe fn operator_shl_14(os: *mut (), loc: *const ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_15(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_16(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_17(stream: *mut (), value: *const [(); 18]) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_18(stream: *mut (), value: *const [(); 31]) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_19(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_message_20(
    args_0: *const [(); 31],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *mut (),
) -> () {
    {
        let mut sstr: () = unimplemented!();
        {
            (unsafe {
                let _stream: *mut () = &mut sstr;
                let _value: *const [(); 31] = args_0;
                test_concat_18(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut () = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_16(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut () = &mut sstr;
                        let _value: *const [(); 18] = args_2;
                        test_concat_17(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut () = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_16(_stream, _value)
                        });
                        (unsafe {
                            let _stream: *mut () = &mut sstr;
                            let _value: *mut () = args_4;
                            test_concat_15(_stream, _value)
                        })
                    }
                }
            }
        };
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_21(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        let mut sstr: () = unimplemented!();
        {
            (unsafe {
                let _stream: *mut () = &mut sstr;
                let _value: *const [(); 18] = args_0;
                test_concat_17(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut () = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_19(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut () = &mut sstr;
                        let _value: *const [(); 18] = args_2;
                        test_concat_17(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut () = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_16(_stream, _value)
                        });
                        {
                            (unsafe {
                                let _stream: *mut () = &mut sstr;
                                let _value: *const [(); 18] = args_4;
                                test_concat_17(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_5;
                                    test_concat_16(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_6;
                                    test_concat_15(_stream, _value)
                                })
                            }
                        }
                    }
                }
            }
        };
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn print_tests_22(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        check.spec::<fn((), std_basic_format_string_char_)>(0).call(
            unimplemented!(),
            std_basic_format_string_char_::new_1({ &std::mem::transmute(*b"{{\0") }),
        )
    });
    (unsafe {
        check.spec::<fn((), std_basic_format_string_char_)>(0).call(
            unimplemented!(),
            std_basic_format_string_char_::new_2({ &std::mem::transmute(*b"{{:^}}\0") }),
        )
    });
    (unsafe {
        let mut _args: () = (' ' as ());
        check
            .spec::<fn((), std_basic_format_string_char__char_, *mut ())>(1)
            .call(
                unimplemented!(),
                std_basic_format_string_char__char_::new_1({
                    &std::mem::transmute(*b"{{:{}^}}\0")
                }),
                &mut _args,
            )
    });
    (unsafe {
        check.spec::<fn((), std_basic_format_string_char_)>(0).call(
            unimplemented!(),
            std_basic_format_string_char_::new_3({ &std::mem::transmute(*b"{{:{{}}^}}\0") }),
        )
    });
    (unsafe {
        let mut _args: () = (' ' as ());
        check
            .spec::<fn((), std_basic_format_string_char__char_, *mut ())>(1)
            .call(
                unimplemented!(),
                std_basic_format_string_char__char_::new_2({
                    &std::mem::transmute(*b"{{:{{{}}}^}}\0")
                }),
                &mut _args,
            )
    });
    (unsafe {
        let mut _args_2: () = false;
        let mut _args_3: () = true;
        check
            .spec::<fn((), std_basic_format_string_char__bool__bool_, *mut (), *mut ())>(2)
            .call(
                unimplemented!(),
                std_basic_format_string_char__bool__bool_::new({
                    &std::mem::transmute(*b"hello {0:} {1:}\0")
                }),
                &mut _args_2,
                &mut _args_3,
            )
    });
    (unsafe {
        let mut _args_2: () = false;
        let mut _args_3: () = true;
        check
            .spec::<fn((), std_basic_format_string_char__bool__bool_, *mut (), *mut ())>(2)
            .call(
                unimplemented!(),
                std_basic_format_string_char__bool__bool_::new({
                    &std::mem::transmute(*b"hello {1:} {0:}\0")
                }),
                &mut _args_2,
                &mut _args_3,
            )
    });
    (unsafe {
        let mut _args_2: () = 1_();
        let mut _args_3: () = 2_();
        let mut _args_4: () = 3_();
        let mut _args_5: () = 4_();
        let mut _args_6: () = 5_();
        let mut _args_7: () = 6_();
        let mut _args_8: () = 7_();
        let mut _args_9: () = 8_();
        let mut _args_10: () = 9_();
        let mut _args_11: () = 0_();
        let mut _args_12: () = 1_();
        let mut _args_13: () = 2_();
        let mut _args_14: () = 3_();
        let mut _args_15: () = 4_();
        let mut _args_16: () = 5_();
        let mut _args_17: () = 6_();
        let mut _args_18: () = 7_();
        let mut _args_19: () = 8_();
        let mut _args_20: () = 9_();
        let mut _args_21: () = 0_();
        check  .spec::<fn(() ,std_basic_format_string_char__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_ ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,)>(3) .call ( unimplemented!() , std_basic_format_string_char__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_ :: new ( { & std::mem::transmute(*b"{}{}{}{}{}{}{}{}{}{}\t{}{}{}{}{}{}{}{}{}{}\0") } , )  , & mut _args_2 , & mut _args_3 , & mut _args_4 , & mut _args_5 , & mut _args_6 , & mut _args_7 , & mut _args_8 , & mut _args_9 , & mut _args_10 , & mut _args_11 , & mut _args_12 , & mut _args_13 , & mut _args_14 , & mut _args_15 , & mut _args_16 , & mut _args_17 , & mut _args_18 , & mut _args_19 , & mut _args_20 , & mut _args_21 , )
    });
    (unsafe {
        let mut _args_2: () = ('\0' as ());
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__char__const_char__ref_arr6arr_,
                *mut (),
                *const [(); 6],
            )>(4)
            .call(
                (unsafe {
                    operator_literal_sv_23(
                        (&[
                            (104 as libc::c_char),
                            (101 as libc::c_char),
                            (108 as libc::c_char),
                            (108 as libc::c_char),
                            (111 as libc::c_char),
                            (0 as libc::c_char),
                            (119 as libc::c_char),
                            (111 as libc::c_char),
                            (114 as libc::c_char),
                            (108 as libc::c_char),
                            (100 as libc::c_char),
                            (0 as libc::c_char),
                        ])
                            .as_ptr(),
                        11_(),
                    )
                }),
                std_basic_format_string_char__char__const_char__ref_arr6arr_::new({
                    &std::mem::transmute(*b"hello{}{}\0")
                }),
                &mut _args_2,
                &std::mem::transmute(*b"world\0"),
            )
    });
    (unsafe {
        let _e: () = (unsafe {
            operator_literal_sv_23(
                (&[
                    (104 as libc::c_char),
                    (101 as libc::c_char),
                    (108 as libc::c_char),
                    (108 as libc::c_char),
                    (111 as libc::c_char),
                    (0 as libc::c_char),
                    (119 as libc::c_char),
                    (111 as libc::c_char),
                    (114 as libc::c_char),
                    (108 as libc::c_char),
                    (100 as libc::c_char),
                    (0 as libc::c_char),
                ])
                    .as_ptr(),
                11_(),
            )
        });
        let _fmt: std_basic_format_string_char__const_char__ref_arr6arr_ = {
            let mut __tmp_0: () = (unsafe {
                operator_literal_sv_23(
                    (&[
                        (104 as libc::c_char),
                        (101 as libc::c_char),
                        (108 as libc::c_char),
                        (108 as libc::c_char),
                        (111 as libc::c_char),
                        (0 as libc::c_char),
                        (123 as libc::c_char),
                        (125 as libc::c_char),
                        (0 as libc::c_char),
                    ])
                        .as_ptr(),
                    8_(),
                )
            });
            std_basic_format_string_char__const_char__ref_arr6arr_::new({ &mut __tmp_0 })
        };
        check
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr6arr_, *const [(); 6])>(
                5,
            )
            .call(_e, _fmt, &std::mem::transmute(*b"world\0"))
    });
    (unsafe {
        let _e: () = (unsafe {
            operator_literal_sv_23(
                (&[
                    (104 as libc::c_char),
                    (101 as libc::c_char),
                    (108 as libc::c_char),
                    (108 as libc::c_char),
                    (111 as libc::c_char),
                    (0 as libc::c_char),
                    (119 as libc::c_char),
                    (111 as libc::c_char),
                    (114 as libc::c_char),
                    (108 as libc::c_char),
                    (100 as libc::c_char),
                    (0 as libc::c_char),
                ])
                    .as_ptr(),
                11_(),
            )
        });
        let mut _args: () = (unsafe {
            operator_literal_sv_23(
                (&[
                    (0 as libc::c_char),
                    (119 as libc::c_char),
                    (111 as libc::c_char),
                    (114 as libc::c_char),
                    (108 as libc::c_char),
                    (100 as libc::c_char),
                    (0 as libc::c_char),
                ])
                    .as_ptr(),
                6_(),
            )
        });
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(6)
            .call(
                _e,
                std_basic_format_string_char__std_string_view_::new({
                    &std::mem::transmute(*b"hello{}\0")
                }),
                &mut _args,
            )
    });
    (unsafe {
        let _e: () = (unsafe { operator_literal_sv_23(c"\xc2\xa1".as_ptr(), 2_()) });
        let _fmt: std_basic_format_string_char__const_char__ref_arr3arr_ = {
            let mut __tmp_1: () = (unsafe { operator_literal_sv_23(c"{}".as_ptr(), 2_()) });
            std_basic_format_string_char__const_char__ref_arr3arr_::new({ &mut __tmp_1 })
        };
        check
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr3arr_, *const [(); 3])>(
                7,
            )
            .call(_e, _fmt, &std::mem::transmute(*b"\xc2\xa1\0"))
    });
    (unsafe {
        let _e: () = (unsafe { operator_literal_sv_23(c"\xdf\xbf".as_ptr(), 2_()) });
        let _fmt: std_basic_format_string_char__const_char__ref_arr3arr_ = {
            let mut __tmp_2: () = (unsafe { operator_literal_sv_23(c"{:}".as_ptr(), 3_()) });
            std_basic_format_string_char__const_char__ref_arr3arr_::new({ &mut __tmp_2 })
        };
        check
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr3arr_, *const [(); 3])>(
                7,
            )
            .call(_e, _fmt, &std::mem::transmute(*b"\xdf\xbf\0"))
    });
    (unsafe {
        let _e: () = (unsafe { operator_literal_sv_23(c"\xe0\xa0\x80".as_ptr(), 3_()) });
        let _fmt: std_basic_format_string_char__const_char__ref_arr4arr_ = {
            let mut __tmp_3: () = (unsafe { operator_literal_sv_23(c"{}".as_ptr(), 2_()) });
            std_basic_format_string_char__const_char__ref_arr4arr_::new({ &mut __tmp_3 })
        };
        check
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr4arr_, *const [(); 4])>(
                8,
            )
            .call(_e, _fmt, &std::mem::transmute(*b"\xe0\xa0\x80\0"))
    });
    (unsafe {
        let _e: () = (unsafe { operator_literal_sv_23(c"\xef\xbf\xbd".as_ptr(), 3_()) });
        let _fmt: std_basic_format_string_char__const_char__ref_arr4arr_ = {
            let mut __tmp_4: () = (unsafe { operator_literal_sv_23(c"{}".as_ptr(), 2_()) });
            std_basic_format_string_char__const_char__ref_arr4arr_::new({ &mut __tmp_4 })
        };
        check
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr4arr_, *const [(); 4])>(
                8,
            )
            .call(_e, _fmt, &std::mem::transmute(*b"\xef\xbf\xbd\0"))
    });
    (unsafe {
        let _e: () = (unsafe { operator_literal_sv_23(c"\xf0\x90\x80\x80".as_ptr(), 4_()) });
        let _fmt: std_basic_format_string_char__const_char__ref_arr5arr_ = {
            let mut __tmp_5: () = (unsafe { operator_literal_sv_23(c"{}".as_ptr(), 2_()) });
            std_basic_format_string_char__const_char__ref_arr5arr_::new({ &mut __tmp_5 })
        };
        check
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr5arr_, *const [(); 5])>(
                9,
            )
            .call(_e, _fmt, &std::mem::transmute(*b"\xf0\x90\x80\x80\0"))
    });
    (unsafe {
        let _e: () = (unsafe { operator_literal_sv_23(c"\xf4\x8f\xbf\xbf".as_ptr(), 4_()) });
        let _fmt: std_basic_format_string_char__const_char__ref_arr5arr_ = {
            let mut __tmp_6: () = (unsafe { operator_literal_sv_23(c"{}".as_ptr(), 2_()) });
            std_basic_format_string_char__const_char__ref_arr5arr_::new({ &mut __tmp_6 })
        };
        check
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr5arr_, *const [(); 5])>(
                9,
            )
            .call(_e, _fmt, &std::mem::transmute(*b"\xf4\x8f\xbf\xbf\0"))
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(1).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(1).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(1).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(1).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
}
pub static mut test_file_24: std::cell::LazyCell<FnPtr<Generic>> = std::cell::LazyCell::new(
    || unsafe {
        lambda_unsafe! ( Generic , { } , | mut e : () , mut fmt : std_basic_format_string_char_ , | { let mut expected : () = unimplemented!()  ;
  ;
 ;
 let mut sstr : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_2 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_7 ( _condition , c"out == expected" .as_ptr() , c"println.pass.cpp" .as_ptr() , 50_() , & mut _arg , ) } )  ;
 } , | mut e : () , mut fmt : std_basic_format_string_char__char_ , args : *mut () , | { let mut expected : () = unimplemented!()  ;
  ;
 ;
 let mut sstr : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__char_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_2 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_25 ( _condition , c"out == expected" .as_ptr() , c"println.pass.cpp" .as_ptr() , 50_() , & mut _arg , ) } )  ;
 } , | mut e : () , mut fmt : std_basic_format_string_char__bool__bool_ , args_2 : *mut () , args_3 : *mut () , | { let mut expected : () = unimplemented!()  ;
  ;
 ;
 let mut sstr : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__bool__bool_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_2 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_26 ( _condition , c"out == expected" .as_ptr() , c"println.pass.cpp" .as_ptr() , 50_() , & mut _arg , ) } )  ;
 } , | mut e : () , mut fmt : std_basic_format_string_char__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , args_5 : *mut () , args_6 : *mut () , args_7 : *mut () , args_8 : *mut () , args_9 : *mut () , args_10 : *mut () , args_11 : *mut () , args_12 : *mut () , args_13 : *mut () , args_14 : *mut () , args_15 : *mut () , args_16 : *mut () , args_17 : *mut () , args_18 : *mut () , args_19 : *mut () , args_20 : *mut () , args_21 : *mut () , | { let mut expected : () = unimplemented!()  ;
  ;
 ;
 let mut sstr : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_2 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_27 ( _condition , c"out == expected" .as_ptr() , c"println.pass.cpp" .as_ptr() , 50_() , & mut _arg , ) } )  ;
 } , | mut e : () , mut fmt : std_basic_format_string_char__char__const_char__ref_arr6arr_ , args_2 : *mut () , args_3 : *const [ () ; 6] , | { let mut expected : () = unimplemented!()  ;
  ;
 ;
 let mut sstr : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__char__const_char__ref_arr6arr_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_2 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_28 ( _condition , c"out == expected" .as_ptr() , c"println.pass.cpp" .as_ptr() , 50_() , & mut _arg , ) } )  ;
 } , | mut e : () , mut fmt : std_basic_format_string_char__const_char__ref_arr6arr_ , args : *const [ () ; 6] , | { let mut expected : () = unimplemented!()  ;
  ;
 ;
 let mut sstr : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__const_char__ref_arr6arr_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_2 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_29 ( _condition , c"out == expected" .as_ptr() , c"println.pass.cpp" .as_ptr() , 50_() , & mut _arg , ) } )  ;
 } , | mut e : () , mut fmt : std_basic_format_string_char__std_string_view_ , args : *mut () , | { let mut expected : () = unimplemented!()  ;
  ;
 ;
 let mut sstr : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_string_view_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_2 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_30 ( _condition , c"out == expected" .as_ptr() , c"println.pass.cpp" .as_ptr() , 50_() , & mut _arg , ) } )  ;
 } , | mut e : () , mut fmt : std_basic_format_string_char__const_char__ref_arr3arr_ , args : *const [ () ; 3] , | { let mut expected : () = unimplemented!()  ;
  ;
 ;
 let mut sstr : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__const_char__ref_arr3arr_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_2 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_31 ( _condition , c"out == expected" .as_ptr() , c"println.pass.cpp" .as_ptr() , 50_() , & mut _arg , ) } )  ;
 } , | mut e : () , mut fmt : std_basic_format_string_char__const_char__ref_arr4arr_ , args : *const [ () ; 4] , | { let mut expected : () = unimplemented!()  ;
  ;
 ;
 let mut sstr : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__const_char__ref_arr4arr_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_2 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_32 ( _condition , c"out == expected" .as_ptr() , c"println.pass.cpp" .as_ptr() , 50_() , & mut _arg , ) } )  ;
 } , | mut e : () , mut fmt : std_basic_format_string_char__const_char__ref_arr5arr_ , args : *const [ () ; 5] , | { let mut expected : () = unimplemented!()  ;
  ;
 ;
 let mut sstr : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__const_char__ref_arr5arr_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_2 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_33 ( _condition , c"out == expected" .as_ptr() , c"println.pass.cpp" .as_ptr() , 50_() , & mut _arg , ) } )  ;
 } )
    },
);
pub static mut test_exception_34: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(Generic, {}, |mut _a0: (), mut _a1: ()| {}, |mut _a0: (),
                                                                    mut _a1: (),
                                                                    _a2: *mut (
        )| {})
    });
pub unsafe fn test_println_blank_line_35() {
    let mut expected: () = unimplemented!();
    let mut sstr: () = unimplemented!();
    unimplemented!();
    let mut out: () = unimplemented!();
    (unsafe {
        let _condition: () = unimplemented!();
        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
            {
                let expected: *mut () = &mut expected;
                let out: *mut () = &mut out;
            },
            || {
                (unsafe {
                    let mut _args: *const () = (unimplemented!() as *const ());
                    test_eprintf_2(c"%s".as_ptr(), &mut _args)
                });
            }
        );
        test_require_6(
            _condition,
            c"out == expected".as_ptr(),
            c"println.pass.cpp".as_ptr(),
            68_(),
            &mut _arg,
        )
    });
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe {
        print_tests_22(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_file_24)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_34)).copy_from(),
        )
    });
    (unsafe { test_println_blank_line_35() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_replacement_character_13);
    std::cell::LazyCell::force(&*&raw const test_file_24);
    std::cell::LazyCell::force(&*&raw const test_exception_34);
}
