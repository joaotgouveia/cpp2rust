#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::ranges::in_out_out_result<T1, T2, T3>;

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::ranges::in_out_out_result<T4, T5, T6>
f1(const std::ranges::in_out_out_result<T1, T2, T3> &a0)
  requires(std::convertible_to<const T1 &, T4> &&
           std::convertible_to<const T2 &, T5> &&
           std::convertible_to<const T3 &, T6>)
{
  return static_cast<std::ranges::in_out_out_result<T4, T5, T6>>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::ranges::in_out_out_result<T4, T5, T6>
f2(std::ranges::in_out_out_result<T1, T2, T3> &&a0)
  requires(std::convertible_to<T1, T4> && std::convertible_to<T2, T5> &&
           std::convertible_to<T3, T6>)
{
  return static_cast<std::ranges::in_out_out_result<T4, T5, T6>>(std::move(a0));
}
