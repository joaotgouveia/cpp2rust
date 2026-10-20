#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <barrier>
#include <chrono>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
auto f57(const T2 &a0, const std::remove_extent_t<T1> &a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2> auto f58(const T2 &a0) {
  return std::allocate_shared<T1, T2>(a0);
}

template <typename T1, typename T2>
auto f59(const T2 &a0, std::size_t a1, const std::remove_extent_t<T1> &a2) {
  return std::allocate_shared<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> auto f60(const T2 &a0, std::size_t a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2> auto f61(const T2 &a0) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0);
}

template <typename T1, typename T2> auto f62(const T2 &a0, std::size_t a1) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f63(std::shared_ptr<T2> &&a0) {
  return std::const_pointer_cast<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::shared_ptr<T1> f64(std::shared_ptr<T2> &&a0) {
  return std::dynamic_pointer_cast<T1, T2>(std::move(a0));
}

template <typename T1> auto f65(const std::remove_extent_t<T1> &a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1> auto f66() { return std::make_shared<T1>(); }

template <typename T1>
auto f67(std::size_t a0, const std::remove_extent_t<T1> &a1) {
  return std::make_shared<T1>(a0, a1);
}

template <typename T1> auto f68(std::size_t a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1> auto f69() {
  return std::make_shared_for_overwrite<T1>();
}

template <typename T1> auto f70(std::size_t a0) {
  return std::make_shared_for_overwrite<T1>(a0);
}

template <typename T1>
std::strong_ordering f71(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2>
std::strong_ordering f72(const std::shared_ptr<T1> &a0,
                         const std::shared_ptr<T2> &a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2>
std::shared_ptr<T1> f73(std::shared_ptr<T2> &&a0) {
  return std::reinterpret_pointer_cast<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::shared_ptr<T1> f74(std::shared_ptr<T2> &&a0,
                        typename std::shared_ptr<T1>::element_type *a1) {
  return std::shared_ptr<T1>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f75(std::shared_ptr<T2> &&a0) {
  return std::static_pointer_cast<T1, T2>(std::move(a0));
}
