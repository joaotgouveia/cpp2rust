#include <algorithm>
#include <regex>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
std::basic_regex<T1, T2> f10(const T1 *a0,
                             typename std::basic_regex<T1, T2>::flag_type a1) {
  return std::basic_regex<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> f11(const T1 *a0, std::size_t a1,
                             typename std::basic_regex<T1, T2>::flag_type a2) {
  return std::basic_regex<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> f12(const std::basic_regex<T1, T2> &a0) {
  return std::basic_regex<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> f13(std::basic_regex<T1, T2> &&a0) {
  return std::basic_regex<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> f14(std::initializer_list<T1> a0,
                             typename std::basic_regex<T1, T2>::flag_type a1) {
  return std::basic_regex<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f15(std::basic_regex<T1, T2> &a0,
                              const std::basic_regex<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f16(std::basic_regex<T1, T2> &a0,
                              std::basic_regex<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f17(std::basic_regex<T1, T2> &a0, const T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f18(std::basic_regex<T1, T2> &a0,
                              std::initializer_list<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f19(std::basic_regex<T1, T2> &a0, const T1 *a1,
                              typename std::basic_regex<T1, T2>::flag_type a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f20(std::basic_regex<T1, T2> &a0, const T1 *a1,
                              std::size_t a2,
                              typename std::basic_regex<T1, T2>::flag_type a3) {
  return a0.assign(a1, a2, a3);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f21(std::basic_regex<T1, T2> &a0,
                              std::initializer_list<T1> a1,
                              typename std::basic_regex<T1, T2>::flag_type a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_regex<T1, T2> f22(const std::basic_string<T1, T3, T4> &a0,
                             typename std::basic_regex<T1, T2>::flag_type a1) {
  return std::basic_regex<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_regex<T1, T2> &f23(std::basic_regex<T1, T2> &a0,
                              const std::basic_string<T1, T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_regex<T1, T2> &f24(std::basic_regex<T1, T2> &a0,
                              const std::basic_string<T1, T3, T4> &a1,
                              typename std::basic_regex<T1, T2>::flag_type a2) {
  return a0.template assign<T3, T4>(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_regex<T1, T2> &f25(std::basic_regex<T1, T2> &a0, T3 a1, T3 a2,
                              typename std::basic_regex<T1, T2>::flag_type a3) {
  return a0.template assign<T3>(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::flag_type f26() {
  return std::basic_regex<T1, T2>::icase;
}

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::flag_type f27() {
  return std::basic_regex<T1, T2>::nosubs;
}

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::flag_type f28() {
  return std::basic_regex<T1, T2>::optimize;
}

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::flag_type f29() {
  return std::basic_regex<T1, T2>::collate;
}

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::flag_type f30() {
  return std::basic_regex<T1, T2>::ECMAScript;
}

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::flag_type f31() {
  return std::basic_regex<T1, T2>::basic;
}

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::flag_type f32() {
  return std::basic_regex<T1, T2>::extended;
}

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::flag_type f33() {
  return std::basic_regex<T1, T2>::awk;
}

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::flag_type f34() {
  return std::basic_regex<T1, T2>::grep;
}

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::flag_type f35() {
  return std::basic_regex<T1, T2>::egrep;
}
