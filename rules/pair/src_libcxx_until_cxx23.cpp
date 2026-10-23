#include <array>
#include <tuple>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <
    typename T1, typename T2, typename T3, typename T4,
    typename std::enable_if<std::is_convertible<const T3 &, T1>::value &&
                                std::is_convertible<const T4 &, T2>::value,
                            int>::type T5 = 0>
std::pair<T1, T2> f39(const std::tuple<T3, T4> &a0) {
  return std::pair<T1, T2>(a0);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename std::enable_if<std::is_constructible<T1, const T3 &>::value &&
                                std::is_constructible<T2, const T4 &>::value &&
                                !(std::is_convertible<const T3 &, T1>::value &&
                                  std::is_convertible<const T4 &, T2>::value),
                            int>::type T5 = 0>
std::pair<T1, T2> f40(const std::tuple<T3, T4> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<std::is_convertible<T3, T1>::value &&
                                      std::is_convertible<T4, T2>::value,
                                  int>::type T5 = 0>
std::pair<T1, T2> f41(std::tuple<T3, T4> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<std::is_constructible<T1, T3>::value &&
                                      std::is_constructible<T2, T4>::value &&
                                      !(std::is_convertible<T3, T1>::value &&
                                        std::is_convertible<T4, T2>::value),
                                  void>::type T5 = 0>
std::pair<T1, T2> f42(std::tuple<T3, T4> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename std::enable_if<std::is_assignable<T1 &, const T3 &>::value &&
                                std::is_assignable<T2 &, const T4 &>::value,
                            int>::type T5 = 0>
std::pair<T1, T2> &f43(std::pair<T1, T2> &a0, const std::tuple<T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<std::is_assignable<T1 &, T3 &&>::value &&
                                      std::is_assignable<T2 &, T4 &&>::value,
                                  int>::type T5 = 0>
std::pair<T1, T2> &f44(std::pair<T1, T2> &a0, std::tuple<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <
    typename T1, typename T2, typename T3,
    typename std::enable_if<std::is_convertible<const T3 &, T1>::value &&
                                std::is_convertible<const T3 &, T2>::value,
                            int>::type T4 = 0>
std::pair<T1, T2> f45(const std::array<T3, 2> &a0) {
  return std::pair<T1, T2>(a0);
}

template <
    typename T1, typename T2, typename T3,
    typename std::enable_if<std::is_constructible<T1, const T3 &>::value &&
                                std::is_constructible<T2, const T3 &>::value &&
                                !(std::is_convertible<const T3 &, T1>::value &&
                                  std::is_convertible<const T3 &, T2>::value),
                            int>::type T4 = 0>
std::pair<T1, T2> f46(const std::array<T3, 2> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<std::is_convertible<T3, T1>::value &&
                                      std::is_convertible<T3, T2>::value,
                                  int>::type T4 = 0>
std::pair<T1, T2> f47(std::array<T3, 2> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<std::is_constructible<T1, T3>::value &&
                                      std::is_constructible<T2, T3>::value &&
                                      !(std::is_convertible<T3, T1>::value &&
                                        std::is_convertible<T3, T2>::value),
                                  int>::type T4 = 0>
std::pair<T1, T2> f48(std::array<T3, 2> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <
    typename T1, typename T2, typename T3,
    typename std::enable_if<std::is_assignable<T1 &, const T3 &>::value &&
                                std::is_assignable<T2 &, const T3 &>::value,
                            int>::type T4 = 0>
std::pair<T1, T2> &f49(std::pair<T1, T2> &a0, const std::array<T3, 2> &a1) {
  return a0.template operator= <T3>(a1);
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<std::is_assignable<T1 &, T3>::value &&
                                      std::is_assignable<T2 &, T3>::value,
                                  int>::type T4 = 0>
std::pair<T1, T2> &f50(std::pair<T1, T2> &a0, std::array<T3, 2> &&a1) {
  return a0.template operator= <T3>(std::move(a1));
}
