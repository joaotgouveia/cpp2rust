#include <algorithm>
#include <utility>
#include <valarray>

template <typename T1>
auto f54(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator!= <T1>(a0, a1)) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
auto f55(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator!= <T1>(a0, a1)) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
auto f56(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1)
    -> decltype(std::operator!= <T1>(a0, a1)) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
auto f57(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator% <T1>(a0, a1)) {
  return std::operator% <T1>(a0, a1);
}

template <typename T1>
auto f58(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator% <T1>(a0, a1)) {
  return std::operator% <T1>(a0, a1);
}

template <typename T1>
auto f59(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator% <T1>(a0, a1)) {
  return std::operator% <T1>(a0, a1);
}

template <typename T1>
auto f60(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator& <T1>(a0, a1)) {
  return std::operator& <T1>(a0, a1);
}

template <typename T1>
auto f61(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator& <T1>(a0, a1)) {
  return std::operator& <T1>(a0, a1);
}

template <typename T1>
auto f62(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator& <T1>(a0, a1)) {
  return std::operator& <T1>(a0, a1);
}

template <typename T1>
auto f63(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator&& <T1>(a0, a1)) {
  return std::operator&& <T1>(a0, a1);
}

template <typename T1>
auto f64(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator&& <T1>(a0, a1)) {
  return std::operator&& <T1>(a0, a1);
}

template <typename T1>
auto f65(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1)
    -> decltype(std::operator&& <T1>(a0, a1)) {
  return std::operator&& <T1>(a0, a1);
}

template <typename T1>
auto f66(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator* <T1>(a0, a1)) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1>
auto f67(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator* <T1>(a0, a1)) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1>
auto f68(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator* <T1>(a0, a1)) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1>
auto f69(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator+ <T1>(a0, a1)) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1>
auto f70(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator+ <T1>(a0, a1)) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1>
auto f71(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator+ <T1>(a0, a1)) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1>
auto f72(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator- <T1>(a0, a1)) {
  return std::operator- <T1>(a0, a1);
}

template <typename T1>
auto f73(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator- <T1>(a0, a1)) {
  return std::operator- <T1>(a0, a1);
}

template <typename T1>
auto f74(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator- <T1>(a0, a1)) {
  return std::operator- <T1>(a0, a1);
}

template <typename T1>
auto f75(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator/ <T1>(a0, a1)) {
  return std::operator/ <T1>(a0, a1);
}

template <typename T1>
auto f76(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator/ <T1>(a0, a1)) {
  return std::operator/ <T1>(a0, a1);
}

template <typename T1>
auto f77(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator/ <T1>(a0, a1)) {
  return std::operator/ <T1>(a0, a1);
}

template <typename T1>
auto f78(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator< <T1>(a0, a1)) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1>
auto f79(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator< <T1>(a0, a1)) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1>
auto f80(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator< <T1>(a0, a1)) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1>
auto f81(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator<< <T1>(a0, a1)) {
  return std::operator<< <T1>(a0, a1);
}

template <typename T1>
auto f82(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator<< <T1>(a0, a1)) {
  return std::operator<< <T1>(a0, a1);
}

template <typename T1>
auto f83(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1)
    -> decltype(std::operator<< <T1>(a0, a1)) {
  return std::operator<< <T1>(a0, a1);
}

template <typename T1>
auto f84(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator<= <T1>(a0, a1)) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1>
auto f85(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator<= <T1>(a0, a1)) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1>
auto f86(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1)
    -> decltype(std::operator<= <T1>(a0, a1)) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1>
auto f87(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator== <T1>(a0, a1)) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1>
auto f88(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator== <T1>(a0, a1)) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1>
auto f89(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1)
    -> decltype(std::operator== <T1>(a0, a1)) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1>
auto f90(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator><T1>(a0, a1)) {
  return std::operator><T1>(a0, a1);
}

template <typename T1>
auto f91(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator><T1>(a0, a1)) {
  return std::operator><T1>(a0, a1);
}

template <typename T1>
auto f92(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator><T1>(a0, a1)) {
  return std::operator><T1>(a0, a1);
}

template <typename T1>
auto f93(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator>= <T1>(a0, a1)) {
  return std::operator>= <T1>(a0, a1);
}

