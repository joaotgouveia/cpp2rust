#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <codecvt>
#include <ios>
#include <string>
#include <utility>

int f9(const std::codecvt<char, char, std::mbstate_t> &a0, __mbstate_t &a1,
       const char *a2, const char *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

int f10(const std::codecvt<wchar_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
        const char *a2, const char *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

using t9 = std::codecvt<char16_t, char, std::mbstate_t>;

using t10 = std::codecvt<char16_t, char, __mbstate_t>::intern_type;

using t11 = std::codecvt<char16_t, char, __mbstate_t>::extern_type;

using t12 = std::codecvt<char16_t, char, __mbstate_t>::state_type;

int f11(const std::codecvt<char16_t, char, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f12(const std::codecvt<char16_t, char, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f13(const std::codecvt<char16_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
        const char *a2, const char *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

int f14(const std::codecvt<char16_t, char, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f15() {
  return std::codecvt<char16_t, char, __mbstate_t>::id;
}

using t13 = std::codecvt<char32_t, char, std::mbstate_t>;

using t14 = std::codecvt<char32_t, char, __mbstate_t>::intern_type;

using t15 = std::codecvt<char32_t, char, __mbstate_t>::extern_type;

using t16 = std::codecvt<char32_t, char, __mbstate_t>::state_type;

int f16(const std::codecvt<char32_t, char, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f17(const std::codecvt<char32_t, char, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f18(const std::codecvt<char32_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
        const char *a2, const char *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

int f19(const std::codecvt<char32_t, char, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f20() {
  return std::codecvt<char32_t, char, __mbstate_t>::id;
}

template <typename T1, typename T2, typename T3>
using t17 = std::codecvt<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t18 = typename std::codecvt<T1, T2, T3>::result;

template <typename T1, typename T2, typename T3>
using t19 = typename std::codecvt<T1, T2, T3>::intern_type;

template <typename T1, typename T2, typename T3>
using t20 = typename std::codecvt<T1, T2, T3>::extern_type;

template <typename T1, typename T2, typename T3>
using t21 = typename std::codecvt<T1, T2, T3>::state_type;

template <typename T1, typename T2, typename T3>
std::codecvt<T1, T2, T3> f21(std::size_t a0) {
  return std::codecvt<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
typename std::codecvt<T1, T2, T3>::result
f22(const std::codecvt<T1, T2, T3> &a0,
    typename std::codecvt<T1, T2, T3>::state_type &a1,
    const typename std::codecvt<T1, T2, T3>::intern_type *a2,
    const typename std::codecvt<T1, T2, T3>::intern_type *a3,
    const typename std::codecvt<T1, T2, T3>::intern_type *&a4,
    typename std::codecvt<T1, T2, T3>::extern_type *a5,
    typename std::codecvt<T1, T2, T3>::extern_type *a6,
    typename std::codecvt<T1, T2, T3>::extern_type *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

template <typename T1, typename T2, typename T3>
typename std::codecvt<T1, T2, T3>::result
f23(const std::codecvt<T1, T2, T3> &a0,
    typename std::codecvt<T1, T2, T3>::state_type &a1,
    typename std::codecvt<T1, T2, T3>::extern_type *a2,
    typename std::codecvt<T1, T2, T3>::extern_type *a3,
    typename std::codecvt<T1, T2, T3>::extern_type *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
typename std::codecvt<T1, T2, T3>::result
f24(const std::codecvt<T1, T2, T3> &a0,
    typename std::codecvt<T1, T2, T3>::state_type &a1,
    const typename std::codecvt<T1, T2, T3>::extern_type *a2,
    const typename std::codecvt<T1, T2, T3>::extern_type *a3,
    const typename std::codecvt<T1, T2, T3>::extern_type *&a4,
    typename std::codecvt<T1, T2, T3>::intern_type *a5,
    typename std::codecvt<T1, T2, T3>::intern_type *a6,
    typename std::codecvt<T1, T2, T3>::intern_type *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

template <typename T1, typename T2, typename T3>
int f25(const std::codecvt<T1, T2, T3> &a0) {
  return a0.encoding();
}

template <typename T1, typename T2, typename T3>
bool f26(const std::codecvt<T1, T2, T3> &a0) {
  return a0.always_noconv();
}

template <typename T1, typename T2, typename T3>
int f27(const std::codecvt<T1, T2, T3> &a0,
        typename std::codecvt<T1, T2, T3>::state_type &a1,
        const typename std::codecvt<T1, T2, T3>::extern_type *a2,
        const typename std::codecvt<T1, T2, T3>::extern_type *a3,
        std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
int f28(const std::codecvt<T1, T2, T3> &a0) {
  return a0.max_length();
}

template <typename T1, typename T2, typename T3> const std::locale::id &f29() {
  return std::codecvt<T1, T2, T3>::id;
}

using t22 = std::codecvt_base;

using t23 = std::codecvt_base::result;

std::codecvt_base f30() { return std::codecvt_base(); }

std::codecvt_base f31(const std::codecvt_base &a0) {
  return std::codecvt_base(a0);
}

std::codecvt_base f32(std::codecvt_base &&a0) {
  return std::codecvt_base(std::move(a0));
}

std::codecvt_base &f33(std::codecvt_base &a0, const std::codecvt_base &a1) {
  return a0.operator=(a1);
}

std::codecvt_base &f34(std::codecvt_base &a0, std::codecvt_base &&a1) {
  return a0.operator=(std::move(a1));
}

using t24 = std::codecvt_mode;

std::codecvt_mode f35() { return std::consume_header; }

std::codecvt_mode f36() { return std::generate_header; }

std::codecvt_mode f37() { return std::little_endian; }
