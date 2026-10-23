#include <algorithm>
#include <utility>

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f12(const std::ranges::subrange<T1, T2, T3> &a0)
  requires((T3 == std::ranges::subrange_kind::sized)) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.size();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
std::ranges::subrange<T1, T2, T3> f13(T4 a0, T2 a1)
  requires((!std::ranges::subrange<T1, T2, T3>::_S_store_size)) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return std::ranges::subrange<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
std::ranges::subrange<T1, T2, T3> f14(T4 &&a0)
  requires(std::ranges::borrowed_range<T4> &&
           std::ranges::__detail::__convertible_to_non_slicing<
               std::ranges::iterator_t<T4>, T1> &&
           std::convertible_to<std::ranges::sentinel_t<T4>, T2>) &&
          (std::ranges::subrange<T1, T2, T3>::_S_store_size &&
           std::ranges::sized_range<T4>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return std::ranges::subrange<T1, T2, T3>(std::forward<T4>(a0));
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
std::ranges::subrange<T1, T2, T3> f15(T4 &&a0)
  requires(std::ranges::borrowed_range<T4> &&
           std::ranges::__detail::__convertible_to_non_slicing<
               std::ranges::iterator_t<T4>, T1> &&
           std::convertible_to<std::ranges::sentinel_t<T4>, T2>) &&
          ((!std::ranges::subrange<T1, T2, T3>::_S_store_size)) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return std::ranges::subrange<T1, T2, T3>(std::forward<T4>(a0));
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
T4 f16(const std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::__detail::__pair_like_convertible_from<T4, const T1 &,
                                                               const T2 &>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return static_cast<T4>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
bool f17(std::ranges::subrange<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
bool f18(const std::ranges::subrange<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f19(std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::subrange<T1, T2, T3>>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>)
{
  return a0.data();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f20(const std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::range<const std::ranges::subrange<T1, T2, T3>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::subrange<T1, T2, T3>>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>)
{
  return a0.data();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
decltype(auto) f21(std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<std::ranges::subrange<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>)
{
  return a0.front();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
decltype(auto) f22(const std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
              const std::ranges::subrange<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>)
{
  return a0.front();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
decltype(auto) f23(std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::subrange<T1, T2, T3>> &&
           std::ranges::common_range<std::ranges::subrange<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>)
{
  return a0.back();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
decltype(auto) f24(const std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::subrange<T1, T2, T3>> &&
           std::ranges::common_range<
               const std::ranges::subrange<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>)
{
  return a0.back();
}
