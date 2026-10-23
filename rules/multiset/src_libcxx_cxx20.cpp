#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2> ||
                               std::__is_transparently_comparable_v<T2, T1, T4>,
                           int>
              T6 = 0>
typename std::multiset<T1, T2, T3>::iterator f107(std::multiset<T1, T2, T3> &a0,
                                                  const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2> ||
                               std::__is_transparently_comparable_v<T2, T1, T4>,
                           int>
              T6 = 0>
typename std::multiset<T1, T2, T3>::const_iterator
f108(const std::multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2>, int> T6 = 0>
typename std::multiset<T1, T2, T3>::size_type
f109(const std::multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template count<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2> ||
                               std::__is_transparently_comparable_v<T2, T1, T4>,
                           int>
              T6 = 0>
bool f110(const std::multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template contains<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2> ||
                               std::__is_transparently_comparable_v<T2, T1, T4>,
                           int>
              T6 = 0>
typename std::multiset<T1, T2, T3>::iterator f111(std::multiset<T1, T2, T3> &a0,
                                                  const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2> ||
                               std::__is_transparently_comparable_v<T2, T1, T4>,
                           int>
              T6 = 0>
typename std::multiset<T1, T2, T3>::const_iterator
f112(const std::multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2> ||
                               std::__is_transparently_comparable_v<T2, T1, T4>,
                           int>
              T6 = 0>
typename std::multiset<T1, T2, T3>::iterator f113(std::multiset<T1, T2, T3> &a0,
                                                  const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2> ||
                               std::__is_transparently_comparable_v<T2, T1, T4>,
                           int>
              T6 = 0>
typename std::multiset<T1, T2, T3>::const_iterator
f114(const std::multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2>, int> T6 = 0>
std::pair<typename std::multiset<T1, T2, T3>::iterator,
          typename std::multiset<T1, T2, T3>::iterator>
f115(std::multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2>, int> T6 = 0>
std::pair<typename std::multiset<T1, T2, T3>::const_iterator,
          typename std::multiset<T1, T2, T3>::const_iterator>
f116(const std::multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(
      !(std::is_same<T4, typename std::multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}
