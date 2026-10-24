#include <atomic>
#include <compare>
#include <cstddef>
#include <memory>
#include <type_traits>
#include <utility>

template <typename T1, typename T2,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f133(const T2 &a0, const std::remove_extent_t<T1> &a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f134(const T2 &a0) {
  return std::allocate_shared<T1, T2>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f135(const T2 &a0, std::size_t a1,
                         const std::remove_extent_t<T1> &a2) {
  return std::allocate_shared<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f136(const T2 &a0, std::size_t a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<!std::is_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f137(const T2 &a0) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f138(const T2 &a0) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f139(const T2 &a0, std::size_t a1) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f140(const std::remove_extent_t<T1> &a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f141() {
  return std::make_shared<T1>();
}

template <typename T1,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f142(std::size_t a0, const std::remove_extent_t<T1> &a1) {
  return std::make_shared<T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f143(std::size_t a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<!std::is_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f144() {
  return std::make_shared_for_overwrite<T1>();
}

template <typename T1,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f145() {
  return std::make_shared_for_overwrite<T1>();
}

template <typename T1,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f146(std::size_t a0) {
  return std::make_shared_for_overwrite<T1>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f147(std::shared_ptr<T2> &&a0,
                         std::remove_extent_t<T1> *a1) {
  return std::shared_ptr<T1>(std::move(a0), a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__compatible_with_v<T2, T1>, int> T3 = 0>
std::shared_ptr<T1> f148(const std::shared_ptr<T2> &a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::shared_ptr<T1>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__compatible_with_v<T2, T1>, int> T3 = 0>
std::shared_ptr<T1> f149(std::shared_ptr<T2> &&a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__compatible_with_v<T2, T1>, int> T3 = 0>
std::shared_ptr<T1> &f150(std::shared_ptr<T1> &a0,
                          const std::shared_ptr<T2> &a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__compatible_with_v<T2, T1>, int> T3 = 0>
std::shared_ptr<T1> &f151(std::shared_ptr<T1> &a0, std::shared_ptr<T2> &&a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(std::move(a1));
}
