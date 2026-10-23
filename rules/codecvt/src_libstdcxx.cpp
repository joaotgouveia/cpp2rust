#include <algorithm>
#include <codecvt>
#include <ios>
#include <string>
#include <utility>

int f28(const std::codecvt<char, char, std::mbstate_t> &a0, __mbstate_t &a1,
        const char *a2, const char *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

int f29(const std::codecvt<wchar_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
        const char *a2, const char *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

int f30(const std::codecvt<char16_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
        const char *a2, const char *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

int f31(const std::codecvt<char32_t, char, std::mbstate_t> &a0, __mbstate_t &a1,
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
std::codecvt<T1, T2, T3> f32(std::size_t a0) {
  return std::codecvt<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
typename std::codecvt<T1, T2, T3>::result
f33(const std::codecvt<T1, T2, T3> &a0, T3 &a1, T1 const *a2, T1 const *a3,
    T1 const *&a4, T2 *a5, T2 *a6, T2 *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

template <typename T1, typename T2, typename T3>
typename std::codecvt<T1, T2, T3>::result
f34(const std::codecvt<T1, T2, T3> &a0, T3 &a1, T2 *a2, T2 *a3, T2 *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
typename std::codecvt<T1, T2, T3>::result
f35(const std::codecvt<T1, T2, T3> &a0, T3 &a1, T2 const *a2, T2 const *a3,
    T2 const *&a4, T1 *a5, T1 *a6, T1 *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

template <typename T1, typename T2, typename T3>
int f36(const std::codecvt<T1, T2, T3> &a0) {
  return a0.encoding();
}

template <typename T1, typename T2, typename T3>
bool f37(const std::codecvt<T1, T2, T3> &a0) {
  return a0.always_noconv();
}

template <typename T1, typename T2, typename T3>
int f38(const std::codecvt<T1, T2, T3> &a0, T3 &a1, T2 const *a2, T2 const *a3,
        std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
int f39(const std::codecvt<T1, T2, T3> &a0) {
  return a0.max_length();
}

template <typename T1, typename T2, typename T3> const std::locale::id &f40() {
  return std::codecvt<T1, T2, T3>::id;
}

using t31 = std::codecvt_base;

using t32 = std::codecvt_base::result;

std::codecvt_base f41() { return std::codecvt_base(); }

std::codecvt_base f42(const std::codecvt_base &a0) {
  return std::codecvt_base(a0);
}

std::codecvt_base f43(std::codecvt_base &&a0) {
  return std::codecvt_base(std::move(a0));
}

std::codecvt_base &f44(std::codecvt_base &a0, const std::codecvt_base &a1) {
  return a0.operator=(a1);
}

std::codecvt_base &f45(std::codecvt_base &a0, std::codecvt_base &&a1) {
  return a0.operator=(std::move(a1));
}
