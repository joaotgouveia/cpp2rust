#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
auto f123(const std::multiset<T1, T2, T3> &a0,
          const std::multiset<T1, T2, T3> &a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f124(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2> ||
                               std::__is_transparently_comparable_v<T2, T1, T4>,
                           int>
              T6 = 0>
typename std::set<T1, T2, T3>::iterator f125(std::set<T1, T2, T3> &a0,
                                             const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2> ||
                               std::__is_transparently_comparable_v<T2, T1, T4>,
                           int>
              T6 = 0>
typename std::set<T1, T2, T3>::const_iterator
f126(const std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2>, int> T6 = 0>
typename std::set<T1, T2, T3>::size_type f127(const std::set<T1, T2, T3> &a0,
                                              const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template count<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2> ||
                               std::__is_transparently_comparable_v<T2, T1, T4>,
                           int>
              T6 = 0>
bool f128(const std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template contains<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2> ||
                               std::__is_transparently_comparable_v<T2, T1, T4>,
                           int>
              T6 = 0>
typename std::set<T1, T2, T3>::iterator f129(std::set<T1, T2, T3> &a0,
                                             const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2> ||
                               std::__is_transparently_comparable_v<T2, T1, T4>,
                           int>
              T6 = 0>
typename std::set<T1, T2, T3>::const_iterator
f130(const std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2> ||
                               std::__is_transparently_comparable_v<T2, T1, T4>,
                           int>
              T6 = 0>
typename std::set<T1, T2, T3>::iterator f131(std::set<T1, T2, T3> &a0,
                                             const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2> ||
                               std::__is_transparently_comparable_v<T2, T1, T4>,
                           int>
              T6 = 0>
typename std::set<T1, T2, T3>::const_iterator
f132(const std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2>, int> T6 = 0>
std::pair<typename std::set<T1, T2, T3>::iterator,
          typename std::set<T1, T2, T3>::iterator>
f133(std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2>, int> T6 = 0>
std::pair<typename std::set<T1, T2, T3>::const_iterator,
          typename std::set<T1, T2, T3>::const_iterator>
f134(const std::set<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}
