#include <algorithm>
#include <regex>
#include <string>
#include <utility>

template <typename T1, typename T2>
std::basic_regex<T1, T2> f31(const std::basic_regex<T1, T2> &a0) {
  return std::basic_regex<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> f32(std::basic_regex<T1, T2> &&a0) {
  return std::basic_regex<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f33(std::basic_regex<T1, T2> &a0,
                              const std::basic_regex<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f34(std::basic_regex<T1, T2> &a0,
                              std::basic_regex<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_regex<T1, T2> &f35(std::basic_regex<T1, T2> &a0, T3 a1, T3 a2,
                              std::regex_constants::syntax_option_type a3) {
  return a0.template assign<T3>(std::move(a1), std::move(a2), a3);
}
