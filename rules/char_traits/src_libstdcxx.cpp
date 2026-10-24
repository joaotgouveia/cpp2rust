#include <algorithm>
#include <string>
#include <utility>

bool f34(const std::char_traits<char>::char_type &a0,
         const std::char_traits<char>::char_type &a1) {
  return std::char_traits<char>::eq(a0, a1);
}

bool f35(const std::char_traits<char>::char_type &a0,
         const std::char_traits<char>::char_type &a1) {
  return std::char_traits<char>::lt(a0, a1);
}

std::char_traits<char>::char_type
f36(const std::char_traits<char>::int_type &a0) {
  return std::char_traits<char>::to_char_type(a0);
}

std::char_traits<char>::int_type
f37(const std::char_traits<char>::char_type &a0) {
  return std::char_traits<char>::to_int_type(a0);
}

bool f38(const std::char_traits<char>::int_type &a0,
         const std::char_traits<char>::int_type &a1) {
  return std::char_traits<char>::eq_int_type(a0, a1);
}

std::char_traits<char>::int_type
f39(const std::char_traits<char>::int_type &a0) {
  return std::char_traits<char>::not_eof(a0);
}

using t12 = std::char_traits<wchar_t>::char_type;

using t13 = std::char_traits<wchar_t>::int_type;

using t14 = std::char_traits<wchar_t>::off_type;

using t15 = std::char_traits<wchar_t>::pos_type;

using t16 = std::char_traits<wchar_t>::state_type;

void f40(std::char_traits<wchar_t>::char_type &a0,
         const std::char_traits<wchar_t>::char_type &a1) {
  return std::char_traits<wchar_t>::assign(a0, a1);
}

bool f41(const std::char_traits<wchar_t>::char_type &a0,
         const std::char_traits<wchar_t>::char_type &a1) {
  return std::char_traits<wchar_t>::eq(a0, a1);
}

bool f42(const std::char_traits<wchar_t>::char_type &a0,
         const std::char_traits<wchar_t>::char_type &a1) {
  return std::char_traits<wchar_t>::lt(a0, a1);
}

int f43(const std::char_traits<wchar_t>::char_type *a0,
        const std::char_traits<wchar_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<wchar_t>::compare(a0, a1, a2);
}

std::size_t f44(const std::char_traits<wchar_t>::char_type *a0) {
  return std::char_traits<wchar_t>::length(a0);
}

const std::char_traits<wchar_t>::char_type *
f45(const std::char_traits<wchar_t>::char_type *a0, std::size_t a1,
    const std::char_traits<wchar_t>::char_type &a2) {
  return std::char_traits<wchar_t>::find(a0, a1, a2);
}

std::char_traits<wchar_t>::char_type *
f46(std::char_traits<wchar_t>::char_type *a0,
    const std::char_traits<wchar_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<wchar_t>::move(a0, a1, a2);
}

std::char_traits<wchar_t>::char_type *
f47(std::char_traits<wchar_t>::char_type *a0,
    const std::char_traits<wchar_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<wchar_t>::copy(a0, a1, a2);
}

std::char_traits<wchar_t>::char_type *
f48(std::char_traits<wchar_t>::char_type *a0, std::size_t a1,
    std::char_traits<wchar_t>::char_type a2) {
  return std::char_traits<wchar_t>::assign(a0, a1, a2);
}

std::char_traits<wchar_t>::char_type
f49(const std::char_traits<wchar_t>::int_type &a0) {
  return std::char_traits<wchar_t>::to_char_type(a0);
}

std::char_traits<wchar_t>::int_type
f50(const std::char_traits<wchar_t>::char_type &a0) {
  return std::char_traits<wchar_t>::to_int_type(a0);
}

bool f51(const std::char_traits<wchar_t>::int_type &a0,
         const std::char_traits<wchar_t>::int_type &a1) {
  return std::char_traits<wchar_t>::eq_int_type(a0, a1);
}

std::char_traits<wchar_t>::int_type f52() {
  return std::char_traits<wchar_t>::eof();
}

