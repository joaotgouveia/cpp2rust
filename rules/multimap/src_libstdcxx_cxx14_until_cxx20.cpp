#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<T5, typename std::multimap<
                                      T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f68(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<T5, typename std::multimap<
                                      T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f69(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<T5, typename std::multimap<
                                      T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f70(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template count<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<T5, typename std::multimap<
                                      T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f71(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<T5, typename std::multimap<
                                      T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f72(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<T5, typename std::multimap<
                                      T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f73(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<T5, typename std::multimap<
                                      T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f74(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<T5, typename std::multimap<
                                      T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f75(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template equal_range<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<T5, typename std::multimap<
                                      T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f76(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template equal_range<T5>(a1);
}
