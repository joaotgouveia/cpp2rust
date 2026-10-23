#include <initializer_list>
#include <set>
#include <utility>

template <
    typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
    std::enable_if_t<std::__is_transparent_v<T2> ||
                         std::__is_transparently_comparable_v<T2, T1, T4>,
                     int>
        T6 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value)),
        int>::type T7 = 0>
typename std::set<T1, T2, T3>::iterator f114(std::set<T1, T2, T3> &a0,
                                             const T4 &a1) {
  return a0.template find<T4>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
    std::enable_if_t<std::__is_transparent_v<T2> ||
                         std::__is_transparently_comparable_v<T2, T1, T4>,
                     int>
        T6 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value)),
        int>::type T7 = 0>
typename std::set<T1, T2, T3>::const_iterator
f115(const std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template find<T4>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
    std::enable_if_t<std::__is_transparent_v<T2>, int> T6 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value)),
        int>::type T7 = 0>
typename std::set<T1, T2, T3>::size_type f116(const std::set<T1, T2, T3> &a0,
                                              const T4 &a1) {
  return a0.template count<T4>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
    std::enable_if_t<std::__is_transparent_v<T2> ||
                         std::__is_transparently_comparable_v<T2, T1, T4>,
                     int>
        T6 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value)),
        int>::type T7 = 0>
typename std::set<T1, T2, T3>::iterator f117(std::set<T1, T2, T3> &a0,
                                             const T4 &a1) {
  return a0.template lower_bound<T4>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
    std::enable_if_t<std::__is_transparent_v<T2> ||
                         std::__is_transparently_comparable_v<T2, T1, T4>,
                     int>
        T6 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value)),
        int>::type T7 = 0>
typename std::set<T1, T2, T3>::const_iterator
f118(const std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template lower_bound<T4>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
    std::enable_if_t<std::__is_transparent_v<T2> ||
                         std::__is_transparently_comparable_v<T2, T1, T4>,
                     int>
        T6 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value)),
        int>::type T7 = 0>
typename std::set<T1, T2, T3>::iterator f119(std::set<T1, T2, T3> &a0,
                                             const T4 &a1) {
  return a0.template upper_bound<T4>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
    std::enable_if_t<std::__is_transparent_v<T2> ||
                         std::__is_transparently_comparable_v<T2, T1, T4>,
                     int>
        T6 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value)),
        int>::type T7 = 0>
typename std::set<T1, T2, T3>::const_iterator
f120(const std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template upper_bound<T4>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
    std::enable_if_t<std::__is_transparent_v<T2>, int> T6 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value)),
        int>::type T7 = 0>
std::pair<typename std::set<T1, T2, T3>::iterator,
          typename std::set<T1, T2, T3>::iterator>
f121(std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template equal_range<T4>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
    std::enable_if_t<std::__is_transparent_v<T2>, int> T6 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, typename std::set<T1, T2, T3>::key_type>::value)),
        int>::type T7 = 0>
std::pair<typename std::set<T1, T2, T3>::const_iterator,
          typename std::set<T1, T2, T3>::const_iterator>
f122(const std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template equal_range<T4>(a1);
}
