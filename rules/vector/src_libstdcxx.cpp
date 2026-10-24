#include <algorithm>
#include <utility>
#include <vector>

template <typename T1> std::vector<bool, T1> f59(std::size_t a0, const T1 &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1> f60(std::size_t a0, const bool &a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1>
void f61(std::vector<bool, T1> &a0, std::size_t a1, const bool &a2) {
  return a0.assign(a1, a2);
}

template <typename T1> std::_Bit_iterator f62(std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1>
std::_Bit_const_iterator f63(const std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1> std::_Bit_iterator f64(std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
std::_Bit_const_iterator f65(const std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
std::reverse_iterator<std::_Bit_iterator> f66(std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
std::reverse_iterator<std::_Bit_const_iterator>
f67(const std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
std::reverse_iterator<std::_Bit_iterator> f68(std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
std::reverse_iterator<std::_Bit_const_iterator>
f69(const std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
std::_Bit_const_iterator f70(const std::vector<bool, T1> &a0) {
  return a0.cbegin();
}

template <typename T1>
std::_Bit_const_iterator f71(const std::vector<bool, T1> &a0) {
  return a0.cend();
}

template <typename T1>
std::reverse_iterator<std::_Bit_const_iterator>
f72(const std::vector<bool, T1> &a0) {
  return a0.crbegin();
}

template <typename T1>
std::reverse_iterator<std::_Bit_const_iterator>
f73(const std::vector<bool, T1> &a0) {
  return a0.crend();
}

template <typename T1> std::size_t f74(const std::vector<bool, T1> &a0) {
  return a0.size();
}

template <typename T1> std::size_t f75(const std::vector<bool, T1> &a0) {
  return a0.max_size();
}

template <typename T1> std::size_t f76(const std::vector<bool, T1> &a0) {
  return a0.capacity();
}

template <typename T1>
std::_Bit_reference f77(std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
bool f78(const std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::_Bit_reference f79(std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.at(a1);
}

template <typename T1>
bool f80(const std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.at(a1);
}

template <typename T1> void f81(std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.reserve(a1);
}

template <typename T1> std::_Bit_reference f82(std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1> bool f83(const std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1> std::_Bit_reference f84(std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1> bool f85(const std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1> void f86(std::vector<bool, T1> &a0, bool a1) {
  return a0.push_back(a1);
}

template <typename T1>
void f87(std::_Bit_reference a0, std::_Bit_reference a1) {
  return std::vector<bool, T1>::swap(std::move(a0), std::move(a1));
}

template <typename T1>
std::_Bit_iterator f88(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       const bool &a2) {
  return a0.insert(std::move(a1), a2);
}

template <typename T1>
std::_Bit_iterator f89(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1) {
  return a0.insert(std::move(a1));
}

template <typename T1>
std::_Bit_iterator f90(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       std::size_t a2, const bool &a3) {
  return a0.insert(std::move(a1), a2, a3);
}

template <typename T1>
std::_Bit_iterator f91(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       std::initializer_list<bool> a2) {
  return a0.insert(std::move(a1), std::move(a2));
}

template <typename T1>
std::_Bit_iterator f92(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1) {
  return a0.erase(std::move(a1));
}

template <typename T1>
std::_Bit_iterator f93(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       std::_Bit_const_iterator a2) {
  return a0.erase(std::move(a1), std::move(a2));
}

template <typename T1>
void f94(std::vector<bool, T1> &a0, std::size_t a1, bool a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2, typename T3 = std::_RequireInputIter<T2>>
std::vector<bool, T1> f95(T2 a0, T2 a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3 = std::_RequireInputIter<T2>>
void f96(std::vector<bool, T1> &a0, T2 a1, T2 a2) {
  return a0.template assign<T2>(a1, a2);
}

template <typename T1, typename T2, typename T3 = std::_RequireInputIter<T2>>
std::_Bit_iterator f97(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       T2 a2, T2 a3) {
  return a0.template insert<T2>(std::move(a1), a2, a3);
}

template <typename T1, typename... T2>
std::_Bit_iterator f98(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                       T2 &&...a2) {
  return a0.template emplace<T2...>(std::move(a1), std::forward<T2>(a2)...);
}

template <typename T1, typename T2>
std::vector<T1, T2> f99(std::size_t a0, const T2 &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f100(std::size_t a0, const T1 &a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
void f101(std::vector<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
std::size_t f102(const std::vector<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
std::size_t f103(const std::vector<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f104(std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f105(std::vector<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2>
std::size_t f106(const std::vector<T1, T2> &a0) {
  return a0.capacity();
}

template <typename T1, typename T2>
void f107(std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f108(std::vector<T1, T2> &a0,
                                             std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f109(const std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f110(std::vector<T1, T2> &a0,
                                             std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f111(const std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f112(std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f113(const std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f114(std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f115(const std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f116(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     std::size_t a2, const T1 &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
std::vector<T1, T2> f117(T3 a0, T3 a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
void f118(std::vector<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
typename std::vector<T1, T2>::iterator
f119(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}

template <typename T1, typename T2>
using t39 = typename std::vector<T1, T2>::iterator::iterator_type;

template <typename T1, typename T2>
using t40 = typename std::vector<T1, T2>::const_iterator::iterator_type;
