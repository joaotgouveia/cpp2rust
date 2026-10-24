#include <algorithm>
#include <initializer_list>
#include <iosfwd>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> f274(std::from_range_t a0, T4 &&a1, const T3 &a2)
  requires(std::__detail::__container_compatible_range<T4, T1>)
{
  return std::basic_string<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f275(std::basic_string<T1, T2, T3> &a0, T4 &&a1)
  requires(std::__detail::__container_compatible_range<T4, T1>)
{
  return a0.template append_range<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f276(std::basic_string<T1, T2, T3> &a0, T4 &&a1)
  requires(std::__detail::__container_compatible_range<T4, T1>)
{
  return a0.template assign_range<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::iterator
f277(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1, T4 &&a2)
  requires(std::__detail::__container_compatible_range<T4, T1>)
{
  return a0.template insert_range<T4>(a1, std::forward<T4>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f278(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, T4 &&a3)
  requires(std::__detail::__container_compatible_range<T4, T1>)
{
  return a0.template replace_with_range<T4>(a1, a2, std::forward<T4>(a3));
}
