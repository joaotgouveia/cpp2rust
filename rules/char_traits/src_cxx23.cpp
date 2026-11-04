#include <algorithm>
#include <ios>
#include <iosfwd>
#include <utility>

using t1 = std::char_traits<char>;

using t2 = std::char_traits<char>::char_type;

using t3 = std::char_traits<char>::int_type;

using t4 = std::char_traits<char>::pos_type;

using t5 = std::char_traits<char>::off_type;

using t6 = std::char_traits<char>::state_type;

using t7 = std::char_traits<char>::comparison_category;

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

using t8 = std::char_traits<wchar_t>;

using t9 = std::char_traits<wchar_t>::char_type;

using t10 = std::char_traits<wchar_t>::int_type;

using t11 = std::char_traits<wchar_t>::off_type;

using t12 = std::char_traits<wchar_t>::pos_type;

using t13 = std::char_traits<wchar_t>::state_type;

using t14 = std::char_traits<wchar_t>::comparison_category;

void f20(std::char_traits<wchar_t>::char_type &a0,
         const std::char_traits<wchar_t>::char_type &a1) {
  return std::char_traits<wchar_t>::assign(a0, a1);
}

bool f21(const std::char_traits<wchar_t>::char_type &a0,
         const std::char_traits<wchar_t>::char_type &a1) {
  return std::char_traits<wchar_t>::eq(a0, a1);
}

bool f22(const std::char_traits<wchar_t>::char_type &a0,
         const std::char_traits<wchar_t>::char_type &a1) {
  return std::char_traits<wchar_t>::lt(a0, a1);
}

int f23(const std::char_traits<wchar_t>::char_type *a0,
        const std::char_traits<wchar_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<wchar_t>::compare(a0, a1, a2);
}

std::size_t f24(const std::char_traits<wchar_t>::char_type *a0) {
  return std::char_traits<wchar_t>::length(a0);
}

const std::char_traits<wchar_t>::char_type *
f25(const std::char_traits<wchar_t>::char_type *a0, std::size_t a1,
    const std::char_traits<wchar_t>::char_type &a2) {
  return std::char_traits<wchar_t>::find(a0, a1, a2);
}

std::char_traits<wchar_t>::char_type *
f26(std::char_traits<wchar_t>::char_type *a0,
    const std::char_traits<wchar_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<wchar_t>::move(a0, a1, a2);
}

std::char_traits<wchar_t>::char_type *
f27(std::char_traits<wchar_t>::char_type *a0,
    const std::char_traits<wchar_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<wchar_t>::copy(a0, a1, a2);
}

std::char_traits<wchar_t>::char_type *
f28(std::char_traits<wchar_t>::char_type *a0, std::size_t a1,
    std::char_traits<wchar_t>::char_type a2) {
  return std::char_traits<wchar_t>::assign(a0, a1, a2);
}

std::char_traits<wchar_t>::char_type
f29(const std::char_traits<wchar_t>::int_type &a0) {
  return std::char_traits<wchar_t>::to_char_type(a0);
}

std::char_traits<wchar_t>::int_type
f30(const std::char_traits<wchar_t>::char_type &a0) {
  return std::char_traits<wchar_t>::to_int_type(a0);
}

bool f31(const std::char_traits<wchar_t>::int_type &a0,
         const std::char_traits<wchar_t>::int_type &a1) {
  return std::char_traits<wchar_t>::eq_int_type(a0, a1);
}

std::char_traits<wchar_t>::int_type f32() {
  return std::char_traits<wchar_t>::eof();
}

std::char_traits<wchar_t>::int_type
f33(const std::char_traits<wchar_t>::int_type &a0) {
  return std::char_traits<wchar_t>::not_eof(a0);
}

std::char_traits<wchar_t> f34() { return std::char_traits<wchar_t>(); }

std::char_traits<wchar_t> f35(const std::char_traits<wchar_t> &a0) {
  return std::char_traits<wchar_t>(a0);
}

std::char_traits<wchar_t> f36(std::char_traits<wchar_t> &&a0) {
  return std::char_traits<wchar_t>(std::move(a0));
}

std::char_traits<wchar_t> &f37(std::char_traits<wchar_t> &a0,
                               const std::char_traits<wchar_t> &a1) {
  return a0.operator=(a1);
}

std::char_traits<wchar_t> &f38(std::char_traits<wchar_t> &a0,
                               std::char_traits<wchar_t> &&a1) {
  return a0.operator=(std::move(a1));
}

