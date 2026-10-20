#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <string>
#include <utility>

using t1 = std::char_traits<char>;

using t2 = std::char_traits<char>::char_type;

using t3 = std::char_traits<char>::int_type;

using t4 = std::char_traits<char>::pos_type;

using t5 = std::char_traits<char>::off_type;

using t6 = std::char_traits<char>::state_type;

void f1(std::char_traits<char>::char_type &a0,
        const std::char_traits<char>::char_type &a1) {
  return std::char_traits<char>::assign(a0, a1);
}

bool f2(const std::char_traits<char>::char_type &a0,
        const std::char_traits<char>::char_type &a1) {
  return std::char_traits<char>::eq(a0, a1);
}

bool f3(const std::char_traits<char>::char_type &a0,
        const std::char_traits<char>::char_type &a1) {
  return std::char_traits<char>::lt(a0, a1);
}

int f4(const std::char_traits<char>::char_type *a0,
       const std::char_traits<char>::char_type *a1, std::size_t a2) {
  return std::char_traits<char>::compare(a0, a1, a2);
}

std::size_t f5(const std::char_traits<char>::char_type *a0) {
  return std::char_traits<char>::length(a0);
}

const std::char_traits<char>::char_type *
f6(const std::char_traits<char>::char_type *a0, std::size_t a1,
   const std::char_traits<char>::char_type &a2) {
  return std::char_traits<char>::find(a0, a1, a2);
}

std::char_traits<char>::char_type *
f7(std::char_traits<char>::char_type *a0,
   const std::char_traits<char>::char_type *a1, std::size_t a2) {
  return std::char_traits<char>::move(a0, a1, a2);
}

std::char_traits<char>::char_type *
f8(std::char_traits<char>::char_type *a0,
   const std::char_traits<char>::char_type *a1, std::size_t a2) {
  return std::char_traits<char>::copy(a0, a1, a2);
}

std::char_traits<char>::char_type *f9(std::char_traits<char>::char_type *a0,
                                      std::size_t a1,
                                      std::char_traits<char>::char_type a2) {
  return std::char_traits<char>::assign(a0, a1, a2);
}

std::char_traits<char>::char_type
f10(const std::char_traits<char>::int_type &a0) {
  return std::char_traits<char>::to_char_type(a0);
}

std::char_traits<char>::int_type
f11(const std::char_traits<char>::char_type &a0) {
  return std::char_traits<char>::to_int_type(a0);
}

bool f12(const std::char_traits<char>::int_type &a0,
         const std::char_traits<char>::int_type &a1) {
  return std::char_traits<char>::eq_int_type(a0, a1);
}

std::char_traits<char>::int_type f13() { return std::char_traits<char>::eof(); }

std::char_traits<char>::int_type
f14(const std::char_traits<char>::int_type &a0) {
  return std::char_traits<char>::not_eof(a0);
}

std::char_traits<char> f15() { return std::char_traits<char>(); }

std::char_traits<char> f16(const std::char_traits<char> &a0) {
  return std::char_traits<char>(a0);
}

std::char_traits<char> f17(std::char_traits<char> &&a0) {
  return std::char_traits<char>(std::move(a0));
}

std::char_traits<char> &f18(std::char_traits<char> &a0,
                            const std::char_traits<char> &a1) {
  return a0.operator=(a1);
}

std::char_traits<char> &f19(std::char_traits<char> &a0,
                            std::char_traits<char> &&a1) {
  return a0.operator=(std::move(a1));
}

std::char_traits<char>::pos_type f20() {
  return std::char_traits<char>::pos_type();
}

std::char_traits<char>::pos_type f21(std::streamoff a0) {
  return std::char_traits<char>::pos_type(a0);
}

std::char_traits<char>::pos_type
f22(const std::char_traits<char>::pos_type &a0) {
  return std::char_traits<char>::pos_type(a0);
}

std::char_traits<char>::pos_type &
f23(std::char_traits<char>::pos_type &a0,
    const std::char_traits<char>::pos_type &a1) {
  return a0.operator=(a1);
}

