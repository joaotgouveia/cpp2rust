#include <atomic>
#include <compare>
#include <cstddef>
#include <memory>
#include <type_traits>
#include <utility>

template <typename T1, typename T2,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f143(const T2 &a0, const std::remove_extent_t<T1> &a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f144(const T2 &a0) {
  return std::allocate_shared<T1, T2>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f145(const T2 &a0, std::size_t a1,
                         const std::remove_extent_t<T1> &a2) {
  return std::allocate_shared<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f146(const T2 &a0, std::size_t a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<!std::is_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f147(const T2 &a0) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f148(const T2 &a0) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f149(const T2 &a0, std::size_t a1) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f150(const std::remove_extent_t<T1> &a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f151() {
  return std::make_shared<T1>();
}

template <typename T1,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f152(std::size_t a0, const std::remove_extent_t<T1> &a1) {
  return std::make_shared<T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f153(std::size_t a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<!std::is_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f154() {
  return std::make_shared_for_overwrite<T1>();
}

template <typename T1,
          std::__enable_if_t<std::is_bounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f155() {
  return std::make_shared_for_overwrite<T1>();
}

template <typename T1,
          std::__enable_if_t<std::is_unbounded_array<T1>::value, int> T2 = 0>
std::shared_ptr<T1> f156(std::size_t a0) {
  return std::make_shared_for_overwrite<T1>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f157(std::shared_ptr<T2> &&a0,
                         std::remove_extent_t<T1> *a1) {
  return std::shared_ptr<T1>(std::move(a0), a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__compatible_with_v<T2, T1>, int> T3 = 0>
std::shared_ptr<T1> f158(const std::shared_ptr<T2> &a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::shared_ptr<T1>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__compatible_with_v<T2, T1>, int> T3 = 0>
std::shared_ptr<T1> f159(std::shared_ptr<T2> &&a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__compatible_with_v<T2, T1>, int> T3 = 0>
std::shared_ptr<T1> &f160(std::shared_ptr<T1> &a0,
                          const std::shared_ptr<T2> &a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__compatible_with_v<T2, T1>, int> T3 = 0>
std::shared_ptr<T1> &f161(std::shared_ptr<T1> &a0, std::shared_ptr<T2> &&a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(std::move(a1));
}