template <typename T1>
auto f94(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator>= <T1>(a0, a1)) {
  return std::operator>= <T1>(a0, a1);
}

template <typename T1>
auto f95(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1)
    -> decltype(std::operator>= <T1>(a0, a1)) {
  return std::operator>= <T1>(a0, a1);
}

template <typename T1>
auto f96(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator>> <T1>(a0, a1)) {
  return std::operator>> <T1>(a0, a1);
}

template <typename T1>
auto f97(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator>> <T1>(a0, a1)) {
  return std::operator>> <T1>(a0, a1);
}

template <typename T1>
auto f98(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1)
    -> decltype(std::operator>> <T1>(a0, a1)) {
  return std::operator>> <T1>(a0, a1);
}

template <typename T1>
auto f99(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator^ <T1>(a0, a1)) {
  return std::operator^ <T1>(a0, a1);
}

template <typename T1>
auto f100(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator^ <T1>(a0, a1)) {
  return std::operator^ <T1>(a0, a1);
}

template <typename T1>
auto f101(const typename std::valarray<T1>::value_type &a0,
          const std::valarray<T1> &a1)
    -> decltype(std::operator^ <T1>(a0, a1)) {
  return std::operator^ <T1>(a0, a1);
}

template <typename T1>
auto f102(const std::valarray<T1> &a0,
          const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator| <T1>(a0, a1)) {
  return std::operator| <T1>(a0, a1);
}

template <typename T1>
auto f103(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator| <T1>(a0, a1)) {
  return std::operator| <T1>(a0, a1);
}

template <typename T1>
auto f104(const typename std::valarray<T1>::value_type &a0,
          const std::valarray<T1> &a1)
    -> decltype(std::operator| <T1>(a0, a1)) {
  return std::operator| <T1>(a0, a1);
}

template <typename T1>
auto f105(const std::valarray<T1> &a0,
          const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator|| <T1>(a0, a1)) {
  return std::operator|| <T1>(a0, a1);
}

template <typename T1>
auto f106(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator|| <T1>(a0, a1)) {
  return std::operator|| <T1>(a0, a1);
}

template <typename T1>
auto f107(const typename std::valarray<T1>::value_type &a0,
          const std::valarray<T1> &a1)
    -> decltype(std::operator|| <T1>(a0, a1)) {
  return std::operator|| <T1>(a0, a1);
}

template <typename T1>
std::valarray<T1> f108(const T1 *__restrict a0, std::size_t a1) {
  return std::valarray<T1>(a0, a1);
}

template <typename T1>
std::valarray<T1> f109(const std::valarray<T1> &a0,
                       const std::valarray<bool> &a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::valarray<T1> &f110(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
std::valarray<T1> &f111(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator/=(a1);
}

template <typename T1>
std::valarray<T1> &f112(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator%=(a1);
}

template <typename T1>
std::valarray<T1> &f113(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::valarray<T1> &f114(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::valarray<T1> &f115(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator^=(a1);
}

template <typename T1>
std::valarray<T1> &f116(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator|=(a1);
}

template <typename T1>
std::valarray<T1> &f117(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator&=(a1);
}

template <typename T1>
std::valarray<T1> &f118(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator<<=(a1);
}

template <typename T1>
std::valarray<T1> &f119(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator>>=(a1);
}

template <typename T1>
auto f120(const std::valarray<T1> &a0, T1(a1)(T1))
    -> decltype(a0.apply(std::forward<T1(T1)>(a1))) {
  return a0.apply(std::forward<T1(T1)>(a1));
}

template <typename T1>
auto f121(const std::valarray<T1> &a0, T1(a1)(const T1 &))
    -> decltype(a0.apply(std::forward<T1(const T1 &)>(a1))) {
  return a0.apply(std::forward<T1(const T1 &)>(a1));
}

template <typename T1, typename T2>
std::valarray<T1> f122(const std::_Expr<T2, T1> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1, typename T2>
std::valarray<T1> &f123(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f124(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator*= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f125(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator/= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f126(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator%= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f127(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator+= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f128(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator-= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f129(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator^= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f130(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator|= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f131(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator&= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f132(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator<<= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f133(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator>>= <T2>(a1);
}
