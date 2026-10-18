// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>
#include <sys/types.h>

using t1 = std::byte;
using t2 = size_t;
using t3 = ptrdiff_t;
using t4 = std::ptrdiff_t;
using t5 = __typeof__(sizeof(0));
using t6 = ssize_t;
using t9 = std::size_t;

std::byte f1(const std::byte &a0, unsigned a1) { return a0 << a1; }

std::byte f2(const std::byte &a0, unsigned a1) { return a0 >> a1; }

std::byte f3(std::byte &a0, unsigned a1) { return a0 <<= a1; }

std::byte f4(std::byte &a0, unsigned a1) { return a0 >>= a1; }

template <typename T1> std::byte f5(std::byte a0, T1 a1) { return a0 << a1; }
