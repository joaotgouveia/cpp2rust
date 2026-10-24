#include <algorithm>
#include <utility>
#include <vector>

template <typename T1> using t37 = typename std::vector<bool, T1>::reference;

template <typename T1> std::vector<bool, T1> f58(std::size_t a0, const T1 &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1> f59(std::size_t a0, const bool &a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1>
void f60(std::vector<bool, T1> &a0, std::size_t a1, const bool &a2) {
  return a0.assign(a1, a2);
}

template <typename T1> std::_Bit_iterator f61(std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1>
std::_Bit_const_iterator f62(const std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1> std::_Bit_iterator f63(std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
std::_Bit_const_iterator f64(const std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
std::reverse_iterator<std::_Bit_iterator> f65(std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
std::reverse_iterator<std::_Bit_const_iterator>
f66(const std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
std::reverse_iterator<std::_Bit_iterator> f67(std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
std::reverse_iterator<std::_Bit_const_iterator>
f68(const std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
std::_Bit_const_iterator f69(const std::vector<bool, T1> &a0) {
  return a0.cbegin();
}

template <typename T1>
std::_Bit_const_iterator f70(const std::vector<bool, T1> &a0) {
  return a0.cend();
}

template <typename T1>
std::reverse_iterator<std::_Bit_const_iterator>
f71(const std::vector<bool, T1> &a0) {
  return a0.crbegin();
}

template <typename T1>
std::reverse_iterator<std::_Bit_const_iterator>
f72(const std::vector<bool, T1> &a0) {
  return a0.crend();
}

template <typename T1> std::size_t f73(const std::vector<bool, T1> &a0) {
  return a0.size();
}

template <typename T1> std::size_t f74(const std::vector<bool, T1> &a0) {
  return a0.max_size();
}

template <typename T1> std::size_t f75(const std::vector<bool, T1> &a0) {
  return a0.capacity();
}

template <typename T1>
std::_Bit_reference f76(std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
bool f77(const std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::_Bit_reference f78(std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.at(a1);
}

template <typename T1>
bool f79(const std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.at(a1);
}

template <typename T1> void f80(std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.reserve(a1);
}

template <typename T1> std::_Bit_reference f81(std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1> bool f82(const std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1> std::_Bit_reference f83(std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1> bool f84(const std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1> void f85(std::vector<bool, T1> &a0, bool a1) {
  return a0.push_back(a1);
}

template <typename T1>
void f86(std::_Bit_reference a0, std::_Bit_reference a1) {
  return std::vector<bool, T1>::swap(std::move(a0), std::move(a1));
}

template <typename T1>
std::_Bit_iterator f87(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       const bool &a2) {
  return a0.insert(std::move(a1), a2);
}

template <typename T1>
std::_Bit_iterator f88(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1) {
  return a0.insert(std::move(a1));
}

template <typename T1>
std::_Bit_iterator f89(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       std::size_t a2, const bool &a3) {
  return a0.insert(std::move(a1), a2, a3);
}

template <typename T1>
std::_Bit_iterator f90(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       std::initializer_list<bool> a2) {
  return a0.insert(std::move(a1), std::move(a2));
}

template <typename T1>
std::_Bit_iterator f91(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1) {
  return a0.erase(std::move(a1));
}

template <typename T1>
std::_Bit_iterator f92(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       std::_Bit_const_iterator a2) {
  return a0.erase(std::move(a1), std::move(a2));
}

template <typename T1>
void f93(std::vector<bool, T1> &a0, std::size_t a1, bool a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2, typename T3 = std::_RequireInputIter<T2>>
std::vector<bool, T1> f94(T2 a0, T2 a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3 = std::_RequireInputIter<T2>>
void f95(std::vector<bool, T1> &a0, T2 a1, T2 a2) {
  return a0.template assign<T2>(a1, a2);
}

template <typename T1, typename T2, typename T3 = std::_RequireInputIter<T2>>
std::_Bit_iterator f96(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       T2 a2, T2 a3) {
  return a0.template insert<T2>(std::move(a1), a2, a3);
}

template <typename T1, typename... T2>
std::_Bit_iterator f97(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       T2 &&...a2) {
  return a0.template emplace<T2...>(std::move(a1), std::forward<T2>(a2)...);
}

template <typename T1>
typename std::vector<bool, T1>::reference
f98(const typename std::vector<bool, T1>::reference &a0) {
  return typename std::vector<bool, T1>::reference(a0);
}

template <typename T1>
bool f99(const typename std::vector<bool, T1>::reference &a0) {
  return static_cast<bool>(a0);
}

template <typename T1>
typename std::vector<bool, T1>::reference &
f100(typename std::vector<bool, T1>::reference &a0, bool a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference &
f101(typename std::vector<bool, T1>::reference &a0,
     const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator=(a1);
}

template <typename T1>
bool f102(const typename std::vector<bool, T1>::reference &a0,
          const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator==(a1);
}

template <typename T1>
bool f103(const typename std::vector<bool, T1>::reference &a0,
          const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator<(a1);
}

template <typename T1>
void f104(typename std::vector<bool, T1>::reference &a0) {
  return a0.flip();
}

template <typename T1>
void f105(bool &a0, typename std::vector<bool, T1>::reference a1) {
  return swap(a0, a1);
}

template <typename T1>
void f106(typename std::vector<bool, T1>::reference a0, bool &a1) {
  return swap(a0, a1);
}

template <typename T1>
void f107(typename std::vector<bool, T1>::reference a0,
          typename std::vector<bool, T1>::reference a1) {
  return swap(a0, a1);
}

template <typename T1>
using t38 = typename std::vector<bool, T1>::iterator::reference;

template <typename T1>
using t39 = typename std::vector<bool, T1>::iterator::iterator;

template <typename T1> typename std::vector<bool, T1>::iterator f108() {
  return typename std::vector<bool, T1>::iterator();
}

template <typename T1>
typename std::vector<bool, T1>::iterator f109(unsigned long *a0,
                                              unsigned int a1) {
  return typename std::vector<bool, T1>::iterator(a0, a1);
}

template <typename T1>
std::_Bit_reference f110(const typename std::vector<bool, T1>::iterator &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f111(typename std::vector<bool, T1>::iterator &a0) {
  return a0.operator++();
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f112(typename std::vector<bool, T1>::iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f113(typename std::vector<bool, T1>::iterator &a0) {
  return a0.operator--();
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f114(typename std::vector<bool, T1>::iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f115(typename std::vector<bool, T1>::iterator &a0, long a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f116(typename std::vector<bool, T1>::iterator &a0, long a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::_Bit_reference f117(const typename std::vector<bool, T1>::iterator &a0,
                         long a1) {
  return a0.operator[](a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f118(const typename std::vector<bool, T1>::iterator &a0) {
  return typename std::vector<bool, T1>::iterator(a0);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f119(typename std::vector<bool, T1>::iterator &&a0) {
  return typename std::vector<bool, T1>::iterator(std::move(a0));
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f120(typename std::vector<bool, T1>::iterator &a0,
     const typename std::vector<bool, T1>::iterator &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f121(typename std::vector<bool, T1>::iterator &a0,
     typename std::vector<bool, T1>::iterator &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f122(const typename std::vector<bool, T1>::iterator &a0, long a1) {
  return operator-(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f123(long a0, const typename std::vector<bool, T1>::iterator &a1) {
  return operator+(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f124(const typename std::vector<bool, T1>::iterator &a0, long a1) {
  return operator+(a0, a1);
}

template <typename T1>
using t40 = typename std::vector<bool, T1>::const_iterator::const_reference;

template <typename T1>
using t41 = typename std::vector<bool, T1>::const_iterator::const_iterator;

template <typename T1> typename std::vector<bool, T1>::const_iterator f125() {
  return typename std::vector<bool, T1>::const_iterator();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator f126(unsigned long *a0,
                                                    unsigned int a1) {
  return typename std::vector<bool, T1>::const_iterator(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f127(const std::_Bit_iterator &a0) {
  return typename std::vector<bool, T1>::const_iterator(a0);
}

template <typename T1>
bool f128(const typename std::vector<bool, T1>::const_iterator &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f129(typename std::vector<bool, T1>::const_iterator &a0) {
  return a0.operator++();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f130(typename std::vector<bool, T1>::const_iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f131(typename std::vector<bool, T1>::const_iterator &a0) {
  return a0.operator--();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f132(typename std::vector<bool, T1>::const_iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f133(typename std::vector<bool, T1>::const_iterator &a0, long a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f134(typename std::vector<bool, T1>::const_iterator &a0, long a1) {
  return a0.operator-=(a1);
}

template <typename T1>
bool f135(const typename std::vector<bool, T1>::const_iterator &a0, long a1) {
  return a0.operator[](a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f136(const typename std::vector<bool, T1>::const_iterator &a0) {
  return typename std::vector<bool, T1>::const_iterator(a0);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f137(typename std::vector<bool, T1>::const_iterator &&a0) {
  return typename std::vector<bool, T1>::const_iterator(std::move(a0));
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f138(typename std::vector<bool, T1>::const_iterator &a0,
     const typename std::vector<bool, T1>::const_iterator &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f139(typename std::vector<bool, T1>::const_iterator &a0,
     typename std::vector<bool, T1>::const_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f140(long a0, const typename std::vector<bool, T1>::const_iterator &a1) {
  return operator+(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f141(const typename std::vector<bool, T1>::const_iterator &a0, long a1) {
  return operator-(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f142(const typename std::vector<bool, T1>::const_iterator &a0, long a1) {
  return operator+(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f143(std::size_t a0, const T2 &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f144(std::size_t a0, const T1 &a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
void f145(std::vector<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
std::size_t f146(const std::vector<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
std::size_t f147(const std::vector<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f148(std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f149(std::vector<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2>
std::size_t f150(const std::vector<T1, T2> &a0) {
  return a0.capacity();
}

template <typename T1, typename T2>
void f151(std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f152(std::vector<T1, T2> &a0,
                                             std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f153(const std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f154(std::vector<T1, T2> &a0,
                                             std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f155(const std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f156(std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f157(const std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f158(std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f159(const std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f160(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     std::size_t a2, const T1 &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
std::vector<T1, T2> f161(T3 a0, T3 a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
void f162(std::vector<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
typename std::vector<T1, T2>::iterator
f163(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}

template <typename T1, typename T2>
using t42 = typename std::vector<T1, T2>::iterator::iterator_type;

template <typename T1, typename T2>
using t43 = typename std::vector<T1, T2>::const_iterator::iterator_type;
