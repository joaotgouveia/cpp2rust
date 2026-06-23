// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifndef CPP2RUST_LIBCXX_TEST_CHECK_ASSERTION_H
#define CPP2RUST_LIBCXX_TEST_CHECK_ASSERTION_H

#include <cpp2rust_libcxx_test.h>

#define TEST_LIBCPP_ASSERT_FAILURE(expr, message)                              \
  cpp2rust_expect_death([&] { (void)(expr); })
#define EXPECT_DEATH(...) cpp2rust_expect_death([&] { __VA_ARGS__; })
#define EXPECT_ANY_DEATH(...) cpp2rust_expect_death([&] { __VA_ARGS__; })
#define EXPECT_DEATH_MATCHES(matcher, ...)                                     \
  cpp2rust_expect_death([&] { __VA_ARGS__; })
#define EXPECT_STD_ABORT(...) cpp2rust_expect_death([&] { __VA_ARGS__; })
#define EXPECT_STD_TERMINATE(...) cpp2rust_expect_death([&] { __VA_ARGS__; })

#endif // CPP2RUST_LIBCXX_TEST_CHECK_ASSERTION_H