std::streamoff f24(const std::char_traits<char>::pos_type &a0) {
  return static_cast<std::streamoff>(a0);
}

void f25(std::char_traits<char>::pos_type &a0, std::mbstate_t a1) {
  return a0.state(std::move(a1));
}

std::mbstate_t f26(const std::char_traits<char>::pos_type &a0) {
  return a0.state();
}

std::char_traits<char>::pos_type &f27(std::char_traits<char>::pos_type &a0,
                                      std::streamoff a1) {
  return a0.operator+=(a1);
}

std::char_traits<char>::pos_type &f28(std::char_traits<char>::pos_type &a0,
                                      std::streamoff a1) {
  return a0.operator-=(a1);
}

std::char_traits<char>::pos_type f29(const std::char_traits<char>::pos_type &a0,
                                     std::streamoff a1) {
  return a0.operator+(a1);
}

std::char_traits<char>::pos_type f30(const std::char_traits<char>::pos_type &a0,
                                     std::streamoff a1) {
  return a0.operator-(a1);
}

std::streamoff f31(const std::char_traits<char>::pos_type &a0,
                   const std::char_traits<char>::pos_type &a1) {
  return a0.operator-(a1);
}

using t7 = std::char_traits<wchar_t>;

using t8 = std::char_traits<wchar_t>::char_type;

using t9 = std::char_traits<wchar_t>::int_type;

using t10 = std::char_traits<wchar_t>::off_type;

using t11 = std::char_traits<wchar_t>::pos_type;

using t12 = std::char_traits<wchar_t>::state_type;

void f32(std::char_traits<wchar_t>::char_type &a0,
         const std::char_traits<wchar_t>::char_type &a1) {
  return std::char_traits<wchar_t>::assign(a0, a1);
}

bool f33(const std::char_traits<wchar_t>::char_type &a0,
         const std::char_traits<wchar_t>::char_type &a1) {
  return std::char_traits<wchar_t>::eq(a0, a1);
}

bool f34(const std::char_traits<wchar_t>::char_type &a0,
         const std::char_traits<wchar_t>::char_type &a1) {
  return std::char_traits<wchar_t>::lt(a0, a1);
}

int f35(const std::char_traits<wchar_t>::char_type *a0,
        const std::char_traits<wchar_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<wchar_t>::compare(a0, a1, a2);
}

std::size_t f36(const std::char_traits<wchar_t>::char_type *a0) {
  return std::char_traits<wchar_t>::length(a0);
}

const std::char_traits<wchar_t>::char_type *
f37(const std::char_traits<wchar_t>::char_type *a0, std::size_t a1,
    const std::char_traits<wchar_t>::char_type &a2) {
  return std::char_traits<wchar_t>::find(a0, a1, a2);
}

std::char_traits<wchar_t>::char_type *
f38(std::char_traits<wchar_t>::char_type *a0,
    const std::char_traits<wchar_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<wchar_t>::move(a0, a1, a2);
}

std::char_traits<wchar_t>::char_type *
f39(std::char_traits<wchar_t>::char_type *a0,
    const std::char_traits<wchar_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<wchar_t>::copy(a0, a1, a2);
}

std::char_traits<wchar_t>::char_type *
f40(std::char_traits<wchar_t>::char_type *a0, std::size_t a1,
    std::char_traits<wchar_t>::char_type a2) {
  return std::char_traits<wchar_t>::assign(a0, a1, a2);
}

std::char_traits<wchar_t>::char_type
f41(const std::char_traits<wchar_t>::int_type &a0) {
  return std::char_traits<wchar_t>::to_char_type(a0);
}

std::char_traits<wchar_t>::int_type
f42(const std::char_traits<wchar_t>::char_type &a0) {
  return std::char_traits<wchar_t>::to_int_type(a0);
}

bool f43(const std::char_traits<wchar_t>::int_type &a0,
         const std::char_traits<wchar_t>::int_type &a1) {
  return std::char_traits<wchar_t>::eq_int_type(a0, a1);
}

std::char_traits<wchar_t>::int_type f44() {
  return std::char_traits<wchar_t>::eof();
}

