#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<std::__is_transparently_comparable_v<
                                 T3, T1, std::__remove_cvref_t<T5>>,
                             int>
              T6 = 0>
T2 &f162(std::map<T1, T2, T3, T4> &a0, T5 &&a1)
  requires(!(std::is_same<
             T5, const typename std::map<T1, T2, T3, T4>::key_type &>::value))
{
  return a0.template at<T5>(std::forward<T5>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<std::__is_transparently_comparable_v<
                                 T3, T1, std::__remove_cvref_t<T5>>,
                             int>
              T6 = 0>
const T2 &f163(const std::map<T1, T2, T3, T4> &a0, T5 &&a1)
  requires(!(std::is_same<
             T5, const typename std::map<T1, T2, T3, T4>::key_type &>::value))
{
  return a0.template at<T5>(std::forward<T5>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<
              std::is_constructible<std::pair<const T1, T2>, T5>::value, int>
              T6 = 0>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f164(std::map<T1, T2, T3, T4> &a0, T5 &&a1)
  requires(
      !(std::is_same<
          T5, const typename std::map<T1, T2, T3, T4>::value_type &>::value) &&
      !(std::is_same<T5,
                     typename std::map<T1, T2, T3, T4>::value_type>::value) &&
      !(std::is_same<T5, std::initializer_list<typename std::map<
                             T1, T2, T3, T4>::value_type>>::value) &&
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::node_type>::value))
{
  return a0.template insert<T5>(std::forward<T5>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<
              std::is_constructible<std::pair<const T1, T2>, T5>::value, int>
              T6 = 0>
typename std::map<T1, T2, T3, T4>::iterator
f165(std::map<T1, T2, T3, T4> &a0,
     typename std::map<T1, T2, T3, T4>::const_iterator a1, T5 &&a2)
  requires(
      !(std::is_same<
          T5, const typename std::map<T1, T2, T3, T4>::value_type &>::value) &&
      !(std::is_same<T5,
                     typename std::map<T1, T2, T3, T4>::value_type>::value) &&
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::node_type>::value))
{
  return a0.template insert<T5>(a1, std::forward<T5>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0>
typename std::map<T1, T2, T3, T4>::iterator f166(std::map<T1, T2, T3, T4> &a0,
                                                 const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0>
typename std::map<T1, T2, T3, T4>::const_iterator
f167(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3>, int> T7 = 0>
typename std::map<T1, T2, T3, T4>::size_type
f168(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template count<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0>
bool f169(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template contains<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0>
typename std::map<T1, T2, T3, T4>::iterator f170(std::map<T1, T2, T3, T4> &a0,
                                                 const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0>
typename std::map<T1, T2, T3, T4>::const_iterator
f171(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0>
typename std::map<T1, T2, T3, T4>::iterator f172(std::map<T1, T2, T3, T4> &a0,
                                                 const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0>
typename std::map<T1, T2, T3, T4>::const_iterator
f173(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3>, int> T7 = 0>
std::pair<typename std::map<T1, T2, T3, T4>::iterator,
          typename std::map<T1, T2, T3, T4>::iterator>
f174(std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template equal_range<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3>, int> T7 = 0>
std::pair<typename std::map<T1, T2, T3, T4>::const_iterator,
          typename std::map<T1, T2, T3, T4>::const_iterator>
f175(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template equal_range<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f176(const std::map<T1, T2, T3, T4> &a0,
          const std::map<T1, T2, T3, T4> &a1) {
  return std::operator<=> <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f177(const std::multimap<T1, T2, T3, T4> &a0,
          const std::multimap<T1, T2, T3, T4> &a1) {
  return std::operator<=> <T1, T2, T3, T4>(a0, a1);
}
