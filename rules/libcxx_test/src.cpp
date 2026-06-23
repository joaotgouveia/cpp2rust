// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cpp2rust_libcxx_test.h>

template <typename T1> void f1(T1 a0) { return ::cpp2rust_expect_death(a0); }
