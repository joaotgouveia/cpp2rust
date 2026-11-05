// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <algorithm>
#include <deque>
#include <ext/algorithm>
#include <iterator>
#include <memory>

std::size_t f1(std::size_t a0) { return std::__deque_buf_size(a0); }

template <typename T1>
const T1 &f2(const T1 &a0, const T1 &a1, const T1 &a2) {
  return __gnu_cxx::__median(a0, a1, a2);
}

template <typename T1> T1 f3(T1 a0, T1 a1, T1 a2) {
  return std::__rotate(a0, a1, a2);
}

template <typename T1, typename T2> T1 f4(T1 a0, T2 a1) {
  return std::__uninitialized_default_n(a0, a1);
}

template <typename T1>
auto f5(const T1 &a0) -> decltype(std::__iterator_category(a0)) {
  return std::__iterator_category(a0);
}

template <typename T1, typename T2>
const T1 &f6(const T1 &a0, const T1 &a1, const T1 &a2, T2 a3) {
  return __gnu_cxx::__median(a0, a1, a2, a3);
}

template <typename T1, typename T2> T1 f7(T1 a0, T1 a1, T1 a2, T2 a3) {
  return std::__rotate(a0, a1, a2, a3);
}
