#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f23()
  requires(std::default_initializable<T1>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f24(T1 a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(a0);
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f25(T1 a0, T2 a1)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
auto f26(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f27(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.end();
}

template <typename T1, typename T2>
auto f28(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::same_as<T1, T2>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.end();
}

template <typename T1, typename T2>
bool f29(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.empty();
}

template <typename T1, typename T2>
auto f30(const std::ranges::iota_view<T1, T2> &a0)
  requires((std::same_as<T1, T2> && std::ranges::__advanceable<T1>) ||
           (std::__integer_like<T1> && std::__integer_like<T2>) ||
           std::sized_sentinel_for<T2, T1>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
bool f31(std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
bool f32(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
auto f33(std::ranges::iota_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
auto f34(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::range<std::ranges::iota_view<T1, T2> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::iota_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
decltype(auto) f35(std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
decltype(auto) f36(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::iota_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
decltype(auto) f37(std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::iota_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
decltype(auto) f38(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::iota_view<T1, T2> const> &&
           std::ranges::common_range<std::ranges::iota_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.back();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f39(std::ranges::iota_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.template operator[]<T3>(a1);
}

template <typename T1, typename T2, typename T3>
decltype(auto) f40(const std::ranges::iota_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.template operator[]<T3>(a1);
}
