#include <algorithm>
#include <barrier>
#include <chrono>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::shared_ptr<T1> f58(const T2 &a0, const std::remove_extent_t<T1> &a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2> std::shared_ptr<T1> f59(const T2 &a0) {
  return std::allocate_shared<T1, T2>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f60(const T2 &a0, std::size_t a1,
                        const std::remove_extent_t<T1> &a2) {
  return std::allocate_shared<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f61(const T2 &a0, std::size_t a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename... T3>
std::shared_ptr<T1> f62(const T2 &a0, T3 &&...a1) {
  return std::allocate_shared<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}

template <typename T1, typename T2> std::shared_ptr<T1> f63(const T2 &a0) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f64(const T2 &a0, std::size_t a1) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0, a1);
}

template <typename T1>
std::shared_ptr<T1> f65(const std::remove_extent_t<T1> &a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1> std::shared_ptr<T1> f66() {
  return std::make_shared<T1>();
}

template <typename T1>
std::shared_ptr<T1> f67(std::size_t a0, const std::remove_extent_t<T1> &a1) {
  return std::make_shared<T1>(a0, a1);
}

template <typename T1> std::shared_ptr<T1> f68(std::size_t a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1, typename... T2> std::shared_ptr<T1> f69(T2 &&...a0) {
  return std::make_shared<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1> std::shared_ptr<T1> f70() {
  return std::make_shared_for_overwrite<T1>();
}

template <typename T1> std::shared_ptr<T1> f71(std::size_t a0) {
  return std::make_shared_for_overwrite<T1>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f72(std::shared_ptr<T2> &&a0,
                        typename std::shared_ptr<T1>::element_type *a1) {
  return std::shared_ptr<T1>(std::move(a0), a1);
}
