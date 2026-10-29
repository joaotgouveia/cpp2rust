// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <array>
#include <cstddef>
#include <initializer_list>

template <typename T1, std::size_t T2>
std::initializer_list<T1> f1(const T1 (&a0)[T2]) {
  return std::initializer_list<T1>{a0};
}

template <typename T1, std::size_t T2> std::array<T1, T2> f2(T1 (&&a0)[T2]) {
  return std::array<T1, T2>{a0};
}
