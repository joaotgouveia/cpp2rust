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
typename std::map<T1, T2, T3, T4>::iterator f151(std::map<T1, T2, T3, T4> &a0,
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
f152(const std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
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
f153(const std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
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
typename std::map<T1, T2, T3, T4>::iterator f154(std::map<T1, T2, T3, T4> &a0,
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
f155(const std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
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
typename std::map<T1, T2, T3, T4>::iterator f156(std::map<T1, T2, T3, T4> &a0,
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
f157(const std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
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
f158(std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
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
f159(const std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template equal_range<T5>(a1);
}
