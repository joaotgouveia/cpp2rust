#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <array>
#include <cstddef>
#include <span>
#include <utility>

template <typename T1, std::size_t T2>
using t16 = typename std::span<T1, T2>::const_iterator;

template <typename T1, std::size_t T2>
using t17 = typename std::span<T1, T2>::const_reverse_iterator;

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::const_iterator f30(const std::span<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::const_iterator f31(const std::span<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::const_reverse_iterator
f32(const std::span<T1, T2> &a0) {
  return a0.crbegin();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::const_reverse_iterator
f33(const std::span<T1, T2> &a0) {
  return a0.crend();
}
