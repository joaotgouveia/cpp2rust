#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f74(const T3 &a0) {
  return std::basic_string<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f75(const std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f76(const std::basic_string<T1, T2, T3> &a0,
                                  const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f77(std::basic_string<T1, T2, T3> &&a0,
                                  const T3 &a1) {
  return std::basic_string<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f78(std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f79(std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f80(std::basic_string<T1, T2, T3> &a0,
                                   std::initializer_list<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
void f81(std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1, T1 a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f82(const std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f83(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f84(const std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f85(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f86(std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f87(const std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::reference
f88(std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::const_reference
f89(const std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f90(std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f91(std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f92(std::basic_string<T1, T2, T3> &a0,
                                   std::initializer_list<T1> a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f93(std::basic_string<T1, T2, T3> &a0, const T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.append(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f94(std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0.append(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f95(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::size_type a1, T1 a2) {
  return a0.append(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f96(std::basic_string<T1, T2, T3> &a0,
                                   std::initializer_list<T1> a1) {
  return a0.append(a1);
}

template <typename T1, typename T2, typename T3>
void f97(std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.push_back(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f98(std::basic_string<T1, T2, T3> &a0, const T1 *a1,
    typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f99(std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f100(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, T1 a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &f101(std::basic_string<T1, T2, T3> &a0,
                                    std::initializer_list<T1> a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f102(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, T1 a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f103(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     std::initializer_list<T1> a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f104(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T1 *a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f105(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T1 *a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f106(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, T1 a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f107(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1, T1 a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f108(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T1 *a3,
     typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f109(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T1 *a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f110(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3, T1 a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f111(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, const T1 *a3,
     typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f112(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, const T1 *a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f113(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     typename std::basic_string<T1, T2, T3>::size_type a3, T1 a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f114(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, T1 *a3,
     T1 *a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f115(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, const T1 *a3,
     const T1 *a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f116(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     typename std::basic_string<T1, T2, T3>::iterator a3,
     typename std::basic_string<T1, T2, T3>::iterator a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f117(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     typename std::basic_string<T1, T2, T3>::const_iterator a3,
     typename std::basic_string<T1, T2, T3>::const_iterator a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f118(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     std::initializer_list<T1> a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f119(const std::basic_string<T1, T2, T3> &a0, T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.copy(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
const T1 *f120(const std::basic_string<T1, T2, T3> &a0) {
  return a0.c_str();
}

template <typename T1, typename T2, typename T3>
const T1 *f121(const std::basic_string<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f122(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f123(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f124(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f125(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.rfind(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f126(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f127(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f128(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_first_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f129(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f130(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f131(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_last_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f132(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f133(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f134(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_first_not_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f135(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f136(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f137(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_last_not_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f138(const std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f139(const std::basic_string<T1, T2, T3> &a0, T1 a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
int f140(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0.compare(a1);
}

template <typename T1, typename T2, typename T3>
int f141(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T1 *a3) {
  return a0.compare(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
int f142(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T1 *a3,
         typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.compare(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> f143(T4 a0, T4 a1, const T3 &a2) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
using t31 = typename std::basic_string<T1, T2, T3>::iterator::iterator_type;

template <typename T1, typename T2, typename T3>
using t32 =
    typename std::basic_string<T1, T2, T3>::const_iterator::iterator_type;

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f144(std::basic_istream<T1, T2> &&a0,
                                 std::basic_string<T1, T2, T3> &a1) {
  return std::getline<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f145(std::basic_istream<T1, T2> &&a0,
                                 std::basic_string<T1, T2, T3> &a1, T1 a2) {
  return std::getline<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f146(std::basic_istream<T1, T2> &a0,
                                 std::basic_string<T1, T2, T3> &a1) {
  return std::getline<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f147(std::basic_istream<T1, T2> &a0,
                                 std::basic_string<T1, T2, T3> &a1, T1 a2) {
  return std::getline<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f148(std::basic_string<T1, T2, T3> &&a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return std::move(a0) + a1;
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f149(std::basic_string<T1, T2, T3> &&a0,
                                   const T1 *a1) {
  return std::move(a0) + a1;
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f150(std::basic_string<T1, T2, T3> &&a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return std::move(a0) + std::move(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f151(std::basic_string<T1, T2, T3> &&a0, T1 a1) {
  return std::move(a0) + a1;
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f152(const std::basic_string<T1, T2, T3> &a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return a0 + std::move(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f153(const std::basic_string<T1, T2, T3> &a0,
                                   const T1 *a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f154(const std::basic_string<T1, T2, T3> &a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f155(const std::basic_string<T1, T2, T3> &a0,
                                   T1 a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f156(const T1 *a0,
                                   std::basic_string<T1, T2, T3> &&a1) {
  return a0 + std::move(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f157(const T1 *a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f158(T1 a0, std::basic_string<T1, T2, T3> &&a1) {
  return a0 + std::move(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f159(T1 a0,
                                   const std::basic_string<T1, T2, T3> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f160(std::basic_ostream<T1, T2> &a0,
                                 const std::basic_string<T1, T2, T3> &a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3>
bool f161(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3>
bool f162(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f163(std::basic_istream<T1, T2> &a0,
                                 std::basic_string<T1, T2, T3> &a1) {
  return a0 >> a1;
}

double f164(const std::string &a0, std::size_t *a1) {
  return std::stod(a0, a1);
}

double f165(const std::wstring &a0, std::size_t *a1) {
  return std::stod(a0, a1);
}

float f166(const std::string &a0, std::size_t *a1) { return std::stof(a0, a1); }

float f167(const std::wstring &a0, std::size_t *a1) {
  return std::stof(a0, a1);
}

int f168(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoi(a0, a1, a2);
}

int f169(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoi(a0, a1, a2);
}

long f170(const std::string &a0, std::size_t *a1, int a2) {
  return std::stol(a0, a1, a2);
}

long f171(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stol(a0, a1, a2);
}

long double f172(const std::string &a0, std::size_t *a1) {
  return std::stold(a0, a1);
}

long double f173(const std::wstring &a0, std::size_t *a1) {
  return std::stold(a0, a1);
}

long long f174(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoll(a0, a1, a2);
}

long long f175(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoll(a0, a1, a2);
}

unsigned long f176(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoul(a0, a1, a2);
}

unsigned long f177(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoul(a0, a1, a2);
}

unsigned long long f178(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoull(a0, a1, a2);
}

unsigned long long f179(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoull(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
void f180(std::basic_string<T1, T2, T3> &a0,
          std::basic_string<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}

std::string f181(long double a0) { return std::to_string(a0); }

std::string f182(int a0) { return std::to_string(a0); }

std::string f183(long long a0) { return std::to_string(a0); }

std::string f184(long a0) { return std::to_string(a0); }

std::string f185(double a0) { return std::to_string(a0); }

std::string f186(float a0) { return std::to_string(a0); }

std::string f187(unsigned int a0) { return std::to_string(a0); }

std::string f188(unsigned long long a0) { return std::to_string(a0); }

std::string f189(unsigned long a0) { return std::to_string(a0); }

std::wstring f190(long double a0) { return std::to_wstring(a0); }

std::wstring f191(int a0) { return std::to_wstring(a0); }

std::wstring f192(long long a0) { return std::to_wstring(a0); }

std::wstring f193(long a0) { return std::to_wstring(a0); }

std::wstring f194(double a0) { return std::to_wstring(a0); }

std::wstring f195(float a0) { return std::to_wstring(a0); }

std::wstring f196(unsigned int a0) { return std::to_wstring(a0); }

std::wstring f197(unsigned long long a0) { return std::to_wstring(a0); }

std::wstring f198(unsigned long a0) { return std::to_wstring(a0); }
