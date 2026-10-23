#include <atomic>
#include <compare>
#include <cstddef>
#include <memory>
#include <type_traits>
#include <utility>

template <typename T1, typename T2,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f122(const T2 &a0, const std::remove_extent_t<T1> &a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f123(const T2 &a0) {
  return std::allocate_shared<T1, T2>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f124(const T2 &a0, std::size_t a1,
                         const std::remove_extent_t<T1> &a2) {
  return std::allocate_shared<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f125(const T2 &a0, std::size_t a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<!std::is_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f126(const T2 &a0) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f127(const T2 &a0) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f128(const T2 &a0, std::size_t a1) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f129(const std::remove_extent_t<T1> &a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f130() {
  return std::make_shared<T1>();
}

template <typename T1,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f131(std::size_t a0, const std::remove_extent_t<T1> &a1) {
  return std::make_shared<T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f132(std::size_t a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<!std::is_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f133() {
  return std::make_shared_for_overwrite<T1>();
}

template <typename T1,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f134() {
  return std::make_shared_for_overwrite<T1>();
}

template <typename T1,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f135(std::size_t a0) {
  return std::make_shared_for_overwrite<T1>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f136(std::shared_ptr<T2> &&a0,
                         std::remove_extent_t<T1> *a1) {
  return std::shared_ptr<T1>(std::move(a0), a1);
}
