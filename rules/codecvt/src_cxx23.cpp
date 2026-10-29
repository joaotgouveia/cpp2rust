#include <algorithm>
#include <codecvt>
#include <cwchar>
#include <ios>
#include <locale>
#include <utility>

using t1 = std::codecvt<char, char, std::mbstate_t>;

using t2 = std::codecvt<char, char, __mbstate_t>::intern_type;

using t3 = std::codecvt<char, char, __mbstate_t>::extern_type;

using t4 = std::codecvt<char, char, __mbstate_t>::state_type;

std::codecvt_base::result f1(const std::codecvt<char, char, std::mbstate_t> &a0,
                             __mbstate_t &a1, const char *a2, const char *a3,
                             const char *&a4, char *a5, char *a6, char *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result f2(const std::codecvt<char, char, std::mbstate_t> &a0,
                             __mbstate_t &a1, char *a2, char *a3, char *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result f3(const std::codecvt<char, char, std::mbstate_t> &a0,
                             __mbstate_t &a1, const char *a2, const char *a3,
                             const char *&a4, char *a5, char *a6, char *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f4(const std::codecvt<char, char, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f5(const std::codecvt<char, char, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f6(const std::codecvt<char, char, std::mbstate_t> &a0, __mbstate_t &a1,
       const char *a2, const char *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

int f7(const std::codecvt<char, char, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f8() {
  return std::codecvt<char, char, __mbstate_t>::id;
}

using t5 = std::codecvt<wchar_t, char, std::mbstate_t>;

using t6 = std::codecvt<wchar_t, char, __mbstate_t>::intern_type;

using t7 = std::codecvt<wchar_t, char, __mbstate_t>::extern_type;

using t8 = std::codecvt<wchar_t, char, __mbstate_t>::state_type;

std::codecvt_base::result
f9(const std::codecvt<wchar_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
   const wchar_t *a2, const wchar_t *a3, const wchar_t *&a4, char *a5, char *a6,
   char *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result
f10(const std::codecvt<wchar_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
    char *a2, char *a3, char *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f11(const std::codecvt<wchar_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
    const char *a2, const char *a3, const char *&a4, wchar_t *a5, wchar_t *a6,
    wchar_t *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f12(const std::codecvt<wchar_t, char, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f13(const std::codecvt<wchar_t, char, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f14(const std::codecvt<wchar_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
        const char *a2, const char *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

int f15(const std::codecvt<wchar_t, char, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f16() {
  return std::codecvt<wchar_t, char, __mbstate_t>::id;
}

using t9 = std::codecvt<char16_t, char, std::mbstate_t>;

using t10 = std::codecvt<char16_t, char, __mbstate_t>::intern_type;

using t11 = std::codecvt<char16_t, char, __mbstate_t>::extern_type;

using t12 = std::codecvt<char16_t, char, __mbstate_t>::state_type;

std::codecvt_base::result
f17(const std::codecvt<char16_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
    const char16_t *a2, const char16_t *a3, const char16_t *&a4, char *a5,
    char *a6, char *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result
f18(const std::codecvt<char16_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
    char *a2, char *a3, char *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f19(const std::codecvt<char16_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
    const char *a2, const char *a3, const char *&a4, char16_t *a5, char16_t *a6,
    char16_t *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f20(const std::codecvt<char16_t, char, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f21(const std::codecvt<char16_t, char, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f22(const std::codecvt<char16_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
        const char *a2, const char *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

int f23(const std::codecvt<char16_t, char, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f24() {
  return std::codecvt<char16_t, char, __mbstate_t>::id;
}

using t13 = std::codecvt<char16_t, char8_t, std::mbstate_t>;

using t14 = std::codecvt<char16_t, char8_t, __mbstate_t>::intern_type;

using t15 = std::codecvt<char16_t, char8_t, __mbstate_t>::extern_type;

using t16 = std::codecvt<char16_t, char8_t, __mbstate_t>::state_type;

std::codecvt_base::result
f25(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0, __mbstate_t &a1,
    const char16_t *a2, const char16_t *a3, const char16_t *&a4, char8_t *a5,
    char8_t *a6, char8_t *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result
f26(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0, __mbstate_t &a1,
    char8_t *a2, char8_t *a3, char8_t *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f27(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0, __mbstate_t &a1,
    const char8_t *a2, const char8_t *a3, const char8_t *&a4, char16_t *a5,
    char16_t *a6, char16_t *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f28(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f29(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f30(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0,
        __mbstate_t &a1, const char8_t *a2, const char8_t *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

int f31(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f32() {
  return std::codecvt<char16_t, char8_t, __mbstate_t>::id;
}

using t17 = std::codecvt<char32_t, char, std::mbstate_t>;

using t18 = std::codecvt<char32_t, char, __mbstate_t>::intern_type;

using t19 = std::codecvt<char32_t, char, __mbstate_t>::extern_type;

using t20 = std::codecvt<char32_t, char, __mbstate_t>::state_type;

std::codecvt_base::result
f33(const std::codecvt<char32_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
    const char32_t *a2, const char32_t *a3, const char32_t *&a4, char *a5,
    char *a6, char *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result
f34(const std::codecvt<char32_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
    char *a2, char *a3, char *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f35(const std::codecvt<char32_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
    const char *a2, const char *a3, const char *&a4, char32_t *a5, char32_t *a6,
    char32_t *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f36(const std::codecvt<char32_t, char, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f37(const std::codecvt<char32_t, char, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f38(const std::codecvt<char32_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
        const char *a2, const char *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

int f39(const std::codecvt<char32_t, char, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f40() {
  return std::codecvt<char32_t, char, __mbstate_t>::id;
}

using t21 = std::codecvt<char32_t, char8_t, std::mbstate_t>;

using t22 = std::codecvt<char32_t, char8_t, __mbstate_t>::intern_type;

using t23 = std::codecvt<char32_t, char8_t, __mbstate_t>::extern_type;

using t24 = std::codecvt<char32_t, char8_t, __mbstate_t>::state_type;

std::codecvt_base::result
f41(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0, __mbstate_t &a1,
    const char32_t *a2, const char32_t *a3, const char32_t *&a4, char8_t *a5,
    char8_t *a6, char8_t *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result
f42(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0, __mbstate_t &a1,
    char8_t *a2, char8_t *a3, char8_t *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f43(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0, __mbstate_t &a1,
    const char8_t *a2, const char8_t *a3, const char8_t *&a4, char32_t *a5,
    char32_t *a6, char32_t *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f44(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f45(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f46(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0,
        __mbstate_t &a1, const char8_t *a2, const char8_t *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

int f47(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f48() {
  return std::codecvt<char32_t, char8_t, __mbstate_t>::id;
}

template <typename T1, typename T2, typename T3>
using t25 = std::codecvt<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t26 = typename std::codecvt<T1, T2, T3>::result;

template <typename T1, typename T2, typename T3>
using t27 = typename std::codecvt<T1, T2, T3>::intern_type;

template <typename T1, typename T2, typename T3>
using t28 = typename std::codecvt<T1, T2, T3>::extern_type;

template <typename T1, typename T2, typename T3>
using t29 = typename std::codecvt<T1, T2, T3>::state_type;

template <typename T1, typename T2, typename T3>
std::codecvt<T1, T2, T3> f49(std::size_t a0) {
  return std::codecvt<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
typename std::codecvt<T1, T2, T3>::result
f50(const std::codecvt<T1, T2, T3> &a0, T3 &a1, T1 const *a2, T1 const *a3,
    T1 const *&a4, T2 *a5, T2 *a6, T2 *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

template <typename T1, typename T2, typename T3>
typename std::codecvt<T1, T2, T3>::result
f51(const std::codecvt<T1, T2, T3> &a0, T3 &a1, T2 *a2, T2 *a3, T2 *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
typename std::codecvt<T1, T2, T3>::result
f52(const std::codecvt<T1, T2, T3> &a0, T3 &a1, T2 const *a2, T2 const *a3,
    T2 const *&a4, T1 *a5, T1 *a6, T1 *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

template <typename T1, typename T2, typename T3>
int f53(const std::codecvt<T1, T2, T3> &a0) {
  return a0.encoding();
}

template <typename T1, typename T2, typename T3>
bool f54(const std::codecvt<T1, T2, T3> &a0) {
  return a0.always_noconv();
}

template <typename T1, typename T2, typename T3>
int f55(const std::codecvt<T1, T2, T3> &a0, T3 &a1, T2 const *a2, T2 const *a3,
        std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
int f56(const std::codecvt<T1, T2, T3> &a0) {
  return a0.max_length();
}

template <typename T1, typename T2, typename T3> const std::locale::id &f57() {
  return std::codecvt<T1, T2, T3>::id;
}

using t30 = std::codecvt_base;

using t31 = std::codecvt_base::result;

std::codecvt_base f58() { return std::codecvt_base(); }

std::codecvt_base f59(const std::codecvt_base &a0) {
  return std::codecvt_base(a0);
}

std::codecvt_base f60(std::codecvt_base &&a0) {
  return std::codecvt_base(std::move(a0));
}

std::codecvt_base &f61(std::codecvt_base &a0, const std::codecvt_base &a1) {
  return a0.operator=(a1);
}

std::codecvt_base &f62(std::codecvt_base &a0, std::codecvt_base &&a1) {
  return a0.operator=(std::move(a1));
}

std::codecvt_base::result f63() { return std::codecvt_base::ok; }

std::codecvt_base::result f64() { return std::codecvt_base::partial; }

std::codecvt_base::result f65() { return std::codecvt_base::error; }

std::codecvt_base::result f66() { return std::codecvt_base::noconv; }

using t32 = std::codecvt_mode;

std::codecvt_mode f67() { return std::consume_header; }

std::codecvt_mode f68() { return std::generate_header; }

std::codecvt_mode f69() { return std::little_endian; }
