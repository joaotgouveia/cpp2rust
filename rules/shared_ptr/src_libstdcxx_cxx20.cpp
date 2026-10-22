#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <barrier>
#include <chrono>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
auto f64(const T2 &a0, const std::remove_extent_t<T1> &a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2> auto f65(const T2 &a0) {
  return std::allocate_shared<T1, T2>(a0);
}

template <typename T1, typename T2>
auto f66(const T2 &a0, std::size_t a1, const std::remove_extent_t<T1> &a2) {
  return std::allocate_shared<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> auto f67(const T2 &a0, std::size_t a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2> auto f68(const T2 &a0) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0);
}

template <typename T1, typename T2> auto f69(const T2 &a0, std::size_t a1) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0, a1);
}

template <typename T1> auto f70(const std::remove_extent_t<T1> &a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1> auto f71() { return std::make_shared<T1>(); }

template <typename T1>
auto f72(std::size_t a0, const std::remove_extent_t<T1> &a1) {
  return std::make_shared<T1>(a0, a1);
}

template <typename T1> auto f73(std::size_t a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1> auto f74() {
  return std::make_shared_for_overwrite<T1>();
}

template <typename T1> auto f75(std::size_t a0) {
  return std::make_shared_for_overwrite<T1>(a0);
}
