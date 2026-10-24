#include <algorithm>
#include <regex>
#include <utility>

template <typename T1, typename T2, typename T3>
typename std::enable_if<
    std::__and_<std::is_assignable<T1 &, const T2 &>,
                std::is_assignable<T1 &, const T3 &>>::value,
    std::pair<T1, T1> &>::type
f9(std::sub_match<T1> &a0, const std::pair<T2, T3> &a1) {
  return a0.template operator= <T2, T3>(a1);
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::__and_<std::is_assignable<T1 &, T2 &&>,
                                    std::is_assignable<T1 &, T3 &&>>::value,
                        std::pair<T1, T1> &>::type
f10(std::sub_match<T1> &a0, std::pair<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}
