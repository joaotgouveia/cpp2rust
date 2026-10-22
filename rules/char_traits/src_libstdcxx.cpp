#include "../common/init.h"
#include "../common/traits.h"
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

std::char_traits<char>::pos_type f40() {
  return std::char_traits<char>::pos_type();
}

std::char_traits<char>::pos_type f41(std::streamoff a0) {
  return std::char_traits<char>::pos_type(a0);
}

std::char_traits<char>::pos_type
f42(const std::char_traits<char>::pos_type &a0) {
  return std::char_traits<char>::pos_type(a0);
}

std::char_traits<char>::pos_type &
f43(std::char_traits<char>::pos_type &a0,
    const std::char_traits<char>::pos_type &a1) {
  return a0.operator=(a1);
}

std::streamoff f44(const std::char_traits<char>::pos_type &a0) {
  return static_cast<std::streamoff>(a0);
}

void f45(std::char_traits<char>::pos_type &a0, std::mbstate_t a1) {
  return a0.state(std::move(a1));
}

std::mbstate_t f46(const std::char_traits<char>::pos_type &a0) {
  return a0.state();
}

std::char_traits<char>::pos_type &f47(std::char_traits<char>::pos_type &a0,
                                      std::streamoff a1) {
  return a0.operator+=(a1);
}

std::char_traits<char>::pos_type &f48(std::char_traits<char>::pos_type &a0,
                                      std::streamoff a1) {
  return a0.operator-=(a1);
}

std::char_traits<char>::pos_type f49(const std::char_traits<char>::pos_type &a0,
                                     std::streamoff a1) {
  return a0.operator+(a1);
}

std::char_traits<char>::pos_type f50(const std::char_traits<char>::pos_type &a0,
                                     std::streamoff a1) {
  return a0.operator-(a1);
}

std::streamoff f51(const std::char_traits<char>::pos_type &a0,
                   const std::char_traits<char>::pos_type &a1) {
  return a0.operator-(a1);
}

using t12 = std::char_traits<wchar_t>::char_type;

using t13 = std::char_traits<wchar_t>::int_type;

using t14 = std::char_traits<wchar_t>::off_type;

using t15 = std::char_traits<wchar_t>::pos_type;

using t16 = std::char_traits<wchar_t>::state_type;

void f52(std::char_traits<wchar_t>::char_type &a0,
         const std::char_traits<wchar_t>::char_type &a1) {
  return std::char_traits<wchar_t>::assign(a0, a1);
}

bool f53(const std::char_traits<wchar_t>::char_type &a0,
         const std::char_traits<wchar_t>::char_type &a1) {
  return std::char_traits<wchar_t>::eq(a0, a1);
}

bool f54(const std::char_traits<wchar_t>::char_type &a0,
         const std::char_traits<wchar_t>::char_type &a1) {
  return std::char_traits<wchar_t>::lt(a0, a1);
}

int f55(const std::char_traits<wchar_t>::char_type *a0,
        const std::char_traits<wchar_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<wchar_t>::compare(a0, a1, a2);
}

std::size_t f56(const std::char_traits<wchar_t>::char_type *a0) {
  return std::char_traits<wchar_t>::length(a0);
}

const std::char_traits<wchar_t>::char_type *
f57(const std::char_traits<wchar_t>::char_type *a0, std::size_t a1,
    const std::char_traits<wchar_t>::char_type &a2) {
  return std::char_traits<wchar_t>::find(a0, a1, a2);
}

std::char_traits<wchar_t>::char_type *
f58(std::char_traits<wchar_t>::char_type *a0,
    const std::char_traits<wchar_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<wchar_t>::move(a0, a1, a2);
}

std::char_traits<wchar_t>::char_type *
f59(std::char_traits<wchar_t>::char_type *a0,
    const std::char_traits<wchar_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<wchar_t>::copy(a0, a1, a2);
}

std::char_traits<wchar_t>::char_type *
f60(std::char_traits<wchar_t>::char_type *a0, std::size_t a1,
    std::char_traits<wchar_t>::char_type a2) {
  return std::char_traits<wchar_t>::assign(a0, a1, a2);
}

std::char_traits<wchar_t>::char_type
f61(const std::char_traits<wchar_t>::int_type &a0) {
  return std::char_traits<wchar_t>::to_char_type(a0);
}

std::char_traits<wchar_t>::int_type
f62(const std::char_traits<wchar_t>::char_type &a0) {
  return std::char_traits<wchar_t>::to_int_type(a0);
}

bool f63(const std::char_traits<wchar_t>::int_type &a0,
         const std::char_traits<wchar_t>::int_type &a1) {
  return std::char_traits<wchar_t>::eq_int_type(a0, a1);
}

