#include <algorithm>
#include <cstddef>
#include <initializer_list>
#include <regex>
#include <string>
#include <utility>

template <typename T1, typename T2> using t1 = std::basic_regex<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::basic_regex<T1, T2>::value_type;

template <typename T1, typename T2>
using t3 = typename std::basic_regex<T1, T2>::traits_type;

template <typename T1, typename T2>
using t4 = typename std::basic_regex<T1, T2>::string_type;

template <typename T1, typename T2>
using t5 = typename std::basic_regex<T1, T2>::flag_type;

template <typename T1, typename T2>
using t6 = typename std::basic_regex<T1, T2>::locale_type;

template <typename T1, typename T2> std::basic_regex<T1, T2> f1() {
  return std::basic_regex<T1, T2>();
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> f2(const T1 *a0,
                            std::regex_constants::syntax_option_type a1) {
  return std::basic_regex<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> f3(const T1 *a0, std::size_t a1,
                            std::regex_constants::syntax_option_type a2) {
  return std::basic_regex<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> f4(std::initializer_list<T1> a0,
                            std::regex_constants::syntax_option_type a1) {
  return std::basic_regex<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f5(std::basic_regex<T1, T2> &a0, const T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f6(std::basic_regex<T1, T2> &a0,
                             std::initializer_list<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f7(std::basic_regex<T1, T2> &a0,
                             const std::basic_regex<T1, T2> &a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f8(std::basic_regex<T1, T2> &a0,
                             std::basic_regex<T1, T2> &&a1) {
  return a0.assign(std::move(a1));
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f9(std::basic_regex<T1, T2> &a0, const T1 *a1,
                             std::regex_constants::syntax_option_type a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f10(std::basic_regex<T1, T2> &a0, const T1 *a1,
                              std::size_t a2,
                              std::regex_constants::syntax_option_type a3) {
  return a0.assign(a1, a2, a3);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f11(std::basic_regex<T1, T2> &a0,
                              std::initializer_list<T1> a1,
                              std::regex_constants::syntax_option_type a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
unsigned int f12(const std::basic_regex<T1, T2> &a0) {
  return a0.mark_count();
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type
f13(const std::basic_regex<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::locale_type
f14(std::basic_regex<T1, T2> &a0,
    typename std::basic_regex<T1, T2>::locale_type a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::locale_type
f15(const std::basic_regex<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2>
void f16(std::basic_regex<T1, T2> &a0, std::basic_regex<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_regex<T1, T2> f17(const std::basic_string<T1, T3, T4> &a0,
                             std::regex_constants::syntax_option_type a1) {
  return std::basic_regex<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_regex<T1, T2> f18(T3 a0, T3 a1,
                             std::regex_constants::syntax_option_type a2) {
  return std::basic_regex<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_regex<T1, T2> &f19(std::basic_regex<T1, T2> &a0,
                              const std::basic_string<T1, T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_regex<T1, T2> &f20(std::basic_regex<T1, T2> &a0,
                              const std::basic_string<T1, T3, T4> &a1,
                              std::regex_constants::syntax_option_type a2) {
  return a0.template assign<T3, T4>(a1, a2);
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f21() {
  return std::basic_regex<T1, T2>::icase;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f22() {
  return std::basic_regex<T1, T2>::nosubs;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f23() {
  return std::basic_regex<T1, T2>::optimize;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f24() {
  return std::basic_regex<T1, T2>::collate;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f25() {
  return std::basic_regex<T1, T2>::ECMAScript;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f26() {
  return std::basic_regex<T1, T2>::basic;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f27() {
  return std::basic_regex<T1, T2>::extended;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f28() {
  return std::basic_regex<T1, T2>::awk;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f29() {
  return std::basic_regex<T1, T2>::grep;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f30() {
  return std::basic_regex<T1, T2>::egrep;
}
