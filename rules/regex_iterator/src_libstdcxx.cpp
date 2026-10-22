#include "../common/init.h"
#include "../common/traits.h"
#include <regex>
#include <utility>

template <typename T1, typename T2, typename T3>
std::regex_iterator<T1, T2, T3> f10(const std::regex_iterator<T1, T2, T3> &a0) {
  return std::regex_iterator<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::regex_iterator<T1, T2, T3> &
f11(std::regex_iterator<T1, T2, T3> &a0,
    const std::regex_iterator<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}