std::char_traits<wchar_t>::int_type
f53(const std::char_traits<wchar_t>::int_type &a0) {
  return std::char_traits<wchar_t>::not_eof(a0);
}

using t17 = std::char_traits<char16_t>::char_type;

using t18 = std::char_traits<char16_t>::int_type;

using t19 = std::char_traits<char16_t>::off_type;

using t20 = std::char_traits<char16_t>::pos_type;

using t21 = std::char_traits<char16_t>::state_type;

void f54(std::char_traits<char16_t>::char_type &a0,
         const std::char_traits<char16_t>::char_type &a1) {
  return std::char_traits<char16_t>::assign(a0, a1);
}

bool f55(const std::char_traits<char16_t>::char_type &a0,
         const std::char_traits<char16_t>::char_type &a1) {
  return std::char_traits<char16_t>::eq(a0, a1);
}

bool f56(const std::char_traits<char16_t>::char_type &a0,
         const std::char_traits<char16_t>::char_type &a1) {
  return std::char_traits<char16_t>::lt(a0, a1);
}

int f57(const std::char_traits<char16_t>::char_type *a0,
        const std::char_traits<char16_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char16_t>::compare(a0, a1, a2);
}

std::size_t f58(const std::char_traits<char16_t>::char_type *a0) {
  return std::char_traits<char16_t>::length(a0);
}

const std::char_traits<char16_t>::char_type *
f59(const std::char_traits<char16_t>::char_type *a0, std::size_t a1,
    const std::char_traits<char16_t>::char_type &a2) {
  return std::char_traits<char16_t>::find(a0, a1, a2);
}

std::char_traits<char16_t>::char_type *
f60(std::char_traits<char16_t>::char_type *a0,
    const std::char_traits<char16_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char16_t>::move(a0, a1, a2);
}

std::char_traits<char16_t>::char_type *
f61(std::char_traits<char16_t>::char_type *a0,
    const std::char_traits<char16_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char16_t>::copy(a0, a1, a2);
}

std::char_traits<char16_t>::char_type *
f62(std::char_traits<char16_t>::char_type *a0, std::size_t a1,
    std::char_traits<char16_t>::char_type a2) {
  return std::char_traits<char16_t>::assign(a0, a1, a2);
}

std::char_traits<char16_t>::char_type
f63(const std::char_traits<char16_t>::int_type &a0) {
  return std::char_traits<char16_t>::to_char_type(a0);
}

bool f64(const std::char_traits<char16_t>::int_type &a0,
         const std::char_traits<char16_t>::int_type &a1) {
  return std::char_traits<char16_t>::eq_int_type(a0, a1);
}

std::char_traits<char16_t>::int_type
f65(const std::char_traits<char16_t>::char_type &a0) {
  return std::char_traits<char16_t>::to_int_type(a0);
}

std::char_traits<char16_t>::int_type f66() {
  return std::char_traits<char16_t>::eof();
}

std::char_traits<char16_t>::int_type
f67(const std::char_traits<char16_t>::int_type &a0) {
  return std::char_traits<char16_t>::not_eof(a0);
}

using t22 = std::char_traits<char32_t>::char_type;

using t23 = std::char_traits<char32_t>::int_type;

using t24 = std::char_traits<char32_t>::off_type;

using t25 = std::char_traits<char32_t>::pos_type;

using t26 = std::char_traits<char32_t>::state_type;

void f68(std::char_traits<char32_t>::char_type &a0,
         const std::char_traits<char32_t>::char_type &a1) {
  return std::char_traits<char32_t>::assign(a0, a1);
}

bool f69(const std::char_traits<char32_t>::char_type &a0,
         const std::char_traits<char32_t>::char_type &a1) {
  return std::char_traits<char32_t>::eq(a0, a1);
}

bool f70(const std::char_traits<char32_t>::char_type &a0,
         const std::char_traits<char32_t>::char_type &a1) {
  return std::char_traits<char32_t>::lt(a0, a1);
}

