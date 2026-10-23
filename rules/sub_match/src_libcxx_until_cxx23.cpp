#include <array>
#include <regex>
#include <tuple>
#include <utility>

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::is_assignable<T1 &, const T2 &>::value &&
                                 std::is_assignable<T1 &, const T3 &>::value,
                             int>
              T4 = 0>
std::sub_match<T1> &f11(std::sub_match<T1> &a0, const std::tuple<T2, T3> &a1) {
  return a0.template operator= <T2, T3>(a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::is_assignable<T1 &, T2 &&>::value &&
                                 std::is_assignable<T1 &, T3 &&>::value,
                             int>
              T4 = 0>
std::sub_match<T1> &f12(std::sub_match<T1> &a0, std::tuple<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_assignable<T1 &, const T2 &>::value &&
                                 std::is_assignable<T1 &, const T2 &>::value,
                             int>
              T3 = 0>
std::sub_match<T1> &f13(std::sub_match<T1> &a0, const std::array<T2, 2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_assignable<T1 &, T2>::value &&
                                 std::is_assignable<T1 &, T2>::value,
                             int>
              T3 = 0>
std::sub_match<T1> &f14(std::sub_match<T1> &a0, std::array<T2, 2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}
