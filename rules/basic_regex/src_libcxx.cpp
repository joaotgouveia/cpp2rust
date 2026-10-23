#include <cstddef>
#include <initializer_list>
#include <regex>
#include <string>
#include <utility>

template <typename T1, typename T2>
std::basic_regex<T1, T2>
f37(const typename std::basic_regex<T1, T2>::value_type *a0,
    typename std::basic_regex<T1, T2>::flag_type a1) {
  return std::basic_regex<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2>
f38(const typename std::basic_regex<T1, T2>::value_type *a0, std::size_t a1,
    typename std::basic_regex<T1, T2>::flag_type a2) {
  return std::basic_regex<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2>
f39(std::initializer_list<typename std::basic_regex<T1, T2>::value_type> a0,
    typename std::basic_regex<T1, T2>::flag_type a1) {
  return std::basic_regex<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &
f40(std::basic_regex<T1, T2> &a0,
    const typename std::basic_regex<T1, T2>::value_type *a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &
f41(std::basic_regex<T1, T2> &a0,
    std::initializer_list<typename std::basic_regex<T1, T2>::value_type> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &
f42(std::basic_regex<T1, T2> &a0,
    const typename std::basic_regex<T1, T2>::value_type *a1,
    typename std::basic_regex<T1, T2>::flag_type a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &
f43(std::basic_regex<T1, T2> &a0,
    const typename std::basic_regex<T1, T2>::value_type *a1, std::size_t a2,
    typename std::basic_regex<T1, T2>::flag_type a3) {
  return a0.assign(a1, a2, a3);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &
f44(std::basic_regex<T1, T2> &a0,
    std::initializer_list<typename std::basic_regex<T1, T2>::value_type> a1,
    typename std::basic_regex<T1, T2>::flag_type a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_regex<T1, T2>
f45(const std::basic_string<typename std::basic_regex<T1, T2>::value_type, T3,
                            T4> &a0,
    typename std::basic_regex<T1, T2>::flag_type a1) {
  return std::basic_regex<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_regex<T1, T2> &
f46(std::basic_regex<T1, T2> &a0,
    const std::basic_string<typename std::basic_regex<T1, T2>::value_type, T3,
                            T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_regex<T1, T2> &
f47(std::basic_regex<T1, T2> &a0,
    const std::basic_string<typename std::basic_regex<T1, T2>::value_type, T3,
                            T4> &a1,
    typename std::basic_regex<T1, T2>::flag_type a2) {
  return a0.template assign<T3, T4>(a1, a2);
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f48() {
  return std::basic_regex<T1, T2>::icase;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f49() {
  return std::basic_regex<T1, T2>::nosubs;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f50() {
  return std::basic_regex<T1, T2>::optimize;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f51() {
  return std::basic_regex<T1, T2>::collate;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f52() {
  return std::basic_regex<T1, T2>::ECMAScript;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f53() {
  return std::basic_regex<T1, T2>::basic;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f54() {
  return std::basic_regex<T1, T2>::extended;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f55() {
  return std::basic_regex<T1, T2>::awk;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f56() {
  return std::basic_regex<T1, T2>::grep;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f57() {
  return std::basic_regex<T1, T2>::egrep;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f58() {
  return std::basic_regex<T1, T2>::multiline;
}
