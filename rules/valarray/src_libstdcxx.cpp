#include <algorithm>
#include <utility>
#include <valarray>

template <typename T1>
auto f47(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator!= <T1>(a0, a1)) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
auto f48(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator!= <T1>(a0, a1)) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
auto f49(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1)
    -> decltype(std::operator!= <T1>(a0, a1)) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
auto f50(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator% <T1>(a0, a1)) {
  return std::operator% <T1>(a0, a1);
}

template <typename T1>
auto f51(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator% <T1>(a0, a1)) {
  return std::operator% <T1>(a0, a1);
}

template <typename T1>
auto f52(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator% <T1>(a0, a1)) {
  return std::operator% <T1>(a0, a1);
}

template <typename T1>
auto f53(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator& <T1>(a0, a1)) {
  return std::operator& <T1>(a0, a1);
}

template <typename T1>
auto f54(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator& <T1>(a0, a1)) {
  return std::operator& <T1>(a0, a1);
}

template <typename T1>
auto f55(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator& <T1>(a0, a1)) {
  return std::operator& <T1>(a0, a1);
}

template <typename T1>
auto f56(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator&& <T1>(a0, a1)) {
  return std::operator&& <T1>(a0, a1);
}

template <typename T1>
auto f57(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator&& <T1>(a0, a1)) {
  return std::operator&& <T1>(a0, a1);
}

template <typename T1>
auto f58(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1)
    -> decltype(std::operator&& <T1>(a0, a1)) {
  return std::operator&& <T1>(a0, a1);
}

template <typename T1>
auto f59(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator* <T1>(a0, a1)) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1>
auto f60(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator* <T1>(a0, a1)) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1>
auto f61(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator* <T1>(a0, a1)) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1>
auto f62(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator+ <T1>(a0, a1)) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1>
auto f63(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator+ <T1>(a0, a1)) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1>
auto f64(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator+ <T1>(a0, a1)) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1>
auto f65(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator- <T1>(a0, a1)) {
  return std::operator- <T1>(a0, a1);
}

template <typename T1>
auto f66(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator- <T1>(a0, a1)) {
  return std::operator- <T1>(a0, a1);
}

template <typename T1>
auto f67(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator- <T1>(a0, a1)) {
  return std::operator- <T1>(a0, a1);
}

template <typename T1>
auto f68(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator/ <T1>(a0, a1)) {
  return std::operator/ <T1>(a0, a1);
}

template <typename T1>
auto f69(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator/ <T1>(a0, a1)) {
  return std::operator/ <T1>(a0, a1);
}

template <typename T1>
auto f70(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator/ <T1>(a0, a1)) {
  return std::operator/ <T1>(a0, a1);
}

template <typename T1>
auto f71(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator< <T1>(a0, a1)) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1>
auto f72(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator< <T1>(a0, a1)) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1>
auto f73(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator< <T1>(a0, a1)) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1>
auto f74(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator<< <T1>(a0, a1)) {
  return std::operator<< <T1>(a0, a1);
}

template <typename T1>
auto f75(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator<< <T1>(a0, a1)) {
  return std::operator<< <T1>(a0, a1);
}

template <typename T1>
auto f76(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1)
    -> decltype(std::operator<< <T1>(a0, a1)) {
  return std::operator<< <T1>(a0, a1);
}

template <typename T1>
auto f77(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator<= <T1>(a0, a1)) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1>
auto f78(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator<= <T1>(a0, a1)) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1>
auto f79(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1)
    -> decltype(std::operator<= <T1>(a0, a1)) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1>
auto f80(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator== <T1>(a0, a1)) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1>
auto f81(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator== <T1>(a0, a1)) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1>
auto f82(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1)
    -> decltype(std::operator== <T1>(a0, a1)) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1>
auto f83(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator><T1>(a0, a1)) {
  return std::operator><T1>(a0, a1);
}

template <typename T1>
auto f84(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator><T1>(a0, a1)) {
  return std::operator><T1>(a0, a1);
}

template <typename T1>
auto f85(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator><T1>(a0, a1)) {
  return std::operator><T1>(a0, a1);
}

template <typename T1>
auto f86(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator>= <T1>(a0, a1)) {
  return std::operator>= <T1>(a0, a1);
}

template <typename T1>
auto f87(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator>= <T1>(a0, a1)) {
  return std::operator>= <T1>(a0, a1);
}

template <typename T1>
auto f88(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1)
    -> decltype(std::operator>= <T1>(a0, a1)) {
  return std::operator>= <T1>(a0, a1);
}

template <typename T1>
auto f89(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator>> <T1>(a0, a1)) {
  return std::operator>> <T1>(a0, a1);
}

template <typename T1>
auto f90(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator>> <T1>(a0, a1)) {
  return std::operator>> <T1>(a0, a1);
}

template <typename T1>
auto f91(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1)
    -> decltype(std::operator>> <T1>(a0, a1)) {
  return std::operator>> <T1>(a0, a1);
}

template <typename T1>
auto f92(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator^ <T1>(a0, a1)) {
  return std::operator^ <T1>(a0, a1);
}

template <typename T1>
auto f93(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator^ <T1>(a0, a1)) {
  return std::operator^ <T1>(a0, a1);
}

template <typename T1>
auto f94(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator^ <T1>(a0, a1)) {
  return std::operator^ <T1>(a0, a1);
}

template <typename T1>
auto f95(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator| <T1>(a0, a1)) {
  return std::operator| <T1>(a0, a1);
}

template <typename T1>
auto f96(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator| <T1>(a0, a1)) {
  return std::operator| <T1>(a0, a1);
}

template <typename T1>
auto f97(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::operator| <T1>(a0, a1)) {
  return std::operator| <T1>(a0, a1);
}

template <typename T1>
auto f98(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::operator|| <T1>(a0, a1)) {
  return std::operator|| <T1>(a0, a1);
}

template <typename T1>
auto f99(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::operator|| <T1>(a0, a1)) {
  return std::operator|| <T1>(a0, a1);
}

template <typename T1>
auto f100(const typename std::valarray<T1>::value_type &a0,
          const std::valarray<T1> &a1)
    -> decltype(std::operator|| <T1>(a0, a1)) {
  return std::operator|| <T1>(a0, a1);
}

template <typename T1>
std::valarray<T1> f101(const T1 *__restrict a0, std::size_t a1) {
  return std::valarray<T1>(a0, a1);
}

template <typename T1>
std::valarray<T1> f102(const std::valarray<T1> &a0,
                       const std::valarray<bool> &a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::valarray<T1> &f103(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
std::valarray<T1> &f104(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator/=(a1);
}

template <typename T1>
std::valarray<T1> &f105(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator%=(a1);
}

template <typename T1>
std::valarray<T1> &f106(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::valarray<T1> &f107(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::valarray<T1> &f108(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator^=(a1);
}

template <typename T1>
std::valarray<T1> &f109(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator|=(a1);
}

template <typename T1>
std::valarray<T1> &f110(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator&=(a1);
}

template <typename T1>
std::valarray<T1> &f111(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator<<=(a1);
}

template <typename T1>
std::valarray<T1> &f112(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator>>=(a1);
}
