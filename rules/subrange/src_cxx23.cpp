#include <algorithm>
#include <utility>

template <typename T1, typename T2, std::ranges::subrange_kind T3>
using t1 = std::ranges::subrange<T1, T2, T3>;

template <typename T1, typename T2, std::ranges::subrange_kind T3>
std::ranges::subrange<T1, T2, T3> f1()
  requires(std::default_initializable<T1>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return std::ranges::subrange<T1, T2, T3>();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
T1 f2(const std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::copyable<T1>) && ((T3 == std::ranges::subrange_kind::sized ||
                                   !std::sized_sentinel_for<T2, T1>))
{
  return a0.begin();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
T1 f3(std::ranges::subrange<T1, T2, T3> &a0)
  requires((!std::copyable<T1>)) && ((T3 == std::ranges::subrange_kind::sized ||
                                      !std::sized_sentinel_for<T2, T1>))
{
  return a0.begin();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
T2 f4(const std::ranges::subrange<T1, T2, T3> &a0)
  requires((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.end();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
bool f5(const std::ranges::subrange<T1, T2, T3> &a0)
  requires((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.empty();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f6(const std::ranges::subrange<T1, T2, T3> &a0)
  requires((T3 == std::ranges::subrange_kind::sized)) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.size();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
std::ranges::subrange<T1, T2, T3>
f7(const std::ranges::subrange<T1, T2, T3> &a0, std::iter_difference_t<T1> a1)
  requires(std::forward_iterator<T1>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.next(std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
std::ranges::subrange<T1, T2, T3> f8(std::ranges::subrange<T1, T2, T3> &&a0,
                                     std::iter_difference_t<T1> a1)
  requires((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return std::move(a0).next(std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
std::ranges::subrange<T1, T2, T3>
f9(const std::ranges::subrange<T1, T2, T3> &a0, std::iter_difference_t<T1> a1)
  requires(std::bidirectional_iterator<T1>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.prev(std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
std::ranges::subrange<T1, T2, T3> &f10(std::ranges::subrange<T1, T2, T3> &a0,
                                       std::iter_difference_t<T1> a1)
  requires((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.advance(std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
std::ranges::subrange<T1, T2, T3> f11(T4 a0, T2 a1)
  requires((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return std::ranges::subrange<T1, T2, T3>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
std::ranges::subrange<T1, T2, T3> f12(T4 &&a0)
  requires(std::ranges::borrowed_range<T4> &&
           std::convertible_to<std::ranges::sentinel_t<T4>, T2> &&
           std::ranges::sized_range<T4> &&
           ((T3 == std::ranges::subrange_kind::sized ||
             !std::sized_sentinel_for<T2, T1>)))
{
  return std::ranges::subrange<T1, T2, T3>(std::forward<T4>(a0));
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
std::ranges::subrange<T1, T2, T3> f13(T4 &&a0)
  requires(std::ranges::borrowed_range<T4> &&
           std::convertible_to<std::ranges::sentinel_t<T4>, T2> &&
           ((T3 == std::ranges::subrange_kind::sized ||
             !std::sized_sentinel_for<T2, T1>)))
{
  return std::ranges::subrange<T1, T2, T3>(std::forward<T4>(a0));
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
T4 f14(const std::ranges::subrange<T1, T2, T3> &a0)
  requires((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return static_cast<T4>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
bool f15(std::ranges::subrange<T1, T2, T3> &a0)
  requires((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
bool f16(const std::ranges::subrange<T1, T2, T3> &a0)
  requires((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f17(std::ranges::subrange<T1, T2, T3> &a0)
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

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f18(const std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::range<const std::ranges::subrange<T1, T2, T3>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::subrange<T1, T2, T3>>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.data();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
decltype(auto) f19(std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<std::ranges::subrange<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.front();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
decltype(auto) f20(const std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
              const std::ranges::subrange<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.front();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
decltype(auto) f21(std::ranges::subrange<T1, T2, T3> &a0)
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

template <typename T1, typename T2, std::ranges::subrange_kind T3>
decltype(auto) f22(const std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::subrange<T1, T2, T3>> &&
           std::ranges::common_range<
               const std::ranges::subrange<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.back();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f23(std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::input_range<std::ranges::subrange<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.cbegin();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f24(const std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::input_range<const std::ranges::subrange<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.cbegin();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f25(std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::input_range<std::ranges::subrange<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.cend();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f26(const std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::input_range<const std::ranges::subrange<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.cend();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = std::ranges::subrange<T1, T2, T3>>
decltype(auto) f27(std::ranges::subrange<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          (std::ranges::random_access_range<T4>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.template operator[]<T4>(
      std::forward<std::ranges::range_difference_t<T4>>(a1));
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = const std::ranges::subrange<T1, T2, T3>>
decltype(auto) f28(const std::ranges::subrange<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          (std::ranges::random_access_range<T4>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.template operator[]<T4>(
      std::forward<std::ranges::range_difference_t<T4>>(a1));
}
