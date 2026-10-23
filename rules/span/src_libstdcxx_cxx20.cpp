#include <algorithm>
#include <array>
#include <cstddef>
#include <span>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, std::size_t T2>
std::span<const std::byte,
          T2 == std::dynamic_extent ? std::dynamic_extent : T2 * sizeof(T1)>
f19(std::span<T1, T2> a0) {
  return std::as_bytes<T1, T2>(a0);
}

template <typename T1, std::size_t T2>
std::span<std::byte,
          T2 == std::dynamic_extent ? std::dynamic_extent : T2 * sizeof(T1)>
f20(std::span<T1, T2> a0) {
  return std::as_writable_bytes<T1, T2>(a0);
}

template <typename T1, std::size_t T2> std::span<T1, T2> f21() {
  return std::span<T1, T2>();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::element_type &f22(const std::span<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::element_type &f23(const std::span<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::element_type &
f24(const std::span<T1, T2> &a0, typename std::span<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, std::size_t T2, std::size_t T3>
std::span<T1, T2> f25(typename std::type_identity<T1>::type (&a0)[T3]) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3, std::size_t T4>
std::span<T1, T2> f26(std::array<T3, T4> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3, std::size_t T4>
std::span<T1, T2> f27(const std::array<T3, T4> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3, std::size_t T4>
std::span<T1, T2> f28(const std::span<T3, T4> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2> std::size_t f29() {
  return std::span<T1, T2>::extent;
}
