#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f26()
  requires(std::default_initializable<T1>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f27(T1 a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f28(T1 a0, T2 a1)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2>
f29(typename std::ranges::iota_view<T1, T2>::__iterator a0,
    typename std::ranges::iota_view<T1, T2>::__iterator a1)
  requires(std::same_as<T1, T2>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2>
f30(typename std::ranges::iota_view<T1, T2>::__iterator a0, T2 a1)
  requires(std::same_as<T2, std::unreachable_sentinel_t>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2>
f31(typename std::ranges::iota_view<T1, T2>::__iterator a0,
    typename std::ranges::iota_view<T1, T2>::__sentinel a1)
  requires((!std::same_as<T1, T2> &&
            !std::same_as<T2, std::unreachable_sentinel_t>)) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
auto f32(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f33(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.end();
}

template <typename T1, typename T2>
auto f34(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::same_as<T1, T2>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.end();
}

template <typename T1, typename T2>
bool f35(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.empty();
}

template <typename T1, typename T2>
auto f36(const std::ranges::iota_view<T1, T2> &a0)
  requires((std::same_as<T1, T2> && std::ranges::__advanceable<T1>) ||
           (std::__integer_like<T1> && std::__integer_like<T2>) ||
           std::sized_sentinel_for<T2, T1>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
bool f37(std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
bool f38(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
auto f39(std::ranges::iota_view<T1, T2> &a0)
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
auto f40(const std::ranges::iota_view<T1, T2> &a0)
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
decltype(auto) f41(std::ranges::iota_view<T1, T2> &a0)
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
decltype(auto) f42(const std::ranges::iota_view<T1, T2> &a0)
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
decltype(auto) f43(std::ranges::iota_view<T1, T2> &a0)
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
decltype(auto) f44(const std::ranges::iota_view<T1, T2> &a0)
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
decltype(auto) f45(std::ranges::iota_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.template operator[]<T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
decltype(auto) f46(const std::ranges::iota_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.template operator[]<T3>(std::move(a1));
}
