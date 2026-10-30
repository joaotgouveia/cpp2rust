// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: f64) -> f64 {
    a0.cos()
}

unsafe fn f2(a0: f64) -> f64 {
    a0.round()
}

unsafe fn f3(a0: f64) -> f64 {
    a0.sin()
}

unsafe fn f99(a0: f64) -> f64 {
    a0.log2()
}

unsafe fn f167(a0: *const ::core::ffi::c_char) -> f64 {
    f64::NAN
}

unsafe fn f168(a0: *const ::core::ffi::c_char) -> f32 {
    f32::NAN
}

unsafe fn f169(a0: *const ::core::ffi::c_char) -> f64 {
    f64::NAN
}
