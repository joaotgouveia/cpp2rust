#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
auto f75(const std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template count<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f76(const std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template contains<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f77(std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f78(const std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f79(std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f80(const std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f81(std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f82(const std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f83(std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f84(const std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}