std::char_traits<wchar_t>::int_type
f45(const std::char_traits<wchar_t>::int_type &a0) {
  return std::char_traits<wchar_t>::not_eof(a0);
}

std::char_traits<wchar_t> f46() { return std::char_traits<wchar_t>(); }

std::char_traits<wchar_t> f47(const std::char_traits<wchar_t> &a0) {
  return std::char_traits<wchar_t>(a0);
}

std::char_traits<wchar_t> f48(std::char_traits<wchar_t> &&a0) {
  return std::char_traits<wchar_t>(std::move(a0));
}

std::char_traits<wchar_t> &f49(std::char_traits<wchar_t> &a0,
                               const std::char_traits<wchar_t> &a1) {
  return a0.operator=(a1);
}

std::char_traits<wchar_t> &f50(std::char_traits<wchar_t> &a0,
                               std::char_traits<wchar_t> &&a1) {
  return a0.operator=(std::move(a1));
}

std::char_traits<wchar_t>::pos_type f51() {
  return std::char_traits<wchar_t>::pos_type();
}

std::char_traits<wchar_t>::pos_type f52(std::streamoff a0) {
  return std::char_traits<wchar_t>::pos_type(a0);
}

std::char_traits<wchar_t>::pos_type
f53(const std::char_traits<wchar_t>::pos_type &a0) {
  return std::char_traits<wchar_t>::pos_type(a0);
}

std::char_traits<wchar_t>::pos_type &
f54(std::char_traits<wchar_t>::pos_type &a0,
    const std::char_traits<wchar_t>::pos_type &a1) {
  return a0.operator=(a1);
}

std::streamoff f55(const std::char_traits<wchar_t>::pos_type &a0) {
  return static_cast<std::streamoff>(a0);
}

void f56(std::char_traits<wchar_t>::pos_type &a0, std::mbstate_t a1) {
  return a0.state(std::move(a1));
}

std::mbstate_t f57(const std::char_traits<wchar_t>::pos_type &a0) {
  return a0.state();
}

std::char_traits<wchar_t>::pos_type &
f58(std::char_traits<wchar_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator+=(a1);
}

std::char_traits<wchar_t>::pos_type &
f59(std::char_traits<wchar_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator-=(a1);
}

std::char_traits<wchar_t>::pos_type
f60(const std::char_traits<wchar_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator+(a1);
}

std::char_traits<wchar_t>::pos_type
f61(const std::char_traits<wchar_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator-(a1);
}

std::streamoff f62(const std::char_traits<wchar_t>::pos_type &a0,
                   const std::char_traits<wchar_t>::pos_type &a1) {
  return a0.operator-(a1);
}

using t13 = std::char_traits<char16_t>;

using t14 = std::char_traits<char16_t>::char_type;

using t15 = std::char_traits<char16_t>::int_type;

using t16 = std::char_traits<char16_t>::off_type;

using t17 = std::char_traits<char16_t>::pos_type;

using t18 = std::char_traits<char16_t>::state_type;

void f63(std::char_traits<char16_t>::char_type &a0,
         const std::char_traits<char16_t>::char_type &a1) {
  return std::char_traits<char16_t>::assign(a0, a1);
}

bool f64(const std::char_traits<char16_t>::char_type &a0,
         const std::char_traits<char16_t>::char_type &a1) {
  return std::char_traits<char16_t>::eq(a0, a1);
}

bool f65(const std::char_traits<char16_t>::char_type &a0,
         const std::char_traits<char16_t>::char_type &a1) {
  return std::char_traits<char16_t>::lt(a0, a1);
}

int f66(const std::char_traits<char16_t>::char_type *a0,
        const std::char_traits<char16_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char16_t>::compare(a0, a1, a2);
}

std::size_t f67(const std::char_traits<char16_t>::char_type *a0) {
  return std::char_traits<char16_t>::length(a0);
}

const std::char_traits<char16_t>::char_type *
f68(const std::char_traits<char16_t>::char_type *a0, std::size_t a1,
    const std::char_traits<char16_t>::char_type &a2) {
  return std::char_traits<char16_t>::find(a0, a1, a2);
}

