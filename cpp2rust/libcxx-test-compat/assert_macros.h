// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <assert.h>
#include_next <assert_macros.h>

#undef TEST_REQUIRE
#undef TEST_FAIL

#define TEST_REQUIRE(CONDITION, ARG) cpp2rust_assert_fail(CONDITION)
#define TEST_FAIL(ARG) cpp2rust_assert_fail(false)
