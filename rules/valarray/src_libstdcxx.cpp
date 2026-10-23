#include <algorithm>
#include <utility>
#include <valarray>

template <typename T1>
auto f15(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(a0 != a1) {
  return a0 != a1;
}

template <typename T1>
auto f16(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(a0 != a1) {
  return a0 != a1;
}

template <typename T1>
auto f17(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(a0 != a1) {
  return a0 != a1;
}

template <typename T1>
auto f18(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(a0 % a1) {
  return a0 % a1;
}

template <typename T1>
auto f19(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(a0 % a1) {
  return a0 % a1;
}

template <typename T1>
auto f20(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(a0 % a1) {
  return a0 % a1;
}

template <typename T1>
auto f21(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(a0 & a1) {
  return a0 & a1;
}

template <typename T1>
auto f22(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(a0 & a1) {
  return a0 & a1;
}

template <typename T1>
auto f23(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(a0 & a1) {
  return a0 & a1;
}

template <typename T1>
auto f24(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator&& <T1>(a0, a1)) {
  return std::operator&& <T1>(a0, a1);
}

template <typename T1>
auto f25(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator&& <T1>(a0, a1)) {
  return std::operator&& <T1>(a0, a1);
}

template <typename T1>
auto f26(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1)
    -> decltype(std::operator&& <T1>(a0, a1)) {
  return std::operator&& <T1>(a0, a1);
}

template <typename T1>
auto f27(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(a0 * a1) {
  return a0 * a1;
}

template <typename T1>
auto f28(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(a0 * a1) {
  return a0 * a1;
}

template <typename T1>
auto f29(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(a0 * a1) {
  return a0 * a1;
}

template <typename T1>
auto f30(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(a0 + a1) {
  return a0 + a1;
}

template <typename T1>
auto f31(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(a0 + a1) {
  return a0 + a1;
}

template <typename T1>
auto f32(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(a0 + a1) {
  return a0 + a1;
}

template <typename T1>
auto f33(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(a0 - a1) {
  return a0 - a1;
}

template <typename T1>
auto f34(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(a0 - a1) {
  return a0 - a1;
}

template <typename T1>
auto f35(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(a0 - a1) {
  return a0 - a1;
}

template <typename T1>
auto f36(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(a0 / a1) {
  return a0 / a1;
}

template <typename T1>
auto f37(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(a0 / a1) {
  return a0 / a1;
}

template <typename T1>
auto f38(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(a0 / a1) {
  return a0 / a1;
}

template <typename T1>
auto f39(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(a0 < a1) {
  return a0 < a1;
}

template <typename T1>
auto f40(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(a0 < a1) {
  return a0 < a1;
}

template <typename T1>
auto f41(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(a0 < a1) {
  return a0 < a1;
}

template <typename T1>
auto f42(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(a0 << a1) {
  return a0 << a1;
}

template <typename T1>
auto f43(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(a0 << a1) {
  return a0 << a1;
}

template <typename T1>
auto f44(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(a0 << a1) {
  return a0 << a1;
}

template <typename T1>
auto f45(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(a0 <= a1) {
  return a0 <= a1;
}

template <typename T1>
auto f46(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(a0 <= a1) {
  return a0 <= a1;
}

template <typename T1>
auto f47(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(a0 <= a1) {
  return a0 <= a1;
}

template <typename T1>
auto f48(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(a0 == a1) {
  return a0 == a1;
}

template <typename T1>
auto f49(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(a0 == a1) {
  return a0 == a1;
}

template <typename T1>
auto f50(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(a0 == a1) {
  return a0 == a1;
}

template <typename T1>
auto f51(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(a0 > a1) {
  return a0 > a1;
}

template <typename T1>
auto f52(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(a0 > a1) {
  return a0 > a1;
}

template <typename T1>
auto f53(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(a0 > a1) {
  return a0 > a1;
}

template <typename T1>
auto f54(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(a0 >= a1) {
  return a0 >= a1;
}

template <typename T1>
auto f55(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(a0 >= a1) {
  return a0 >= a1;
}

template <typename T1>
auto f56(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(a0 >= a1) {
  return a0 >= a1;
}

template <typename T1>
auto f57(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(a0 >> a1) {
  return a0 >> a1;
}

template <typename T1>
auto f58(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(a0 >> a1) {
  return a0 >> a1;
}

template <typename T1>
auto f59(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(a0 >> a1) {
  return a0 >> a1;
}

template <typename T1>
auto f60(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(a0 ^ a1) {
  return a0 ^ a1;
}

template <typename T1>
auto f61(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(a0 ^ a1) {
  return a0 ^ a1;
}

template <typename T1>
auto f62(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(a0 ^ a1) {
  return a0 ^ a1;
}

template <typename T1>
auto f63(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(a0 | a1) {
  return a0 | a1;
}

template <typename T1>
auto f64(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(a0 | a1) {
  return a0 | a1;
}

template <typename T1>
auto f65(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(a0 | a1) {
  return a0 | a1;
}

template <typename T1>
auto f66(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator|| <T1>(a0, a1)) {
  return std::operator|| <T1>(a0, a1);
}

template <typename T1>
auto f67(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator|| <T1>(a0, a1)) {
  return std::operator|| <T1>(a0, a1);
}

template <typename T1>
auto f68(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1)
    -> decltype(std::operator|| <T1>(a0, a1)) {
  return std::operator|| <T1>(a0, a1);
}

template <typename T1> std::valarray<T1> f69(const T1 &a0, std::size_t a1) {
  return std::valarray<T1>(a0, a1);
}

template <typename T1>
std::valarray<T1> f70(const T1 *__restrict a0, std::size_t a1) {
  return std::valarray<T1>(a0, a1);
}

template <typename T1> std::valarray<T1> f71(const std::slice_array<T1> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1> std::valarray<T1> f72(const std::gslice_array<T1> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1> std::valarray<T1> f73(const std::mask_array<T1> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1>
std::valarray<T1> f74(const std::indirect_array<T1> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1> std::valarray<T1> f75(std::initializer_list<T1> a0) {
  return std::valarray<T1>(a0);
}

template <typename T1>
std::valarray<T1> &f76(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f77(std::valarray<T1> &a0, const std::slice_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f78(std::valarray<T1> &a0, const std::gslice_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f79(std::valarray<T1> &a0, const std::mask_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f80(std::valarray<T1> &a0,
                       const std::indirect_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f81(std::valarray<T1> &a0, std::initializer_list<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 &f82(std::valarray<T1> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
const T1 &f83(const std::valarray<T1> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::slice_array<T1> f84(std::valarray<T1> &a0, std::slice a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::gslice_array<T1> f85(std::valarray<T1> &a0, const std::gslice &a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::valarray<T1> f86(const std::valarray<T1> &a0,
                      const std::valarray<bool> &a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::mask_array<T1> f87(std::valarray<T1> &a0, const std::valarray<bool> &a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::indirect_array<T1> f88(std::valarray<T1> &a0,
                            const std::valarray<std::size_t> &a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::valarray<T1> &f89(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
std::valarray<T1> &f90(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator/=(a1);
}

template <typename T1>
std::valarray<T1> &f91(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator%=(a1);
}

template <typename T1>
std::valarray<T1> &f92(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::valarray<T1> &f93(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::valarray<T1> &f94(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator^=(a1);
}

template <typename T1>
std::valarray<T1> &f95(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator&=(a1);
}

template <typename T1>
std::valarray<T1> &f96(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator|=(a1);
}

template <typename T1>
std::valarray<T1> &f97(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator<<=(a1);
}

template <typename T1>
std::valarray<T1> &f98(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator>>=(a1);
}

template <typename T1>
std::valarray<T1> &f99(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
std::valarray<T1> &f100(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator/=(a1);
}

template <typename T1>
std::valarray<T1> &f101(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator%=(a1);
}

template <typename T1>
std::valarray<T1> &f102(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::valarray<T1> &f103(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::valarray<T1> &f104(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator^=(a1);
}

template <typename T1>
std::valarray<T1> &f105(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator|=(a1);
}

template <typename T1>
std::valarray<T1> &f106(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator&=(a1);
}

template <typename T1>
std::valarray<T1> &f107(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator<<=(a1);
}

template <typename T1>
std::valarray<T1> &f108(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator>>=(a1);
}

template <typename T1> T1 f109(const std::valarray<T1> &a0) { return a0.sum(); }

template <typename T1> T1 f110(const std::valarray<T1> &a0) { return a0.min(); }

template <typename T1> T1 f111(const std::valarray<T1> &a0) { return a0.max(); }

template <typename T1> void f112(std::valarray<T1> &a0, std::size_t a1, T1 a2) {
  return a0.resize(a1, a2);
}