std::char_traits<wchar_t>::int_type f64() {
  return std::char_traits<wchar_t>::eof();
}

std::char_traits<wchar_t>::int_type
f65(const std::char_traits<wchar_t>::int_type &a0) {
  return std::char_traits<wchar_t>::not_eof(a0);
}

std::char_traits<wchar_t>::pos_type f66() {
  return std::char_traits<wchar_t>::pos_type();
}

std::char_traits<wchar_t>::pos_type f67(std::streamoff a0) {
  return std::char_traits<wchar_t>::pos_type(a0);
}

std::char_traits<wchar_t>::pos_type
f68(const std::char_traits<wchar_t>::pos_type &a0) {
  return std::char_traits<wchar_t>::pos_type(a0);
}

std::char_traits<wchar_t>::pos_type &
f69(std::char_traits<wchar_t>::pos_type &a0,
    const std::char_traits<wchar_t>::pos_type &a1) {
  return a0.operator=(a1);
}

std::streamoff f70(const std::char_traits<wchar_t>::pos_type &a0) {
  return static_cast<std::streamoff>(a0);
}

void f71(std::char_traits<wchar_t>::pos_type &a0, std::mbstate_t a1) {
  return a0.state(std::move(a1));
}

std::mbstate_t f72(const std::char_traits<wchar_t>::pos_type &a0) {
  return a0.state();
}

std::char_traits<wchar_t>::pos_type &
f73(std::char_traits<wchar_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator+=(a1);
}

std::char_traits<wchar_t>::pos_type &
f74(std::char_traits<wchar_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator-=(a1);
}

std::char_traits<wchar_t>::pos_type
f75(const std::char_traits<wchar_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator+(a1);
}

std::char_traits<wchar_t>::pos_type
f76(const std::char_traits<wchar_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator-(a1);
}

std::streamoff f77(const std::char_traits<wchar_t>::pos_type &a0,
                   const std::char_traits<wchar_t>::pos_type &a1) {
  return a0.operator-(a1);
}

using t17 = std::char_traits<char16_t>::char_type;

using t18 = std::char_traits<char16_t>::int_type;

using t19 = std::char_traits<char16_t>::off_type;

using t20 = std::char_traits<char16_t>::pos_type;

using t21 = std::char_traits<char16_t>::state_type;

void f78(std::char_traits<char16_t>::char_type &a0,
         const std::char_traits<char16_t>::char_type &a1) {
  return std::char_traits<char16_t>::assign(a0, a1);
}

bool f79(const std::char_traits<char16_t>::char_type &a0,
         const std::char_traits<char16_t>::char_type &a1) {
  return std::char_traits<char16_t>::eq(a0, a1);
}

bool f80(const std::char_traits<char16_t>::char_type &a0,
         const std::char_traits<char16_t>::char_type &a1) {
  return std::char_traits<char16_t>::lt(a0, a1);
}

int f81(const std::char_traits<char16_t>::char_type *a0,
        const std::char_traits<char16_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char16_t>::compare(a0, a1, a2);
}

std::size_t f82(const std::char_traits<char16_t>::char_type *a0) {
  return std::char_traits<char16_t>::length(a0);
}

const std::char_traits<char16_t>::char_type *
f83(const std::char_traits<char16_t>::char_type *a0, std::size_t a1,
    const std::char_traits<char16_t>::char_type &a2) {
  return std::char_traits<char16_t>::find(a0, a1, a2);
}

std::char_traits<char16_t>::char_type *
f84(std::char_traits<char16_t>::char_type *a0,
    const std::char_traits<char16_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char16_t>::move(a0, a1, a2);
}

std::char_traits<char16_t>::char_type *
f85(std::char_traits<char16_t>::char_type *a0,
    const std::char_traits<char16_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char16_t>::copy(a0, a1, a2);
}

std::char_traits<char16_t>::char_type *
f86(std::char_traits<char16_t>::char_type *a0, std::size_t a1,
    std::char_traits<char16_t>::char_type a2) {
  return std::char_traits<char16_t>::assign(a0, a1, a2);
}

std::char_traits<char16_t>::char_type
f87(const std::char_traits<char16_t>::int_type &a0) {
  return std::char_traits<char16_t>::to_char_type(a0);
}

bool f88(const std::char_traits<char16_t>::int_type &a0,
         const std::char_traits<char16_t>::int_type &a1) {
  return std::char_traits<char16_t>::eq_int_type(a0, a1);
}

std::char_traits<char16_t>::int_type
f89(const std::char_traits<char16_t>::char_type &a0) {
  return std::char_traits<char16_t>::to_int_type(a0);
}

std::char_traits<char16_t>::int_type f90() {
  return std::char_traits<char16_t>::eof();
}