using t15 = std::char_traits<char16_t>;

using t16 = std::char_traits<char16_t>::char_type;

using t17 = std::char_traits<char16_t>::int_type;

using t18 = std::char_traits<char16_t>::off_type;

using t19 = std::char_traits<char16_t>::pos_type;

using t20 = std::char_traits<char16_t>::state_type;

using t21 = std::char_traits<char16_t>::comparison_category;

void f39(std::char_traits<char16_t>::char_type &a0,
         const std::char_traits<char16_t>::char_type &a1) {
  return std::char_traits<char16_t>::assign(a0, a1);
}

bool f40(const std::char_traits<char16_t>::char_type &a0,
         const std::char_traits<char16_t>::char_type &a1) {
  return std::char_traits<char16_t>::eq(a0, a1);
}

bool f41(const std::char_traits<char16_t>::char_type &a0,
         const std::char_traits<char16_t>::char_type &a1) {
  return std::char_traits<char16_t>::lt(a0, a1);
}

int f42(const std::char_traits<char16_t>::char_type *a0,
        const std::char_traits<char16_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char16_t>::compare(a0, a1, a2);
}

std::size_t f43(const std::char_traits<char16_t>::char_type *a0) {
  return std::char_traits<char16_t>::length(a0);
}

const std::char_traits<char16_t>::char_type *
f44(const std::char_traits<char16_t>::char_type *a0, std::size_t a1,
    const std::char_traits<char16_t>::char_type &a2) {
  return std::char_traits<char16_t>::find(a0, a1, a2);
}

std::char_traits<char16_t>::char_type *
f45(std::char_traits<char16_t>::char_type *a0,
    const std::char_traits<char16_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char16_t>::move(a0, a1, a2);
}

std::char_traits<char16_t>::char_type *
f46(std::char_traits<char16_t>::char_type *a0,
    const std::char_traits<char16_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char16_t>::copy(a0, a1, a2);
}

std::char_traits<char16_t>::char_type *
f47(std::char_traits<char16_t>::char_type *a0, std::size_t a1,
    std::char_traits<char16_t>::char_type a2) {
  return std::char_traits<char16_t>::assign(a0, a1, a2);
}

std::char_traits<char16_t>::char_type
f48(const std::char_traits<char16_t>::int_type &a0) {
  return std::char_traits<char16_t>::to_char_type(a0);
}

bool f49(const std::char_traits<char16_t>::int_type &a0,
         const std::char_traits<char16_t>::int_type &a1) {
  return std::char_traits<char16_t>::eq_int_type(a0, a1);
}

std::char_traits<char16_t>::int_type
f50(const std::char_traits<char16_t>::char_type &a0) {
  return std::char_traits<char16_t>::to_int_type(a0);
}

std::char_traits<char16_t>::int_type f51() {
  return std::char_traits<char16_t>::eof();
}

std::char_traits<char16_t>::int_type
f52(const std::char_traits<char16_t>::int_type &a0) {
  return std::char_traits<char16_t>::not_eof(a0);
}

std::char_traits<char16_t> f53() { return std::char_traits<char16_t>(); }

std::char_traits<char16_t> f54(const std::char_traits<char16_t> &a0) {
  return std::char_traits<char16_t>(a0);
}

std::char_traits<char16_t> f55(std::char_traits<char16_t> &&a0) {
  return std::char_traits<char16_t>(std::move(a0));
}

std::char_traits<char16_t> &f56(std::char_traits<char16_t> &a0,
                                const std::char_traits<char16_t> &a1) {
  return a0.operator=(a1);
}

std::char_traits<char16_t> &f57(std::char_traits<char16_t> &a0,
                                std::char_traits<char16_t> &&a1) {
  return a0.operator=(std::move(a1));
}

using t22 = std::char_traits<char8_t>;

using t23 = std::char_traits<char8_t>::char_type;

using t24 = std::char_traits<char8_t>::int_type;

using t25 = std::char_traits<char8_t>::pos_type;

using t26 = std::char_traits<char8_t>::off_type;

using t27 = std::char_traits<char8_t>::state_type;

using t28 = std::char_traits<char8_t>::comparison_category;

void f58(std::char_traits<char8_t>::char_type &a0,
         const std::char_traits<char8_t>::char_type &a1) {
  return std::char_traits<char8_t>::assign(a0, a1);
}