std::char_traits<char16_t>::char_type *
f69(std::char_traits<char16_t>::char_type *a0,
    const std::char_traits<char16_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char16_t>::move(a0, a1, a2);
}

std::char_traits<char16_t>::char_type *
f70(std::char_traits<char16_t>::char_type *a0,
    const std::char_traits<char16_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char16_t>::copy(a0, a1, a2);
}

std::char_traits<char16_t>::char_type *
f71(std::char_traits<char16_t>::char_type *a0, std::size_t a1,
    std::char_traits<char16_t>::char_type a2) {
  return std::char_traits<char16_t>::assign(a0, a1, a2);
}

std::char_traits<char16_t>::char_type
f72(const std::char_traits<char16_t>::int_type &a0) {
  return std::char_traits<char16_t>::to_char_type(a0);
}

bool f73(const std::char_traits<char16_t>::int_type &a0,
         const std::char_traits<char16_t>::int_type &a1) {
  return std::char_traits<char16_t>::eq_int_type(a0, a1);
}

std::char_traits<char16_t>::int_type
f74(const std::char_traits<char16_t>::char_type &a0) {
  return std::char_traits<char16_t>::to_int_type(a0);
}

std::char_traits<char16_t>::int_type f75() {
  return std::char_traits<char16_t>::eof();
}

std::char_traits<char16_t>::int_type
f76(const std::char_traits<char16_t>::int_type &a0) {
  return std::char_traits<char16_t>::not_eof(a0);
}

std::char_traits<char16_t> f77() { return std::char_traits<char16_t>(); }

std::char_traits<char16_t> f78(const std::char_traits<char16_t> &a0) {
  return std::char_traits<char16_t>(a0);
}

std::char_traits<char16_t> f79(std::char_traits<char16_t> &&a0) {
  return std::char_traits<char16_t>(std::move(a0));
}

std::char_traits<char16_t> &f80(std::char_traits<char16_t> &a0,
                                const std::char_traits<char16_t> &a1) {
  return a0.operator=(a1);
}

std::char_traits<char16_t> &f81(std::char_traits<char16_t> &a0,
                                std::char_traits<char16_t> &&a1) {
  return a0.operator=(std::move(a1));
}

std::char_traits<char16_t>::pos_type f82() {
  return std::char_traits<char16_t>::pos_type();
}

std::char_traits<char16_t>::pos_type f83(std::streamoff a0) {
  return std::char_traits<char16_t>::pos_type(a0);
}

std::char_traits<char16_t>::pos_type
f84(const std::char_traits<char16_t>::pos_type &a0) {
  return std::char_traits<char16_t>::pos_type(a0);
}

std::char_traits<char16_t>::pos_type &
f85(std::char_traits<char16_t>::pos_type &a0,
    const std::char_traits<char16_t>::pos_type &a1) {
  return a0.operator=(a1);
}

std::streamoff f86(const std::char_traits<char16_t>::pos_type &a0) {
  return static_cast<std::streamoff>(a0);
}

void f87(std::char_traits<char16_t>::pos_type &a0, std::mbstate_t a1) {
  return a0.state(std::move(a1));
}

std::mbstate_t f88(const std::char_traits<char16_t>::pos_type &a0) {
  return a0.state();
}

std::char_traits<char16_t>::pos_type &
f89(std::char_traits<char16_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator+=(a1);
}

std::char_traits<char16_t>::pos_type &
f90(std::char_traits<char16_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator-=(a1);
}

std::char_traits<char16_t>::pos_type
f91(const std::char_traits<char16_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator+(a1);
}

std::char_traits<char16_t>::pos_type
f92(const std::char_traits<char16_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator-(a1);
}

std::streamoff f93(const std::char_traits<char16_t>::pos_type &a0,
                   const std::char_traits<char16_t>::pos_type &a1) {
  return a0.operator-(a1);
}

using t19 = std::char_traits<char32_t>;

using t20 = std::char_traits<char32_t>::char_type;

using t21 = std::char_traits<char32_t>::int_type;

using t22 = std::char_traits<char32_t>::off_type;

