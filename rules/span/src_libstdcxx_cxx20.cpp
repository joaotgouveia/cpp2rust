#include <algorithm>
#include <array>
#include <cstddef>
#include <span>
#include <utility>

template <typename T1, std::size_t T2>
std::span<const std::byte,
          T2 == std::dynamic_extent ? std::dynamic_extent : T2 * sizeof(T1)>
f24(std::span<T1, T2> a0) {
  return std::as_bytes<T1, T2>(a0);
}

template <typename T1, std::size_t T2>
std::span<std::byte,
          T2 == std::dynamic_extent ? std::dynamic_extent : T2 * sizeof(T1)>
f25(std::span<T1, T2> a0) {
  return std::as_writable_bytes<T1, T2>(a0);
}

template <typename T1, std::size_t T2> std::span<T1, T2> f26() {
  return std::span<T1, T2>();
}

template <typename T1, std::size_t T2, std::size_t T3>
std::span<T1, T2> f27(typename std::type_identity<T1>::type (&a0)[T3]) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3, std::size_t T4>
std::span<T1, T2> f28(std::array<T3, T4> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3, std::size_t T4>
std::span<T1, T2> f29(const std::array<T3, T4> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3, std::size_t T4>
std::span<T1, T2> f30(const std::span<T3, T4> &a0) {
  return std::span<T1, T2>(a0);
}
