#include "../common/init.h"
#include "../common/traits.h"
#include <atomic>
#include <compare>
#include <cstddef>
#include <memory>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::shared_ptr<T1> f105(const T2 &a0, const std::remove_extent_t<T1> &a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2> std::shared_ptr<T1> f106(const T2 &a0) {
  return std::allocate_shared<T1, T2>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f107(const T2 &a0, std::size_t a1,
                         const std::remove_extent_t<T1> &a2) {
  return std::allocate_shared<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f108(const T2 &a0, std::size_t a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2> std::shared_ptr<T1> f109(const T2 &a0) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f110(const T2 &a0, std::size_t a1) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0, a1);
}

template <typename T1>
std::shared_ptr<T1> f111(const std::remove_extent_t<T1> &a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1> std::shared_ptr<T1> f112() {
  return std::make_shared<T1>();
}

template <typename T1>
std::shared_ptr<T1> f113(std::size_t a0, const std::remove_extent_t<T1> &a1) {
  return std::make_shared<T1>(a0, a1);
}

template <typename T1> std::shared_ptr<T1> f114(std::size_t a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1> std::shared_ptr<T1> f115() {
  return std::make_shared_for_overwrite<T1>();
}

template <typename T1> std::shared_ptr<T1> f116(std::size_t a0) {
  return std::make_shared_for_overwrite<T1>(a0);
}