bool f59(const std::char_traits<char8_t>::char_type &a0,
         const std::char_traits<char8_t>::char_type &a1) {
  return std::char_traits<char8_t>::eq(a0, a1);
}

bool f60(const std::char_traits<char8_t>::char_type &a0,
         const std::char_traits<char8_t>::char_type &a1) {
  return std::char_traits<char8_t>::lt(a0, a1);
}

int f61(const std::char_traits<char8_t>::char_type *a0,
        const std::char_traits<char8_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char8_t>::compare(a0, a1, a2);
}

std::size_t f62(const std::char_traits<char8_t>::char_type *a0) {
  return std::char_traits<char8_t>::length(a0);
}

const std::char_traits<char8_t>::char_type *
f63(const std::char_traits<char8_t>::char_type *a0, std::size_t a1,
    const std::char_traits<char8_t>::char_type &a2) {
  return std::char_traits<char8_t>::find(a0, a1, a2);
}

std::char_traits<char8_t>::char_type *
f64(std::char_traits<char8_t>::char_type *a0,
    const std::char_traits<char8_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char8_t>::move(a0, a1, a2);
}

std::char_traits<char8_t>::char_type *
f65(std::char_traits<char8_t>::char_type *a0,
    const std::char_traits<char8_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char8_t>::copy(a0, a1, a2);
}

std::char_traits<char8_t>::char_type *
f66(std::char_traits<char8_t>::char_type *a0, std::size_t a1,
    std::char_traits<char8_t>::char_type a2) {
  return std::char_traits<char8_t>::assign(a0, a1, a2);
}

std::char_traits<char8_t>::char_type
f67(const std::char_traits<char8_t>::int_type &a0) {
  return std::char_traits<char8_t>::to_char_type(a0);
}

std::char_traits<char8_t>::int_type
f68(const std::char_traits<char8_t>::char_type &a0) {
  return std::char_traits<char8_t>::to_int_type(a0);
}

bool f69(const std::char_traits<char8_t>::int_type &a0,
         const std::char_traits<char8_t>::int_type &a1) {
  return std::char_traits<char8_t>::eq_int_type(a0, a1);
}

std::char_traits<char8_t>::int_type f70() {
  return std::char_traits<char8_t>::eof();
}

std::char_traits<char8_t>::int_type
f71(const std::char_traits<char8_t>::int_type &a0) {
  return std::char_traits<char8_t>::not_eof(a0);
}

std::char_traits<char8_t> f72() { return std::char_traits<char8_t>(); }

std::char_traits<char8_t> f73(const std::char_traits<char8_t> &a0) {
  return std::char_traits<char8_t>(a0);
}

std::char_traits<char8_t> f74(std::char_traits<char8_t> &&a0) {
  return std::char_traits<char8_t>(std::move(a0));
}

std::char_traits<char8_t> &f75(std::char_traits<char8_t> &a0,
                               const std::char_traits<char8_t> &a1) {
  return a0.operator=(a1);
}

std::char_traits<char8_t> &f76(std::char_traits<char8_t> &a0,
                               std::char_traits<char8_t> &&a1) {
  return a0.operator=(std::move(a1));
}

using t29 = std::char_traits<char32_t>;

using t30 = std::char_traits<char32_t>::char_type;

using t31 = std::char_traits<char32_t>::int_type;

using t32 = std::char_traits<char32_t>::off_type;

using t33 = std::char_traits<char32_t>::pos_type;

using t34 = std::char_traits<char32_t>::state_type;

using t35 = std::char_traits<char32_t>::comparison_category;

void f77(std::char_traits<char32_t>::char_type &a0,
         const std::char_traits<char32_t>::char_type &a1) {
  return std::char_traits<char32_t>::assign(a0, a1);
}

bool f78(const std::char_traits<char32_t>::char_type &a0,
         const std::char_traits<char32_t>::char_type &a1) {
  return std::char_traits<char32_t>::eq(a0, a1);
}

bool f79(const std::char_traits<char32_t>::char_type &a0,
         const std::char_traits<char32_t>::char_type &a1) {
  return std::char_traits<char32_t>::lt(a0, a1);
}

int f80(const std::char_traits<char32_t>::char_type *a0,
        const std::char_traits<char32_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char32_t>::compare(a0, a1, a2);
}

std::size_t f81(const std::char_traits<char32_t>::char_type *a0) {
  return std::char_traits<char32_t>::length(a0);
}