int f71(const std::char_traits<char32_t>::char_type *a0,
        const std::char_traits<char32_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char32_t>::compare(a0, a1, a2);
}

std::size_t f72(const std::char_traits<char32_t>::char_type *a0) {
  return std::char_traits<char32_t>::length(a0);
}

const std::char_traits<char32_t>::char_type *
f73(const std::char_traits<char32_t>::char_type *a0, std::size_t a1,
    const std::char_traits<char32_t>::char_type &a2) {
  return std::char_traits<char32_t>::find(a0, a1, a2);
}

std::char_traits<char32_t>::char_type *
f74(std::char_traits<char32_t>::char_type *a0,
    const std::char_traits<char32_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char32_t>::move(a0, a1, a2);
}

std::char_traits<char32_t>::char_type *
f75(std::char_traits<char32_t>::char_type *a0,
    const std::char_traits<char32_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char32_t>::copy(a0, a1, a2);
}

std::char_traits<char32_t>::char_type *
f76(std::char_traits<char32_t>::char_type *a0, std::size_t a1,
    std::char_traits<char32_t>::char_type a2) {
  return std::char_traits<char32_t>::assign(a0, a1, a2);
}

std::char_traits<char32_t>::char_type
f77(const std::char_traits<char32_t>::int_type &a0) {
  return std::char_traits<char32_t>::to_char_type(a0);
}

std::char_traits<char32_t>::int_type
f78(const std::char_traits<char32_t>::char_type &a0) {
  return std::char_traits<char32_t>::to_int_type(a0);
}

bool f79(const std::char_traits<char32_t>::int_type &a0,
         const std::char_traits<char32_t>::int_type &a1) {
  return std::char_traits<char32_t>::eq_int_type(a0, a1);
}

std::char_traits<char32_t>::int_type f80() {
  return std::char_traits<char32_t>::eof();
}

std::char_traits<char32_t>::int_type
f81(const std::char_traits<char32_t>::int_type &a0) {
  return std::char_traits<char32_t>::not_eof(a0);
}

template <typename T1> using t27 = std::char_traits<T1>;

template <typename T1> void f82(T1 &a0, T1 const &a1) {
  return std::char_traits<T1>::assign(a0, a1);
}

template <typename T1> bool f83(T1 const &a0, T1 const &a1) {
  return std::char_traits<T1>::eq(a0, a1);
}

template <typename T1> bool f84(T1 const &a0, T1 const &a1) {
  return std::char_traits<T1>::lt(a0, a1);
}

template <typename T1> int f85(T1 const *a0, T1 const *a1, std::size_t a2) {
  return std::char_traits<T1>::compare(a0, a1, a2);
}

template <typename T1> std::size_t f86(T1 const *a0) {
  return std::char_traits<T1>::length(a0);
}

template <typename T1>
T1 const *f87(T1 const *a0, std::size_t a1, T1 const &a2) {
  return std::char_traits<T1>::find(a0, a1, a2);
}

template <typename T1> T1 *f88(T1 *a0, T1 const *a1, std::size_t a2) {
  return std::char_traits<T1>::move(a0, a1, a2);
}

template <typename T1> T1 *f89(T1 *a0, T1 const *a1, std::size_t a2) {
  return std::char_traits<T1>::copy(a0, a1, a2);
}

template <typename T1> T1 *f90(T1 *a0, std::size_t a1, T1 a2) {
  return std::char_traits<T1>::assign(a0, a1, std::move(a2));
}

template <typename T1> T1 f91(const unsigned long &a0) {
  return std::char_traits<T1>::to_char_type(a0);
}

template <typename T1> unsigned long f92(T1 const &a0) {
  return std::char_traits<T1>::to_int_type(a0);
}

template <typename T1>
bool f93(const unsigned long &a0, const unsigned long &a1) {
  return std::char_traits<T1>::eq_int_type(a0, a1);
}

template <typename T1> unsigned long f94() {
  return std::char_traits<T1>::eof();
}

template <typename T1> unsigned long f95(const unsigned long &a0) {
  return std::char_traits<T1>::not_eof(a0);
}
