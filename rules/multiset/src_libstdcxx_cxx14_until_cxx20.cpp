#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<
                  T4, typename std::multiset<T1, T2, T3>::key_type>::value)),
              int>::type T5 = 0>
auto f61(const std::multiset<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template count<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<
                  T4, typename std::multiset<T1, T2, T3>::key_type>::value)),
              int>::type T5 = 0>
auto f62(std::multiset<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<
                  T4, typename std::multiset<T1, T2, T3>::key_type>::value)),
              int>::type T5 = 0>
auto f63(const std::multiset<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<
                  T4, typename std::multiset<T1, T2, T3>::key_type>::value)),
              int>::type T5 = 0>
auto f64(std::multiset<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<
                  T4, typename std::multiset<T1, T2, T3>::key_type>::value)),
              int>::type T5 = 0>
auto f65(const std::multiset<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<
                  T4, typename std::multiset<T1, T2, T3>::key_type>::value)),
              int>::type T5 = 0>
auto f66(std::multiset<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<
                  T4, typename std::multiset<T1, T2, T3>::key_type>::value)),
              int>::type T5 = 0>
auto f67(const std::multiset<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<
                  T4, typename std::multiset<T1, T2, T3>::key_type>::value)),
              int>::type T5 = 0>
auto f68(std::multiset<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<
                  T4, typename std::multiset<T1, T2, T3>::key_type>::value)),
              int>::type T5 = 0>
auto f69(const std::multiset<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template equal_range<T4>(a1);
}
