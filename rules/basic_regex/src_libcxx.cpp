#include <cstddef>
#include <initializer_list>
#include <regex>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T3>::value, int>
              T4 = 0>
std::basic_regex<T1, T2> &f37(std::basic_regex<T1, T2> &a0, T3 a1, T3 a2,
                              std::regex_constants::syntax_option_type a3) {
  return a0.template assign<T3>(a1, a2, a3);
}

template <
    typename T1, typename T2, typename T3,
    std::__enable_if_t<std::__has_forward_iterator_category<T3>::value, int>
        T4 = 0>
std::basic_regex<T1, T2> &f38(std::basic_regex<T1, T2> &a0, T3 a1, T3 a2,
                              std::regex_constants::syntax_option_type a3) {
  return a0.template assign<T3>(a1, a2, a3);
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f39() {
  return std::basic_regex<T1, T2>::multiline;
}
