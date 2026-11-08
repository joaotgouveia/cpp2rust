// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1<T1: libcc2rs::Integral, const T2: i128>() -> libcc2rs::IntegralConstant<T1, T2> {
    libcc2rs::IntegralConstant(std::marker::PhantomData)
}
