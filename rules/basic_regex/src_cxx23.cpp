#include <algorithm>
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
std::basic_regex<T1, T2> f4(const std::basic_regex<T1, T2> &a0) {
  return std::basic_regex<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> f5(std::basic_regex<T1, T2> &&a0) {
  return std::basic_regex<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> f6(std::initializer_list<T1> a0,
                            std::regex_constants::syntax_option_type a1) {
  return std::basic_regex<T1, T2>(std::forward<std::initializer_list<T1>>(a0),
                                  a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f7(std::basic_regex<T1, T2> &a0,
                             const std::basic_regex<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f8(std::basic_regex<T1, T2> &a0,
                             std::basic_regex<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f9(std::basic_regex<T1, T2> &a0, const T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f10(std::basic_regex<T1, T2> &a0,
                              std::initializer_list<T1> a1) {
  return a0.operator=(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f11(std::basic_regex<T1, T2> &a0,
                              const std::basic_regex<T1, T2> &a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f12(std::basic_regex<T1, T2> &a0,
                              std::basic_regex<T1, T2> &&a1) {
  return a0.assign(std::move(a1));
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f13(std::basic_regex<T1, T2> &a0, const T1 *a1,
                              std::regex_constants::syntax_option_type a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f14(std::basic_regex<T1, T2> &a0, const T1 *a1,
                              std::size_t a2,
                              std::regex_constants::syntax_option_type a3) {
  return a0.assign(a1, a2, a3);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f15(std::basic_regex<T1, T2> &a0,
                              std::initializer_list<T1> a1,
                              std::regex_constants::syntax_option_type a2) {
  return a0.assign(std::forward<std::initializer_list<T1>>(a1), a2);
}

template <typename T1, typename T2>
unsigned int f16(const std::basic_regex<T1, T2> &a0) {
  return a0.mark_count();
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type
f17(const std::basic_regex<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::locale_type
f18(std::basic_regex<T1, T2> &a0,
    typename std::basic_regex<T1, T2>::locale_type a1) {
  return a0.imbue(
      std::forward<typename std::basic_regex<T1, T2>::locale_type>(a1));
}

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::locale_type
f19(const std::basic_regex<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2>
void f20(std::basic_regex<T1, T2> &a0, std::basic_regex<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_regex<T1, T2> f21(const std::basic_string<T1, T3, T4> &a0,
                             std::regex_constants::syntax_option_type a1) {
  return std::basic_regex<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_regex<T1, T2> f22(T3 a0, T3 a1,
                             std::regex_constants::syntax_option_type a2) {
  return std::basic_regex<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_regex<T1, T2> &f23(std::basic_regex<T1, T2> &a0,
                              const std::basic_string<T1, T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_regex<T1, T2> &f24(std::basic_regex<T1, T2> &a0,
                              const std::basic_string<T1, T3, T4> &a1,
                              std::regex_constants::syntax_option_type a2) {
  return a0.template assign<T3, T4>(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_regex<T1, T2> &f25(std::basic_regex<T1, T2> &a0, T3 a1, T3 a2,
                              std::regex_constants::syntax_option_type a3) {
  return a0.template assign<T3>(std::move(a1), std::move(a2), a3);
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f26() {
  return std::basic_regex<T1, T2>::icase;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f27() {
  return std::basic_regex<T1, T2>::nosubs;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f28() {
  return std::basic_regex<T1, T2>::optimize;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f29() {
  return std::basic_regex<T1, T2>::collate;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f30() {
  return std::basic_regex<T1, T2>::ECMAScript;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f31() {
  return std::basic_regex<T1, T2>::basic;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f32() {
  return std::basic_regex<T1, T2>::extended;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f33() {
  return std::basic_regex<T1, T2>::awk;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f34() {
  return std::basic_regex<T1, T2>::grep;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f35() {
  return std::basic_regex<T1, T2>::egrep;
}

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f36() {
  return std::basic_regex<T1, T2>::multiline;
}
