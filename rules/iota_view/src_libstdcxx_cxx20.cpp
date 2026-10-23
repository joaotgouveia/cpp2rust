#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f3()
  requires(std::default_initializable<T1>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f4(T1 a0)
  requires(std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(a0);
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f5(T1 a0, T2 a1)
  requires(std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
auto f6(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f7(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.end();
}

template <typename T1, typename T2>
auto f8(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::same_as<T1, T2>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.end();
}

template <typename T1, typename T2>
bool f9(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.empty();
}

template <typename T1, typename T2>
auto f10(const std::ranges::iota_view<T1, T2> &a0)
  requires((std::same_as<T1, T2> && std::ranges::__detail::__advanceable<T1>) ||
           (std::integral<T1> && std::integral<T2>) ||
           std::sized_sentinel_for<T2, T1>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.size();
}

template <typename T1, typename T2>
bool f11(std::ranges::iota_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f12(const std::ranges::iota_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f13(std::ranges::iota_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>)
{
  return a0.data();
}

template <typename T1, typename T2>
auto f14(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::range<const std::ranges::iota_view<T1, T2>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::iota_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>)
{
  return a0.data();
}

template <typename T1, typename T2>
decltype(auto) f15(std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f16(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::forward_range<const std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f17(std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::iota_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>)
{
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f18(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::iota_view<T1, T2>> &&
           std::ranges::common_range<const std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>)
{
  return a0.back();
}