std::char_traits<char16_t>::int_type
f91(const std::char_traits<char16_t>::int_type &a0) {
  return std::char_traits<char16_t>::not_eof(a0);
}

std::char_traits<char16_t>::pos_type f92() {
  return std::char_traits<char16_t>::pos_type();
}

std::char_traits<char16_t>::pos_type f93(std::streamoff a0) {
  return std::char_traits<char16_t>::pos_type(a0);
}

std::char_traits<char16_t>::pos_type
f94(const std::char_traits<char16_t>::pos_type &a0) {
  return std::char_traits<char16_t>::pos_type(a0);
}

std::char_traits<char16_t>::pos_type &
f95(std::char_traits<char16_t>::pos_type &a0,
    const std::char_traits<char16_t>::pos_type &a1) {
  return a0.operator=(a1);
}

std::streamoff f96(const std::char_traits<char16_t>::pos_type &a0) {
  return static_cast<std::streamoff>(a0);
}

void f97(std::char_traits<char16_t>::pos_type &a0, std::mbstate_t a1) {
  return a0.state(std::move(a1));
}

std::mbstate_t f98(const std::char_traits<char16_t>::pos_type &a0) {
  return a0.state();
}

std::char_traits<char16_t>::pos_type &
f99(std::char_traits<char16_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator+=(a1);
}

std::char_traits<char16_t>::pos_type &
f100(std::char_traits<char16_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator-=(a1);
}

std::char_traits<char16_t>::pos_type
f101(const std::char_traits<char16_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator+(a1);
}

std::char_traits<char16_t>::pos_type
f102(const std::char_traits<char16_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator-(a1);
}

std::streamoff f103(const std::char_traits<char16_t>::pos_type &a0,
                    const std::char_traits<char16_t>::pos_type &a1) {
  return a0.operator-(a1);
}

using t22 = std::char_traits<char32_t>::char_type;

using t23 = std::char_traits<char32_t>::int_type;

using t24 = std::char_traits<char32_t>::off_type;

using t25 = std::char_traits<char32_t>::pos_type;

using t26 = std::char_traits<char32_t>::state_type;

void f104(std::char_traits<char32_t>::char_type &a0,
          const std::char_traits<char32_t>::char_type &a1) {
  return std::char_traits<char32_t>::assign(a0, a1);
}

bool f105(const std::char_traits<char32_t>::char_type &a0,
          const std::char_traits<char32_t>::char_type &a1) {
  return std::char_traits<char32_t>::eq(a0, a1);
}

bool f106(const std::char_traits<char32_t>::char_type &a0,
          const std::char_traits<char32_t>::char_type &a1) {
  return std::char_traits<char32_t>::lt(a0, a1);
}

int f107(const std::char_traits<char32_t>::char_type *a0,
         const std::char_traits<char32_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char32_t>::compare(a0, a1, a2);
}

std::size_t f108(const std::char_traits<char32_t>::char_type *a0) {
  return std::char_traits<char32_t>::length(a0);
}

const std::char_traits<char32_t>::char_type *
f109(const std::char_traits<char32_t>::char_type *a0, std::size_t a1,
     const std::char_traits<char32_t>::char_type &a2) {
  return std::char_traits<char32_t>::find(a0, a1, a2);
}

std::char_traits<char32_t>::char_type *
f110(std::char_traits<char32_t>::char_type *a0,
     const std::char_traits<char32_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char32_t>::move(a0, a1, a2);
}

std::char_traits<char32_t>::char_type *
f111(std::char_traits<char32_t>::char_type *a0,
     const std::char_traits<char32_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char32_t>::copy(a0, a1, a2);
}

std::char_traits<char32_t>::char_type *
f112(std::char_traits<char32_t>::char_type *a0, std::size_t a1,
     std::char_traits<char32_t>::char_type a2) {
  return std::char_traits<char32_t>::assign(a0, a1, a2);
}

std::char_traits<char32_t>::char_type
f113(const std::char_traits<char32_t>::int_type &a0) {
  return std::char_traits<char32_t>::to_char_type(a0);
}

std::char_traits<char32_t>::int_type
f114(const std::char_traits<char32_t>::char_type &a0) {
  return std::char_traits<char32_t>::to_int_type(a0);
}

bool f115(const std::char_traits<char32_t>::int_type &a0,
          const std::char_traits<char32_t>::int_type &a1) {
  return std::char_traits<char32_t>::eq_int_type(a0, a1);
}

std::char_traits<char32_t>::int_type f116() {
  return std::char_traits<char32_t>::eof();
}

std::char_traits<char32_t>::int_type
f117(const std::char_traits<char32_t>::int_type &a0) {
  return std::char_traits<char32_t>::not_eof(a0);
}

