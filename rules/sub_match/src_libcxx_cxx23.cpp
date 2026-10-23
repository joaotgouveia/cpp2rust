#include <regex>
#include <utility>

template <typename T1, typename T2 = void>
const std::sub_match<T1> &f15(const std::sub_match<T1> &a0,
                              const std::sub_match<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2 = void>
const std::sub_match<T1> &f16(const std::sub_match<T1> &a0,
                              std::sub_match<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
const std::sub_match<T1> &f17(const std::sub_match<T1> &a0,
                              const std::pair<T2, T3> &a1) {
  return a0.template operator= <T2, T3>(a1);
}

template <typename T1, typename T2, typename T3>
const std::sub_match<T1> &f18(const std::sub_match<T1> &a0,
                              std::pair<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}

template <typename T1, typename T2>
std::sub_match<T1> &f19(std::sub_match<T1> &a0, T2 &&a1) {
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
const std::sub_match<T1> &f20(const std::sub_match<T1> &a0, T2 &&a1) {
  return a0.template operator= <T2>(std::forward<T2>(a1));
}
