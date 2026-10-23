#include <algorithm>
#include <iterator>
#include <ranges>
#include <type_traits>
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
std::ranges::subrange<T1, T2, T3>
f6(const std::ranges::subrange<T1, T2, T3> &a0, std::iter_difference_t<T1> a1)
  requires(std::forward_iterator<T1>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.next(a1);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
std::ranges::subrange<T1, T2, T3> f7(std::ranges::subrange<T1, T2, T3> &&a0,
                                     std::iter_difference_t<T1> a1)
  requires((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return std::move(a0).next(a1);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
std::ranges::subrange<T1, T2, T3>
f8(const std::ranges::subrange<T1, T2, T3> &a0, std::iter_difference_t<T1> a1)
  requires(std::bidirectional_iterator<T1>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.prev(a1);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
std::ranges::subrange<T1, T2, T3> &f9(std::ranges::subrange<T1, T2, T3> &a0,
                                      std::iter_difference_t<T1> a1)
  requires((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.advance(a1);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
decltype(auto) f10(std::ranges::subrange<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.template operator[]<T4>(a1);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
decltype(auto) f11(const std::ranges::subrange<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>) &&
          ((T3 == std::ranges::subrange_kind::sized ||
            !std::sized_sentinel_for<T2, T1>))
{
  return a0.template operator[]<T4>(a1);
}
