#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T8 = 0>
typename std::map<T1, T2, T3, T4>::iterator f129(std::map<T1, T2, T3, T4> &a0,
                                                 const T5 &a1) {
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T8 = 0>
typename std::map<T1, T2, T3, T4>::const_iterator
f130(const std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3>, int> T7 = 0,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T8 = 0>
typename std::map<T1, T2, T3, T4>::size_type
f131(const std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template count<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T8 = 0>
typename std::map<T1, T2, T3, T4>::iterator f132(std::map<T1, T2, T3, T4> &a0,
                                                 const T5 &a1) {
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T8 = 0>
typename std::map<T1, T2, T3, T4>::const_iterator
f133(const std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T8 = 0>
typename std::map<T1, T2, T3, T4>::iterator f134(std::map<T1, T2, T3, T4> &a0,
                                                 const T5 &a1) {
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T8 = 0>
typename std::map<T1, T2, T3, T4>::const_iterator
f135(const std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3>, int> T7 = 0,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T8 = 0>
std::pair<typename std::map<T1, T2, T3, T4>::iterator,
          typename std::map<T1, T2, T3, T4>::iterator>
f136(std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template equal_range<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3>, int> T7 = 0,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T8 = 0>
std::pair<typename std::map<T1, T2, T3, T4>::const_iterator,
          typename std::map<T1, T2, T3, T4>::const_iterator>
f137(const std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template equal_range<T5>(a1);
}
