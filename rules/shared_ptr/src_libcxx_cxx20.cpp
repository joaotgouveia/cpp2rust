#include <atomic>
#include <compare>
#include <cstddef>
#include <memory>
#include <type_traits>
#include <utility>

template <
    typename T1, typename T2,
    typename std::enable_if<std::is_bounded_array<T1>::value, int>::type T3 = 0>
std::shared_ptr<T1> f92(const T2 &a0, const std::remove_extent_t<T1> &a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <
    typename T1, typename T2,
    typename std::enable_if<std::is_bounded_array<T1>::value, int>::type T3 = 0>
std::shared_ptr<T1> f93(const T2 &a0) {
  return std::allocate_shared<T1, T2>(a0);
}

template <typename T1, typename T2,
          typename std::enable_if<std::is_unbounded_array<T1>::value, int>::type
              T3 = 0>
std::shared_ptr<T1> f94(const T2 &a0, std::size_t a1,
                        const std::remove_extent_t<T1> &a2) {
  return std::allocate_shared<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2,
          typename std::enable_if<std::is_unbounded_array<T1>::value, int>::type
              T3 = 0>
std::shared_ptr<T1> f95(const T2 &a0, std::size_t a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          typename std::enable_if<!std::is_array<T1>::value, int>::type T3 = 0>
std::shared_ptr<T1> f96(const T2 &a0) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0);
}

template <
    typename T1, typename T2,
    typename std::enable_if<std::is_bounded_array<T1>::value, int>::type T3 = 0>
std::shared_ptr<T1> f97(const T2 &a0) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0);
}

template <typename T1, typename T2,
          typename std::enable_if<std::is_unbounded_array<T1>::value, int>::type
              T3 = 0>
std::shared_ptr<T1> f98(const T2 &a0, std::size_t a1) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0, a1);
}

template <typename T1, typename std::enable_if<std::is_bounded_array<T1>::value,
                                               int>::type T2 = 0>
std::shared_ptr<T1> f99(const std::remove_extent_t<T1> &a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1, typename std::enable_if<std::is_bounded_array<T1>::value,
                                               int>::type T2 = 0>
std::shared_ptr<T1> f100() {
  return std::make_shared<T1>();
}

template <typename T1,
          typename std::enable_if<std::is_unbounded_array<T1>::value, int>::type
              T2 = 0>
std::shared_ptr<T1> f101(std::size_t a0, const std::remove_extent_t<T1> &a1) {
  return std::make_shared<T1>(a0, a1);
}

template <typename T1,
          typename std::enable_if<std::is_unbounded_array<T1>::value, int>::type
              T2 = 0>
std::shared_ptr<T1> f102(std::size_t a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1,
          typename std::enable_if<!std::is_array<T1>::value, int>::type T2 = 0>
std::shared_ptr<T1> f103() {
  return std::make_shared_for_overwrite<T1>();
}

template <typename T1, typename std::enable_if<std::is_bounded_array<T1>::value,
                                               int>::type T2 = 0>
std::shared_ptr<T1> f104() {
  return std::make_shared_for_overwrite<T1>();
}

template <typename T1,
          typename std::enable_if<std::is_unbounded_array<T1>::value, int>::type
              T2 = 0>
std::shared_ptr<T1> f105(std::size_t a0) {
  return std::make_shared_for_overwrite<T1>(a0);
}