std::char_traits<char32_t>::pos_type f118() {
  return std::char_traits<char32_t>::pos_type();
}

std::char_traits<char32_t>::pos_type f119(std::streamoff a0) {
  return std::char_traits<char32_t>::pos_type(a0);
}

std::char_traits<char32_t>::pos_type
f120(const std::char_traits<char32_t>::pos_type &a0) {
  return std::char_traits<char32_t>::pos_type(a0);
}

std::char_traits<char32_t>::pos_type &
f121(std::char_traits<char32_t>::pos_type &a0,
     const std::char_traits<char32_t>::pos_type &a1) {
  return a0.operator=(a1);
}

std::streamoff f122(const std::char_traits<char32_t>::pos_type &a0) {
  return static_cast<std::streamoff>(a0);
}

void f123(std::char_traits<char32_t>::pos_type &a0, std::mbstate_t a1) {
  return a0.state(std::move(a1));
}

std::mbstate_t f124(const std::char_traits<char32_t>::pos_type &a0) {
  return a0.state();
}

std::char_traits<char32_t>::pos_type &
f125(std::char_traits<char32_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator+=(a1);
}

std::char_traits<char32_t>::pos_type &
f126(std::char_traits<char32_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator-=(a1);
}

std::char_traits<char32_t>::pos_type
f127(const std::char_traits<char32_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator+(a1);
}

std::char_traits<char32_t>::pos_type
f128(const std::char_traits<char32_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator-(a1);
}

std::streamoff f129(const std::char_traits<char32_t>::pos_type &a0,
                    const std::char_traits<char32_t>::pos_type &a1) {
  return a0.operator-(a1);
}

template <typename T1> using t27 = std::char_traits<T1>;

template <typename T1>
void f130(typename std::char_traits<T1>::char_type &a0,
          const typename std::char_traits<T1>::char_type &a1) {
  return std::char_traits<T1>::assign(a0, a1);
}

template <typename T1>
bool f131(const typename std::char_traits<T1>::char_type &a0,
          const typename std::char_traits<T1>::char_type &a1) {
  return std::char_traits<T1>::eq(a0, a1);
}

template <typename T1>
bool f132(const typename std::char_traits<T1>::char_type &a0,
          const typename std::char_traits<T1>::char_type &a1) {
  return std::char_traits<T1>::lt(a0, a1);
}

template <typename T1>
int f133(const typename std::char_traits<T1>::char_type *a0,
         const typename std::char_traits<T1>::char_type *a1, std::size_t a2) {
  return std::char_traits<T1>::compare(a0, a1, a2);
}

template <typename T1>
std::size_t f134(const typename std::char_traits<T1>::char_type *a0) {
  return std::char_traits<T1>::length(a0);
}

template <typename T1>
const typename std::char_traits<T1>::char_type *
f135(const typename std::char_traits<T1>::char_type *a0, std::size_t a1,
     const typename std::char_traits<T1>::char_type &a2) {
  return std::char_traits<T1>::find(a0, a1, a2);
}

template <typename T1>
typename std::char_traits<T1>::char_type *
f136(typename std::char_traits<T1>::char_type *a0,
     const typename std::char_traits<T1>::char_type *a1, std::size_t a2) {
  return std::char_traits<T1>::move(a0, a1, a2);
}

template <typename T1>
typename std::char_traits<T1>::char_type *
f137(typename std::char_traits<T1>::char_type *a0,
     const typename std::char_traits<T1>::char_type *a1, std::size_t a2) {
  return std::char_traits<T1>::copy(a0, a1, a2);
}

template <typename T1>
typename std::char_traits<T1>::char_type *
f138(typename std::char_traits<T1>::char_type *a0, std::size_t a1,
     typename std::char_traits<T1>::char_type a2) {
  return std::char_traits<T1>::assign(a0, a1, a2);
}

template <typename T1>
typename std::char_traits<T1>::char_type
f139(const typename std::char_traits<T1>::int_type &a0) {
  return std::char_traits<T1>::to_char_type(a0);
}

template <typename T1>
typename std::char_traits<T1>::int_type
f140(const typename std::char_traits<T1>::char_type &a0) {
  return std::char_traits<T1>::to_int_type(a0);
}

template <typename T1>
bool f141(const typename std::char_traits<T1>::int_type &a0,
          const typename std::char_traits<T1>::int_type &a1) {
  return std::char_traits<T1>::eq_int_type(a0, a1);
}

template <typename T1> typename std::char_traits<T1>::int_type f142() {
  return std::char_traits<T1>::eof();
}

template <typename T1>
typename std::char_traits<T1>::int_type
f143(const typename std::char_traits<T1>::int_type &a0) {
  return std::char_traits<T1>::not_eof(a0);
}
