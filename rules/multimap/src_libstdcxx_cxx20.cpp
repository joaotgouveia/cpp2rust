#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f78(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f79(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f80(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template count<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f81(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template contains<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f82(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f83(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f84(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f85(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f86(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template equal_range<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f87(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template equal_range<T5>(a1);
}