using t23 = std::char_traits<char32_t>::pos_type;

using t24 = std::char_traits<char32_t>::state_type;

void f94(std::char_traits<char32_t>::char_type &a0,
         const std::char_traits<char32_t>::char_type &a1) {
  return std::char_traits<char32_t>::assign(a0, a1);
}

bool f95(const std::char_traits<char32_t>::char_type &a0,
         const std::char_traits<char32_t>::char_type &a1) {
  return std::char_traits<char32_t>::eq(a0, a1);
}

bool f96(const std::char_traits<char32_t>::char_type &a0,
         const std::char_traits<char32_t>::char_type &a1) {
  return std::char_traits<char32_t>::lt(a0, a1);
}

int f97(const std::char_traits<char32_t>::char_type *a0,
        const std::char_traits<char32_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char32_t>::compare(a0, a1, a2);
}

std::size_t f98(const std::char_traits<char32_t>::char_type *a0) {
  return std::char_traits<char32_t>::length(a0);
}

const std::char_traits<char32_t>::char_type *
f99(const std::char_traits<char32_t>::char_type *a0, std::size_t a1,
    const std::char_traits<char32_t>::char_type &a2) {
  return std::char_traits<char32_t>::find(a0, a1, a2);
}

std::char_traits<char32_t>::char_type *
f100(std::char_traits<char32_t>::char_type *a0,
     const std::char_traits<char32_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char32_t>::move(a0, a1, a2);
}

std::char_traits<char32_t>::char_type *
f101(std::char_traits<char32_t>::char_type *a0,
     const std::char_traits<char32_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char32_t>::copy(a0, a1, a2);
}

std::char_traits<char32_t>::char_type *
f102(std::char_traits<char32_t>::char_type *a0, std::size_t a1,
     std::char_traits<char32_t>::char_type a2) {
  return std::char_traits<char32_t>::assign(a0, a1, a2);
}

std::char_traits<char32_t>::char_type
f103(const std::char_traits<char32_t>::int_type &a0) {
  return std::char_traits<char32_t>::to_char_type(a0);
}

std::char_traits<char32_t>::int_type
f104(const std::char_traits<char32_t>::char_type &a0) {
  return std::char_traits<char32_t>::to_int_type(a0);
}

bool f105(const std::char_traits<char32_t>::int_type &a0,
          const std::char_traits<char32_t>::int_type &a1) {
  return std::char_traits<char32_t>::eq_int_type(a0, a1);
}

std::char_traits<char32_t>::int_type f106() {
  return std::char_traits<char32_t>::eof();
}

std::char_traits<char32_t>::int_type
f107(const std::char_traits<char32_t>::int_type &a0) {
  return std::char_traits<char32_t>::not_eof(a0);
}

std::char_traits<char32_t> f108() { return std::char_traits<char32_t>(); }

std::char_traits<char32_t> f109(const std::char_traits<char32_t> &a0) {
  return std::char_traits<char32_t>(a0);
}

std::char_traits<char32_t> f110(std::char_traits<char32_t> &&a0) {
  return std::char_traits<char32_t>(std::move(a0));
}

std::char_traits<char32_t> &f111(std::char_traits<char32_t> &a0,
                                 const std::char_traits<char32_t> &a1) {
  return a0.operator=(a1);
}

std::char_traits<char32_t> &f112(std::char_traits<char32_t> &a0,
                                 std::char_traits<char32_t> &&a1) {
  return a0.operator=(std::move(a1));
}

std::char_traits<char32_t>::pos_type f113() {
  return std::char_traits<char32_t>::pos_type();
}

std::char_traits<char32_t>::pos_type f114(std::streamoff a0) {
  return std::char_traits<char32_t>::pos_type(a0);
}

std::char_traits<char32_t>::pos_type
f115(const std::char_traits<char32_t>::pos_type &a0) {
  return std::char_traits<char32_t>::pos_type(a0);
}

std::char_traits<char32_t>::pos_type &
f116(std::char_traits<char32_t>::pos_type &a0,
     const std::char_traits<char32_t>::pos_type &a1) {
  return a0.operator=(a1);
}

