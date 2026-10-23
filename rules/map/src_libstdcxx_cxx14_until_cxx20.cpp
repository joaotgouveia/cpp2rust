#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f83(std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f84(const std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f85(const std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template count<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f86(std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f87(const std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f88(std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f89(const std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f90(std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template equal_range<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              (!(std::is_same<
                  T5, typename std::map<T1, T2, T3, T4>::key_type>::value)),
              int>::type T6 = 0>
auto f91(const std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template equal_range<T5>(a1);
}
