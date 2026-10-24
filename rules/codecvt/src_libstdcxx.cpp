#include <algorithm>
#include <codecvt>
#include <ios>
#include <string>
#include <utility>

std::codecvt_base::result
f28(const std::codecvt<char, char, std::mbstate_t> &a0, __mbstate_t &a1,
    const char *a2, const char *a3, const char *&a4, char *a5, char *a6,
    char *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result
f29(const std::codecvt<char, char, std::mbstate_t> &a0, __mbstate_t &a1,
    char *a2, char *a3, char *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f30(const std::codecvt<char, char, std::mbstate_t> &a0, __mbstate_t &a1,
    const char *a2, const char *a3, const char *&a4, char *a5, char *a6,
    char *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f31(const std::codecvt<char, char, std::mbstate_t> &a0, __mbstate_t &a1,
        const char *a2, const char *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

std::codecvt_base::result
f32(const std::codecvt<wchar_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
    const wchar_t *a2, const wchar_t *a3, const wchar_t *&a4, char *a5,
    char *a6, char *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result
f33(const std::codecvt<wchar_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
    char *a2, char *a3, char *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f34(const std::codecvt<wchar_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
    const char *a2, const char *a3, const char *&a4, wchar_t *a5, wchar_t *a6,
    wchar_t *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f35(const std::codecvt<wchar_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
        const char *a2, const char *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

std::codecvt_base::result
f36(const std::codecvt<char16_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
    const char16_t *a2, const char16_t *a3, const char16_t *&a4, char *a5,
    char *a6, char *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result
f37(const std::codecvt<char16_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
    char *a2, char *a3, char *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f38(const std::codecvt<char16_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
    const char *a2, const char *a3, const char *&a4, char16_t *a5, char16_t *a6,
    char16_t *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f39(const std::codecvt<char16_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
        const char *a2, const char *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

std::codecvt_base::result
f40(const std::codecvt<char32_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
    const char32_t *a2, const char32_t *a3, const char32_t *&a4, char *a5,
    char *a6, char *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result
f41(const std::codecvt<char32_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
    char *a2, char *a3, char *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f42(const std::codecvt<char32_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
    const char *a2, const char *a3, const char *&a4, char32_t *a5, char32_t *a6,
    char32_t *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f43(const std::codecvt<char32_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
        const char *a2, const char *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
using t26 = std::codecvt<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t27 = typename std::codecvt<T1, T2, T3>::result;

template <typename T1, typename T2, typename T3>
using t28 = typename std::codecvt<T1, T2, T3>::intern_type;

template <typename T1, typename T2, typename T3>
using t29 = typename std::codecvt<T1, T2, T3>::extern_type;

template <typename T1, typename T2, typename T3>
using t30 = typename std::codecvt<T1, T2, T3>::state_type;

template <typename T1, typename T2, typename T3>
std::codecvt<T1, T2, T3> f44(std::size_t a0) {
  return std::codecvt<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
typename std::codecvt<T1, T2, T3>::result
f45(const std::codecvt<T1, T2, T3> &a0, T3 &a1, T1 const *a2, T1 const *a3,
    T1 const *&a4, T2 *a5, T2 *a6, T2 *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

template <typename T1, typename T2, typename T3>
typename std::codecvt<T1, T2, T3>::result
f46(const std::codecvt<T1, T2, T3> &a0, T3 &a1, T2 *a2, T2 *a3, T2 *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
typename std::codecvt<T1, T2, T3>::result
f47(const std::codecvt<T1, T2, T3> &a0, T3 &a1, T2 const *a2, T2 const *a3,
    T2 const *&a4, T1 *a5, T1 *a6, T1 *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

template <typename T1, typename T2, typename T3>
int f48(const std::codecvt<T1, T2, T3> &a0) {
  return a0.encoding();
}

template <typename T1, typename T2, typename T3>
bool f49(const std::codecvt<T1, T2, T3> &a0) {
  return a0.always_noconv();
}

template <typename T1, typename T2, typename T3>
int f50(const std::codecvt<T1, T2, T3> &a0, T3 &a1, T2 const *a2, T2 const *a3,
        std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
int f51(const std::codecvt<T1, T2, T3> &a0) {
  return a0.max_length();
}

template <typename T1, typename T2, typename T3> const std::locale::id &f52() {
  return std::codecvt<T1, T2, T3>::id;
}

using t31 = std::codecvt_base;

using t32 = std::codecvt_base::result;

std::codecvt_base f53() { return std::codecvt_base(); }

std::codecvt_base f54(const std::codecvt_base &a0) {
  return std::codecvt_base(a0);
}

std::codecvt_base f55(std::codecvt_base &&a0) {
  return std::codecvt_base(std::move(a0));
}

std::codecvt_base &f56(std::codecvt_base &a0, const std::codecvt_base &a1) {
  return a0.operator=(a1);
}

std::codecvt_base &f57(std::codecvt_base &a0, std::codecvt_base &&a1) {
  return a0.operator=(std::move(a1));
}

std::codecvt_base::result f58() { return std::codecvt_base::ok; }

std::codecvt_base::result f59() { return std::codecvt_base::partial; }

std::codecvt_base::result f60() { return std::codecvt_base::error; }

std::codecvt_base::result f61() { return std::codecvt_base::noconv; }
