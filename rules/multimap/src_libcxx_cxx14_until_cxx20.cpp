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
              (!(std::is_same<T5, typename std::multimap<
                                      T1, T2, T3, T4>::key_type>::value)),
              int>::type T8 = 0>
typename std::multimap<T1, T2, T3, T4>::iterator
f100(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0,
          typename std::enable_if<
              (!(std::is_same<T5, typename std::multimap<
                                      T1, T2, T3, T4>::key_type>::value)),
              int>::type T8 = 0>
typename std::multimap<T1, T2, T3, T4>::const_iterator
f101(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template find<T5>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6 = T3, std::enable_if_t<std::__is_transparent_v<T3>, int> T7 = 0,
    typename std::enable_if<
        (!(std::is_same<
            T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value)),
        int>::type T8 = 0>
typename std::multimap<T1, T2, T3, T4>::size_type
f102(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template count<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0,
          typename std::enable_if<
              (!(std::is_same<T5, typename std::multimap<
                                      T1, T2, T3, T4>::key_type>::value)),
              int>::type T8 = 0>
typename std::multimap<T1, T2, T3, T4>::iterator
f103(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0,
          typename std::enable_if<
              (!(std::is_same<T5, typename std::multimap<
                                      T1, T2, T3, T4>::key_type>::value)),
              int>::type T8 = 0>
typename std::multimap<T1, T2, T3, T4>::const_iterator
f104(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0,
          typename std::enable_if<
              (!(std::is_same<T5, typename std::multimap<
                                      T1, T2, T3, T4>::key_type>::value)),
              int>::type T8 = 0>
typename std::multimap<T1, T2, T3, T4>::iterator
f105(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = T3,
          std::enable_if_t<std::__is_transparent_v<T3> ||
                               std::__is_transparently_comparable_v<T3, T1, T5>,
                           int>
              T7 = 0,
          typename std::enable_if<
              (!(std::is_same<T5, typename std::multimap<
                                      T1, T2, T3, T4>::key_type>::value)),
              int>::type T8 = 0>
typename std::multimap<T1, T2, T3, T4>::const_iterator
f106(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template upper_bound<T5>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6 = T3, std::enable_if_t<std::__is_transparent_v<T3>, int> T7 = 0,
    typename std::enable_if<
        (!(std::is_same<
            T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value)),
        int>::type T8 = 0>
std::pair<typename std::multimap<T1, T2, T3, T4>::iterator,
          typename std::multimap<T1, T2, T3, T4>::iterator>
f107(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template equal_range<T5>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6 = T3, std::enable_if_t<std::__is_transparent_v<T3>, int> T7 = 0,
    typename std::enable_if<
        (!(std::is_same<
            T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value)),
        int>::type T8 = 0>
std::pair<typename std::multimap<T1, T2, T3, T4>::const_iterator,
          typename std::multimap<T1, T2, T3, T4>::const_iterator>
f108(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template equal_range<T5>(a1);
}
