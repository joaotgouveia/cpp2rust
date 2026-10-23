#include <algorithm>
#include <utility>

template <typename T1> using t1 = std::ranges::in_found_result<T1>;

template <typename T1, typename T2>
std::ranges::in_found_result<T2> f1(const std::ranges::in_found_result<T1> &a0)
  requires(std::convertible_to<const T1 &, T2>)
{
  return static_cast<std::ranges::in_found_result<T2>>(a0);
}

template <typename T1, typename T2>
std::ranges::in_found_result<T2> f2(std::ranges::in_found_result<T1> &&a0)
  requires(std::convertible_to<T1, T2>)
{
  return static_cast<std::ranges::in_found_result<T2>>(std::move(a0));
}