const std::char_traits<char32_t>::char_type *
f82(const std::char_traits<char32_t>::char_type *a0, std::size_t a1,
    const std::char_traits<char32_t>::char_type &a2) {
  return std::char_traits<char32_t>::find(a0, a1, a2);
}

std::char_traits<char32_t>::char_type *
f83(std::char_traits<char32_t>::char_type *a0,
    const std::char_traits<char32_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char32_t>::move(a0, a1, a2);
}

std::char_traits<char32_t>::char_type *
f84(std::char_traits<char32_t>::char_type *a0,
    const std::char_traits<char32_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char32_t>::copy(a0, a1, a2);
}

std::char_traits<char32_t>::char_type *
f85(std::char_traits<char32_t>::char_type *a0, std::size_t a1,
    std::char_traits<char32_t>::char_type a2) {
  return std::char_traits<char32_t>::assign(a0, a1, a2);
}

std::char_traits<char32_t>::char_type
f86(const std::char_traits<char32_t>::int_type &a0) {
  return std::char_traits<char32_t>::to_char_type(a0);
}

std::char_traits<char32_t>::int_type
f87(const std::char_traits<char32_t>::char_type &a0) {
  return std::char_traits<char32_t>::to_int_type(a0);
}

bool f88(const std::char_traits<char32_t>::int_type &a0,
         const std::char_traits<char32_t>::int_type &a1) {
  return std::char_traits<char32_t>::eq_int_type(a0, a1);
}

std::char_traits<char32_t>::int_type f89() {
  return std::char_traits<char32_t>::eof();
}

std::char_traits<char32_t>::int_type
f90(const std::char_traits<char32_t>::int_type &a0) {
  return std::char_traits<char32_t>::not_eof(a0);
}

std::char_traits<char32_t> f91() { return std::char_traits<char32_t>(); }

std::char_traits<char32_t> f92(const std::char_traits<char32_t> &a0) {
  return std::char_traits<char32_t>(a0);
}

std::char_traits<char32_t> f93(std::char_traits<char32_t> &&a0) {
  return std::char_traits<char32_t>(std::move(a0));
}

std::char_traits<char32_t> &f94(std::char_traits<char32_t> &a0,
                                const std::char_traits<char32_t> &a1) {
  return a0.operator=(a1);
}

std::char_traits<char32_t> &f95(std::char_traits<char32_t> &a0,
                                std::char_traits<char32_t> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t36 = std::char_traits<T1>;

template <typename T1> void f96(T1 &a0, T1 const &a1) {
  return std::char_traits<T1>::assign(a0, a1);
}

template <typename T1> bool f97(T1 const &a0, T1 const &a1) {
  return std::char_traits<T1>::eq(a0, a1);
}

template <typename T1> bool f98(T1 const &a0, T1 const &a1) {
  return std::char_traits<T1>::lt(a0, a1);
}

template <typename T1> int f99(T1 const *a0, T1 const *a1, std::size_t a2) {
  return std::char_traits<T1>::compare(a0, a1, a2);
}

template <typename T1> std::size_t f100(T1 const *a0) {
  return std::char_traits<T1>::length(a0);
}

template <typename T1>
T1 const *f101(T1 const *a0, std::size_t a1, T1 const &a2) {
  return std::char_traits<T1>::find(a0, a1, a2);
}

template <typename T1> T1 *f102(T1 *a0, T1 const *a1, std::size_t a2) {
  return std::char_traits<T1>::move(a0, a1, a2);
}

template <typename T1> T1 *f103(T1 *a0, T1 const *a1, std::size_t a2) {
  return std::char_traits<T1>::copy(a0, a1, a2);
}

template <typename T1> T1 *f104(T1 *a0, std::size_t a1, T1 a2) {
  return std::char_traits<T1>::assign(a0, a1, std::move(a2));
}

template <typename T1> T1 f105(const unsigned long &a0) {
  return std::char_traits<T1>::to_char_type(a0);
}

template <typename T1> unsigned long f106(T1 const &a0) {
  return std::char_traits<T1>::to_int_type(a0);
}

template <typename T1>
bool f107(const unsigned long &a0, const unsigned long &a1) {
  return std::char_traits<T1>::eq_int_type(a0, a1);
}

template <typename T1> unsigned long f108() {
  return std::char_traits<T1>::eof();
}

template <typename T1> unsigned long f109(const unsigned long &a0) {
  return std::char_traits<T1>::not_eof(a0);
}
