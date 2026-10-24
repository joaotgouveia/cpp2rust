#include <algorithm>
#include <iterator>
#include <ranges>
#include <type_traits>
#include <utility>

template <typename T1> using t2 = std::ranges::borrowed_subrange_t<T1>;

template <std::size_t T1, typename T2, typename T3,
          std::ranges::subrange_kind T4>
auto f29(std::ranges::subrange<T2, T3, T4> &&a0)
  requires((T1 < 2))
{
  return std::ranges::get<T1, T2, T3, T4>(std::move(a0));
}

template <std::size_t T1, typename T2, typename T3,
          std::ranges::subrange_kind T4>
auto f30(const std::ranges::subrange<T2, T3, T4> &a0)
  requires(((T1 == 0 && std::copyable<T2>) || T1 == 1))
{
  return std::ranges::get<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
std::make_unsigned_t<std::iter_difference_t<T1>>
f31(const std::ranges::subrange<T1, T2, T3> &a0)
  requires((T3 == std::ranges::subrange_kind::sized)) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.size();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
std::ranges::subrange<T1, T2, T3> f32(T4 a0, T2 a1)
  requires(std::ranges::subrange<T1, T2, T3>::_MustProvideSizeAtConstruction) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>)) &&
          (std::ranges::__convertible_to_non_slicing<T4, T1>)
{
  return std::ranges::subrange<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
std::ranges::subrange<T1, T2, T3>
f33(T4 a0, T2 a1, std::make_unsigned_t<std::iter_difference_t<T1>> a2)
  requires((T3 == std::ranges::subrange_kind::sized)) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>)) &&
          (std::ranges::__convertible_to_non_slicing<T4, T1>)
{
  return std::ranges::subrange<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
std::ranges::subrange<T1, T2, T3> f34(T4 &&a0)
  requires(std::ranges::borrowed_range<T4> &&
           std::ranges::__convertible_to_non_slicing<
               std::ranges::iterator_t<T4>, T1> &&
           std::convertible_to<std::ranges::sentinel_t<T4>, T2>) &&
          ((!std::ranges::subrange<T1, T2, T3>::_StoreSize)) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>)) &&
          (std::__different_from<T4, std::ranges::subrange<T1, T2, T3>>)
{
  return std::ranges::subrange<T1, T2, T3>(std::forward<T4>(a0));
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
std::ranges::subrange<T1, T2, T3> f35(T4 &&a0)
  requires(std::ranges::borrowed_range<T4> &&
           std::ranges::__convertible_to_non_slicing<
               std::ranges::iterator_t<T4>, T1> &&
           std::convertible_to<std::ranges::sentinel_t<T4>, T2>) &&
          (std::ranges::subrange<T1, T2, T3>::_StoreSize &&
           std::ranges::sized_range<T4>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>)) &&
          (std::__different_from<T4, std::ranges::subrange<T1, T2, T3>>)
{
  return std::ranges::subrange<T1, T2, T3>(std::forward<T4>(a0));
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
std::ranges::subrange<T1, T2, T3>
f36(T4 &&a0, std::make_unsigned_t<std::iter_difference_t<T1>> a1)
  requires(std::ranges::__convertible_to_non_slicing<
               std::ranges::iterator_t<T4>, T1> &&
           std::convertible_to<std::ranges::sentinel_t<T4>, T2>) &&
          ((T3 == std::ranges::subrange_kind::sized)) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>)) &&
          (std::ranges::borrowed_range<T4>)
{
  return std::ranges::subrange<T1, T2, T3>(std::forward<T4>(a0), a1);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
T4 f37(const std::ranges::subrange<T1, T2, T3> &a0)
  requires((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>)) &&
          (std::ranges::__pair_like_convertible_from<T4, const T1 &,
                                                     const T2 &>)
{
  return static_cast<T4>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = std::ranges::subrange<T1, T2, T3>>
bool f38(std::ranges::subrange<T1, T2, T3> &a0)
  requires((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = std::ranges::subrange<T1, T2, T3>>
bool f39(const std::ranges::subrange<T1, T2, T3> &a0)
  requires((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = std::ranges::subrange<T1, T2, T3>>
auto f40(std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::subrange<T1, T2, T3>>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.data();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = std::ranges::subrange<T1, T2, T3>>
auto f41(const std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::range<std::ranges::subrange<T1, T2, T3> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::subrange<T1, T2, T3> const>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.data();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = std::ranges::subrange<T1, T2, T3>>
decltype(auto) f42(std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<std::ranges::subrange<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.front();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = std::ranges::subrange<T1, T2, T3>>
decltype(auto) f43(const std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
              std::ranges::subrange<T1, T2, T3> const>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.front();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = std::ranges::subrange<T1, T2, T3>>
decltype(auto) f44(std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::subrange<T1, T2, T3>> &&
           std::ranges::common_range<std::ranges::subrange<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.back();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = std::ranges::subrange<T1, T2, T3>>
decltype(auto) f45(const std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::subrange<T1, T2, T3> const> &&
           std::ranges::common_range<
               std::ranges::subrange<T1, T2, T3> const>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.back();
}

using t3 = std::ranges::subrange_kind;

std::ranges::subrange_kind f46() { return std::ranges::subrange_kind::unsized; }

std::ranges::subrange_kind f47() { return std::ranges::subrange_kind::sized; }