std::streamoff f117(const std::char_traits<char32_t>::pos_type &a0) {
  return static_cast<std::streamoff>(a0);
}

void f118(std::char_traits<char32_t>::pos_type &a0, std::mbstate_t a1) {
  return a0.state(std::move(a1));
}

std::mbstate_t f119(const std::char_traits<char32_t>::pos_type &a0) {
  return a0.state();
}

std::char_traits<char32_t>::pos_type &
f120(std::char_traits<char32_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator+=(a1);
}

std::char_traits<char32_t>::pos_type &
f121(std::char_traits<char32_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator-=(a1);
}

std::char_traits<char32_t>::pos_type
f122(const std::char_traits<char32_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator+(a1);
}

std::char_traits<char32_t>::pos_type
f123(const std::char_traits<char32_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator-(a1);
}

std::streamoff f124(const std::char_traits<char32_t>::pos_type &a0,
                    const std::char_traits<char32_t>::pos_type &a1) {
  return a0.operator-(a1);
}

template <typename T1> using t25 = std::char_traits<T1>;

template <typename T1>
void f125(typename std::char_traits<T1>::char_type &a0,
          const typename std::char_traits<T1>::char_type &a1) {
  return std::char_traits<T1>::assign(a0, a1);
}

template <typename T1>
bool f126(const typename std::char_traits<T1>::char_type &a0,
          const typename std::char_traits<T1>::char_type &a1) {
  return std::char_traits<T1>::eq(a0, a1);
}

template <typename T1>
bool f127(const typename std::char_traits<T1>::char_type &a0,
          const typename std::char_traits<T1>::char_type &a1) {
  return std::char_traits<T1>::lt(a0, a1);
}

template <typename T1>
int f128(const typename std::char_traits<T1>::char_type *a0,
         const typename std::char_traits<T1>::char_type *a1, std::size_t a2) {
  return std::char_traits<T1>::compare(a0, a1, a2);
}

template <typename T1>
std::size_t f129(const typename std::char_traits<T1>::char_type *a0) {
  return std::char_traits<T1>::length(a0);
}

template <typename T1>
const typename std::char_traits<T1>::char_type *
f130(const typename std::char_traits<T1>::char_type *a0, std::size_t a1,
     const typename std::char_traits<T1>::char_type &a2) {
  return std::char_traits<T1>::find(a0, a1, a2);
}

template <typename T1>
typename std::char_traits<T1>::char_type *
f131(typename std::char_traits<T1>::char_type *a0,
     const typename std::char_traits<T1>::char_type *a1, std::size_t a2) {
  return std::char_traits<T1>::move(a0, a1, a2);
}

template <typename T1>
typename std::char_traits<T1>::char_type *
f132(typename std::char_traits<T1>::char_type *a0,
     const typename std::char_traits<T1>::char_type *a1, std::size_t a2) {
  return std::char_traits<T1>::copy(a0, a1, a2);
}

template <typename T1>
typename std::char_traits<T1>::char_type *
f133(typename std::char_traits<T1>::char_type *a0, std::size_t a1,
     typename std::char_traits<T1>::char_type a2) {
  return std::char_traits<T1>::assign(a0, a1, a2);
}

template <typename T1>
typename std::char_traits<T1>::char_type
f134(const typename std::char_traits<T1>::int_type &a0) {
  return std::char_traits<T1>::to_char_type(a0);
}

template <typename T1>
typename std::char_traits<T1>::int_type
f135(const typename std::char_traits<T1>::char_type &a0) {
  return std::char_traits<T1>::to_int_type(a0);
}

template <typename T1>
bool f136(const typename std::char_traits<T1>::int_type &a0,
          const typename std::char_traits<T1>::int_type &a1) {
  return std::char_traits<T1>::eq_int_type(a0, a1);
}

template <typename T1> typename std::char_traits<T1>::int_type f137() {
  return std::char_traits<T1>::eof();
}

template <typename T1>
typename std::char_traits<T1>::int_type
f138(const typename std::char_traits<T1>::int_type &a0) {
  return std::char_traits<T1>::not_eof(a0);
}
