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
pub struct types_type_list_float__double__long_double_ {}
impl CppMove for types_type_list_float__double__long_double_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_short__int__long__long_long_ {}
impl CppMove for types_type_list_short__int__long__long_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub fn for_each_0(mut _a0: types_type_list_float__double__long_double_, f: TestFloat) {
    let f: Value<TestFloat> = Rc::new(RefCell::new(f));
    ({
        let _arg0: i32 = {
            ({ TestFloatImpl::operator_call_1_float(&f.as_pointer()) });
            0
        };
        let _arg1: i32 = {
            ({ TestFloatImpl::operator_call_1_double(&f.as_pointer()) });
            0
        };
        let _arg2: i32 = {
            ({ TestFloatImpl::operator_call_1_long_double(&f.as_pointer()) });
            0
        };
        swallow_1(_arg0, _arg1, _arg2)
    });
}
pub fn for_each_2(mut _a0: types_type_list_short__int__long__long_long_, f: TestInt) {
    let f: Value<TestInt> = Rc::new(RefCell::new(f));
    ({
        let _arg0: i32 = {
            ({ TestIntImpl::operator_call_1_short(&f.as_pointer()) });
            0
        };
        let _arg1: i32 = {
            ({ TestIntImpl::operator_call_1_int(&f.as_pointer()) });
            0
        };
        let _arg2: i32 = {
            ({ TestIntImpl::operator_call_1_long(&f.as_pointer()) });
            0
        };
        let _arg3: i32 = {
            ({ TestIntImpl::operator_call_1_long_long(&f.as_pointer()) });
            0
        };
        swallow_3(_arg0, _arg1, _arg2, _arg3)
    });
}
pub fn swallow_1(mut _a0: i32, mut _a1: i32, mut _a2: i32) {}
pub fn swallow_3(mut _a0: i32, mut _a1: i32, mut _a2: i32, mut _a3: i32) {}
pub fn get_maximal_order_4() -> u32 {
    {
        return 128_u32;
    }
    panic!("ub: non-void function does not return a value")
}
pub fn get_maximal_order_5() -> u32 {
    {
        return 128_u32;
    }
    panic!("ub: non-void function does not return a value")
}
pub fn get_maximal_order_6() -> u32 {
    {
        return 128_u32;
    }
    panic!("ub: non-void function does not return a value")
}
pub fn sample_points_7() -> Vec<f32> {
    return Vec::from([
        (-12.34_f64 as f32),
        (-7.42_f64 as f32),
        (-1_f64 as f32),
        (-0.5_f64 as f32),
        (-0.1_f64 as f32),
        (0_f64 as f32),
        (0.1_f64 as f32),
        (0.5_f64 as f32),
        (1_f64 as f32),
        (5.67_f64 as f32),
        (15.67_f64 as f32),
    ]);
}
pub fn sample_points_8() -> Vec<f64> {
    return Vec::from([
        -12.34_f64, -7.42_f64, -1_f64, -0.5_f64, -0.1_f64, 0_f64, 0.1_f64, 0.5_f64, 1_f64,
        5.67_f64, 15.67_f64,
    ]);
}
pub fn sample_points_9() -> Vec<f64> {
    return Vec::from([
        -12.34_f64, -7.42_f64, -1_f64, -0.5_f64, -0.1_f64, 0_f64, 0.1_f64, 0.5_f64, 1_f64,
        5.67_f64, 15.67_f64,
    ]);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct CompareFloatingValues_float_ {
    #[offset(0)]
    abs_tol: f32,
    #[offset(4)]
    rel_tol: f32,
}
impl CompareFloatingValues_float_ {
    pub fn new() -> Self {
        let __this: Value<CompareFloatingValues_float_> = Rc::new(RefCell::new(Self {
            abs_tol: 0_f32,
            rel_tol: 0_f32,
        }));
        let this: Ptr<CompareFloatingValues_float_> = __this.as_pointer();
        field!(this, abs_tol).write(
            ({
                FnPtr::<fn() -> f32>::new(|| -> f32 {
                    {
                        {
                            return 1e-05_f32;
                        }
                        panic!("ub: non-void function does not return a value")
                    }
                })
                .call()
            }),
        );
        field!(this, rel_tol).write(this.with(|__s| __s.abs_tol));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Callable2<f32, f32, bool> for CompareFloatingValues_float_ {
    fn call(&self, a1: f32, a2: f32) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CompareFloatingValues_float_Impl::operator_call(__it, a1, a2)
        })
    }
}
value_comparator!(CompareFloatingValues_float_, f32);
impl CppMove for CompareFloatingValues_float_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for CompareFloatingValues_float_ {
    fn default() -> Self {
        { CompareFloatingValues_float_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct CompareFloatingValues_double_ {
    #[offset(0)]
    abs_tol: f64,
    #[offset(8)]
    rel_tol: f64,
}
impl CompareFloatingValues_double_ {
    pub fn new() -> Self {
        let __this: Value<CompareFloatingValues_double_> = Rc::new(RefCell::new(Self {
            abs_tol: 0_f64,
            rel_tol: 0_f64,
        }));
        let this: Ptr<CompareFloatingValues_double_> = __this.as_pointer();
        field!(this, abs_tol).write(
            ({
                FnPtr::<fn() -> f64>::new(|| -> f64 {
                    {
                        {
                            {
                                return 1e-11_f64;
                            }
                        }
                        panic!("ub: non-void function does not return a value")
                    }
                })
                .call()
            }),
        );
        field!(this, rel_tol).write(this.with(|__s| __s.abs_tol));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Callable2<f64, f64, bool> for CompareFloatingValues_double_ {
    fn call(&self, a1: f64, a2: f64) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CompareFloatingValues_double_Impl::operator_call(__it, a1, a2)
        })
    }
}
value_comparator!(CompareFloatingValues_double_, f64);
impl CppMove for CompareFloatingValues_double_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for CompareFloatingValues_double_ {
    fn default() -> Self {
        { CompareFloatingValues_double_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct CompareFloatingValues_long_double_ {
    #[offset(0)]
    #[byte_size(16)]
    abs_tol: f64,
    #[offset(16)]
    #[byte_size(16)]
    rel_tol: f64,
}
impl CompareFloatingValues_long_double_ {
    pub fn new() -> Self {
        let __this: Value<CompareFloatingValues_long_double_> = Rc::new(RefCell::new(Self {
            abs_tol: 0_f64,
            rel_tol: 0_f64,
        }));
        let this: Ptr<CompareFloatingValues_long_double_> = __this.as_pointer();
        field!(this, abs_tol).write(
            ({
                FnPtr::<fn() -> f64>::new(|| -> f64 {
                    {
                        {
                            {
                                return 1e-12_f64;
                            }
                        }
                        panic!("ub: non-void function does not return a value")
                    }
                })
                .call()
            }),
        );
        field!(this, rel_tol).write(this.with(|__s| __s.abs_tol));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Callable2<f64, f64, bool> for CompareFloatingValues_long_double_ {
    fn call(&self, a1: f64, a2: f64) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CompareFloatingValues_long_double_Impl::operator_call(__it, a1, a2)
        })
    }
}
value_comparator!(CompareFloatingValues_long_double_, f64);
impl CppMove for CompareFloatingValues_long_double_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for CompareFloatingValues_long_double_ {
    fn default() -> Self {
        { CompareFloatingValues_long_double_::new() }
    }
}
pub fn get_roots_10(mut n: u32) -> Vec<f32> {
    'switch: {
        match { n } {
            __v if __v == 0_u32 => {
                return Vec::new();
            }
            __v if __v == 1_u32 => {
                return Vec::from([0_f32]);
            }
            __v if __v == 2_u32 => {
                return Vec::from([(0.707106781186548_f64 as f32)]);
            }
            __v if __v == 3_u32 => {
                return Vec::from([0_f32, (1.224744871391589_f64 as f32)]);
            }
            __v if __v == 4_u32 => {
                return Vec::from([
                    (0.52464762327529_f64 as f32),
                    (1.650680123885785_f64 as f32),
                ]);
            }
            __v if __v == 5_u32 => {
                return Vec::from([
                    0_f32,
                    (0.958572464613819_f64 as f32),
                    (2.020182870456086_f64 as f32),
                ]);
            }
            __v if __v == 6_u32 => {
                return Vec::from([
                    (0.436077411927617_f64 as f32),
                    (1.335849074013697_f64 as f32),
                    (2.350604973674492_f64 as f32),
                ]);
            }
            __v if __v == 7_u32 => {
                return Vec::from([
                    0_f32,
                    (0.816287882858965_f64 as f32),
                    (1.673551628767471_f64 as f32),
                    (2.651961356835233_f64 as f32),
                ]);
            }
            __v if __v == 8_u32 => {
                return Vec::from([
                    (0.381186990207322_f64 as f32),
                    (1.15719371244678_f64 as f32),
                    (1.981656756695843_f64 as f32),
                    (2.930637420257244_f64 as f32),
                ]);
            }
            __v if __v == 9_u32 => {
                return Vec::from([
                    0_f32,
                    (0.723551018752838_f64 as f32),
                    (1.468553289216668_f64 as f32),
                    (2.266580584531843_f64 as f32),
                    (3.190993201781528_f64 as f32),
                ]);
            }
            __v if __v == 10_u32 => {
                return Vec::from([
                    (0.342901327223705_f64 as f32),
                    (1.036610829789514_f64 as f32),
                    (1.756683649299882_f64 as f32),
                    (2.53273167423279_f64 as f32),
                    (3.436159118837738_f64 as f32),
                ]);
            }
            __v if __v == 11_u32 => {
                return Vec::from([
                    0_f32,
                    (0.656809566821_f64 as f32),
                    (1.326557084494933_f64 as f32),
                    (2.025948015825755_f64 as f32),
                    (2.783290099781652_f64 as f32),
                    (3.668470846559583_f64 as f32),
                ]);
            }
            __v if __v == 12_u32 => {
                return Vec::from([
                    (0.314240376254359_f64 as f32),
                    (0.947788391240164_f64 as f32),
                    (1.597682635152605_f64 as f32),
                    (2.27950708050106_f64 as f32),
                    (3.02063702512089_f64 as f32),
                    (3.889724897869782_f64 as f32),
                ]);
            }
            __v if __v == 13_u32 => {
                return Vec::from([
                    0_f32,
                    (0.60576387917106_f64 as f32),
                    (1.220055036590748_f64 as f32),
                    (1.853107651601512_f64 as f32),
                    (2.519735685678238_f64 as f32),
                    (3.24660897837241_f64 as f32),
                    (4.10133759617864_f64 as f32),
                ]);
            }
            __v if __v == 14_u32 => {
                return Vec::from([
                    (0.29174551067256_f64 as f32),
                    (0.8787137873294_f64 as f32),
                    (1.47668273114114_f64 as f32),
                    (2.09518325850772_f64 as f32),
                    (2.7484707249854_f64 as f32),
                    (3.46265693360227_f64 as f32),
                    (4.30444857047363_f64 as f32),
                ]);
            }
            __v if __v == 15_u32 => {
                return Vec::from([
                    (0_f64 as f32),
                    (0.56506958325558_f64 as f32),
                    (1.13611558521092_f64 as f32),
                    (1.71999257518649_f64 as f32),
                    (2.32573248617386_f64 as f32),
                    (2.9671669279056_f64 as f32),
                    (3.66995037340445_f64 as f32),
                    (4.49999070730939_f64 as f32),
                ]);
            }
            __v if __v == 16_u32 => {
                return Vec::from([
                    (0.27348104613815_f64 as f32),
                    (0.82295144914466_f64 as f32),
                    (1.38025853919888_f64 as f32),
                    (1.95178799091625_f64 as f32),
                    (2.54620215784748_f64 as f32),
                    (3.17699916197996_f64 as f32),
                    (3.86944790486012_f64 as f32),
                    (4.68873893930582_f64 as f32),
                ]);
            }
            __v if __v == 17_u32 => {
                return Vec::from([
                    0_f32,
                    (0.5316330013427_f64 as f32),
                    (1.0676487257435_f64 as f32),
                    (1.6129243142212_f64 as f32),
                    (2.1735028266666_f64 as f32),
                    (2.7577629157039_f64 as f32),
                    (3.3789320911415_f64 as f32),
                    (4.0619466758755_f64 as f32),
                    (4.8713451936744_f64 as f32),
                ]);
            }
            __v if __v == 18_u32 => {
                return Vec::from([
                    (0.2582677505191_f64 as f32),
                    (0.7766829192674_f64 as f32),
                    (1.3009208583896_f64 as f32),
                    (1.8355316042616_f64 as f32),
                    (2.3862990891667_f64 as f32),
                    (2.9613775055316_f64 as f32),
                    (3.5737690684863_f64 as f32),
                    (4.2481178735681_f64 as f32),
                    (5.0483640088745_f64 as f32),
                ]);
            }
            __v if __v == 19_u32 => {
                return Vec::from([
                    0_f32,
                    (0.5035201634239_f64 as f32),
                    (1.0103683871343_f64 as f32),
                    (1.5241706193935_f64 as f32),
                    (2.0492317098506_f64 as f32),
                    (2.5911337897945_f64 as f32),
                    (3.1578488183476_f64 as f32),
                    (3.762187351964_f64 as f32),
                    (4.4285328066038_f64 as f32),
                    (5.2202716905375_f64 as f32),
                ]);
            }
            __v if __v == 20_u32 => {
                return Vec::from([
                    (0.2453407083009_f64 as f32),
                    (0.7374737285454_f64 as f32),
                    (1.2340762153953_f64 as f32),
                    (1.7385377121166_f64 as f32),
                    (2.2549740020893_f64 as f32),
                    (2.7888060584281_f64 as f32),
                    (3.347854567332_f64 as f32),
                    (3.9447640401156_f64 as f32),
                    (4.6036824495507_f64 as f32),
                    (5.3874808900112_f64 as f32),
                ]);
            }
            _ => {
                assert!(false);
                return Vec::from([(-42_i32 as f32)]);
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn get_roots_11(mut n: u32) -> Vec<f64> {
    'switch: {
        match { n } {
            __v if __v == 0_u32 => {
                return Vec::new();
            }
            __v if __v == 1_u32 => {
                return Vec::from([0_f64]);
            }
            __v if __v == 2_u32 => {
                return Vec::from([0.707106781186548_f64]);
            }
            __v if __v == 3_u32 => {
                return Vec::from([0_f64, 1.224744871391589_f64]);
            }
            __v if __v == 4_u32 => {
                return Vec::from([0.52464762327529_f64, 1.650680123885785_f64]);
            }
            __v if __v == 5_u32 => {
                return Vec::from([0_f64, 0.958572464613819_f64, 2.020182870456086_f64]);
            }
            __v if __v == 6_u32 => {
                return Vec::from([
                    0.436077411927617_f64,
                    1.335849074013697_f64,
                    2.350604973674492_f64,
                ]);
            }
            __v if __v == 7_u32 => {
                return Vec::from([
                    0_f64,
                    0.816287882858965_f64,
                    1.673551628767471_f64,
                    2.651961356835233_f64,
                ]);
            }
            __v if __v == 8_u32 => {
                return Vec::from([
                    0.381186990207322_f64,
                    1.15719371244678_f64,
                    1.981656756695843_f64,
                    2.930637420257244_f64,
                ]);
            }
            __v if __v == 9_u32 => {
                return Vec::from([
                    0_f64,
                    0.723551018752838_f64,
                    1.468553289216668_f64,
                    2.266580584531843_f64,
                    3.190993201781528_f64,
                ]);
            }
            __v if __v == 10_u32 => {
                return Vec::from([
                    0.342901327223705_f64,
                    1.036610829789514_f64,
                    1.756683649299882_f64,
                    2.53273167423279_f64,
                    3.436159118837738_f64,
                ]);
            }
            __v if __v == 11_u32 => {
                return Vec::from([
                    0_f64,
                    0.656809566821_f64,
                    1.326557084494933_f64,
                    2.025948015825755_f64,
                    2.783290099781652_f64,
                    3.668470846559583_f64,
                ]);
            }
            __v if __v == 12_u32 => {
                return Vec::from([
                    0.314240376254359_f64,
                    0.947788391240164_f64,
                    1.597682635152605_f64,
                    2.27950708050106_f64,
                    3.02063702512089_f64,
                    3.889724897869782_f64,
                ]);
            }
            __v if __v == 13_u32 => {
                return Vec::from([
                    0_f64,
                    0.60576387917106_f64,
                    1.220055036590748_f64,
                    1.853107651601512_f64,
                    2.519735685678238_f64,
                    3.24660897837241_f64,
                    4.10133759617864_f64,
                ]);
            }
            __v if __v == 14_u32 => {
                return Vec::from([
                    0.29174551067256_f64,
                    0.8787137873294_f64,
                    1.47668273114114_f64,
                    2.09518325850772_f64,
                    2.7484707249854_f64,
                    3.46265693360227_f64,
                    4.30444857047363_f64,
                ]);
            }
            __v if __v == 15_u32 => {
                return Vec::from([
                    0_f64,
                    0.56506958325558_f64,
                    1.13611558521092_f64,
                    1.71999257518649_f64,
                    2.32573248617386_f64,
                    2.9671669279056_f64,
                    3.66995037340445_f64,
                    4.49999070730939_f64,
                ]);
            }
            __v if __v == 16_u32 => {
                return Vec::from([
                    0.27348104613815_f64,
                    0.82295144914466_f64,
                    1.38025853919888_f64,
                    1.95178799091625_f64,
                    2.54620215784748_f64,
                    3.17699916197996_f64,
                    3.86944790486012_f64,
                    4.68873893930582_f64,
                ]);
            }
            __v if __v == 17_u32 => {
                return Vec::from([
                    0_f64,
                    0.5316330013427_f64,
                    1.0676487257435_f64,
                    1.6129243142212_f64,
                    2.1735028266666_f64,
                    2.7577629157039_f64,
                    3.3789320911415_f64,
                    4.0619466758755_f64,
                    4.8713451936744_f64,
                ]);
            }
            __v if __v == 18_u32 => {
                return Vec::from([
                    0.2582677505191_f64,
                    0.7766829192674_f64,
                    1.3009208583896_f64,
                    1.8355316042616_f64,
                    2.3862990891667_f64,
                    2.9613775055316_f64,
                    3.5737690684863_f64,
                    4.2481178735681_f64,
                    5.0483640088745_f64,
                ]);
            }
            __v if __v == 19_u32 => {
                return Vec::from([
                    0_f64,
                    0.5035201634239_f64,
                    1.0103683871343_f64,
                    1.5241706193935_f64,
                    2.0492317098506_f64,
                    2.5911337897945_f64,
                    3.1578488183476_f64,
                    3.762187351964_f64,
                    4.4285328066038_f64,
                    5.2202716905375_f64,
                ]);
            }
            __v if __v == 20_u32 => {
                return Vec::from([
                    0.2453407083009_f64,
                    0.7374737285454_f64,
                    1.2340762153953_f64,
                    1.7385377121166_f64,
                    2.2549740020893_f64,
                    2.7888060584281_f64,
                    3.347854567332_f64,
                    3.9447640401156_f64,
                    4.6036824495507_f64,
                    5.3874808900112_f64,
                ]);
            }
            _ => {
                assert!(false);
                return Vec::from([(-42_i32 as f64)]);
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn get_roots_12(mut n: u32) -> Vec<f64> {
    'switch: {
        match { n } {
            __v if __v == 0_u32 => {
                return Vec::new();
            }
            __v if __v == 1_u32 => {
                return Vec::from([0_f64]);
            }
            __v if __v == 2_u32 => {
                return Vec::from([0.707106781186548_f64]);
            }
            __v if __v == 3_u32 => {
                return Vec::from([0_f64, 1.224744871391589_f64]);
            }
            __v if __v == 4_u32 => {
                return Vec::from([0.52464762327529_f64, 1.650680123885785_f64]);
            }
            __v if __v == 5_u32 => {
                return Vec::from([0_f64, 0.958572464613819_f64, 2.020182870456086_f64]);
            }
            __v if __v == 6_u32 => {
                return Vec::from([
                    0.436077411927617_f64,
                    1.335849074013697_f64,
                    2.350604973674492_f64,
                ]);
            }
            __v if __v == 7_u32 => {
                return Vec::from([
                    0_f64,
                    0.816287882858965_f64,
                    1.673551628767471_f64,
                    2.651961356835233_f64,
                ]);
            }
            __v if __v == 8_u32 => {
                return Vec::from([
                    0.381186990207322_f64,
                    1.15719371244678_f64,
                    1.981656756695843_f64,
                    2.930637420257244_f64,
                ]);
            }
            __v if __v == 9_u32 => {
                return Vec::from([
                    0_f64,
                    0.723551018752838_f64,
                    1.468553289216668_f64,
                    2.266580584531843_f64,
                    3.190993201781528_f64,
                ]);
            }
            __v if __v == 10_u32 => {
                return Vec::from([
                    0.342901327223705_f64,
                    1.036610829789514_f64,
                    1.756683649299882_f64,
                    2.53273167423279_f64,
                    3.436159118837738_f64,
                ]);
            }
            __v if __v == 11_u32 => {
                return Vec::from([
                    0_f64,
                    0.656809566821_f64,
                    1.326557084494933_f64,
                    2.025948015825755_f64,
                    2.783290099781652_f64,
                    3.668470846559583_f64,
                ]);
            }
            __v if __v == 12_u32 => {
                return Vec::from([
                    0.314240376254359_f64,
                    0.947788391240164_f64,
                    1.597682635152605_f64,
                    2.27950708050106_f64,
                    3.02063702512089_f64,
                    3.889724897869782_f64,
                ]);
            }
            __v if __v == 13_u32 => {
                return Vec::from([
                    0_f64,
                    0.60576387917106_f64,
                    1.220055036590748_f64,
                    1.853107651601512_f64,
                    2.519735685678238_f64,
                    3.24660897837241_f64,
                    4.10133759617864_f64,
                ]);
            }
            __v if __v == 14_u32 => {
                return Vec::from([
                    0.29174551067256_f64,
                    0.8787137873294_f64,
                    1.47668273114114_f64,
                    2.09518325850772_f64,
                    2.7484707249854_f64,
                    3.46265693360227_f64,
                    4.30444857047363_f64,
                ]);
            }
            __v if __v == 15_u32 => {
                return Vec::from([
                    0_f64,
                    0.56506958325558_f64,
                    1.13611558521092_f64,
                    1.71999257518649_f64,
                    2.32573248617386_f64,
                    2.9671669279056_f64,
                    3.66995037340445_f64,
                    4.49999070730939_f64,
                ]);
            }
            __v if __v == 16_u32 => {
                return Vec::from([
                    0.27348104613815_f64,
                    0.82295144914466_f64,
                    1.38025853919888_f64,
                    1.95178799091625_f64,
                    2.54620215784748_f64,
                    3.17699916197996_f64,
                    3.86944790486012_f64,
                    4.68873893930582_f64,
                ]);
            }
            __v if __v == 17_u32 => {
                return Vec::from([
                    0_f64,
                    0.5316330013427_f64,
                    1.0676487257435_f64,
                    1.6129243142212_f64,
                    2.1735028266666_f64,
                    2.7577629157039_f64,
                    3.3789320911415_f64,
                    4.0619466758755_f64,
                    4.8713451936744_f64,
                ]);
            }
            __v if __v == 18_u32 => {
                return Vec::from([
                    0.2582677505191_f64,
                    0.7766829192674_f64,
                    1.3009208583896_f64,
                    1.8355316042616_f64,
                    2.3862990891667_f64,
                    2.9613775055316_f64,
                    3.5737690684863_f64,
                    4.2481178735681_f64,
                    5.0483640088745_f64,
                ]);
            }
            __v if __v == 19_u32 => {
                return Vec::from([
                    0_f64,
                    0.5035201634239_f64,
                    1.0103683871343_f64,
                    1.5241706193935_f64,
                    2.0492317098506_f64,
                    2.5911337897945_f64,
                    3.1578488183476_f64,
                    3.762187351964_f64,
                    4.4285328066038_f64,
                    5.2202716905375_f64,
                ]);
            }
            __v if __v == 20_u32 => {
                return Vec::from([
                    0.2453407083009_f64,
                    0.7374737285454_f64,
                    1.2340762153953_f64,
                    1.7385377121166_f64,
                    2.2549740020893_f64,
                    2.7888060584281_f64,
                    3.347854567332_f64,
                    3.9447640401156_f64,
                    4.6036824495507_f64,
                    5.3874808900112_f64,
                ]);
            }
            _ => {
                assert!(false);
                return Vec::from([(-42_i32 as f64)]);
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn test_13() {
    {
        let __tmp_0: Value<Vec<f32>> = Rc::new(RefCell::new(Vec::from([
            unimplemented!(),
            unimplemented!(),
        ])));
        let __range2: Ptr<Vec<f32>> = __tmp_0.as_pointer();
        'loop_: for NaN in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut NaN: f32 = (NaN.read());
            let mut n: u32 = 0_u32;
            'loop_: while (n < ({ get_maximal_order_5() })) {
                assert!(unimplemented!());
                n.prefix_inc();
            }
        }
    }
    {
        let __tmp_1: Value<Vec<f32>> = Rc::new(RefCell::new(({ sample_points_7() })));
        let __range2: Ptr<Vec<f32>> = __tmp_1.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: f32 = (x.read());
            let mut n: u32 = 0_u32;
            'loop_: while (n < ({ get_maximal_order_5() })) {
                assert!(!(unimplemented!()));
                n.prefix_inc();
            }
        }
    }
    {
        let h0: Value<FnPtr<fn(f32) -> f32>> = Rc::new(RefCell::new(FnPtr::<fn(f32) -> f32>::new(
            |mut _a0: f32| -> f32 {
                {
                    return 1_f32;
                }
            },
        )));
        let h1: Value<FnPtr<fn(f32) -> f32>> = Rc::new(RefCell::new(FnPtr::<fn(f32) -> f32>::new(
            |mut y: f32| -> f32 {
                {
                    return (2_f32 * y);
                }
            },
        )));
        let h2: Value<FnPtr<fn(f32) -> f32>> = Rc::new(RefCell::new(FnPtr::<fn(f32) -> f32>::new(
            |mut y: f32| -> f32 {
                {
                    return (((4_f32 * y) * y) - 2_f32);
                }
            },
        )));
        let h3: Value<FnPtr<fn(f32) -> f32>> = Rc::new(RefCell::new(FnPtr::<fn(f32) -> f32>::new(
            |mut y: f32| -> f32 {
                {
                    return (y * (((8_f32 * y) * y) - 12_f32));
                }
            },
        )));
        let h4: Value<FnPtr<fn(f32) -> f32>> = Rc::new(RefCell::new(FnPtr::<fn(f32) -> f32>::new(
            |mut y: f32| -> f32 {
                {
                    return ((((16_f64 * unimplemented!()) - (((48_f32 * y) * y) as f64)) + 12_f64)
                        as f32);
                }
            },
        )));
        let h5: Value<FnPtr<fn(f32) -> f32>> = Rc::new(RefCell::new(FnPtr::<fn(f32) -> f32>::new(
            |mut y: f32| -> f32 {
                {
                    return (((y as f64)
                        * (((32_f64 * unimplemented!()) - (((160_f32 * y) * y) as f64)) + 120_f64))
                        as f32);
                }
            },
        )));
        let __tmp_2: Value<Vec<f32>> = Rc::new(RefCell::new(({ sample_points_7() })));
        let __range2: Ptr<Vec<f32>> = __tmp_2.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: f32 = (x.read());
            let compare: Value<CompareFloatingValues_float_> =
                Rc::new(RefCell::new(CompareFloatingValues_float_::new()));
            assert!(
                ({
                    ({
                        let _result: f32 = unimplemented!();
                        let _expected: f32 = ({ (*h0.borrow()).call(x) });
                        CompareFloatingValues_float_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
            assert!(
                ({
                    ({
                        let _result: f32 = unimplemented!();
                        let _expected: f32 = ({ (*h1.borrow()).call(x) });
                        CompareFloatingValues_float_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
            assert!(
                ({
                    ({
                        let _result: f32 = unimplemented!();
                        let _expected: f32 = ({ (*h2.borrow()).call(x) });
                        CompareFloatingValues_float_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
            assert!(
                ({
                    ({
                        let _result: f32 = unimplemented!();
                        let _expected: f32 = ({ (*h3.borrow()).call(x) });
                        CompareFloatingValues_float_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
            assert!(
                ({
                    ({
                        let _result: f32 = unimplemented!();
                        let _expected: f32 = ({ (*h4.borrow()).call(x) });
                        CompareFloatingValues_float_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
            assert!(
                ({
                    ({
                        let _result: f32 = unimplemented!();
                        let _expected: f32 = ({ (*h5.borrow()).call(x) });
                        CompareFloatingValues_float_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
        }
    }
    {
        {
            let mut n: u32 = 0_u32;
            'loop_: while (n < ({ get_maximal_order_5() })) {
                let __tmp_3: Value<Vec<f32>> = Rc::new(RefCell::new(({ sample_points_7() })));
                let __range4: Ptr<Vec<f32>> = __tmp_3.as_pointer();
                'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                    let mut x: f32 = (x.read());
                    assert!((unimplemented!() == unimplemented!()));
                }
                n.prefix_inc();
            }
        }
    }
    {
        {}
    }
    {
        let __tmp_4: Value<Vec<f32>> = Rc::new(RefCell::new(({ sample_points_7() })));
        let __range2: Ptr<Vec<f32>> = __tmp_4.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: f32 = (x.read());
            let mut n: u32 = 1_u32;
            'loop_: while (n < (({ get_maximal_order_5() }) as u32).wrapping_sub(1_u32)) {
                let mut H_next: f32 = unimplemented!();
                let mut H_next_recurrence: f32 =
                    (2_f32 * ((x * unimplemented!()) - ((n as f32) * unimplemented!())));
                if unimplemented!() {
                    break;
                }
                let compare: Value<CompareFloatingValues_float_> =
                    Rc::new(RefCell::new(CompareFloatingValues_float_::new()));
                assert!(
                    ({
                        CompareFloatingValues_float_Impl::operator_call(
                            &compare.as_pointer(),
                            H_next,
                            H_next_recurrence,
                        )
                    })
                );
                n.prefix_inc();
            }
        }
    }
    {
        let tol: Value<f32> = Rc::new(RefCell::new(
            ({
                FnPtr::<fn() -> f32>::new(|| -> f32 {
                    {
                        {
                            return 1e-05_f32;
                        }
                        panic!("ub: non-void function does not return a value")
                    }
                })
                .call()
            }),
        ));
        let is_sign_change: Value<FnPtr<fn(u32, f32) -> bool>> = Rc::new(RefCell::new(lambda!(
            {
                let tol: Value<f32> = Rc::new(RefCell::new((*tol.borrow())));
            },
            |mut n: u32, mut x: f32| -> bool {
                return ((unimplemented!() * unimplemented!()) < 0_f32);
            }
        )));
        let mut n: u32 = 0_u32;
        'loop_: while (n <= 20_u32) {
            let __tmp_5: Value<Vec<f32>> = Rc::new(RefCell::new(({ get_roots_10(n) })));
            let __range3: Ptr<Vec<f32>> = __tmp_5.as_pointer();
            'loop_: for x in IterRange::new(__range3.decay().begin(), __range3.decay().end()) {
                let mut x: f32 = (x.iter_deref().read());
                if (x > 0_f32) {
                    assert!(({ ({ (*is_sign_change.borrow()).call(n, -x,) }) }));
                }
                assert!(({ ({ (*is_sign_change.borrow()).call(n, x,) }) }));
            }
            n.prefix_inc();
        }
    }
    {
        let mut inf: f32 = unimplemented!();
        let mut n: u32 = 1_u32;
        'loop_: while (n < ({ get_maximal_order_5() })) {
            assert!((unimplemented!() == inf));
            assert!((unimplemented!() == (if ((n & 1_u32) != 0) { -inf } else { inf })));
            n.prefix_inc();
        }
    }
    { {} }
}
pub fn test_14() {
    {
        let __tmp_6: Value<Vec<f64>> = Rc::new(RefCell::new(Vec::from([
            unimplemented!(),
            unimplemented!(),
        ])));
        let __range2: Ptr<Vec<f64>> = __tmp_6.as_pointer();
        'loop_: for NaN in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut NaN: f64 = (NaN.read());
            let mut n: u32 = 0_u32;
            'loop_: while (n < ({ get_maximal_order_4() })) {
                assert!(unimplemented!());
                n.prefix_inc();
            }
        }
    }
    {
        let __tmp_7: Value<Vec<f64>> = Rc::new(RefCell::new(({ sample_points_9() })));
        let __range2: Ptr<Vec<f64>> = __tmp_7.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: f64 = (x.read());
            let mut n: u32 = 0_u32;
            'loop_: while (n < ({ get_maximal_order_4() })) {
                assert!(!(unimplemented!()));
                n.prefix_inc();
            }
        }
    }
    {
        let h0: Value<FnPtr<fn(f64) -> f64>> = Rc::new(RefCell::new(FnPtr::<fn(f64) -> f64>::new(
            |mut _a0: f64| -> f64 {
                {
                    return 1_f64;
                }
            },
        )));
        let h1: Value<FnPtr<fn(f64) -> f64>> = Rc::new(RefCell::new(FnPtr::<fn(f64) -> f64>::new(
            |mut y: f64| -> f64 {
                {
                    return (2_f64 * y);
                }
            },
        )));
        let h2: Value<FnPtr<fn(f64) -> f64>> = Rc::new(RefCell::new(FnPtr::<fn(f64) -> f64>::new(
            |mut y: f64| -> f64 {
                {
                    return (((4_f64 * y) * y) - 2_f64);
                }
            },
        )));
        let h3: Value<FnPtr<fn(f64) -> f64>> = Rc::new(RefCell::new(FnPtr::<fn(f64) -> f64>::new(
            |mut y: f64| -> f64 {
                {
                    return (y * (((8_f64 * y) * y) - 12_f64));
                }
            },
        )));
        let h4: Value<FnPtr<fn(f64) -> f64>> = Rc::new(RefCell::new(FnPtr::<fn(f64) -> f64>::new(
            |mut y: f64| -> f64 {
                {
                    return (((16_f64 * unimplemented!()) - ((48_f64 * y) * y)) + 12_f64);
                }
            },
        )));
        let h5: Value<FnPtr<fn(f64) -> f64>> = Rc::new(RefCell::new(FnPtr::<fn(f64) -> f64>::new(
            |mut y: f64| -> f64 {
                {
                    return (y * (((32_f64 * unimplemented!()) - ((160_f64 * y) * y)) + 120_f64));
                }
            },
        )));
        let __tmp_8: Value<Vec<f64>> = Rc::new(RefCell::new(({ sample_points_9() })));
        let __range2: Ptr<Vec<f64>> = __tmp_8.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: f64 = (x.read());
            let compare: Value<CompareFloatingValues_double_> =
                Rc::new(RefCell::new(CompareFloatingValues_double_::new()));
            assert!(
                ({
                    ({
                        let _result: f64 = unimplemented!();
                        let _expected: f64 = ({ (*h0.borrow()).call(x) });
                        CompareFloatingValues_double_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
            assert!(
                ({
                    ({
                        let _result: f64 = unimplemented!();
                        let _expected: f64 = ({ (*h1.borrow()).call(x) });
                        CompareFloatingValues_double_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
            assert!(
                ({
                    ({
                        let _result: f64 = unimplemented!();
                        let _expected: f64 = ({ (*h2.borrow()).call(x) });
                        CompareFloatingValues_double_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
            assert!(
                ({
                    ({
                        let _result: f64 = unimplemented!();
                        let _expected: f64 = ({ (*h3.borrow()).call(x) });
                        CompareFloatingValues_double_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
            assert!(
                ({
                    ({
                        let _result: f64 = unimplemented!();
                        let _expected: f64 = ({ (*h4.borrow()).call(x) });
                        CompareFloatingValues_double_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
            assert!(
                ({
                    ({
                        let _result: f64 = unimplemented!();
                        let _expected: f64 = ({ (*h5.borrow()).call(x) });
                        CompareFloatingValues_double_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
        }
    }
    {
        {}
    }
    {
        {}
    }
    {
        let __tmp_9: Value<Vec<f64>> = Rc::new(RefCell::new(({ sample_points_9() })));
        let __range2: Ptr<Vec<f64>> = __tmp_9.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: f64 = (x.read());
            let mut n: u32 = 1_u32;
            'loop_: while (n < (({ get_maximal_order_4() }) as u32).wrapping_sub(1_u32)) {
                let mut H_next: f64 = unimplemented!();
                let mut H_next_recurrence: f64 =
                    (2_f64 * ((x * unimplemented!()) - ((n as f64) * unimplemented!())));
                if unimplemented!() {
                    break;
                }
                let compare: Value<CompareFloatingValues_double_> =
                    Rc::new(RefCell::new(CompareFloatingValues_double_::new()));
                assert!(
                    ({
                        CompareFloatingValues_double_Impl::operator_call(
                            &compare.as_pointer(),
                            H_next,
                            H_next_recurrence,
                        )
                    })
                );
                n.prefix_inc();
            }
        }
    }
    {
        let tol: Value<f64> = Rc::new(RefCell::new(
            ({
                FnPtr::<fn() -> f64>::new(|| -> f64 {
                    {
                        {
                            {
                                return 1e-09_f64;
                            }
                        }
                        panic!("ub: non-void function does not return a value")
                    }
                })
                .call()
            }),
        ));
        let is_sign_change: Value<FnPtr<fn(u32, f64) -> bool>> = Rc::new(RefCell::new(lambda!(
            {
                let tol: Value<f64> = Rc::new(RefCell::new((*tol.borrow())));
            },
            |mut n: u32, mut x: f64| -> bool {
                return ((unimplemented!() * unimplemented!()) < 0_f64);
            }
        )));
        let mut n: u32 = 0_u32;
        'loop_: while (n <= 20_u32) {
            let __tmp_10: Value<Vec<f64>> = Rc::new(RefCell::new(({ get_roots_11(n) })));
            let __range3: Ptr<Vec<f64>> = __tmp_10.as_pointer();
            'loop_: for x in IterRange::new(__range3.decay().begin(), __range3.decay().end()) {
                let mut x: f64 = (x.iter_deref().read());
                if (x > 0_f64) {
                    assert!(({ ({ (*is_sign_change.borrow()).call(n, -x,) }) }));
                }
                assert!(({ ({ (*is_sign_change.borrow()).call(n, x,) }) }));
            }
            n.prefix_inc();
        }
    }
    {
        let mut inf: f64 = unimplemented!();
        let mut n: u32 = 1_u32;
        'loop_: while (n < ({ get_maximal_order_4() })) {
            assert!((unimplemented!() == inf));
            assert!((unimplemented!() == (if ((n & 1_u32) != 0) { -inf } else { inf })));
            n.prefix_inc();
        }
    }
    {
        {
            const _: () = assert!(
                (::std::mem::size_of::<f64>() == 8_usize),
                "sizeof(double) == 8"
            );;
            let mut n: u32 = 0_u32;
            'loop_: while (n < ({ get_maximal_order_4() })) {
                if (n < 111_u32) {
                    assert!(unimplemented!());
                    assert!(unimplemented!());
                } else {
                    let mut inf: f64 = unimplemented!();
                    assert!((unimplemented!() == inf));
                    assert!((unimplemented!() == (if ((n & 1_u32) != 0) { -inf } else { inf })));
                }
                n.prefix_inc();
            }
        }
    }
}
pub fn test_15() {
    {
        let __tmp_11: Value<Vec<f64>> = Rc::new(RefCell::new(Vec::from([
            unimplemented!(),
            unimplemented!(),
        ])));
        let __range2: Ptr<Vec<f64>> = __tmp_11.as_pointer();
        'loop_: for NaN in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut NaN: f64 = (NaN.read());
            let mut n: u32 = 0_u32;
            'loop_: while (n < ({ get_maximal_order_6() })) {
                assert!(unimplemented!());
                n.prefix_inc();
            }
        }
    }
    {
        let __tmp_12: Value<Vec<f64>> = Rc::new(RefCell::new(({ sample_points_8() })));
        let __range2: Ptr<Vec<f64>> = __tmp_12.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: f64 = (x.read());
            let mut n: u32 = 0_u32;
            'loop_: while (n < ({ get_maximal_order_6() })) {
                assert!(!(unimplemented!()));
                n.prefix_inc();
            }
        }
    }
    {
        let h0: Value<FnPtr<fn(f64) -> f64>> = Rc::new(RefCell::new(FnPtr::<fn(f64) -> f64>::new(
            |mut _a0: f64| -> f64 {
                {
                    return 1_f64;
                }
            },
        )));
        let h1: Value<FnPtr<fn(f64) -> f64>> = Rc::new(RefCell::new(FnPtr::<fn(f64) -> f64>::new(
            |mut y: f64| -> f64 {
                {
                    return (2_f64 * y);
                }
            },
        )));
        let h2: Value<FnPtr<fn(f64) -> f64>> = Rc::new(RefCell::new(FnPtr::<fn(f64) -> f64>::new(
            |mut y: f64| -> f64 {
                {
                    return (((4_f64 * y) * y) - 2_f64);
                }
            },
        )));
        let h3: Value<FnPtr<fn(f64) -> f64>> = Rc::new(RefCell::new(FnPtr::<fn(f64) -> f64>::new(
            |mut y: f64| -> f64 {
                {
                    return (y * (((8_f64 * y) * y) - 12_f64));
                }
            },
        )));
        let h4: Value<FnPtr<fn(f64) -> f64>> = Rc::new(RefCell::new(FnPtr::<fn(f64) -> f64>::new(
            |mut y: f64| -> f64 {
                {
                    return (((16_f64 * unimplemented!()) - ((48_f64 * y) * y)) + 12_f64);
                }
            },
        )));
        let h5: Value<FnPtr<fn(f64) -> f64>> = Rc::new(RefCell::new(FnPtr::<fn(f64) -> f64>::new(
            |mut y: f64| -> f64 {
                {
                    return (y * (((32_f64 * unimplemented!()) - ((160_f64 * y) * y)) + 120_f64));
                }
            },
        )));
        let __tmp_13: Value<Vec<f64>> = Rc::new(RefCell::new(({ sample_points_8() })));
        let __range2: Ptr<Vec<f64>> = __tmp_13.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: f64 = (x.read());
            let compare: Value<CompareFloatingValues_long_double_> =
                Rc::new(RefCell::new(CompareFloatingValues_long_double_::new()));
            assert!(
                ({
                    ({
                        let _result: f64 = unimplemented!();
                        let _expected: f64 = ({ (*h0.borrow()).call(x) });
                        CompareFloatingValues_long_double_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
            assert!(
                ({
                    ({
                        let _result: f64 = unimplemented!();
                        let _expected: f64 = ({ (*h1.borrow()).call(x) });
                        CompareFloatingValues_long_double_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
            assert!(
                ({
                    ({
                        let _result: f64 = unimplemented!();
                        let _expected: f64 = ({ (*h2.borrow()).call(x) });
                        CompareFloatingValues_long_double_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
            assert!(
                ({
                    ({
                        let _result: f64 = unimplemented!();
                        let _expected: f64 = ({ (*h3.borrow()).call(x) });
                        CompareFloatingValues_long_double_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
            assert!(
                ({
                    ({
                        let _result: f64 = unimplemented!();
                        let _expected: f64 = ({ (*h4.borrow()).call(x) });
                        CompareFloatingValues_long_double_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
            assert!(
                ({
                    ({
                        let _result: f64 = unimplemented!();
                        let _expected: f64 = ({ (*h5.borrow()).call(x) });
                        CompareFloatingValues_long_double_Impl::operator_call(
                            &compare.as_pointer(),
                            _result,
                            _expected,
                        )
                    })
                })
            );
        }
    }
    {
        {}
    }
    {
        {
            let mut n: u32 = 0_u32;
            'loop_: while (n < ({ get_maximal_order_6() })) {
                let __tmp_14: Value<Vec<f64>> = Rc::new(RefCell::new(({ sample_points_8() })));
                let __range4: Ptr<Vec<f64>> = __tmp_14.as_pointer();
                'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                    let mut x: f64 = (x.read());
                    assert!((unimplemented!() == unimplemented!()));
                }
                n.prefix_inc();
            }
        }
    }
    {
        let __tmp_15: Value<Vec<f64>> = Rc::new(RefCell::new(({ sample_points_8() })));
        let __range2: Ptr<Vec<f64>> = __tmp_15.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: f64 = (x.read());
            let mut n: u32 = 1_u32;
            'loop_: while (n < (({ get_maximal_order_6() }) as u32).wrapping_sub(1_u32)) {
                let mut H_next: f64 = unimplemented!();
                let mut H_next_recurrence: f64 =
                    (2_f64 * ((x * unimplemented!()) - ((n as f64) * unimplemented!())));
                if unimplemented!() {
                    break;
                }
                let compare: Value<CompareFloatingValues_long_double_> =
                    Rc::new(RefCell::new(CompareFloatingValues_long_double_::new()));
                assert!(
                    ({
                        CompareFloatingValues_long_double_Impl::operator_call(
                            &compare.as_pointer(),
                            H_next,
                            H_next_recurrence,
                        )
                    })
                );
                n.prefix_inc();
            }
        }
    }
    {
        let tol: Value<f64> = Rc::new(RefCell::new(
            ({
                FnPtr::<fn() -> f64>::new(|| -> f64 {
                    {
                        {
                            {
                                return 1e-10_f64;
                            }
                        }
                        panic!("ub: non-void function does not return a value")
                    }
                })
                .call()
            }),
        ));
        let is_sign_change: Value<FnPtr<fn(u32, f64) -> bool>> = Rc::new(RefCell::new(lambda!(
            {
                let tol: Value<f64> = Rc::new(RefCell::new((*tol.borrow())));
            },
            |mut n: u32, mut x: f64| -> bool {
                return ((unimplemented!() * unimplemented!()) < 0_f64);
            }
        )));
        let mut n: u32 = 0_u32;
        'loop_: while (n <= 20_u32) {
            let __tmp_16: Value<Vec<f64>> = Rc::new(RefCell::new(({ get_roots_12(n) })));
            let __range3: Ptr<Vec<f64>> = __tmp_16.as_pointer();
            'loop_: for x in IterRange::new(__range3.decay().begin(), __range3.decay().end()) {
                let mut x: f64 = (x.iter_deref().read());
                if (x > 0_f64) {
                    assert!(({ ({ (*is_sign_change.borrow()).call(n, -x,) }) }));
                }
                assert!(({ ({ (*is_sign_change.borrow()).call(n, x,) }) }));
            }
            n.prefix_inc();
        }
    }
    {
        let mut inf: f64 = unimplemented!();
        let mut n: u32 = 1_u32;
        'loop_: while (n < ({ get_maximal_order_6() })) {
            assert!((unimplemented!() == inf));
            assert!((unimplemented!() == (if ((n & 1_u32) != 0) { -inf } else { inf })));
            n.prefix_inc();
        }
    }
    { {} }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFloat {}
impl Callable0<()> for TestFloat {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFloatImpl::operator_call_1_float(__it)
        })
    }
}
impl CppMove for TestFloat {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestInt {}
impl Callable0<()> for TestInt {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestIntImpl::operator_call_1_short(__it)
        })
    }
}
impl CppMove for TestInt {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({
        for_each_0(
            <types_type_list_float__double__long_double_>::default(),
            <TestFloat>::default(),
        )
    });
    ({
        for_each_2(
            <types_type_list_short__int__long__long_long_>::default(),
            <TestInt>::default(),
        )
    });
    return 0;
}
pub trait CompareFloatingValues_double_Impl {
    fn operator_call(&self, result: f64, expected: f64) -> bool;
}
impl CompareFloatingValues_double_Impl for Ptr<CompareFloatingValues_double_> {
    fn operator_call(&self, mut result: f64, mut expected: f64) -> bool {
        if (unimplemented!()) && (unimplemented!()) {
            return (result == expected);
        }
        if (unimplemented!()) || (unimplemented!()) {
            return false;
        }
        let mut tol: f64 = ((*self).with(|__s| __s.abs_tol)
            + (unimplemented!() * (*self).with(|__s| __s.rel_tol)));
        return (unimplemented!() < tol);
    }
}
pub trait CompareFloatingValues_float_Impl {
    fn operator_call(&self, result: f32, expected: f32) -> bool;
}
impl CompareFloatingValues_float_Impl for Ptr<CompareFloatingValues_float_> {
    fn operator_call(&self, mut result: f32, mut expected: f32) -> bool {
        if (unimplemented!()) && (unimplemented!()) {
            return (result == expected);
        }
        if (unimplemented!()) || (unimplemented!()) {
            return false;
        }
        let mut tol: f32 = ((*self).with(|__s| __s.abs_tol)
            + (unimplemented!() * (*self).with(|__s| __s.rel_tol)));
        return (unimplemented!() < tol);
    }
}
pub trait CompareFloatingValues_long_double_Impl {
    fn operator_call(&self, result: f64, expected: f64) -> bool;
}
impl CompareFloatingValues_long_double_Impl for Ptr<CompareFloatingValues_long_double_> {
    fn operator_call(&self, mut result: f64, mut expected: f64) -> bool {
        if (unimplemented!()) && (unimplemented!()) {
            return (result == expected);
        }
        if (unimplemented!()) || (unimplemented!()) {
            return false;
        }
        let mut tol: f64 = ((*self).with(|__s| __s.abs_tol)
            + (unimplemented!() * (*self).with(|__s| __s.rel_tol)));
        return (unimplemented!() < tol);
    }
}
pub trait TestFloatImpl {
    fn operator_call_1_float(&self);
    fn operator_call_1_double(&self);
    fn operator_call_1_long_double(&self);
}
impl TestFloatImpl for Ptr<TestFloat> {
    fn operator_call_1_float(&self) {
        ({ test_13() });
    }
    fn operator_call_1_double(&self) {
        ({ test_14() });
    }
    fn operator_call_1_long_double(&self) {
        ({ test_15() });
    }
}
pub trait TestIntImpl {
    fn operator_call_1_short(&self);
    fn operator_call_1_int(&self);
    fn operator_call_1_long(&self);
    fn operator_call_1_long_long(&self);
}
impl TestIntImpl for Ptr<TestInt> {
    fn operator_call_1_short(&self) {
        let mut n: u32 = 0_u32;
        'loop_: while (n < ({ get_maximal_order_4() })) {
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: i16 = ((x.read()) as i16);
                assert!((unimplemented!() == unimplemented!()));
            }
            n.prefix_inc();
        }
    }
    fn operator_call_1_int(&self) {
        let mut n: u32 = 0_u32;
        'loop_: while (n < ({ get_maximal_order_4() })) {
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: i32 = (x.read());
                assert!((unimplemented!() == unimplemented!()));
            }
            n.prefix_inc();
        }
    }
    fn operator_call_1_long(&self) {
        let mut n: u32 = 0_u32;
        'loop_: while (n < ({ get_maximal_order_4() })) {
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: i64 = ((x.read()) as i64);
                assert!((unimplemented!() == unimplemented!()));
            }
            n.prefix_inc();
        }
    }
    fn operator_call_1_long_long(&self) {
        let mut n: u32 = 0_u32;
        'loop_: while (n < ({ get_maximal_order_4() })) {
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: i64 = ((x.read()) as i64);
                assert!((unimplemented!() == unimplemented!()));
            }
            n.prefix_inc();
        }
    }
}
pub fn __cpp2rust_init_globals() {}
